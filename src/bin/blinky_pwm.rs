#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{gpio::OutputType, time::khz, timer::{complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin}, Channel}};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let led_y_pwm_pin = ComplementaryPwmPin::new_ch2(p.PB0, OutputType::PushPull);
    let led_o_pwm_pin = ComplementaryPwmPin::new_ch3(p.PB1, OutputType::PushPull);
    let led_r_pwm_pin = ComplementaryPwmPin::new_ch4(p.PB2, OutputType::PushPull);
    let mut pwm = ComplementaryPwm::new(p.TIM8, None, None, None, Some(led_y_pwm_pin), None, Some(led_o_pwm_pin), None, Some(led_r_pwm_pin), khz(10), Default::default());

    pwm.set_duty(Channel::Ch2, pwm.get_max_duty());
    pwm.set_duty(Channel::Ch3, pwm.get_max_duty());
    pwm.set_duty(Channel::Ch4, pwm.get_max_duty());
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    pwm.enable(Channel::Ch4);

    let mut duty = pwm.get_max_duty();
    loop {
        pwm.set_duty(Channel::Ch2, duty);
        pwm.set_duty(Channel::Ch3, duty);
        pwm.set_duty(Channel::Ch4, duty);
        if duty < 2 {
            duty = pwm.get_max_duty();
        } else {
            duty -= 2;
        }
        Timer::after_millis(10).await;
    }
}
