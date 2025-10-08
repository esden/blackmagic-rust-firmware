#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::preamble::*};
use defmt::{error, info};
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

const TMP117_ADDRESS: u8 = 0x48;
const DEVICE_ID: u8 = 0x0F;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = system::init();
    let r = split_resources!(p);
    let mut i2c = system::get_aux_i2c(r.aux);

    loop {
        let mut data = [0u8; 2];
        let ret = i2c.blocking_write_read(TMP117_ADDRESS, &[DEVICE_ID], &mut data);

        if let Err(err) = ret {
            error!("I2C read failed with {}", err);
        } else {
            // TMP117 data sheet is here: https://www.ti.com/lit/ds/symlink/tmp117.pdf
            // Device_ID command is x0F which expected response 0x0117.
            let device_id = (data[0] as u16) << 8 | data[1] as u16;
            if device_id == 0x0117 {
                info!("Device ID: 0x{:04x}", device_id);
            } else {
                error!("Wrong Device ID: 0x{:04x}", device_id);
            }
        }
    }
}
