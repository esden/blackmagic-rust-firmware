#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_stm32::{exti::ExtiInput, gpio::Output};
use embassy_stm32::gpio::{Level, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};



#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut button_user = ExtiInput::new(p.PA10, p.EXTI10, Pull::None);
    let mut button_aux = ExtiInput::new(p.PC14, p.EXTI14, Pull::None);
    let mut led_o = Output::new(p.PB1, Level::High, Speed::Low);
    let mut led_y = Output::new(p.PB0, Level::High, Speed::Low);

    loop {
        let bu_fut = button_user.wait_for_any_edge();
        let ba_fut = button_aux.wait_for_any_edge();
        let result = select(bu_fut, ba_fut).await;
        match result {
            Either::First(_) => {
                led_o.set_level(button_user.get_level());
                info!("user {}", button_user.get_level());
            },
            Either::Second(_) => {
                led_y.set_level(button_aux.get_level());
                info!("aux {}", button_aux.get_level());
            },
        }
    }
}
