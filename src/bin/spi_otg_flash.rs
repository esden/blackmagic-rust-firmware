#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::ospi;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = system::init();
    let r = split_resources!(p);

    let mut flash = system::get_flash_blocking(r.flash);

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
        flash.blocking_read(&mut buf, transfer_config).unwrap();
        info!("Device ID              {=u8:#x}", buf[0]);

        // Read Manufacturer and Device ID
        let mut buf = [0x00_u8; 2];
        transfer_config.instruction = Some(0x90);
        flash.blocking_read(&mut buf, transfer_config).unwrap();
        info!("MFR & Device ID {=[u8]:#x}", buf);

        // Read JEDEC ID
        // Byte 0: Manufacturer ID
        // Byte 1: Memory Type
        // Byte 2: Capacity
        let mut buf = [0x0_u8; 3];
        transfer_config.instruction = Some(0x9F);
        transfer_config.dummy = ospi::DummyCycles::_0;
        flash.blocking_read(&mut buf, transfer_config).unwrap();
        info!("JEDEC ID        {=[u8]:#x}", buf);
    }
}
