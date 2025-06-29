#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Input, Level, Output, Pull, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = true;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI, // 16 MHz
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL10,
            divp: None,
            divq: None,
            divr: Some(PllDiv::DIV1), // 160 MHz
        });
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.voltage_range = VoltageScale::RANGE1;
        config.rcc.hsi48 = Some(Hsi48Config { sync_from_usb: true }); // needed for USB
        config.rcc.mux.iclksel = mux::Iclksel::HSI48; // USB uses ICLK
    }
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let mut led_y = Output::new(p.PB0, Level::High, Speed::VeryHigh);
    let mut led_o = Output::new(p.PB1, Level::High, Speed::VeryHigh);
    let mut _led_r = Output::new(p.PB2, Level::High, Speed::VeryHigh);

    // Target Power Control
    let mut tpwr_en = Output::new(p.PB12, Level::High, Speed::Low);

    let mut tckdi_en = Output::new(p.PC15, Level::Low, Speed::Low);
    let mut tck = Output::new(p.PA5, Level::Low, Speed::VeryHigh);
    let mut tdi = Output::new(p.PA7, Level::Low, Speed::VeryHigh);

    let tdo1 = Input::new(p.PA6, Pull::None);
    let tdo2 = Input::new(p.PB10, Pull::None);

    let mut tms_dir = Output::new(p.PB13, Level::High, Speed::Low);
    let mut tms = Flex::new(p.PA4);
    tms.set_high();
    tms.set_as_output(Speed::VeryHigh);



    loop {

        tpwr_en.set_high();
        tckdi_en.set_high();
        // tms_dir.set_low();
        // tms.set_as_input(Pull::None);
        tms_dir.set_high();
        tms.set_as_output(Speed::VeryHigh);
        tck.set_high();
        tdi.set_low();
        for _ in 0..100 {
            tck.toggle();
            tdi.toggle();
            led_o.set_level(tdo1.get_level());
            led_y.set_level(tdo2.get_level());
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
