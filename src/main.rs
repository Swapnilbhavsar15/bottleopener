#![no_main]
#![no_std]

use cortex_m::asm::delay;
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

    let sda = gpioa.pa12.into_open_drain_output_in_state(PinState::Low);
    let scl = gpioa.pa11.into_open_drain_output_in_state(PinState::Low);

    let mut timer = dp.TIM17.timer(&mut rcc);
    timer.start(50.micros());

    let mut delay = dp.TIM16.delay(&mut rcc);

    // let mut i2c = dp.I2C1.i2c(sda, scl, i2c::Config::new(100.khz()), &mut rcc);
    let mut i2c = bitbang_hal::i2c::I2cBB::new(sda, scl, timer);
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
    pins.pin_en_accl_top.set_low().ok();
    pins.pin_en_accl_bottom.set_low().ok();
    delay.delay(500.millis());

    pins.pin_green.set_high().ok();
    pins.pin_red_128.set_high().ok();

    let mut buf: [u8; 1] = [0];
    let mut xout_up;
    let mut xout_low;
    /*let mut yout_up;
    let mut yout_low;
    let mut zout_up;
    let mut zout_low;*/
    loop {
        match i2c.write_read(0x15, &[0x03], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        xout_up = buf[0];

        match i2c.write_read(0x15, &[0x04], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        xout_low = buf[0];
        /*match i2c.write_read(0x15, &[0x05], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        yout_up = buf[0];
        match i2c.write_read(0x15, &[0x06], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        yout_low = buf[0];
        match i2c.write_read(0x15, &[0x07], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        zout_up = buf[0];
        match i2c.write_read(0x15, &[0x08], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        zout_low = buf[0];*/
        let x1 = i16::from_be_bytes([xout_up, xout_low]) >> 4;
        delay.delay(750.millis());
        match i2c.write_read(0x15, &[0x03], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        xout_up = buf[0];

        match i2c.write_read(0x15, &[0x04], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        xout_low = buf[0];
        let x2 = i16::from_be_bytes([xout_up, xout_low]) >> 4;
        let op = detection(x1, x2);
        if op == true{
            rprintln!("Bottle opened");
             pins.pin_red_2.set_high().ok();
             delay.delay(350.millis());
             pins.pin_red_2.set_low().ok();
        }
        /*let x = i16::from_be_bytes([xout_up, xout_low]) >> 4;
        rprintln!("Xout {}", x);
        let y = i16::from_be_bytes([yout_up, yout_low]) >> 4;
        rprintln!("Yout {}", y);
        let z = i16::from_be_bytes([zout_up, zout_low]) >> 4;
        rprintln!("Zout {}", z);

        delay.delay(300.millis())*/;
    }
}
fn detection(x1: i16, x2: i16) -> bool {
    if x1 - x2 >= 600 {
        return true;
    } else {
        return false;
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
