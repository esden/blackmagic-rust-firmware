#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = system::init();
    let r = split_resources!(p);
    info!("Hello World!");

    let (mut led_y, _, _, _) = system::get_leds(r.leds);
    let (mut tpwr_en, _, _, _) = system::get_tpwr(r.tpwr);

    loop {
        defmt::info!("on!");
        led_y.set_low();
        tpwr_en.set_high();
        Timer::after_secs(5).await;

        defmt::info!("off!");
        led_y.set_high();
        tpwr_en.set_low();
        Timer::after_secs(5).await;
    }
}
