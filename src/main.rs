//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_probe as _;

use crate::hal::{
    pac,
    prelude::*,
    timer::{Channel1, Channel2},
};
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    info!("Starting...");
    // Setup peripherals
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    let channels = (Channel1::new(gpioa.pa8), Channel2::new(gpioa.pa9));

    let pwm = dp.TIM1.pwm_hz(channels, 20.kHz(), &clocks).split();
    let (mut ch1, _ch2) = pwm;
    let max_duty = ch1.get_max_duty();
    ch1.set_duty(max_duty / 2);
    ch1.enable();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
