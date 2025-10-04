#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART4 => usart::InterruptHandler<peripherals::UART4>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let p = system::init();
    let r = split_resources!(p);

    let (mut led_y, _, _, _)  = system::get_leds(r.leds);
    // Uncomment to enable target power
    // let _tpwr_en = {
    //     use embassy_stm32::gpio::{Level, Output, Speed};
    //     Output::new(p.PA5, Level::High, Speed::Low)
    // };

    let mut uart = system::get_uart_primary_blocking(r.uart_primary);

    uart.blocking_write(b"Hello Embassy World!\r\n").unwrap();
    info!("wrote Hello, starting echo");

    let mut buf = [0u8; 1];
    loop {
        match uart.blocking_read(&mut buf) {
            Ok(()) => {
                uart.blocking_write(&buf).unwrap();
                led_y.toggle();
            },
            Err(e) => error!("read failed with \"{}\"", e),
        }
    }
}
