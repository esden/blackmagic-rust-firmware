#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let p = system::init();
    let r = split_resources!(p);

    let (mut led_y, _, _, _) = system::get_leds(r.leds);
    // Uncomment to enable target power
    // let _tpwr_en = {
    //     use embassy_stm32::gpio::{Level, Output, Speed};
    //     Output::new(p.PA5, Level::High, Speed::Low)
    // };

    // Adjust to test out direction swap
    let swap_rx_tx = false;

    // Undortunately the tx/rx direction can't be changed after the driver is created.
    // We might need to contribute that feature to embassy. ;)
    let (mut uart, _uart_dir) = system::get_uart_secondary_blocking(r.uart_secondary, swap_rx_tx);

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
