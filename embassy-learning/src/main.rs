use embassy_executor::{Spawner, task, main};
use embassy_time::{Timer, Duration};
use log::*;

#[task]
async fn run() {
    loop {
        info!("tick");
        Timer::after_secs(1).await;
    }
}

#[task]
/// Task that ticks periodically
async fn tick_periodic() -> ! {
    loop {
        println!("tick 1!");
        // async sleep primitive, suspends the task for 500ms.
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    spawner.spawn(run()).unwrap();
    spawner.spawn(tick_periodic()).unwrap();
}