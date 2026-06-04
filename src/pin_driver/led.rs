use crate::{
    Input, Output, PinConfiguration,
    descriptor::{DescriptorExt, Pin, PinDescriptor, Port},
    operations::{PinMode, Register, set_pin_mode, write_register},
};
use defmt::debug;
use embedded_hal::pwm::ErrorKind;

pub struct Led<I2C> {
    bus: I2C,
    pin: PinDescriptor,
}

impl<I2C, E> Led<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub(crate) async fn try_new(mut bus: I2C, pin: PinDescriptor) -> Result<Self, E> {
        set_pin_mode(&mut bus, &pin, PinMode::Led).await?;
        Ok(Self { bus, pin })
    }
}

impl<I2C, E> PinConfiguration<I2C, E> for Led<I2C>
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

impl<I2C> embedded_hal::pwm::ErrorType for Led<I2C> {
    type Error = ErrorKind;
}

impl<I2C, E> embedded_hal::pwm::SetDutyCycle for Led<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    fn max_duty_cycle(&self) -> u16 {
        255
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let register = get_register_for_port_and_pin(self.pin.port(), self.pin.pin());

        embassy_futures::block_on(async {
            write_register(
                &mut self.bus,
                self.pin.address().address(),
                register,
                duty as u8,
            )
            .await
            .map_err(|_| ErrorKind::Other)
        })?;

        debug!("Set pin {} to {}", self.pin, duty);
        Ok(())
    }
}

impl<I2C, E> crate::async_traits::pwm::SetDutyCycle for Led<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    async fn max_duty_cycle(&self) -> u16 {
        255
    }

    async fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let register = get_register_for_port_and_pin(self.pin.port(), self.pin.pin());

        write_register(
            &mut self.bus,
            self.pin.address().address(),
            register,
            duty as u8,
        )
        .await
        .map_err(|_| ErrorKind::Other)?;

        debug!("Set pin {} to {}", self.pin, duty);
        Ok(())
    }
}

fn get_register_for_port_and_pin(port: Port, pin: Pin) -> Register {
    match (port, pin) {
        (Port::Port0, Pin::Pin0) => Register::DIM4_P00,
        (Port::Port0, Pin::Pin1) => Register::DIM5_P01,
        (Port::Port0, Pin::Pin2) => Register::DIM6_P02,
        (Port::Port0, Pin::Pin3) => Register::DIM7_P03,
        (Port::Port0, Pin::Pin4) => Register::DIM8_P04,
        (Port::Port0, Pin::Pin5) => Register::DIM9_P05,
        (Port::Port0, Pin::Pin6) => Register::DIM10_P06,
        (Port::Port0, Pin::Pin7) => Register::DIM11_P07,
        (Port::Port1, Pin::Pin0) => Register::DIM0_P10,
        (Port::Port1, Pin::Pin1) => Register::DIM1_P11,
        (Port::Port1, Pin::Pin2) => Register::DIM2_P12,
        (Port::Port1, Pin::Pin3) => Register::DIM3_P13,
        (Port::Port1, Pin::Pin4) => Register::DIM12_P14,
        (Port::Port1, Pin::Pin5) => Register::DIM13_P15,
        (Port::Port1, Pin::Pin6) => Register::DIM14_P16,
        (Port::Port1, Pin::Pin7) => Register::DIM15_P17,
    }
}
