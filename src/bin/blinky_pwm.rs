#![no_std]
#![no_main]

use blackmagic_rust_firmware::{split_resources, system::{self, AssignedResources, LedResources, ButtonResources, UsbResources}};
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    let r = split_resources!(p);

    let (mut led_y, mut led_o, mut led_r, mut led_g) = system::get_leds_pwm(r.leds);

    let mut duty = led_y.max_duty_cycle();
    let mut dir = -1;
    info!("Max duty {}", duty);
    loop {
        led_y.set_duty_cycle(duty);
        led_o.set_duty_cycle(duty);
        led_r.set_duty_cycle(duty);
        led_g.set_duty_cycle(duty);
        if duty == 0 {
            info!("Reverse to up ...");
            dir = 1;
            duty += 1;
        } else if duty == led_y.max_duty_cycle() {
            info!("Reverse to down ...");
            dir = -1;
            duty -= 1;
        } else {
            if dir == -1 {
                duty -= 1;
            } else {
                duty += 1;
            }
        }
        Timer::after_millis(10).await;
    }
}
