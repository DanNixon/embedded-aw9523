use crate::{Address, Input, Pin, PinDescriptor, Port};

/// All unconfigured pins for a single AW9523.
pub struct Pins<I2C> {
    pub port0_pin0: Input<I2C>,
    pub port0_pin1: Input<I2C>,
    pub port0_pin2: Input<I2C>,
    pub port0_pin3: Input<I2C>,
    pub port0_pin4: Input<I2C>,
    pub port0_pin5: Input<I2C>,
    pub port0_pin6: Input<I2C>,
    pub port0_pin7: Input<I2C>,

    pub port1_pin0: Input<I2C>,
    pub port1_pin1: Input<I2C>,
    pub port1_pin2: Input<I2C>,
    pub port1_pin3: Input<I2C>,
    pub port1_pin4: Input<I2C>,
    pub port1_pin5: Input<I2C>,
    pub port1_pin6: Input<I2C>,
    pub port1_pin7: Input<I2C>,
}

impl<I2C, E> Pins<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E> + Clone,
{
    pub(crate) async fn new(i2c: I2C, address: Address) -> Result<Self, E> {
        Ok(Self {
            port0_pin0: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin0),
            )
            .await?,
            port0_pin1: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin1),
            )
            .await?,
            port0_pin2: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin2),
            )
            .await?,
            port0_pin3: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin3),
            )
            .await?,
            port0_pin4: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin4),
            )
            .await?,
            port0_pin5: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin5),
            )
            .await?,
            port0_pin6: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin6),
            )
            .await?,
            port0_pin7: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port0, Pin::Pin7),
            )
            .await?,

            port1_pin0: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin0),
            )
            .await?,
            port1_pin1: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin1),
            )
            .await?,
            port1_pin2: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin2),
            )
            .await?,
            port1_pin3: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin3),
            )
            .await?,
            port1_pin4: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin4),
            )
            .await?,
            port1_pin5: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin5),
            )
            .await?,
            port1_pin6: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin6),
            )
            .await?,
            port1_pin7: Input::new(
                i2c.clone(),
                PinDescriptor::new(address, Port::Port1, Pin::Pin7),
            )
            .await?,
        })
    }
}
