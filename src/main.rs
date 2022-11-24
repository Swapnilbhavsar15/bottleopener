#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32g0xx_hal::prelude::*;
use stm32g0xx_hal::stm32;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Test");

    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let gpioa = dp.GPIOA.split(&mut rcc);

    let mut pin_led_green = gpioa.pa3.into_push_pull_output();
    let mut pin_switch = gpioa.pa0.into_push_pull_output();
    let mut pin_led_red_128 = gpioa.pc6.into_push_pull_output();
    let mut pin_led_red_64 = gpioa.pa8.into_push_pull_output();
    let mut pin_led_red_32 = gpioa.pb1.into_push_pull_output();
    let mut pin_led_red_16 = gpioa.pb0.into_push_pull_output();
    let mut pin_led_red_8 = gpioa.pa7.into_push_pull_output();
    let mut pin_led_red_4 = gpioa.pa6.into_push_pull_output();
    let mut pin_led_red_2 = gpioa.pa5.into_push_pull_output();
    let mut pin_led_red_1 = gpioa.pa4.into_push_pull_output();

    pin_led_green.set_low().ok();

    loop {
        for _ in 0..1_000_000 {
            pin_led_green.set_low().ok();
        }
        for _ in 0..1_000_000 {
            pin_led_green.set_high().ok();
        }
    }
}
