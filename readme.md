# A stack-less Rust coroutine library under 100 LoC

## 1. 概述

### 1.1. [文章链接](https://blog.aloni.org/posts/a-stack-less-rust-coroutine-100-loc/)

### 1.2. 无栈协程和 rCore 线程模型的比较

||无栈协程|rCore 线程模型|
|---|---|---|
|栈的使用|没有自己独立的栈。谁调用 `future.poll(cx)` 就是在用谁的栈|拥有独立的用户/内核栈|
|上下文存放在哪里|一般在用户堆上|`TaskContext` 存放在 os 中由 `HEAP_ALLOCATOR` 管理的 `HEAP_SPACE` 区域|
|上下文内容|跨 `.await` 的局部变量**实体**|`TaskContext` struct 中记录了需要保存的**寄存器**的值。局部变量仍然保存在线程用户栈/内核栈（比如当前线程阻塞在系统调用时的线程切换）上|
|内存开销|在堆上存放 future 记录当前状态和局部变量|<ul><li>在堆上存放 `TaskContext`</li><li>在线程的整个生命周期维持其用户/内核栈</li></ul>|
|时间开销|局部变量从栈拷贝到堆上的 future 中的开销|<ul><li>上下文在堆与寄存器之间的拷贝开销</li><li>特权级切换</li><li>页表切换以及 TLB 刷新</li></ul>|

>#### 关于无栈协程的时间开销
>
>来看一段 [Writing an OS in Rust: Async/Await](https://os.phil-opp.com/async-await/#the-full-state-machine-type) 中的代码：
>
>```rust
>ExampleStateMachine::Start(state) => {
>    // from body of `example`
>    let foo_txt_future = async_read_file("foo.txt");
>    // `.await` operation
>    let state = WaitingOnFooTxtState {
>        min_len: state.min_len, // 直接访存堆上的变量
>        foo_txt_future,
>    };
>    *self = ExampleStateMachine::WaitingOnFooTxt(state); // 栈上变量拷贝到堆上
>}
>```

## 2. 代码逻辑

最简单的实现。没有实现 `waker.wake()` 函数的具体逻辑而是置空：

```rust
// src/waker.rs

unsafe fn wake(_: *const ()) { }
```