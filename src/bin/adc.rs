#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::adc;
use embassy_stm32::adc::AdcChannel;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let config = embassy_stm32::Config::default();

    let mut p = embassy_stm32::init(config);

    // **** ADC1 init ****
    let mut adc1 = adc::Adc::new(p.ADC1);
    let mut adc1_pin1 = p.PA3; // ADC IN8
    let mut adc1_pin2 = p.PA2; // ADC IN7
    adc1.set_resolution(adc::Resolution::BITS14);
    adc1.set_averaging(adc::Averaging::Samples1024);
    adc1.set_sample_time(adc::SampleTime::CYCLES160_5);
    let max1 = adc::resolution_to_max_count(adc::Resolution::BITS14);

    // **** ADC1 blocking read ****
    let raw: u16 = adc1.blocking_read(&mut adc1_pin1);
    let volt: f32 = 3.3 * raw as f32 / max1 as f32;
    info!("Read adc1 pin 1 {}", volt);

    let raw: u16 = adc1.blocking_read(&mut adc1_pin2);
    let volt: f32 = 3.3 * raw as f32 / max1 as f32;
    info!("Read adc1 pin 2 {}", volt);

    // **** ADC1 async read ****
    let mut degraded11 = adc1_pin1.degrade_adc();
    let mut degraded12 = adc1_pin2.degrade_adc();
    let mut measurements = [0u16; 2];

    adc1.read(
        p.GPDMA1_CH0.reborrow(),
        [
            (&mut degraded11, adc::SampleTime::CYCLES160_5),
            (&mut degraded12, adc::SampleTime::CYCLES160_5),
        ]
        .into_iter(),
        &mut measurements,
    )
    .await;
    let volt1: f32 = 3.3 * measurements[0] as f32 / max1 as f32;
    let volt2: f32 = 3.3 * measurements[1] as f32 / max1 as f32;

    info!("Async read 1 pin 1 {}", volt1);
    info!("Async read 1 pin 2 {}", volt2);

}
