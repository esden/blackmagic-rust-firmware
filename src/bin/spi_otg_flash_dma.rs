#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::ospi;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let p = system::init();
    let r = split_resources!(p);

    let mut flash = system::get_flash(r.flash);

    // The flash chip that is connected here over SPI is the Winbond W25Q128
    // Refer to the datasheet for protocol details
    loop {
        // Exit Power down and get Device ID
        let mut buf = [0_u8; 1];
        let mut transfer_config = ospi::TransferConfig::default();
        transfer_config.iwidth = ospi::OspiWidth::SING;
        transfer_config.instruction = Some(0xAB);
        transfer_config.isize = ospi::AddressSize::_8Bit;
        transfer_config.dummy = ospi::DummyCycles::_24;
        transfer_config.dwidth = ospi::OspiWidth::SING;
        unwrap!(flash.read(&mut buf, transfer_config).await);
        info!("Device ID              {=u8:#x}", buf[0]);

        // Read Manufacturer and Device ID
        let mut buf = [0x00_u8; 2];
        transfer_config.instruction = Some(0x90);
        unwrap!(flash.read(&mut buf, transfer_config).await);
        info!("MFR & Device ID {=[u8]:#x}", buf);

        // Read JEDEC ID
        // Byte 0: Manufacturer ID
        // Byte 1: Memory Type
        // Byte 2: Capacity
        let mut buf = [0x0_u8; 3];
        transfer_config.instruction = Some(0x9F);
        transfer_config.dummy = ospi::DummyCycles::_0;
        unwrap!(flash.read(&mut buf, transfer_config).await);
        info!("JEDEC ID        {=[u8]:#x}", buf);

        // We need to give the RTT debug interface some breathing room. ;)
        Timer::after_millis(1).await;
    }
}
