#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let p = system::init();
    let r = split_resources!(p);

    let mut button = system::get_button_exti(r.button);
    let (mut led_y, _, _, _) = system::get_leds(r.leds);

    loop {
        button.wait_for_any_edge().await;
        led_y.set_level(button.get_level());
        info!("Button {}", button.get_level());
    }
}
