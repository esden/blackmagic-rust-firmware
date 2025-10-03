#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::{self, AssignedResources, LedResources, UsbResources}};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    let r = split_resources!(p);

    // replace PC7 with the right pin for your board.
    let (mut led_y, mut led_o, mut led_r, mut led_g) = system::get_leds(r.leds);

    led_g.set_high();
    led_g.set_as_output(gpio::Speed::Low);

    loop {
        defmt::info!("half on!");
        led_g.set_as_input(gpio::Pull::None);
        Timer::after_millis(200).await;

        defmt::info!("on!");
        led_g.set_low();
        led_g.set_as_output(gpio::Speed::Low);
        Timer::after_millis(200).await;

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
        led_g.set_high();
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
