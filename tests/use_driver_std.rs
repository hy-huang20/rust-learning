use embassy_executor::{Spawner, task, main};
use embassy_time::{Timer};

///
/// 预期结果：运行时命令行输出 driver std
/// 
/// 测试在 wsl 上使用 embassy 使用的 time driver
/// 是否是 driver_std
/// 
/// 位于 embassy-time/src/driver_std.rs
/// 
/// driver_std 实现了自己的 TimeDriver 类型并且
/// 为其实现了 Driver trait。根据之前的代码分析，
/// 调用者会调用自己实现的 time driver 类型中的
/// schedule_wake() 函数。所以在 
/// TimerDriver::schedule_wake() 的第一行添加
/// 输出信息：
/// 
/// fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
///     println!("driver std");
///     let mut inner = self.inner.lock().unwrap();
///     inner.init();
///     if inner.queue.schedule_wake(at, waker) {
///         self.signaler.signal();
///     }
/// }
/// 
/// 由于需要查看输出结果，所以运行 test 时需要加上 
/// 参数 -- --nocapture 以显示输出结果
/// 因为默认情况下，Cargo 会捕获所有测试中的标准输出
/// 

#[task]
async fn run() {
    loop {
        Timer::after_secs(1).await;
    }
}

#[main]
#[test]
async fn main(spawner: Spawner) {
    spawner.spawn(run()).unwrap();
}