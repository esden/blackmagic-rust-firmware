#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    let r = split_resources!(p);

    // replace PC7 with the right pin for your board.
    let (mut led_y, mut led_o, mut led_r, _) = system::get_leds(r.leds);
    let (mut pin1, mut pin2, mut pin3) = system::get_aux_gpio(r.aux);

    loop {
        defmt::info!("on!");
        led_r.set_low();
        pin1.set_low();
        Timer::after_millis(200).await;

        defmt::info!("on!");
        led_o.set_low();
        pin2.set_low();
        Timer::after_millis(200).await;

        defmt::info!("on!");
        led_y.set_low();
        pin3.set_low();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led_r.set_high();
        pin1.set_high();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led_o.set_high();
        pin2.set_high();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led_y.set_high();
        pin3.set_high();
        Timer::after_millis(200).await;
    }
}
