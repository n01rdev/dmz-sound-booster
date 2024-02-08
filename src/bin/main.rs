#![no_std]
#![no_main]

extern crate alloc;

use defmt::{error, info};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::init;
use embassy_stm32::Config;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_probe as _;

mod audio;
mod bluetooth;
mod csr8645;
mod uart;

/// The `main` function is the main entry point for the application.
///
/// It initializes the peripherals, and then enters a loop where it waits for one second in each iteration.
///
/// # Arguments
///
/// * `_spawner` - A `Spawner` instance that allows spawning tasks onto an executor.
///
/// # Errors
///
/// Returns an error if the peripherals fail to initialize.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    if let Err(e) = init(config) {
        error!("Failed to initialize peripherals: {:?}", e);
        return;
    };
    info!("Peripherals initialized successfully");

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
