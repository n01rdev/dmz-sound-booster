#![no_std]
#![no_main]

use alloc::string::String;
use alloc::vec::Vec;
use crate::bluetooth::bluetooth_service::BluetoothService;
use crate::csr8645::csr8645::Csr8645Error;

/// `BluetoothController` is a struct that controls the Bluetooth services.
///
/// It uses an instance of a type that implements the `BluetoothService` trait to handle Bluetooth operations.
pub struct BluetoothController<'a, T: BluetoothService + 'a> {
    bluetooth_service: T,
}

impl<'a, T: BluetoothService> BluetoothController<'a, T> {
    /// Creates a new instance of `BluetoothController`.
    ///
    /// # Arguments
    ///
    /// * `bluetooth_service` - An instance of a type that implements the `BluetoothService` trait.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `BluetoothController` instance.
    pub fn new(bluetooth_service: T) -> Self {
        Self { bluetooth_service }
    }

    /// Initializes the CSR8645 module with the given settings.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the CSR8645 module.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub fn initialize(&self, pin: &str) -> Result<(), Csr8645Error> {
        self.bluetooth_service.initialize(pin)
    }

    /// Scans for nearby devices.
    ///
    /// # Returns
    ///
    /// A `Result` containing a list of the addresses of the nearby devices or an error.
    pub fn scan_devices(&self) -> Result<Vec<String>, Csr8645Error> {
        self.bluetooth_service.scan_devices()
    }

    /// Connects to a device with the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub fn connect_to_device(&self, address: &str) -> Result<(), Csr8645Error> {
        self.bluetooth_service.connect_to_device(address)
    }

    /// Sends data to the connected device.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to send.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub fn send_data(&self, data: &[u8]) -> Result<(), Csr8645Error> {
        self.bluetooth_service.send_data(data)
    }

    /// Transmits audio data to the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `audio_data` - The audio data to transmit.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub fn transmit_audio(&self, audio_data: &[u8]) -> Result<(), Csr8645Error> {
        self.bluetooth_service.transmit_audio(audio_data)
    }

    /// Receives audio data from the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer where the received audio data will be stored.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub fn receive_audio(&self, buffer: &mut [u8]) -> Result<(), Csr8645Error> {
        self.bluetooth_service.receive_audio(buffer)
    }
}