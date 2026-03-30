# embassy-executor 任务 State

## 1. State 在哪

`TaskPool<F, N>` 是一组 `N` 个 `TaskStorage<F>`，每个 `TaskStorage<F>` 是一个可容纳 1 个任务实例的槽位。

```rust
pub struct TaskStorage<F: Future + 'static> {
    raw: TaskHeader,
    future: UninitCell<F>, // Valid if STATE_SPAWNED
}
```

`State` 就是记录在 `TaskHeader` 中的，表示这个槽位中的任务 future 的状态。

## 2. State 含义

根据 `embassy/embassy-executor/src/raw/mod.rs` 中注释的状态转移过程：

```rust
/// A task can be in one of the following states:
///
/// - Not spawned: the task is ready to spawn.
/// - `SPAWNED`: the task is currently spawned and may be running.
/// - `RUN_ENQUEUED`: the task is enqueued to be polled. Note that the task may be `!SPAWNED`.
///    In this case, the `RUN_ENQUEUED` state will be cleared when the task is next polled, without
///    polling the task's future.
///
/// A task's complete life cycle is as follows:
///
/// ```text
/// ┌────────────┐   ┌────────────────────────┐
/// │Not spawned │◄─5┤Not spawned|Run enqueued│
/// │            ├6─►│                        │
/// └─────┬──────┘   └──────▲─────────────────┘
///       1                 │
///       │    ┌────────────┘
///       │    4
/// ┌─────▼────┴─────────┐
/// │Spawned|Run enqueued│
/// │                    │
/// └─────┬▲─────────────┘
///       2│
///       │3
/// ┌─────▼┴─────┐
/// │  Spawned   │
/// │            │
/// └────────────┘
/// ```
///
/// Transitions:
/// - 1: Task is spawned - `AvailableTask::claim -> Executor::spawn`
/// - 2: During poll - `RunQueue::dequeue_all -> State::run_dequeue`
/// - 3: Task wakes itself, waker wakes task, or task exits - `Waker::wake -> wake_task -> State::run_enqueue`
/// - 4: A run-queued task exits - `TaskStorage::poll -> Poll::Ready`
/// - 5: Task is dequeued. The task's future is not polled, because exiting the task replaces its `poll_fn`.
/// - 6: A task is waken when it is not spawned - `wake_task -> State::run_enqueue`
```

与状态定义与修改相关逻辑在 `embassy-executor/src/raw/state_atomics_arm.rs` 中。

```rust
pub(crate) const STATE_SPAWNED: u32 = 1 << 0;
pub(crate) const STATE_RUN_QUEUED: u32 = 1 << 8;

