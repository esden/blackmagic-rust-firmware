#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals, ucpd::{self, Ucpd}};
use embassy_time::Timer;
use {defmt_rtt as _, embassy_stm32 as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UCPD1 => ucpd::InterruptHandler<peripherals::UCPD1>;
});

fn cc_state_str(state: ucpd::CcVState) -> &'static str {
    match state {
        ucpd::CcVState::LOWEST => "LOWEST (0b00)",
        ucpd::CcVState::LOW => "LOW (0b01)",
        ucpd::CcVState::HIGH => "HIGH (0b10)",
        ucpd::CcVState::HIGHEST => "HIGHEST (0b11)",
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {

    let p = {
        use embassy_stm32::rcc::*;
        use embassy_stm32::Config;
        let mut config = Config::default();
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
        config.enable_ucpd1_dead_battery = true;

        embassy_stm32::init(config)
    };

    let mut ucpd = Ucpd::new(p.UCPD1, Irqs, p.PA15, p.PB15, Default::default());
    ucpd.cc_phy().set_pull(ucpd::CcPull::Sink);

    info!("Hello World!");

    loop {
        let (cc1, cc2) = ucpd.cc_phy().vstate();
        info!("CC1 {} CC2 {}", cc_state_str(cc1), cc_state_str(cc2));
        Timer::after_secs(1).await;
    }
}
