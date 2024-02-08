#![no_std]
#![no_main]

use crate::uart::uart_service::UartService;
use embassy_stm32::Peripherals;

/// `UartController` is a structure that handles high-level operations with the UART.
///
/// This structure provides methods for sending and receiving data through the UART.
pub struct UartController<'a> {
    uart_service: UartService<'a>,
}

impl<'a> UartController<'a> {
    /// Creates a new instance of `UartController`.
    ///
    /// # Arguments
    ///
    /// * `p` - An instance of `Peripherals`.
    /// * `baudrate` - The baud rate for the UART.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub fn new(p: Peripherals, baudrate: u32) -> Result<Self, dyn defmt::Format> {
        let uart_service = UartService::new(p, baudrate)?;

        Ok(Self { uart_service })
    }
}
