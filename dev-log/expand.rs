#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
/// A stack-less Rust coroutine library under 100 LoC
/// https://blog.aloni.org/posts/a-stack-less-rust-coroutine-100-loc/
mod yielder {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Poll, Context};
    pub enum State {
        Halted,
        Running,
    }
    pub struct Fib {
        pub state: State,
    }
    impl Fib {
        pub fn waiter<'a>(&'a mut self) -> Waiter<'a> {
            Waiter { fib: self }
        }
    }
    pub struct Waiter<'a> {
        fib: &'a mut Fib,
    }
    impl<'a> Future for Waiter<'a> {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
            match self.fib.state {
                State::Halted => {
                    self.fib.state = State::Running;
                    Poll::Ready(())
                }
                State::Running => {
                    self.fib.state = State::Halted;
                    Poll::Pending
                }
            }
        }
    }
}
mod executor {
    use std::collections::VecDeque;
    use std::pin::Pin;
    use std::future::Future;
    use std::task::Context;
    use std::task::Poll;
    use crate::waker;
    use crate::yielder::{State, Fib};
    pub struct Executor {
        fibs: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
    }
    impl Executor {
        pub fn new() -> Self {
            Executor { fibs: VecDeque::new() }
        }
        pub fn push<C, F>(&mut self, closure: C)
        where
            F: Future<Output = ()> + 'static,
            C: FnOnce(Fib) -> F,
        {
            let fib = Fib { state: State::Running };
            self.fibs.push_back(Box::pin(closure(fib)));
        }
        pub fn run(&mut self) {
            let waker = waker::create();
            let mut context = Context::from_waker(&waker);
            while let Some(mut fib) = self.fibs.pop_front() {
                match fib.as_mut().poll(&mut context) {
                    Poll::Pending => {
                        self.fibs.push_back(fib);
                    }
                    Poll::Ready(()) => {}
                }
            }
        }
    }
}
mod waker {
    /// Null waker
    use std::task::{RawWaker, RawWakerVTable, Waker};
    pub fn create() -> Waker {
        unsafe { Waker::from_raw(RAW_WAKER) }
    }
    const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe fn clone(_: *const ()) -> RawWaker {
        RAW_WAKER
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
}
use crate::executor::Executor;
pub fn main() {
    let mut exec = Executor::new();
    for instance in 1..=3 {
        exec.push(move |mut fib| async move {
            {
                ::std::io::_print(format_args!("{0} A\n", instance));
            };
            fib.waiter().await;
            {
                ::std::io::_print(format_args!("{0} B\n", instance));
            };
            fib.waiter().await;
            {
                ::std::io::_print(format_args!("{0} C\n", instance));
            };
            fib.waiter().await;
            {
                ::std::io::_print(format_args!("{0} D\n", instance));
            };
        });
    }
    {
        ::std::io::_print(format_args!("Running\n"));
    };
    exec.run();
    {
        ::std::io::_print(format_args!("Done\n"));
    };
}