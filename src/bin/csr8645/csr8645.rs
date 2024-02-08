#![no_std]
#![no_main]

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::pin::Pin;
use defmt::{error, info};
use embassy_stm32::dma::NoDma;
use embassy_stm32::interrupt;
use embassy_stm32::usart::{Config, Uart};

/// Represents an error that can occur in the CSR8645 module.
#[derive(Debug)]
pub enum Csr8645Error {
    UartError(dyn defmt::Format),
    InvalidResponse,
}

impl From<dyn defmt::Format> for Csr8645Error {
    fn from(err: Box<dyn defmt::Format>) -> Csr8645Error {
        Csr8645Error::UartError(err)
    }
}

/// Represents a CSR8645 Bluetooth module.
pub struct Csr8645<'a> {
    uart: Uart<'a, interrupt::USART1, NoDma, NoDma>,
}

impl Csr8645 {
    /// Creates a new instance of `Csr8645`.
    ///
    /// # Arguments
    ///
    /// * `uart` - An instance of `Uart`.
    ///
    /// # Returns
    ///
    /// * `Csr8645` - A new instance of `Csr8645`.
    /// * `Csr8645Error` - An error occurred while creating the `Csr8645` instance.
    pub fn new(uart: Uart<interrupt::USART1, NoDma, NoDma>) -> Result<Self, Csr8645Error> {
        Ok(Self { uart })
    }

    /// Sends a command to the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to send.
    ///
    /// # Returns
    ///
    /// * `()` - The command was sent successfully.
    /// * `Csr8645Error` - An error occurred while sending the command.
    async fn send_command(self: Pin<&mut Self>, command: &[u8]) -> Result<(), Csr8645Error> {
        self.uart.write(command).await.map_err(Csr8645Error::from)
    }

    /// Reads the response from the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer where the response will be stored.
    ///
    /// # Returns
    ///
    /// * `()` - The response was read successfully.
    /// * `Csr8645Error` - An error occurred while reading the response.
    async fn read_response(self: Pin<&mut Self>, buf: &mut [u8]) -> Result<(), Csr8645Error> {
        self.uart.read(buf).await.map_err(Csr8645Error::from)
    }

    // Sets the name of the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name for the module.
    ///
    /// # Returns
    ///
    /// * `()` - The name was set successfully.
    /// * `Csr8645Error` - An error occurred while setting the name.
    pub async fn set_name(self: Pin<&mut Self>, name: &str) -> Result<(), Csr8645Error> {
        let command = format!("AT+NAME={}\r\n", name);
        self.send_command(command.as_bytes()).await
    }

    /// Gets the name of the CSR8645 module.
    ///
    /// # Returns
    ///
    /// * `()` - The name was obtained successfully.
    /// * `Csr8645Error` - An error occurred while getting the name.
    pub async fn get_name(self: Pin<&mut Self>) -> Result<(), Csr8645Error> {
        let command = b"AT+NAME?\r\n";
        self.send_command(command).await?;

        let mut buf = [0u8; 64];
        self.read_response(&mut buf).await?;

        info!("Received: {=[u8]}", &buf);

        Ok(())
    }

    /// Sets the PIN of the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `pin` - The new PIN for the module.
    ///
    /// # Returns
    ///
    /// * `()` - The PIN was set successfully.
    /// * `Csr8645Error` - An error occurred while setting the PIN.
    pub async fn set_pin(self: Pin<&mut Self>, pin: &str) -> Result<(), Csr8645Error> {
        let command = format!("AT+PIN={}\r\n", pin);
        self.send_command(command.as_bytes()).await
    }

    /// Gets the PIN of the CSR8645 module.
    ///
    /// # Returns
    ///
    /// * `String` - The PIN of the module.
    /// * `Csr8645Error` - An error occurred while getting the PIN.
    pub async fn get_pin(self: Pin<&mut Self>) -> Result<String, Csr8645Error> {
        let command = b"AT+PIN?\r\n";
        self.send_command(command).await?;

        let mut buf = [0u8; 64];
        self.read_response(&mut buf).await?;

        // Convert the response to a string and return it
        let pin = String::from_utf8(buf.to_vec()).map_err(|_| Csr8645Error::InvalidResponse)?;
        Ok(pin)
    }

    /// Sets the baud rate of the CSR8645 module.
    ///
    /// # Arguments
    ///
    /// * `baudrate` - The new baud rate for the module.
    ///
    /// # Returns
    ///
    /// * `()` - The baud rate was set successfully.
    /// * `Csr8645Error` - An error occurred while setting the baud rate.
    pub async fn set_baudrate(self: Pin<&mut Self>, baudrate: u32) -> Result<(), Csr8645Error> {
        let command = format!("AT+BAUD={}\r\n", baudrate);
        self.send_command(command.as_bytes()).await
    }

    /// Gets the baud rate of the CSR8645 module.
    ///
    /// # Returns
    ///
    /// * `u32` - The baud rate of the module.
    /// * `Csr8645Error` - An error occurred while getting the baud rate.
    pub async fn get_baudrate(self: Pin<&mut Self>) -> Result<u32, Csr8645Error> {
        let command = b"AT+BAUD?\r\n";
        self.send_command(command).await?;

        let mut buf = [0u8; 64];
        self.read_response(&mut buf).await?;

        // Convert the response to an u32 and return it
        let baudrate = u32::from_str_radix(core::str::from_utf8(&buf).unwrap().trim(), 10)
            .map_err(|_| Csr8645Error::InvalidResponse)?;
        Ok(baudrate)
    }

