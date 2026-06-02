use crate::{Address, Pin, PinDescriptor, Port, UnconfiguredPin};

pub struct Pins<I2C> {
    pub port0_pin0: UnconfiguredPin<I2C>,
    pub port0_pin1: UnconfiguredPin<I2C>,
    pub port0_pin2: UnconfiguredPin<I2C>,
    pub port0_pin3: UnconfiguredPin<I2C>,
    pub port0_pin4: UnconfiguredPin<I2C>,
    pub port0_pin5: UnconfiguredPin<I2C>,
    pub port0_pin6: UnconfiguredPin<I2C>,
    pub port0_pin7: UnconfiguredPin<I2C>,

    pub port1_pin0: UnconfiguredPin<I2C>,
    pub port1_pin1: UnconfiguredPin<I2C>,
    pub port1_pin2: UnconfiguredPin<I2C>,
    pub port1_pin3: UnconfiguredPin<I2C>,
    pub port1_pin4: UnconfiguredPin<I2C>,
    pub port1_pin5: UnconfiguredPin<I2C>,
    pub port1_pin6: UnconfiguredPin<I2C>,
    pub port1_pin7: UnconfiguredPin<I2C>,
}

impl<I2C: Clone> Pins<I2C> {
    pub(crate) fn new(i2c: I2C, address: Address) -> Self {
        Self {
            port0_pin0: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin0),
            ),
            port0_pin1: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin1),
            ),
            port0_pin2: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin2),
            ),
            port0_pin3: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin3),
            ),
            port0_pin4: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin4),
            ),
            port0_pin5: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin5),
            ),
            port0_pin6: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin6),
            ),
            port0_pin7: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin7),
            ),

            port1_pin0: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin0),
            ),
            port1_pin1: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin1),
            ),
            port1_pin2: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin2),
            ),
            port1_pin3: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin3),
            ),
            port1_pin4: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin4),
            ),
            port1_pin5: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin5),
            ),
            port1_pin6: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin6),
            ),
            port1_pin7: UnconfiguredPin::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin7),
            ),
        }
    }
}
