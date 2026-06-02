use crate::{
    InputPin, LedPin, PinConfiguration,
    descriptor::PinDescriptor,
    operations::{GpioDirection, PinMode, set_io_direction, set_io_state, set_pin_mode},
};
use embedded_hal::digital::{ErrorKind, PinState};

pub struct OutputPin<I2C> {
    bus: I2C,
    pin: PinDescriptor,
}

impl<I2C, E> OutputPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub(crate) async fn try_new(mut bus: I2C, pin: PinDescriptor) -> Result<Self, E> {
        set_pin_mode(&mut bus, &pin, PinMode::Gpio).await?;
        set_io_direction(&mut bus, &pin, GpioDirection::Output).await?;
        Ok(Self { bus, pin })
    }
}

impl<I2C, E> PinConfiguration<I2C, E> for OutputPin<I2C>
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

impl<I2C> embedded_hal::digital::ErrorType for OutputPin<I2C> {
    type Error = ErrorKind;
}

impl<I2C, E> embedded_hal::digital::OutputPin for OutputPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High)
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low)
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        embassy_futures::block_on(async {
            set_io_state(&mut self.bus, &self.pin, state)
                .await
                .map_err(|_| ErrorKind::Other)
        })
    }
}

impl<I2C, E> crate::async_traits::digital::OutputPin for OutputPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    async fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High).await
    }

    async fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low).await
    }

    async fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        set_io_state(&mut self.bus, &self.pin, state)
            .await
            .map_err(|_| ErrorKind::Other)
    }
}
