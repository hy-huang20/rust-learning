# embassy 主要数据结构之间的关联

## 1. waker 如何找 task

`Waker` struct 中有且仅有一个 `RawWaker`：

```rust
pub struct RawWaker {
    data: *const (),
    vtable: &'static RawWakerVTable,
}
```

根据 embassy-executor/src/raw/waker.rs 的代码，这个 `RawWaker` 的 data 字段会被设置为一个 `TaskHeader` 指针；vtable 字段设置的虚函数表，当调用 `Waker::wake()` 时会调用 `wake_task()`

### 类型相互转换

注意 waker.rs 中的 2 个类型转换函数：

```rust
pub(crate) unsafe fn from_task(p: TaskRef) -> Waker {
    Waker::from_raw(RawWaker::new(p.as_ptr() as _, &VTABLE))
}

/// Get a task pointer from a waker.
///
/// This can be used as an optimization in wait queues to store task pointers
/// (1 word) instead of full Wakers (2 words). This saves a bit of RAM and helps
/// avoid dynamic dispatch.
///
/// You can use the returned task pointer to wake the task with [`wake_task`].
///
/// # Panics
///
/// Panics if the waker is not created by the Embassy executor.
pub fn task_from_waker(waker: &Waker) -> TaskRef {
    // make sure to compare vtable addresses. Doing `==` on the references
    // will compare the contents, which is slower.
    if waker.vtable() as *const _ != &VTABLE as *const _ {
        panic!("Found waker not created by the Embassy executor. `embassy_time::Timer` only works with the Embassy executor.")
    }
    // safety: our wakers are always created with `TaskRef::as_ptr`
    unsafe { TaskRef::from_ptr(waker.data() as *const TaskHeader) }
}
```

`from_task()` 参数 `TaskRef` 本质上就是一个 `TaskHeader` **指针**：

```rust
pub struct TaskRef {
    ptr: NonNull<TaskHeader>,
}
```

关于 `task_from_waker()`，使用 `waker.data()` 获取存放在 `RawWaker` 中的 data 字段的 `TaskHeader` 指针然后包装成 `TaskRef`。

`TaskHeader` 存放了任务的很多元信息：

```rust
pub(crate) struct TaskHeader {
    pub(crate) state: State,
    pub(crate) run_queue_item: RunQueueItem,
    pub(crate) executor: AtomicPtr<SyncExecutor>,
    poll_fn: SyncUnsafeCell<Option<unsafe fn(TaskRef)>>,
    pub(crate) timer_queue_item: timer_queue::TimerQueueItem,
}
```

## 2. 用途

下面介绍的两个链表都不是额外分配链表节点，而是复用每个任务自己的 `TaskHeader` 相应的 item 字段。

### 2.1. timer queue

timer queue 本质上是一个链表：

```rust
pub struct Queue {
    head: Cell<Option<TaskRef>>,
}
```

`Queue` 中只存放了链表的头节点 head，是一个 `TaskRef`。

**访问链表下一个节点**：访问 `TaskRef` 相当于访问 `TaskHeader`，然后访问 `TaskHeader::timer_queue_item`：

```rust
pub struct TimerQueueItem {
    pub next: Cell<Option<TaskRef>>,
    pub expires_at: Cell<u64>,
}
```

`timer_queue_item.next` 即是 timer queue 中的下一项，即下一个 `TaskRef`。

### 2.2. RunQueue

`RunQueue` 本质上也是一个链表，其中存放链表的头节点：

```rust
pub(crate) struct RunQueue {
    head: AtomicPtr<TaskHeader>,
}
```

**访问下一个链表项**：方法也差不多，`TaskRef::header().run_queue_item.next`。

### 2.3. waker 在 timer 的传递、转换、存放

在 [embassy timer 的分析](./timer.md)中，对 `Timer Future` 进行 poll 调用底层时间驱动的 `embassy_time_driver::schedule_wake()` 函数时，需要传递一个 `cx.waker()` 参数。这个 waker 在 embassy-rp 的 time_driver.rs 实现中被传给 timer queue `Queue::schedule_wake()` 并在其中通过上述的 `embassy_executor::raw::task_from_waker()` 转换成为 `TaskRef`，从而视情况将其插入 timer queue，或者在 timer queue 中找到它并更新对应属性。也就是将 waker.data() 指针存放在 timer queue 链表中。

在 timer queue `Queue::next_expiration()` 中调用的 `wake_task()` 需要接受一个 `TaskRef` 参数以知道要唤醒的是哪个任务。因为 timer queue 链表本来就是一系列的 `TaskRef`，所以遍历即可。

