# tokio async crawler

使用 rust tokio 库实现的一个异步爬虫程序

在 [已有代码](https://gitee.com/taoqi-cat/asyn/tree/master/spider) 的基础上改进

## 运行

```
cargo run
```

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

运行时间：

```
cost time is 5.714586173s
```

### 改进一：使用 join_all

```rust
use futures::future::join_all;

let handles = url_vec.into_iter().map(|url| tokio::spawn(handle_url(url))).collect::<Vec<_>>();
let results = join_all(handles).await;
```

``join_all`` 会并发等待所有任务完成，而不是顺序等待

运行时间：

```
cost time is 5.685361608s
```

### 改进二：使用 FuturesUnordered

### 改进三：自定义 Executor