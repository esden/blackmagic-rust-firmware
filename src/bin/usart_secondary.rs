#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let mut led_o = Output::new(p.PB0, Level::High, Speed::Low);
    // Uncomment to enable target power
    // let _tpwr_en = Output::new(p.PB12, Level::High, Speed::Low);

    // Adjust to test out direction swap
    let swap_rx_tx = false;

    // Configure the direction pin according to the rx/tx direction seleceted
    let _usart_dir = Output::new(
        p.PA8,
        if swap_rx_tx {
            Level::Low
        } else {
            Level::High
        },
        Speed::Low);

    // Configure USART
    let mut config = Config::default();
    config.swap_rx_tx = swap_rx_tx;
    let mut usart = Uart::new_blocking(p.USART1, p.PB7, p.PB6, config).unwrap();

    usart.blocking_write(b"Hello Embassy World!\r\n").unwrap();
    info!("wrote Hello, starting echo");

    let mut buf = [0u8; 1];
    loop {
        match usart.blocking_read(&mut buf) {
            Ok(()) => {
                usart.blocking_write(&buf).unwrap();
                led_o.toggle();
            },
            Err(e) => error!("read failed with \"{}\"", e),
        }
    }
}
