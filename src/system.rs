use assign_resources::assign_resources;
use embassy_stm32::{
    bind_interrupts, exti::ExtiInput,
    gpio::{self, Flex, Input, Output},
    mode, peripherals, time::Hertz,
    timer::{low_level::OutputPolarity, simple_pwm::{PwmPin, SimplePwm, SimplePwmChannel}},
    usart::{self, Uart}, Config, Peri, Peripherals};

assign_resources! {
    leds: LedResources {
        led_yo_tim: TIM3 = LedYOTim,
        led_y: PB5,
        led_o: PB4,
        led_rg_tim: TIM1 = LedRGTim,
        led_r: PA10,
        led_g: PA8,
    }
    button: ButtonResources {
        pin: PA15,
        exti: EXTI15,
    }
    usb: UsbResources {
        peri: USB_OTG_FS = UsbPeri,
        dp: PA12,
        dm: PA11,
    }
    uart_primary: UartPrimaryResources {
        peri: USART2 = UartPrimaryPeri,
        rx_pin: PA3,
        tx_pin: PA2,
        rx_dma: GPDMA1_CH1,
        tx_dma: GPDMA1_CH0,
    }
}

pub mod preamble {
    // pub use crate::split_resources;
    pub use crate::system;
    pub use super::{AssignedResources, LedResources, ButtonResources, UsbResources, UartPrimaryResources};
}

bind_interrupts!(struct UartIrqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

pub fn init() -> Peripherals {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
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
    }

    embassy_stm32::init(config)
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

pub fn get_leds_pwm<'a>(r: LedResources) -> (SimplePwmChannel<'a, peripherals::TIM3>, SimplePwmChannel<'a, peripherals::TIM3>, SimplePwmChannel<'a, peripherals::TIM1>, SimplePwmChannel<'a, peripherals::TIM1>) {
    let pwm_pin_o = PwmPin::new(r.led_o, gpio::OutputType::PushPull);
    let pwm_pin_y = PwmPin::new(r.led_y, gpio::OutputType::PushPull);
    let pwm_yo = SimplePwm::new(
        r.led_yo_tim,
        Some(pwm_pin_o),
        Some(pwm_pin_y),
        None,
        None,
        Hertz::khz(10),
        Default::default());
    let pwm_channels_yo = pwm_yo.split();
    let mut pwm_o = pwm_channels_yo.ch1;
    pwm_o.set_duty_cycle_fully_on();
    pwm_o.set_polarity(OutputPolarity::ActiveLow);
    pwm_o.enable();
    let mut pwm_y = pwm_channels_yo.ch2;
    pwm_y.set_duty_cycle_fully_on();
    pwm_y.set_polarity(OutputPolarity::ActiveLow);
    pwm_y.enable();

    let pwm_pin_g = PwmPin::new(r.led_g, gpio::OutputType::PushPull);
    let pwm_pin_r = PwmPin::new(r.led_r, gpio::OutputType::PushPull);
    let pwm_rg = SimplePwm::new(
        r.led_rg_tim,
        Some(pwm_pin_g),
        None,
        Some(pwm_pin_r),
        None,
        Hertz::khz(10),
        Default::default());
    let pwm_channels_rg = pwm_rg.split();
    let mut pwm_g = pwm_channels_rg.ch1;
    pwm_g.set_duty_cycle_fully_on();
    pwm_g.set_polarity(OutputPolarity::ActiveLow);
    pwm_g.enable();
    let mut pwm_r = pwm_channels_rg.ch3;
    pwm_r.set_duty_cycle_fully_on();
    pwm_r.set_polarity(OutputPolarity::ActiveLow);
    pwm_r.enable();

    (pwm_y, pwm_o, pwm_r, pwm_g)
}

pub fn get_button<'a>(r: ButtonResources) -> Input<'a> {
    Input::new(r.pin, gpio::Pull::None)
}

pub fn get_button_exti<'a>(r: ButtonResources) -> ExtiInput<'a> {
    ExtiInput::new(r.pin, r.exti, gpio::Pull::None)
}

pub fn get_uart_primary_blocking<'a>(r: UartPrimaryResources) -> Uart<'a, mode::Blocking> {
    let config = usart::Config::default();
    Uart::new_blocking(r.peri, r.rx_pin, r.tx_pin, config).unwrap()
}

pub fn get_uart_primary<'a>(r: UartPrimaryResources) -> Uart<'a, mode::Async> {
    let config = usart::Config::default();
    Uart::new(r.peri, r.rx_pin, r.tx_pin, UartIrqs, r.tx_dma, r.rx_dma, config).unwrap()
}
