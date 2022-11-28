#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32g0xx_hal::gpio::gpioa::{PA0, PA1, PA10, PA15, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9};
use stm32g0xx_hal::gpio::gpiob::{PB, PB0, PB1};
use stm32g0xx_hal::gpio::gpioc::PC6;
use stm32g0xx_hal::gpio::{OpenDrain, Output, PushPull};
use stm32g0xx_hal::prelude::*;
use stm32g0xx_hal::{i2c, stm32};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Test");

    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    let sda = gpioa.pa10.into_open_drain_output_in_state(PinState::High);
    let scl = gpioa.pa9.into_open_drain_output_in_state(PinState::High);

    let mut i2c = dp.I2C1.i2c(sda, scl, i2c::Config::new(100.khz()), &mut rcc);
    let mut pin_led_green = gpioa.pa3.into_push_pull_output();
    let mut pin_switch = gpioa.pa0.into_push_pull_output();
    let mut pin_led_red_128 = gpioc.pc6.into_push_pull_output();
    let mut pin_led_red_64 = gpioa.pa8.into_push_pull_output();
    let mut pin_led_red_32 = gpiob.pb1.into_push_pull_output();
    let mut pin_led_red_16 = gpiob.pb0.into_push_pull_output();
    let mut pin_led_red_8 = gpioa.pa7.into_push_pull_output();
    let mut pin_led_red_4 = gpioa.pa6.into_push_pull_output();
    let mut pin_led_red_2 = gpioa.pa5.into_push_pull_output();
    let mut pin_led_red_1 = gpioa.pa4.into_push_pull_output();
    let mut pin_en_accl_bottom = gpioa.pa2.into_push_pull_output();
    let mut pin_en_accl_top = gpioa.pa15.into_push_pull_output();

    let mut pins = Pins {
        pin_red_1: pin_led_red_1,
        pin_red_2: pin_led_red_2,
        pin_red_4: pin_led_red_4,
        pin_red_8: pin_led_red_8,
        pin_red_16: pin_led_red_16,
        pin_red_32: pin_led_red_32,
        pin_red_64: pin_led_red_64,
        pin_red_128: pin_led_red_128,
        pin_green: pin_led_green,
        pin_switch: pin_switch,
        pin_en_accl_bottom: pin_en_accl_bottom,
        pin_en_accl_top: pin_en_accl_top,
    };

    pins.pin_red_128.set_low().ok();
    pins.pin_red_64.set_low().ok();
    pins.pin_red_32.set_low().ok();
    pins.pin_red_16.set_low().ok();
    pins.pin_red_8.set_low().ok();
    pins.pin_red_4.set_low().ok();
    pins.pin_red_2.set_low().ok();
    pins.pin_red_1.set_low().ok();
    pins.pin_green.set_low().ok();
    pins.pin_en_accl_top.set_high().ok();

    let mut buf: [u8; 1] = [0];
    loop {
        match i2c.read(0x03, &mut buf) {
            Ok(_) => rprintln!("Xout Upper {}", buf[0]),
            Err(err) => rprintln!("error: {:?}", err),
        }
        match i2c.read(0x04, &mut buf) {
            Ok(_) => rprintln!("Xout Upper {}", buf[0]),
            Err(err) => rprintln!("error: {:?}", err),
        }
        pins.pin_green.set_high().ok();
    }
}

struct Pins {
    pin_red_1: PA4<Output<PushPull>>,
    pin_red_2: PA5<Output<PushPull>>,
    pin_red_4: PA6<Output<PushPull>>,
    pin_red_8: PA7<Output<PushPull>>,
    pin_red_16: PB0<Output<PushPull>>,
    pin_red_32: PB1<Output<PushPull>>,
    pin_red_64: PA8<Output<PushPull>>,
    pin_red_128: PC6<Output<PushPull>>,
    pin_green: PA3<Output<PushPull>>,
    pin_switch: PA0<Output<PushPull>>,
    pin_en_accl_bottom: PA2<Output<PushPull>>,
    pin_en_accl_top: PA15<Output<PushPull>>,
}
