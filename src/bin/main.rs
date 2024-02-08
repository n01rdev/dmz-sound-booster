#![no_std]
#![no_main]

extern crate alloc;

use defmt::{error, info};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::init;
use embassy_stm32::Config;
use panic_probe as _;

mod audio;
mod bluetooth;
mod csr8645;
mod uart;

use bluetooth::bluetooth_controller::BluetoothController;
use csr8645::csr8645::Csr8645;
use obd::obd_controller::ObdController;

/// The `App` struct represents the main application.
///
/// It contains all the components of the application, such as the Bluetooth module and the OBD-II device.
struct App<'a> {
    bluetooth_module: BluetoothController<'a, Csr8645>,
    obd_module: ObdController,
}

impl<'a> App<'a> {
    /// Creates a new `App` instance.
    ///
    /// Initializes the Bluetooth module and the OBD-II device.
    fn new() -> Self {
        Self {
            bluetooth_module: BluetoothController::new(Csr8645::new()),
            obd_module: ObdController::new(),
        }
    }

    /// Runs the main logic of the application.
    ///
    /// In a loop, it reads the speed and RPM data from the OBD-II device, determines how to alter the audio behavior based on this data, and then alters the audio behavior.
    async fn run(&self) {
        loop {
            // Read the speed and RPM data from the OBD-II device
            let speed = self.obd_module.read_speed().await;
            let rpm = self.obd_module.read_rpm().await;

            // Determine how to alter the audio behavior
            let audio_behavior = map_sensor_data_to_audio_behavior(speed, rpm);

            // Alter the audio behavior
            self.bluetooth_module
                .alter_behavior(audio_behavior)
                .await
                .unwrap();
        }
    }

    #[embassy_executor::task]
    async fn run_app() {
        let app = Self::new();
        app.run().await;
    }
}

/// The `main` function is the main entry point for the application.
///
/// It initializes the peripherals, creates a new `App` instance, and runs the main logic of the application.
///
/// # Arguments
///
/// * `spawner` - A `Spawner` instance that allows spawning tasks onto an executor.
///
/// # Errors
///
/// Returns an error if the peripherals fail to initialize.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    if let Err(e) = init(config) {
        error!("Failed to initialize peripherals: {:?}", e);
        return;
    };
    info!("Peripherals initialized successfully");

    spawner.spawn(App::run_app()).unwrap();
}