    /// Connects to a device.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to connect to.
    ///
    /// # Returns
    ///
    /// * `()` - The device was connected successfully.
    /// * `Csr8645Error` - An error occurred while connecting to the device.
    pub async fn connect(self: Pin<&mut Self>, address: &str) -> Result<(), Csr8645Error> {
        let command = format!("AT+CON{}\r\n", address);
        self.send_command(command.as_bytes()).await
    }

    /// Disconnects from the current device.
    ///
    /// # Returns
    ///
    /// * `()` - The device was disconnected successfully.
    /// * `Csr8645Error` - An error occurred while disconnecting from the device.
    pub async fn disconnect(self: Pin<&mut Self>) -> Result<(), Csr8645Error> {
        let command = b"AT";
        self.send_command(command).await
    }

    /// Checks if the CSR8645 module is connected to a device.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the module is connected to a device, false otherwise.
    /// * `Csr8645Error` - An error occurred while checking the connection status.
    pub async fn check_connection_status(self: Pin<&mut Self>) -> Result<bool, Csr8645Error> {
        let command = b"AT+CON?\r\n";
        self.send_command(command).await?;

        let mut buf = [0u8; 64];
        self.read_response(&mut buf).await?;

        let response = String::from_utf8(buf.to_vec()).map_err(|_| Csr8645Error::InvalidResponse)?;
        Ok(response.contains("OK+CON"))
    }

    /// Scans for nearby devices.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - A list of the addresses of the nearby devices.
    /// * `Csr8645Error` - An error occurred while scanning for devices.
    pub async fn scan(self: Pin<&mut Self>) -> Result<Vec<String>, Csr8645Error> {
        let command = b"AT+DISC?\r\n";
        self.send_command(command).await?;

        let mut buf = [0u8; 512];
        self.read_response(&mut buf).await?;

        let response = String::from_utf8(buf.to_vec()).map_err(|_| Csr8645Error::InvalidResponse)?;
        let addresses = response.split('\n').map(|s| s.to_string()).collect();
        Ok(addresses)
    }

    /// Sends data to the connected device.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to send.
    ///
    /// # Returns
    ///
    /// * `()` - The data was sent successfully.
    /// * `Csr8645Error` - An error occurred while sending the data.
    pub async fn send_data(self: Pin<&mut Self>, data: &[u8]) -> Result<(), Csr8645Error> {
        self.uart.write(data).await.map_err(Csr8645Error::from)
    }

    /// Receives data from the connected device.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer where the received data will be stored.
    ///
    /// # Returns
    ///
    /// * `()` - The data was received successfully.
    /// * `Csr8645Error` - An error occurred while receiving the data.
    pub async fn receive_data(self: Pin<&mut Self>, buf: &mut [u8]) -> Result<(), Csr8645Error> {
        self.uart.read(buf).await.map_err(Csr8645Error::from)
    }

    /// Plays audio data.
    ///
    /// # Arguments
    ///
    /// * `data` - The audio data to play.
    ///
    /// # Returns
    ///
    /// * `()` - The audio data was played successfully.
    /// * `Csr8645Error` - An error occurred while playing the audio data.
    pub async fn play_audio(self: Pin<&mut Self>, data: &[u8]) -> Result<(), Csr8645Error> {
        // Send the audio data to the CSR8645 module
        self.uart.write(data).await.map_err(Csr8645Error::from)
    }

    /// Receives audio data.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer where the received audio data will be stored.
    ///
    /// # Returns
    ///
    /// * `()` - The audio data was received successfully.
    /// * `Csr8645Error` - An error occurred while receiving the audio data.
    pub async fn receive_audio(self: Pin<&mut Self>, buf: &mut [u8]) -> Result<(), Csr8645Error> {
        self.uart.read(buf).await.map_err(Csr8645Error::from)
    }

    /// Gets the current status of the CSR8645 module.
    ///
    /// # Returns
    ///
    /// * `String` - The current status of the module.
    /// * `Csr8645Error` - An error occurred while getting the status.
    pub async fn get_status(self: Pin<&mut Self>) -> Result<String, Csr8645Error> {
        let command = b"AT+STATE?\r\n";
        self.send_command(command).await?;

        let mut buf = [0u8; 64];
        self.read_response(&mut buf).await?;

        // Convert the response to a string and return it
        let status = String::from_utf8(buf.to_vec()).map_err(|_| Csr8645Error::InvalidResponse)?;
        Ok(status)
    }

    /// Enables or disables notifications.
    ///
    /// # Arguments
    ///
    /// * `enable` - True to enable notifications, false to disable them.
    ///
    /// # Returns
    ///
    /// * `()` - The notification setting was changed successfully.
    /// * `Csr8645Error` - An error occurred while changing the notification setting.
    pub async fn set_notifications(self: Pin<&mut Self>, enable: bool) -> Result<(), Csr8645Error> {
        let command = if enable {
            b"AT+NOTI1\r\n"
        } else {
            b"AT+NOTI0\r\n"
        };
        self.send_command(command).await
    }
}