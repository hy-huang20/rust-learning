# tokio async crawler

使用 rust tokio 库实现的一个异步爬虫程序

在 [已有代码](https://gitee.com/taoqi-cat/asyn/tree/master/spider) 的基础上改进

## 运行

```
cargo run --example xxx
```

其中 ``xxx`` 可以是 ``examples/`` 下任意 ``.rs`` 文件不带后缀的文件名

## 关于 Rust tokio 库

TODO

## 原代码

### 顺序等待

已有代码使用了简单的顺序等待逻辑，这导致并发性的丧失：

```rust
let mut handles = Vec::with_capacity(10);
for url in url_vec {
    handles.push(tokio::spawn(handle_url(url)));
}
for handle in handles {
    handle.await; // 而且每个 handle 必须执行完才能执行下一个 handle
}
```

### 改进一：使用 join_all

```rust
use futures::future::join_all;

let handles = url_vec.into_iter().map(|url| tokio::spawn(handle_url(url))).collect::<Vec<_>>();
let results = join_all(handles).await;
```

``join_all`` 会并发等待所有任务完成，而不是顺序等待

### 改进二：使用 FuturesUnordered

```rust
use futures::stream::FuturesUnordered;
use futures::StreamExt;

let mut tasks = FuturesUnordered::new();
for url in url_vec {
    tasks.push(tokio::spawn(handle_url(url)));
}

while let Some(result) = tasks.next().await {
    // 处理每个完成的任务
}
```

``FuturesUnordered`` 提供了流式接口，可以按**任务完成的顺序**处理结果

### 改进三：自定义 Executor

``Tokio`` 的 ``spawn`` 已经使用了 ``Waker`` 机制，它会自动在任务可以继续时唤醒它们。自己实现 ``Executor`` 确实可以完全控制调度策略，但这通常比较复杂，除非你有特殊需求（如特定优先级调度、工作窃取等），否则不建议这样做。