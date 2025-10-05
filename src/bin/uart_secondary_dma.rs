#![no_std]
#![no_main]

use core::fmt::Write;

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = system::init();
    let r = split_resources!(p);

    info!("Hello World!");

    // Uncomment to enable target power
    // let _tpwr_en = {
    //     use embassy_stm32::gpio::{Level, Output, Speed};
    //     Output::new(p.PA5, Level::High, Speed::Low)
    // };

    // Adjust to test out direction swap
    let swap_rx_tx = false;

    // Undortunately the tx/rx direction can't be changed after the driver is created.
    // We might need to contribute that feature to embassy. ;)
    let (mut uart, _uart_dir) = system::get_uart_secondary(r.uart_secondary, swap_rx_tx);

    for n in 0u32.. {
        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

        info!("Writing...");
        uart.write(s.as_bytes()).await.ok();

        info!("wrote DMA");
    }
}
