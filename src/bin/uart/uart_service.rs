#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_stm32::dma::NoDma;
use embassy_stm32::interrupt;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::Peripherals;

/// `UartService` is a structure that handles low-level operations with the UART.
///
/// This structure provides methods for initializing the UART and configuring it.
pub struct UartService<'a> {
    uart: Uart<'a, interrupt::USART1, NoDma, NoDma>,
}

impl<'a> UartService<'a> {
    /// Creates a new instance of `UartService`.
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
        let mut config = Config::default();
        config.baudrate = baudrate;

        let irq = interrupt::USART1.steal();
        let tx_dma = NoDma;
        let rx_dma = NoDma;

        let uart =
            Uart::new(p.USART1, p.PA9, p.PA10, irq, tx_dma, rx_dma, config).map_err(|e| {
                error!(
                    "Failed to initialize UART with baudrate {}: {:?}",
                    baudrate, e
                );
                e
            })?;

        Ok(Self { uart })
    }
}
