# embassy timer

## 1. 概述

[向老师给出的提示](./xyong向勇老师的提示.md)

## 2. 从 .await 到底层时间 Driver

一个简单的例子：

```rust
#[task]
async fn run() {
    loop {
        info!("tick");
        Timer::after_secs(1).await;
    }
}
```

`Timer::after_secs(1)` 返回一个 `Timer Future`：

```rust
impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded_once && self.expires_at <= Instant::now() {
            Poll::Ready(())
        } else {
            embassy_time_driver::schedule_wake(self.expires_at.as_ticks(), cx.waker());
            self.yielded_once = true;
            Poll::Pending
        }
    }
}
```

初始化时 `yielded_once` 为 false 因此这里进入 else 分支调用到时间驱动的 `schedule_wake` 函数。

## 3. embassy-rp

下面分析 embassy-rp 树莓派的代码，位于 [embassy-rp/src/time_driver.rs](https://github.com/hy-huang20/embassy/blob/main/embassy-rp/src/time_driver.rs)。

rp 提供了自己的时间驱动代码 `TimerDriver`，实现了以下 3 个非 trait 函数：

- `set_alarm()` 设置硬件下一次触发的时间 timestamp（硬件设置）并记录到 `TimerDriver.alarms.timestamp` 中（软件设置）。如果发现 timestamp 已过期则取消上述硬件/软件设置并返回 false 表示需要 `next_expiration()` 马上处理 timer queue 中的过期项，因此可以在 `TimerDriver::schedule_wake()` 中看到循环 `set_alarm()` 直到返回 true 的代码

- `check_alarm()` 重点是 `TimerDriver.alarms.timestamp` 过期就调用 `trigger_alarm()`。如果实际没到期而由硬件意外提前触发（可以去看 else 的注释）就重新设置一遍硬件。可以看到这个函数由 `#[interrupt]` 函数调用 

- `trigger_alarm()` 调用 `next_expiration()` 处理 timer queue 中所有到期 `Timer`，并循环 `set_alarm()` 设置下一次硬件中断

rp 自己的时间驱动代码 `TimerDriver`，按照 embassy 规范需要为其实现 `Driver` trait 并实现 `now()` 和 `schedule_wake()`，一个返回当前时间，一个登记在 at 时刻唤醒 waker 对应异步任务。

```rust
impl Driver for TimerDriver {
    fn now(&self) -> u64 {
        loop {
            let hi = TIMER.timerawh().read();
            let lo = TIMER.timerawl().read();
            let hi2 = TIMER.timerawh().read();
            if hi == hi2 {
                return (hi as u64) << 32 | (lo as u64);
            }
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        })
    }
}
```

`now()` 这么写是因为 64 位的时间被分成高低 32 位的两个寄存器，需要保证在读取低位时高位没有发生进位。

`schedule_wake()`
- 在临界区调用 timer queue 的 `Queue::schedule_wake()` 把 `Timer Future` 登记为 at 时唤醒。返回 true 说明当前 `Timer Future` 原不在 timer queue 中（则将 timer queue item 放入 timer queue）或者需要提前唤醒更新 `expires_at` （则更新 timer queue item）
- `Queue::next_expiration()` 检查 timer queue 中所有项若超时则 `wake_task()` 并踢出。返回值为下一次最近需要唤醒的时间。 

>代码中的 `Queue` 是 embassy-time-queue-utils/src/queue_integrated.rs 提供的 timer queue。注意和 embassy 的 RunQueue 进行区分。[timer queue 分析](https://github.com/hy-huang20/rust-os-learning/blob/ae72a74f489ec1d993a5b515e4119e7b8405d1bb/%E8%BF%87%E7%A8%8B%E8%AE%B0%E5%BD%95/rust/rust%E5%BC%82%E6%AD%A5/Embassy/queue.md)
>
>`wake_task()` 会对当前任务调用 `SyncExecutor::enqueue()`，和 [state.md](./state.md) 中的分析串联起来。