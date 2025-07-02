#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
};
use embedded_hal::digital::OutputPin;
use cortex_m::delay::Delay;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();
    let gpiob = dp.GPIOB.split();

    let leds: [&mut dyn OutputPin<Error = core::convert::Infallible>; 3] = [
        &mut gpiob.pb0.into_push_pull_output(),
        &mut gpiob.pb7.into_push_pull_output(),
        &mut gpiob.pb14.into_push_pull_output(),
    ];

    let mut delay = Delay::new(cp.SYST, clocks.hclk().raw());
    let mut index = 0;
    let mut direction = 1; // 1 for forward, -1 for backward

    // initialize with first led on
    leds[0].set_high().unwrap();

    loop {
        // turn off current led
        leds[index as usize].set_low().unwrap();

        // update index for next led
        index = (index as i8 + direction) as usize;

        // reverse direction at the ends
        if index == leds.len() - 1 || index == 0 {
            direction = -direction;
        }

        // turn on next led
        leds[index as usize].set_high().unwrap();

        // wait 200ms before next step
        delay.delay_ms(200_u32);
    }
}