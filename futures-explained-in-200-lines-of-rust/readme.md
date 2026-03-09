# futures explained in 200 lines of rust

## 1. [英文原版](https://web.archive.org/web/20230203001355/https://cfsamson.github.io/books-futures-explained/introduction.html)，[翻译版](https://nkbai.github.io/rust/Futures_Explained_in_200_lines_of_Rust.html)

## 2. 内容

### 2.1. 背景

以下特性仅基于文章中给出的例子总结：

||上下文切换|运行时特权级|代码读/写/使用|跨平台性|内存|时间|
|---|---|---|---|---|---|---|
|`OS 线程`|有|内核态，OS 调度|简单易懂易用|某些系统可能不支持线程|OS 线程栈相当大|系统调用昂贵，上下文切换，且不一定能按你想的那样快速切换回来|
|`有栈协程`（`绿色线程`）|有|用户态运行时|简单易懂易用|跨平台很难正确实现|线程栈大小可以自己定，但固定后的动态伸缩不易实现且有开销|与 OS 线程相比轻量级上下文切换；不一定能按你想的那样快速切换回来|
|`回调`|无|用户态运行时|`回调地狱`阅读困难；将别的逻辑重写为回调困难|大多数语言中易于实现，不过在 Rust 中难于处理任务间状态共享问题|相对较低，内存使用随回调数量线性增长|一般比线程快|
|`Javascript Promise`|无|用户态，`Javascript` 事件循环|解决了`回调`的代码复杂问题|得益于 `Javascript` 优秀的跨平台性|和`回调`一样的内存线性增长|一般比线程快|
|`Rust Future`（`无栈协程`）|无|用户态运行时|和其它语言不同，需要开发者主动提供一个运行时或者使用运行时库|`Future/async/await` 等核心逻辑不依赖于具体平台，与依赖具体平台的运行时解耦|保存在堆上（一般在堆上）的 `future` 结构体，一个 `future` 的大小为其对应的单个任务内部生命周期中最占空间的状态大小|一般比线程快|

### 2.2. 叶子和非叶子 future

- `leaf-future`：在整个异步调用的调用栈中，处于最底层的 `Future`。它内部绝对不会使用 `.await` 去等待其他 `Future`

- `non-leaf-future`：由 `async fn` 或 `async {}` 块编译自动生成的 `Future`。它内部会使用 `.await` 去等待其他的 `Future`（可能是叶子，也可能是其他的非叶子）

### 2.3. 运行时

Rust 的运行时分成 `Executor` 和 `Reactor` 两部分，两部分通过 `Waker` 进行交互。

`Reactor` 负责监听外部事件，并通过 `Waker` 通知 `Executor`。在 os 中可以把它理解为中断处理函数 `trap_handler`。

`Executor` 维护一个 `non-leaf-future` 的就绪队列，收到通知就不断从队列中拿任务调用它们的 `poll()`，返回 `Poll::Pending` 就放回队列，返回 `Poll::Ready` 就踢出队列。队列空了就让出 CPU。

### 2.4. 唤醒器 Waker 和上下文 Context

创建一个 `Waker` 需要创建一个 `vtable`，这个 `vtable` 允许我们使用动态方式调用我们真实的 `Waker` 实现。

### 2.5. 生成器和 async/await

Rust 中的异步使用生成器实现。因此为了理解异步是如何工作的，我们首先需要理解生成器。在 Rust 中，生成器被实现为状态机。

一个计算链的内存占用是由占用空间最大的那个步骤定义的。

这块内容以及后面的 `Pin` 和 Writing an OS in Rust 文章内容重合了。

### 2.6. 完整的例子

见 `src/main.rs`：

- 执行器 Executor
    - `block_on`：在一个 loop 里不断调用 `future.poll()`
    - `Parker`：基于 `Mutex` 和 `Condvar`
- 反应器 Reactor
    - `Reactor`
- leaf-future
    - `Task`
- 唤醒器 Waker
    - `MyWaker`
    - `VTABLE`
