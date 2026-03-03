/// A stack-less Rust coroutine library under 100 LoC
/// https://blog.aloni.org/posts/a-stack-less-rust-coroutine-100-loc/
mod yielder;
mod executor;
mod waker;

use crate::executor::Executor;

pub fn main() {
    let mut exec = Executor::new();

    for instance in 1..=3 {
        exec.push(move |mut fib| async move {
            println!("{} A", instance);
            fib.waiter().await;
            println!("{} B", instance);
            fib.waiter().await;
            println!("{} C", instance);
            fib.waiter().await;
            println!("{} D", instance);
        });
    }

    println!("Running");
    exec.run();
    println!("Done");
}

