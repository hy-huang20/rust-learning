#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use embassy_executor::{Spawner, task, main};
use embassy_time::{Timer, Duration};
use log::*;
#[doc(hidden)]
async fn __run_task() {
    loop {
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        { ::log::__private_api::GlobalLogger },
                        format_args!("tick"),
                        lvl,
                        &(
                            "embassy_learning",
                            "embassy_learning",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            }
        };
        Timer::after_secs(1).await;
    }
}
fn run() -> ::embassy_executor::SpawnToken<impl Sized> {
    const fn __task_pool_get<F, Args, Fut>(
        _: F,
    ) -> &'static ::embassy_executor::raw::TaskPool<Fut, POOL_SIZE>
    where
        F: ::embassy_executor::_export::TaskFn<Args, Fut = Fut>,
        Fut: ::core::future::Future + 'static,
    {
        unsafe { &*POOL.get().cast() }
    }
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolHolder<
        {
            ::embassy_executor::_export::task_pool_size::<_, _, _, POOL_SIZE>(__run_task)
        },
        {
            ::embassy_executor::_export::task_pool_align::<
                _,
                _,
                _,
                POOL_SIZE,
            >(__run_task)
        },
    > = unsafe {
        ::core::mem::transmute(
            ::embassy_executor::_export::task_pool_new::<_, _, _, POOL_SIZE>(__run_task),
        )
    };
    unsafe { __task_pool_get(__run_task)._spawn_async_fn(move || __run_task()) }
}
#[doc(hidden)]
/// Task that ticks periodically
async fn __tick_periodic_task() -> ! {
    loop {
        {
            ::std::io::_print(format_args!("tick 1!\n"));
        };
        Timer::after(Duration::from_millis(500)).await;
    }
}
/// Task that ticks periodically
fn tick_periodic() -> ::embassy_executor::SpawnToken<impl Sized> {
    const fn __task_pool_get<F, Args, Fut>(
        _: F,
    ) -> &'static ::embassy_executor::raw::TaskPool<Fut, POOL_SIZE>
    where
        F: ::embassy_executor::_export::TaskFn<Args, Fut = Fut>,
        Fut: ::core::future::Future + 'static,
    {
        unsafe { &*POOL.get().cast() }
    }
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolHolder<
        {
            ::embassy_executor::_export::task_pool_size::<
                _,
                _,
                _,
                POOL_SIZE,
            >(__tick_periodic_task)
        },
        {
            ::embassy_executor::_export::task_pool_align::<
                _,
                _,
                _,
                POOL_SIZE,
            >(__tick_periodic_task)
        },
    > = unsafe {
        ::core::mem::transmute(
            ::embassy_executor::_export::task_pool_new::<
                _,
                _,
                _,
                POOL_SIZE,
            >(__tick_periodic_task),
        )
    };
    unsafe {
        __task_pool_get(__tick_periodic_task)
            ._spawn_async_fn(move || __tick_periodic_task())
    }
}
#[doc(hidden)]
#[allow(clippy::future_not_send)]
async fn ____embassy_main_task(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();
    spawner.spawn(run()).unwrap();
    spawner.spawn(tick_periodic()).unwrap();
}
#[allow(clippy::future_not_send)]
fn __embassy_main(spawner: Spawner) -> ::embassy_executor::SpawnToken<impl Sized> {
    const fn __task_pool_get<F, Args, Fut>(
        _: F,
    ) -> &'static ::embassy_executor::raw::TaskPool<Fut, POOL_SIZE>
    where
        F: ::embassy_executor::_export::TaskFn<Args, Fut = Fut>,
        Fut: ::core::future::Future + 'static,
    {
        unsafe { &*POOL.get().cast() }
    }
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolHolder<
        {
            ::embassy_executor::_export::task_pool_size::<
                _,
                _,
                _,
                POOL_SIZE,
            >(____embassy_main_task)
        },
        {
            ::embassy_executor::_export::task_pool_align::<
                _,
                _,
                _,
                POOL_SIZE,
            >(____embassy_main_task)
        },
    > = unsafe {
        ::core::mem::transmute(
            ::embassy_executor::_export::task_pool_new::<
                _,
                _,
                _,
                POOL_SIZE,
            >(____embassy_main_task),
        )
    };
    unsafe {
        __task_pool_get(____embassy_main_task)
            ._spawn_async_fn(move || ____embassy_main_task(spawner))
    }
}
fn main() -> ! {
    unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
        ::core::mem::transmute(t)
    }
    let mut executor = ::embassy_executor::Executor::new();
    let executor = unsafe { __make_static(&mut executor) };
    executor
        .run(|spawner| {
            spawner.must_spawn(__embassy_main(spawner));
        })
}
