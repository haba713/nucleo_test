#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // initialize device peripherals
    let device_peripherals = pac::Peripherals::take().unwrap();
    // initialize core peripherals
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    // configure reset and clock control
    let clock_control = device_peripherals.RCC.constrain();
    // set system clock to 16 MHz and freeze configuration
    let clock_config = clock_control.cfgr.sysclk(16.MHz()).freeze();
    // split gpiob for led control
    let gpio_bank_b = device_peripherals.GPIOB.split();

    // configure led pins as push-pull outputs
    let led_array: [&mut dyn OutputPin<Error = core::convert::Infallible>; 3] = [
        &mut gpio_bank_b.pb0.into_push_pull_output(),
        &mut gpio_bank_b.pb7.into_push_pull_output(),
        &mut gpio_bank_b.pb14.into_push_pull_output(),
    ];

    // create delay object using system timer and clock frequency
    let mut delay_timer = Delay::new(core_peripherals.SYST, clock_config.hclk().raw());
    // track current led index
    let mut current_led_index = 0;
    // set direction: 1 for forward, -1 for backward
    let mut direction = 1;

    // initialize with first led on
    led_array[0].set_high().unwrap();

    loop {
        // turn off current led
        led_array[current_led_index as usize].set_low().unwrap();

        // update index for next led
        current_led_index = (current_led_index as i8 + direction) as usize;

        // reverse direction at array boundaries
        if current_led_index == led_array.len() - 1 || current_led_index == 0 {
            direction = -direction;
        }

        // turn on next led
        led_array[current_led_index as usize].set_high().unwrap();

        // wait 200ms before next step
        delay_timer.delay_ms(200_u32);
    }
}
