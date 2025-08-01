#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART4 => usart::InterruptHandler<peripherals::UART4>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let mut led_o = Output::new(p.PB0, Level::High, Speed::Low);
    // Uncomment to enable target power
    // let _tpwr_en = Output::new(p.PB12, Level::High, Speed::Low);

    let config = Config::default();
    let mut usart = Uart::new_blocking(p.UART4, p.PA1, p.PA0, config).unwrap();

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
