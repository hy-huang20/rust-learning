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