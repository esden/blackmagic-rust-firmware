#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // Uncomment to enable target power
    // let _tpwr_en = Output::new(p.PB12, Level::High, Speed::Low);

    // Adjust to test out direction swap
    let swap_rx_tx = false;

    // Configure the direction pin according to the rx/tx direction seleceted
    let _usart_dir = Output::new(
        p.PA8,
        if swap_rx_tx {
            Level::Low
        } else {
            Level::High
        },
        Speed::Low);

    let mut config = Config::default();
    config.swap_rx_tx = swap_rx_tx;
    let mut usart = Uart::new(p.USART1, p.PB7, p.PB6, Irqs, p.GPDMA1_CH0, p.GPDMA1_CH1, config).unwrap();

    for n in 0u32.. {
        let mut s: String<128> = String::new();
        core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

        info!("Writing...");
        usart.write(s.as_bytes()).await.ok();

        info!("wrote DMA");
    }
}
