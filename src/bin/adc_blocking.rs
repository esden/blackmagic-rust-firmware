#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::gpio::{Level, Speed};
use embassy_stm32::{adc, gpio::Output};
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
    let config = embassy_stm32::Config::default();

    let p = embassy_stm32::init(config);

    // LED
    let mut led_o = Output::new(p.PB1, Level::High, Speed::Low);

    // TPWR
    let mut tpwr_en = Output::new(p.PB12, Level::Low, Speed::Low);

    // **** ADC init ****
    let mut adc1 = adc::Adc::new(p.ADC1);
    let mut adc1_pin1 = p.PA3; // ADC IN8
    let mut adc1_pin2 = p.PA2; // ADC IN7
    adc1.set_resolution(adc::Resolution::BITS14);
    adc1.set_averaging(adc::Averaging::Samples1024);
    adc1.set_sample_time(adc::SampleTime::CYCLES160_5);

    let mut cnt = 10;
    loop {
        let tim = Timer::after_millis(10);
        let raw1: u16 = adc1.blocking_read(&mut adc1_pin1);
        let volt1 = calc_voltage(raw1);

        let raw2: u16 = adc1.blocking_read(&mut adc1_pin2);
        let volt2 = calc_voltage(raw2);

        info!("Read  {} {}", volt1, volt2);

        if cnt == 0 {
            cnt = 10;
            tpwr_en.toggle();
            led_o.toggle();
        } else {
            cnt -= 1;
        }

        tim.await;
    }
}
