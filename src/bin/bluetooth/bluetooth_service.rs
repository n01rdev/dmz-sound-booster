#![no_std]

use crate::csr8645::csr8645::{Csr8645, Csr8645Error};
use alloc::string::String;
use alloc::vec::Vec;

/// `BluetoothService` is a trait that defines the methods necessary to handle Bluetooth operations.
///
/// This trait provides an interface for Bluetooth operations such as initialization, scanning for devices,
/// connecting to a device, sending data, transmitting audio, and receiving audio.
pub trait BluetoothService {
    /// Initializes the CSR8645 module with the given settings.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the CSR8645 module.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    fn initialize(&self, pin: &str) -> Result<(), Csr8645Error>;

    /// Scans for nearby devices.
    ///
    /// This method initiates a scan for nearby Bluetooth devices and returns a list of their addresses.
    ///
    /// # Returns
    ///
    /// A `Result` containing a list of the addresses of the nearby devices or an error.
    fn scan_devices(&self) -> Result<Vec<String>, Csr8645Error>;

    /// Connects to a device with the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    fn connect_to_device(&self, address: &str) -> Result<(), Csr8645Error>;

    /// Sends data to the connected device.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to send.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    fn send_data(&self, data: &[u8]) -> Result<(), Csr8645Error>;

    /// Transmits audio data to the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `audio_data` - The audio data to transmit.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    fn transmit_audio(&self, audio_data: &[u8]) -> Result<(), Csr8645Error>;

    /// Receives audio data from the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer where the received audio data will be stored.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    fn receive_audio(&self, buffer: &mut [u8]) -> Result<(), Csr8645Error>;
}

/// `BluetoothServiceImpl` is a struct that implements the `BluetoothService` trait.
///
/// This struct provides the actual implementation of the Bluetooth operations defined in the `BluetoothService` trait.
/// It uses a reference to a `Csr8645` instance to perform these operations.
pub struct BluetoothServiceImpl<'a> {
    /// A reference to a `Csr8645` instance.
    csr8645: &'a Csr8645<'a>,
}

impl<'a> BluetoothServiceImpl<'a> {
    /// Creates a new instance of `BluetoothServiceImpl`.
    ///
    /// # Arguments
    ///
    /// * `csr8645` - A reference to a `Csr8645` instance.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `BluetoothServiceImpl` instance.
    pub fn new(csr8645: &'a Csr8645<'a>) -> Self {
        Self { csr8645 }
    }
}

impl<'a> BluetoothService for BluetoothServiceImpl<'a> {
    fn initialize(&self, pin: &str) -> Result<(), Csr8645Error> {
        self.csr8645.set_pin(pin)
    }

    fn scan_devices(&self) -> Result<Vec<String>, Csr8645Error> {
        self.csr8645.scan()
    }

    fn connect_to_device(&self, address: &str) -> Result<(), Csr8645Error> {
        self.csr8645.connect(address)
    }

    fn send_data(&self, data: &[u8]) -> Result<(), Csr8645Error> {
        self.csr8645.send_data(data)
    }

    fn transmit_audio(&self, audio_data: &[u8]) -> Result<(), Csr8645Error> {
        self.csr8645.send_audio(audio_data)
    }

    fn receive_audio(&self, buffer: &mut [u8]) -> Result<(), Csr8645Error> {
        self.csr8645.receive_audio(buffer)
    }
}