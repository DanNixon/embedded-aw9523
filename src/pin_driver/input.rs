use crate::{
    LedPin, OutputPin, PinConfiguration,
    descriptor::{DescriptorExt, PinDescriptor, Port},
    operations::{
        GpioDirection, PinMode, Register, read_register, set_io_direction, set_pin_mode,
        write_register,
    },
};
use defmt::debug;
use embedded_hal::digital::ErrorKind;

pub struct InputPin<I2C> {
    bus: I2C,
    pin: PinDescriptor,
}

impl<I2C, E> InputPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub(crate) async fn try_new(mut bus: I2C, pin: PinDescriptor) -> Result<Self, E> {
        set_pin_mode(&mut bus, &pin, PinMode::Gpio).await?;
        set_io_direction(&mut bus, &pin, GpioDirection::Input).await?;
        Ok(Self { bus, pin })
    }

    pub async fn set_interrupt(&mut self, enable: bool) -> Result<(), E> {
        let register = match self.pin.port() {
            Port::Port0 => Register::INT_P0,
            Port::Port1 => Register::INT_P1,
        };

        let value = read_register(&mut self.bus, self.pin.address().address(), register).await?;

        let value = match enable {
            true => value & !self.pin.pin().bit(),
            false => value | self.pin.pin().bit(),
        };

        write_register(&mut self.bus, self.pin.address().address(), register, value).await?;

        debug!("Set interrupt enable for pin {} to {}", self.pin, enable);
        Ok(())
    }
}

impl<I2C, E> PinConfiguration<I2C, E> for InputPin<I2C>
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

impl<I2C> embedded_hal::digital::ErrorType for InputPin<I2C> {
    type Error = ErrorKind;
}

impl<I2C, E> embedded_hal::digital::InputPin for InputPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_low()?)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        let register = match self.pin.port() {
            Port::Port0 => Register::INPUT_P0,
            Port::Port1 => Register::INPUT_P1,
        };

        let value = embassy_futures::block_on(async {
            read_register(&mut self.bus, self.pin.address().address(), register)
                .await
                .map_err(|_| ErrorKind::Other)
        })?;

        let is_low = value & self.pin.pin().bit() == 0;

        debug!("Pin {} is low? {}", self.pin, is_low);
        Ok(is_low)
    }
}

impl<I2C, E> crate::async_traits::digital::InputPin for InputPin<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    async fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_low().await?)
    }

    async fn is_low(&mut self) -> Result<bool, Self::Error> {
        let register = match self.pin.port() {
            Port::Port0 => Register::INPUT_P0,
            Port::Port1 => Register::INPUT_P1,
        };

        let value = read_register(&mut self.bus, self.pin.address().address(), register)
            .await
            .map_err(|_| ErrorKind::Other)?;

        let is_low = value & self.pin.pin().bit() == 0;

        debug!("Pin {} is low? {}", self.pin, is_low);
        Ok(is_low)
    }
}
