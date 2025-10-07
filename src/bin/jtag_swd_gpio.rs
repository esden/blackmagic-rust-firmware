#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Speed;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = system::init();
    let r = split_resources!(p);
    info!("Hello World!");

    let (mut led_y, mut led_o, mut _led_r, _) = system::get_leds(r.leds);

    // Target Power Control
    let (mut tpwr_en, _, _, _) = system::get_tpwr(r.tpwr);

    let (
        mut tckdi_en,
        mut tck,
        mut tdi,
        tdo,
        tdo_rx,
        mut tms_dir,
        mut tms
    ) = system::get_jtag_gpio(r.jtag);

    loop {

        tpwr_en.set_high();
        tckdi_en.set_high();
        // {
        //     use embassy_stm32::gpio::Pull;
        //     tms_dir.set_low();
        //     tms.set_as_input(Pull::None);
        // }
        tms_dir.set_high();
        tms.set_as_output(Speed::VeryHigh);
        tck.set_high();
        tdi.set_low();
        for _ in 0..100 {
            tck.toggle();
            tdi.toggle();
            led_o.set_level(tdo.get_level());
            led_y.set_level(tdo_rx.get_level());
            // _led_r.set_level(tms.get_level());
            tms.toggle();
        }
        Timer::after_millis(100).await;
    }

    // // Super tight toggle test loop
    // tpwr_en.set_high();
    // tckdi_en.set_high();
    // loop {
    //     tck.set_high();
    //     tck.set_low();
    // }
}
