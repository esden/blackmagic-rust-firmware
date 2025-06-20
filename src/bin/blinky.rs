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

    // replace PC7 with the right pin for your board.
    let mut led_r = Output::new(p.PB2, Level::High, Speed::Low);
    let mut led_o = Output::new(p.PB1, Level::High, Speed::Low);
    let mut led_y = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        defmt::info!("on!");
        led_r.set_low();
        Timer::after_millis(200).await;

        defmt::info!("on!");
        led_o.set_low();
        Timer::after_millis(200).await;

        defmt::info!("on!");
        led_y.set_low();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led_r.set_high();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led_o.set_high();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led_y.set_high();
        Timer::after_millis(200).await;
    }
}
