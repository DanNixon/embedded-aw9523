use crate::{InputPin, LedPin, OutputPin, PinConfiguration, PinDescriptor};

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
    async fn try_into_output(self) -> Result<OutputPin<I2C>, E> {
        OutputPin::try_new(self.bus, self.pin).await
    }

    async fn try_into_input(self) -> Result<InputPin<I2C>, E> {
        InputPin::try_new(self.bus, self.pin).await
    }

    async fn try_into_led(self) -> Result<LedPin<I2C>, E> {
        LedPin::try_new(self.bus, self.pin).await
    }
}
