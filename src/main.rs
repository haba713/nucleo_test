#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
};
use cortex_m::delay::Delay;

#[derive(Clone, Copy)]
enum LedId {
    Led1,
    Led2,
    Led3,
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();
    let gpiob = dp.GPIOB.split();

    let mut led1 = gpiob.pb0.into_push_pull_output();
    let mut led2 = gpiob.pb7.into_push_pull_output();
    let mut led3 = gpiob.pb14.into_push_pull_output();

    led1.set_low();
    led2.set_low();
    led3.set_low();

    let sequence = [LedId::Led1, LedId::Led2, LedId::Led3, LedId::Led2];

    let mut delay = Delay::new(cp.SYST, clocks.hclk().raw());
    let mut index = 0;

    loop {
        led1.set_low();
        led2.set_low();
        led3.set_low();

        match sequence[index] {
            LedId::Led1 => led1.set_high(),
            LedId::Led2 => led2.set_high(),
            LedId::Led3 => led3.set_high(),
        }

        delay.delay_ms(200_u32);

        index = (index + 1) % sequence.len();
    }
}