#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = system::init();
    let r = split_resources!(p);

    info!("Hello World!");

    let (mut tpwr_en, _, _, _) = system::get_tpwr(r.tpwr);
    tpwr_en.set_high();
    let (_tckdo_en, _cs_dir, mut cs, mut spi) = system::get_jtag_spi(r.jtag);

    // The flash chip that is connected here over SPI is the Winbond W25Q128
    // Refer to the datasheet for protocol details
    loop {
        // Exit Power down and get Device ID
        let mut buf = [0xABu8; 5];
        cs.set_low();
        spi.transfer_in_place(&mut buf).await.ok();
        cs.set_high();
        info!("Device ID              {=u8:#x}", buf[4]);

        // Read Manufacturer and Device ID
        let mut buf = [0x90u8, 0x00, 0x00, 0x00, 0x00, 0x00];
        cs.set_low();
        spi.transfer_in_place(&mut buf).await.ok();
        cs.set_high();
        info!("MFR & Device ID {=[u8]:#x}", buf[4..6]);

        // Read JEDEC ID
        // Byte 0: Manufacturer ID
        // Byte 1: Memory Type
        // Byte 2: Capacity
        let mut buf = [0x9Fu8; 4];
        cs.set_low();
        spi.transfer_in_place(&mut buf).await.ok();
        cs.set_high();
        info!("JEDEC ID        {=[u8]:#x}", buf[1..4]);

        // We need to give the RTT debug interface some breathing room. ;)
        Timer::after_millis(1).await;
    }
}