pub(crate) struct State {
    /// Task is spawned (has a future)
    spawned: AtomicBool,
    /// Task is in the executor run queue
    run_queued: AtomicBool,
    pad: AtomicBool,
    pad2: AtomicBool,
}
```

`Task` 的四种状态，为两个状态位按位或组合的结果：

|task state|State::spawned|State::run_queued|备注|
|---|---|---|---|
|``Not spawned``|false|false|初始状态：槽位为空没有正在运行的 future|
|``Spawned\|Run enqueued``|true|true|任务 future 存在且在 RunQueue 中等待下次 executor.poll() 执行 poll_fn|
|``Spawned``|true|false|任务 future 存在但不在 RunQueue 中所以不会马上被 poll|
|``Not spawned\|Run enqueued``|false|true|此时 poll_fn 已经被换成 poll_exited 了，任务已经执行结束了但还在 RunQueue 中等下次 executor.poll() 被清理|

状态修改函数。注意以下列出的状态修改函数只是单纯修改状态，没有直接执行 poll：

|状态转移|状态修改|备注|
|---|---|---|
|1|`State::spawn()`|spawn 一个任务，任务 future 在 TaskPool 中某个 TaskStorage 槽位中|
|2|`State::run_dequeue()`|后续把任务从 RunQueue 中拿出 poll 执行 poll_fn|
|3|`State::run_enqueue()`|任务被 wake 重新放入 RunQueue 等待下一次 poll 执行|
|4|`State::despawn()`|任务执行完成后等待下一次 poll 被清出 RunQueue|
|5|`State::run_dequeue()`|后续把结束任务从 RunQueue 中拿出 poll 执行空的 poll_exited|
|6|`State::run_enqueue()`|已经退出的任务收到迟到的 wake 被短暂地重新放进 RunQueue，等下一轮 poll 时清出 RunQueue|

## 3. 补充

### 初始化

任务首次 spawn 的时候设置 `poll_fn` 并将 `TaskStorage::future` 设置为任务 future：`宏展开代码 -> TaskPool::_spawn_async_fn() -> TaskPool::spawn_impl() -> AvailableTask::initialize_impl()`

```rust
impl<F: Future + 'static> AvailableTask<F> {
    fn initialize_impl<S>(self, future: impl FnOnce() -> F) -> SpawnToken<S> {
        unsafe {
            self.task.raw.poll_fn.set(Some(TaskStorage::<F>::poll));
            self.task.future.write_in_place(future);

            let task = TaskRef::new(self.task);

            SpawnToken::new(task)
        }
    }
}
```

上面的 `self.task` 即为对应 `TaskStorage` 的引用：

```rust
pub struct AvailableTask<F: Future + 'static> {
    task: &'static TaskStorage<F>,
}
```

### 清理

主要的清理工作在第 `4` 步中执行了：`TaskStorage::poll -> Poll::Ready`：

```rust
unsafe fn poll(p: TaskRef) {
    match future.poll(&mut cx) {
        Poll::Ready(_) => {
            // As the future has finished and this function will not be called
            // again, we can safely drop the future here.
            this.future.drop_in_place();

            // We replace the poll_fn with a despawn function, so that the task is cleaned up
            // when the executor polls it next.
            this.raw.poll_fn.set(Some(poll_exited));

            // Make sure we despawn last, so that other threads can only spawn the task
            // after we're done with it.
            this.raw.state.despawn();
        }
        Poll::Pending => {}
    }
}
```

### __pender()

`SyncExecutor::enqueue()` 当 `RunQueue` 出现从无到有的情况时便会调用 `__pender()` 函数：

```rust
unsafe fn enqueue(&self, task: TaskRef, l: state::Token) {
    if self.run_queue.enqueue(task, l) {
        self.pender.pend();
    }
}
```

`SyncExecutor::enqueue()` 被调用出现在两个地方：（函数按照列出顺序被调用）

- 任务首次 spawn（步骤 `1`）
    - `SyncExecutor::spawn()`
    - `SyncExecutor::enqueue()`
    - `pender.pend()`
    - `__pender()`
- 任务被 wake 时（步骤 `3`）
    - `Waker::wake()`
    - `wake_task(task)`
    - `header.state.run_enqueue(...)`
    - `executor.enqueue(task, l)`
    - `pender.pend()`
    - `__pender()`

### 执行 poll

这里仅以 [cortex_m](https://github.com/hy-huang20/rust-os-learning/blob/main/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/executor/arch/cortex_m.md) 实现中最简单的线程模式 thread mode 举例：

- `__pender()` 执行 `asm!("sev")`
- `thread::Executor::run()` loop 中 `asm!("wfe")` 唤醒继续执行
- loop 执行到 `Executor::poll()`
- `SyncExecutor::poll()`

```rust
impl SyncExecutor {
    pub(crate) unsafe fn poll(&'static self) {
        self.run_queue.dequeue_all(|p| {
            let task = p.header();
            // Run the task
            task.poll_fn.get().unwrap_unchecked()(p);
        });
    }
}
```

这里的 `poll_fn` 如果是任务初次 spawn 时设置的 `TaskStorage::<F>::poll` 便会去 poll 存放在 `TaskStorage` 中的任务 future（即任务 async 函数那个 non-leaf future）。

