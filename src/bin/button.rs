#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let button_usr = Input::new(p.PA10, Pull::None);
    let button_aux = Input::new(p.PC14, Pull::None);
    let mut led_o = Output::new(p.PB1, Level::High, Speed::Low);
    let mut led_y = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        info!("usr {} aux {}", button_usr.get_level(), button_aux.get_level() );
        led_o.set_level(button_usr.get_level());
        led_y.set_level(button_aux.get_level());
    }
}
