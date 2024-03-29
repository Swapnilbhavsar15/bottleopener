#![no_main]
#![no_std]
#![allow(unused_imports)]
use crate::stm32::FLASH;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32g0xx_hal::flash::{FlashPage, UnlockedFlash, WriteErase};
use stm32g0xx_hal::gpio::gpioa::{PA0, PA1, PA10, PA15, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9};
use stm32g0xx_hal::gpio::gpiob::{PB, PB0, PB1};
use stm32g0xx_hal::gpio::gpioc::PC6;
use stm32g0xx_hal::gpio::{OpenDrain, Output, PushPull, PB4, PB5, PC14, PC15};
use stm32g0xx_hal::prelude::*;
use stm32g0xx_hal::stm32;
use stm32g0xx_hal::time::MicroSecond;
use stm32g0xx_hal::timer::stopwatch::Stopwatch;

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

    let mut i2c_timer = dp.TIM17.timer(&mut rcc);
    i2c_timer.start(50.micros());
    let mut i2c = bitbang_hal::i2c::I2cBB::new(scl, sda, i2c_timer);

    let mut delay = dp.TIM16.delay(&mut rcc);

    let stopwatch = Stopwatch::tim1(dp.TIM1, &mut rcc);
    let now = stopwatch.now();
    let mut elapsed = stopwatch.elapsed(now);
    let mut _bottle_opening_count: u8 = 2;

    let flash = dp.FLASH;

    let pin_led_green = gpioa.pa3.into_push_pull_output();
    let pin_switch = gpioa.pa0.into_push_pull_output();

    let pin_led_red_128 = gpioc.pc6.into_push_pull_output();
    let pin_led_red_64 = gpioa.pa8.into_push_pull_output();
    let pin_led_red_32 = gpiob.pb1.into_push_pull_output();
    let pin_led_red_16 = gpiob.pb0.into_push_pull_output();
    let pin_led_red_8 = gpioa.pa7.into_push_pull_output();
    let pin_led_red_4 = gpioa.pa6.into_push_pull_output();
    let pin_led_red_2 = gpioa.pa5.into_push_pull_output();
    let pin_led_red_1 = gpioa.pa4.into_push_pull_output();
    let pin_en_accl_lower_right = gpioa.pa2.into_push_pull_output();
    let pin_en_accl_lower_mid = gpioc.pc15.into_push_pull_output();
    let pin_en_accl_lower_left = gpioc.pc14.into_push_pull_output();
    let pin_en_accl_upper_right = gpioa.pa15.into_push_pull_output();
    let pin_en_accl_upper_mid = gpiob.pb4.into_push_pull_output();
    let pin_en_accl_upper_left = gpiob.pb5.into_push_pull_output();

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
        pin_switch,
        pin_en_accl_lower_right,
        pin_en_accl_lower_mid,
        pin_en_accl_lower_left,
        pin_en_accl_upper_right,
        pin_en_accl_upper_mid,
        pin_en_accl_upper_left,
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
    pins.pin_en_accl_upper_right.set_high().ok();
    pins.pin_en_accl_lower_right.set_low().ok();
    pins.pin_en_accl_lower_mid.set_low().ok();
    pins.pin_en_accl_lower_left.set_low().ok();
    pins.pin_en_accl_upper_mid.set_low().ok();
    pins.pin_en_accl_upper_left.set_low().ok();
    delay.delay(5.micros()); //turn on time for the accelerometer

    pins.pin_green.set_high().ok();

    let mut buf: [u8; 1] = [0];
    let mut xout_up;
    let mut xout_low;
    let mut yout_up;
    let mut yout_low;
    let mut zout_up;
    let mut zout_low;
    const VECTOR_SIZE: i16 = 24; // size of the vector used
                                 // addresses of accelerometer as well as its registers
    const ADDR: u8 = 0x15;
    const ADDR_X_UPPER: u8 = 0x03;
    const ADDR_X_LOWER: u8 = 0x04;
    const ADDR_Y_UPPER: u8 = 0x05;
    const ADDR_Y_LOWER: u8 = 0x06;
    const ADDR_Z_UPPER: u8 = 0x07;
    const ADDR_Z_LOWER: u8 = 0x08;
    let mut state = State {
        x: Vec::new(),
        y: Vec::new(),
        z: Vec::new(),
        x_avg: Vec::new(),
        y_avg: Vec::new(),
        z_avg: Vec::new(),
    };

    loop {
        let mut sum_x = 0;
        let mut sum_y = 0;
        let mut sum_z = 0;
        match i2c.write_read(ADDR, &[ADDR_X_UPPER], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        xout_up = buf[0];

        match i2c.write_read(ADDR, &[ADDR_X_LOWER], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        xout_low = buf[0];
        match i2c.write_read(ADDR, &[ADDR_Y_UPPER], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        yout_up = buf[0];
        match i2c.write_read(ADDR, &[ADDR_Y_LOWER], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        yout_low = buf[0];
        match i2c.write_read(ADDR, &[ADDR_Z_UPPER], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        zout_up = buf[0];
        match i2c.write_read(ADDR, &[ADDR_Z_LOWER], &mut buf) {
            Ok(_) => rprintln!(""),
            Err(err) => rprintln!("error: {:?}", err),
        }
        zout_low = buf[0];

        if state.x.is_full() {
            state.x.remove(0);
            state.y.remove(0);
            state.z.remove(0);
        }
        let x = i16::from_be_bytes([xout_up, xout_low]) >> 4; //Shifted 4 bits data is 12 bits
        state.x.push(x).ok();
        let y = i16::from_be_bytes([yout_up, yout_low]) >> 4; //Shifted 4 bits data is 12 bits
        state.y.push(y).ok();
        let z = i16::from_be_bytes([zout_up, zout_low]) >> 4; //Shifted 4 bits data is 12 bits
        state.z.push(z).ok();

        rprintln!("state.x");
        for a in &state.x {
            rprintln!("{}", a);
            sum_x += a; // sum to calculate average
        }
        rprintln!("state.y");
        for b in &state.y {
            rprintln!("{}", b); // sum to calculate average
            sum_y += b;
        }
        rprintln!("state.z");
        for c in &state.z {
            rprintln!("{}", c); // sum to calculate average
            sum_z += c;
        }

        average(
            sum_x / VECTOR_SIZE,
            sum_y / VECTOR_SIZE,
            sum_z / VECTOR_SIZE,
            &mut state,
        );
        if state.x.is_full() {
            let a: bool = detect(&state);
            elapsed = stopwatch.elapsed(now);
            if a {
                rprintln!("Bottle opened");
                pins.pin_red_128.set_high().ok();
                _bottle_opening_count += 1;
            } else if elapsed > MicroSecond::micros(5_000_000) {
                pins.pin_switch.set_high().ok();
                pins.pin_red_128.set_high().ok();
            }
        }

        delay.delay(5.millis());
    }
}

fn average(x: i16, y: i16, z: i16, state: &mut State) {
    // Funtion to store values in average vector
    if state.x_avg.is_full() {
        state.x_avg.remove(0);
        state.y_avg.remove(0);
        state.z_avg.remove(0);

        state.x_avg.push(x).unwrap();
        state.y_avg.push(y).unwrap();
        state.z_avg.push(z).unwrap();

        rprintln!("X_avg");
        for a in &state.x_avg {
            rprintln!("{}", a);
        }
        rprintln!("Y_avg");
        for b in &state.y_avg {
            rprintln!("{}", b);
        }
        rprintln!("Z_avg");
        for c in &state.z_avg {
            rprintln!("{}", c);
        }
    }
}

fn detect(state: &State) -> bool {
    let x_avg1: i16 = state.x_avg[23];
    let x_avg2: i16 = state.x_avg[0];
    let x2: i16 = state.x[23];
    let x1: i16 = state.x[22];
    let mut avg: i16 = 0;
    let avg_drop: i16 = 400; //Drop in avg vector when bottle opened
    let range: i16 = 20; //limit of range for avg to which it can fluctuate
    for a in 0..8 {
        avg += state.x_avg[a];
    }
    avg /= 8;
    // All zeros set according to data analysis for detection
    x1 > 0
        && x2 < 0
        && x2 - x1 < (0)
        && x_avg2 - x_avg1 <= avg_drop
        && x_avg1 > 0
        && x_avg2 > 0
        && avg - range <= state.x_avg[0]
        && state.x_avg[0] <= avg + range
        && avg - range <= state.x_avg[1]
        && state.x_avg[1] <= avg + range
        && avg - range <= state.x_avg[2]
        && state.x_avg[2] <= avg + range
        && avg - range <= state.x_avg[3]
        && state.x_avg[3] <= avg + range
        && avg - range <= state.x_avg[4]
        && state.x_avg[4] <= avg + range
        && avg - range <= state.x_avg[5]
        && state.x_avg[5] <= avg + range
}

struct State {
    x: Vec<i16, 24>,
    y: Vec<i16, 24>,
    z: Vec<i16, 24>,
    x_avg: Vec<i16, 24>,
    y_avg: Vec<i16, 24>,
    z_avg: Vec<i16, 24>,
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
    pin_en_accl_lower_right: PA2<Output<PushPull>>,
    pin_en_accl_lower_mid: PC15<Output<PushPull>>,
    pin_en_accl_lower_left: PC14<Output<PushPull>>,
    pin_en_accl_upper_right: PA15<Output<PushPull>>,
    pin_en_accl_upper_mid: PB4<Output<PushPull>>,
    pin_en_accl_upper_left: PB5<Output<PushPull>>,
}
