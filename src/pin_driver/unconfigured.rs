use crate::{Input, Led, Output, PinConfiguration, PinDescriptor};

pub struct UnconfiguredPin<I2C> {
    bus: I2C,
    pin: PinDescriptor,
}

impl<I2C> UnconfiguredPin<I2C> {
    pub(crate) fn new(bus: I2C, pin: PinDescriptor) -> Self {
        Self { bus, pin }
    }
}

impl<I2C, E> PinConfiguration<I2C, E> for UnconfiguredPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    async fn try_into_output(self) -> Result<Output<I2C>, E> {
        Output::try_new(self.bus, self.pin).await
    }

    async fn try_into_input(self) -> Result<Input<I2C>, E> {
        Input::try_new(self.bus, self.pin).await
    }

    async fn try_into_led(self) -> Result<Led<I2C>, E> {
        Led::try_new(self.bus, self.pin).await
    }
}
