use assign_resources::assign_resources;
use embassy_stm32::{gpio::{self, Output, Flex}, peripherals, Peri};

assign_resources! {
    leds: LedResources {
        led_yo_tim: TIM3 = LedYOTim,
        led_y: PB5,
        led_o: PB4,
        led_rg_tim: TIM1 = LedRGTim,
        led_r: PA10,
        led_g: PA8,
    }
}

pub fn get_leds<'a>(r: LedResources) -> (Output<'a>, Output<'a>, Output<'a>, Flex<'a>) {
    let led_y = Output::new(r.led_y, gpio::Level::High, gpio::Speed::Low);
    let led_o = Output::new(r.led_o, gpio::Level::High, gpio::Speed::Low);
    let led_r = Output::new(r.led_r, gpio::Level::High, gpio::Speed::Low);
    let mut led_g = Flex::new(r.led_g); // Output::new(r.led_g, gpio::Level::High, gpio::Speed::Low);
    led_g.set_as_input(gpio::Pull::None);
    (
        led_y,
        led_o,
        led_r,
        led_g
    )
}