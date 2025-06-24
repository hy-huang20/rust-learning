use async_await::task::{Task, simple_executor::SimpleExecutor};

fn main() {
    println!("Hello World!");

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

    println!("It did not crash!");
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}