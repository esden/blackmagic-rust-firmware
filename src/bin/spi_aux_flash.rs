#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Hello World, dude!");

    let p = embassy_stm32::init(Default::default());

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let mut spi = Spi::new_blocking(p.SPI3, p.PB3, p.PB5, p.PB4, spi_config);

    // SD_CS
    let mut cs1 = Output::new(p.PB8, Level::High, Speed::VeryHigh);
    // DISPLAY_CS
    let mut cs2 = Output::new(p.PB9, Level::High, Speed::VeryHigh);
    // DISPLAY_DC
    let mut cs3 = Output::new(p.PC13, Level::High, Speed::VeryHigh);

    // The flash chip that is connected here over SPI is the Winbond W25Q128
    // Refer to the datasheet for protocol details
    loop {
        // Exit Power down and get Device ID
        let mut buf = [0xABu8; 5];
        cs1.set_low();
        cs2.set_low();
        cs3.set_low();
        unwrap!(spi.blocking_transfer_in_place(&mut buf));
        cs1.set_high();
        cs2.set_high();
        cs3.set_high();
        info!("Device ID              {=u8:#x}", buf[4]);

        // Read Manufacturer and Device ID
        let mut buf = [0x90u8, 0x00, 0x00, 0x00, 0x00, 0x00];
        cs1.set_low();
        cs2.set_low();
        cs3.set_low();
        unwrap!(spi.blocking_transfer_in_place(&mut buf));
        cs1.set_high();
        cs2.set_high();
        cs3.set_high();
        info!("MFR & Device ID {=[u8]:#x}", buf[4..6]);

        // Read JEDEC ID
        // Byte 0: Manufacturer ID
        // Byte 1: Memory Type
        // Byte 2: Capacity
        let mut buf = [0x9Fu8; 4];
        cs1.set_low();
        cs2.set_low();
        cs3.set_low();
        unwrap!(spi.blocking_transfer_in_place(&mut buf));
        cs1.set_high();
        cs2.set_high();
        cs3.set_high();
        info!("JEDEC ID        {=[u8]:#x}", buf[1..4]);
    }
}
