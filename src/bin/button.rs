#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use cortex_m_rt::entry;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = system::init();
    let r = split_resources!(p);

    let button = system::get_button(r.button);
    let (mut led_y, _, _, _) = system::get_leds(r.leds);

    loop {
        info!("Button {}", button.get_level());
        led_y.set_level(button.get_level());
    }
}
