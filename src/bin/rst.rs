#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led_o = Output::new(p.PB1, Level::High, Speed::VeryHigh);

    let mut rst = Output::new(p.PH1, Level::High, Speed::VeryHigh);
    let rst_s = Input::new(p.PH0, Pull::Up);

    let _tpwr_en = Output::new(p.PB12, Level::High, Speed::Low);

    loop {

        // Assert RESET:
        // Setting rst high we enable the MOSFET
        // Enabling the MOSFET will pull the RST low resetting the device
        // The rst_s is also reverse logic. So we wait for it to go high
        // to confirm that the reset successfully drove the RST line low.
        info!("RST");
        led_o.set_low();
        rst.set_high();
        while rst_s.is_low() {
            info!("RSTs {}", rst_s.get_level());
            Timer::after_millis(1).await;
        }
        info!("RSTs {}", rst_s.get_level());

        // Deassert RESET:
        // Setting rst low will disable the MOSFET
        // Disabling the MOSFET will release the RST line and if nothing else
        // is pulling it low will result in the device being released from reset.
        // The srt_s should return back to being Low after we release RST line.
        info!("!RST");
        led_o.set_high();
        rst.set_low();
        while rst_s.is_high() {
            info!("RSTs {}", rst_s.get_level());
            Timer::after_millis(1).await;
        }
        info!("RSTs {}", rst_s.get_level());

        Timer::after_millis(2000).await;
    }
}
