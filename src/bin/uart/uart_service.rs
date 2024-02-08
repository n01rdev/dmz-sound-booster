#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_stm32::dma::NoDma;
use embassy_stm32::interrupt;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::Peripherals;

/// Initialize and run the UART with the given baudrate.
///
/// # Arguments
///
/// * `p` - The peripherals to use for the UART.
/// * `baudrate` - The baudrate to use for the UART.
///
/// # Errors
///
/// Returns an error if the UART fails to initialize.
pub async fn run(
    p: Peripherals,
    baudrate: u32,
) -> Result<Uart<interrupt::USART1>, dyn defmt::Format> {
    let mut config = Config::default();
    config.baudrate = baudrate;

    let irq = interrupt::USART1.steal();
    let tx_dma = NoDma;
    let rx_dma = NoDma;

    let uart = Uart::new(p.USART1, p.PA9, p.PA10, irq, tx_dma, rx_dma, config).map_err(|e| {
        error!(
            "Failed to initialize UART with baudrate {}: {:?}",
            baudrate, e
        );
        e
    })?;

    Ok(uart)
}
