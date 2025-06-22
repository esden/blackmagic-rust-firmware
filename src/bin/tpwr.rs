#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led_r = Output::new(p.PB2, Level::High, Speed::Low);
    let mut tpwr_en = Output::new(p.PB12, Level::Low, Speed::Low);

    loop {
        defmt::info!("on!");
        led_r.set_low();
        tpwr_en.set_high();
        Timer::after_secs(5).await;

        defmt::info!("off!");
        led_r.set_high();
        tpwr_en.set_low();
        Timer::after_secs(5).await;
    }
}
