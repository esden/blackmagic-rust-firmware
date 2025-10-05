#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_stm32::adc;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

fn calc_voltage(adc_val: u16) -> f32 {
    let r1 = 4_700.0;
    let r2 = 10_000.0;
    let vref = 3.3;
    let max = adc::resolution_to_max_count(adc::Resolution::BITS14);

    let adc_volt = vref * adc_val as f32 / max as f32;

    ((r1 + r2) / r2) * adc_volt
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = system::init();
    let r = split_resources!(p);

    // LED
    let (mut led_y, _, _, _) = system::get_leds(r.leds);

    // TPWR
    let (mut tpwr_en, mut tpwr_sens, mut tpwr_sens_ch) = system::get_tpwr(r.tpwr);

    let mut cnt = 10;
    loop {
        let tim = Timer::after_millis(100);
        let raw: u16 = tpwr_sens.blocking_read(&mut tpwr_sens_ch);
        let volt = calc_voltage(raw);

        info!("Read {} {}", raw, volt);

        if cnt == 0 {
            cnt = 10;
            tpwr_en.toggle();
            led_y.toggle();
        } else {
            cnt -= 1;
        }

        tim.await;
    }
}
