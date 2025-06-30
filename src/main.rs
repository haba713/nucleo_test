#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
};
use cortex_m::delay::Delay;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    let mut delay = Delay::new(cp.SYST, clocks.hclk().raw());
    loop {
        led.toggle();
        delay.delay_ms(2000_u32);
    }
}