use embassy_executor::{Spawner, task, main};

/// 预期结果：编译错误，提示 task functions must be async
/// 
/// embassy-executor-macros 中的 task 函数是平台无关的

#[task]
fn run() {
    // task 函数体置空
}

#[main]
async fn main(spawner: Spawner) {
    spawner.spawn(run()).unwrap();
}