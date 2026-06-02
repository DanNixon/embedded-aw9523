#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_rp::peripherals::I2C1;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::Timer;
use embedded_aw9523::{
    Address, Aw9523, LedPin, PinConfiguration,
    async_traits::{digital::InputPin, pwm::SetDutyCycle},
};
use embedded_hal_async::i2c::I2c;
use panic_probe as _;
use portable_atomic as _;
use static_cell::StaticCell;

embassy_rp::bind_interrupts!(struct Irqs {
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Hello, world!");

    let sda = p.PIN_14;
    let scl = p.PIN_15;

    let config = embassy_rp::i2c::Config::default();
    let mut bus = embassy_rp::i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, config);

    // Set i2c mux port
    bus.write(0x77_u8, &[0b10000000]).await.unwrap();

    static I2C_BUS: StaticCell<
        Mutex<
            NoopRawMutex,
            embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C1, embassy_rp::i2c::Async>,
        >,
    > = StaticCell::new();
    let bus = I2C_BUS.init(Mutex::new(bus));

    let mut pin_control = Aw9523::new(I2cDevice::new(bus), Address::Addr58);
    pin_control.init().await.unwrap();
    let pins = pin_control.pins();

    let led_pins = PinsWithLeds {
        pin1: pins.port0_pin2.try_into_led().await.unwrap(),
        pin2: pins.port0_pin3.try_into_led().await.unwrap(),
        pin3: pins.port0_pin4.try_into_led().await.unwrap(),
        pin4: pins.port0_pin5.try_into_led().await.unwrap(),
        pin5: pins.port0_pin6.try_into_led().await.unwrap(),
    };

    let switch_pins = PinsWithSwitches {
        pin1: pins.port0_pin7.try_into_input().await.unwrap(),
        pin2: pins.port1_pin4.try_into_input().await.unwrap(),
        pin3: pins.port1_pin5.try_into_input().await.unwrap(),
        pin4: pins.port1_pin6.try_into_input().await.unwrap(),
        pin5: pins.port1_pin7.try_into_input().await.unwrap(),
    };

    spawner.spawn(ramp_leds(led_pins).unwrap());
    spawner.spawn(echo_inputs(switch_pins).unwrap());
}

struct PinsWithLeds<I2C> {
    pin1: LedPin<I2C>,
    pin2: LedPin<I2C>,
    pin3: LedPin<I2C>,
    pin4: LedPin<I2C>,
    pin5: LedPin<I2C>,
}

struct PinsWithSwitches<I2C> {
    pin1: embedded_aw9523::InputPin<I2C>,
    pin2: embedded_aw9523::InputPin<I2C>,
    pin3: embedded_aw9523::InputPin<I2C>,
    pin4: embedded_aw9523::InputPin<I2C>,
    pin5: embedded_aw9523::InputPin<I2C>,
}

#[embassy_executor::task]
async fn ramp_leds(
    mut led_pins: PinsWithLeds<
        I2cDevice<
            'static,
            NoopRawMutex,
            embassy_rp::i2c::I2c<'static, I2C1, embassy_rp::i2c::Async>,
        >,
    >,
) -> ! {
    led_pins.pin1.set_duty_cycle_percent(0).await.unwrap();
    led_pins.pin2.set_duty_cycle_percent(0).await.unwrap();
    led_pins.pin3.set_duty_cycle_percent(0).await.unwrap();
    led_pins.pin4.set_duty_cycle_percent(0).await.unwrap();
    led_pins.pin5.set_duty_cycle_percent(0).await.unwrap();

    loop {
        ramp(&mut led_pins.pin1).await;
        ramp(&mut led_pins.pin2).await;
        ramp(&mut led_pins.pin3).await;
        ramp(&mut led_pins.pin4).await;
        ramp(&mut led_pins.pin5).await;
    }
}

async fn ramp<I2C: embedded_hal_async::i2c::I2c>(pin: &mut LedPin<I2C>) {
    for duty in 0u8..=100 {
        pin.set_duty_cycle_percent(duty).await.unwrap();
        Timer::after_millis(5).await;
    }

    for duty in (0u8..100).rev() {
        pin.set_duty_cycle_percent(duty).await.unwrap();
        Timer::after_millis(5).await;
    }
}

#[embassy_executor::task]
async fn echo_inputs(
    mut switch_pins: PinsWithSwitches<
        I2cDevice<
            'static,
            NoopRawMutex,
            embassy_rp::i2c::I2c<'static, I2C1, embassy_rp::i2c::Async>,
        >,
    >,
) -> ! {
    loop {
        info!("Pin 1 is {}", switch_pins.pin1.is_high().await.unwrap());
        info!("Pin 2 is {}", switch_pins.pin2.is_high().await.unwrap());
        info!("Pin 3 is {}", switch_pins.pin3.is_high().await.unwrap());
        info!("Pin 4 is {}", switch_pins.pin4.is_high().await.unwrap());
        info!("Pin 5 is {}", switch_pins.pin5.is_high().await.unwrap());
        Timer::after_millis(500).await;
    }
}
