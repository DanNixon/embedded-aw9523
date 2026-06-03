use crate::{
    descriptor::{Address, DescriptorExt, Port},
    operations::read_input_registers,
};
use defmt::Format;
use embedded_hal::digital::PinState;

#[derive(Debug, Format, PartialEq, Eq, Clone)]
pub struct InputRegisters {
    address: Address,
    port0: u8,
    port1: u8,
}

impl InputRegisters {
    pub async fn read<I2C, E>(i2c: &mut I2C, address: Address) -> Result<Self, E>
    where
        I2C: embedded_hal_async::i2c::I2c<Error = E>,
    {
        let (port0, port1) = read_input_registers(i2c, Address::Addr58).await?;

        Ok(Self {
            address,
            port0,
            port1,
        })
    }

    pub fn pin_state<PIN: DescriptorExt>(
        &self,
        pin: &PIN,
    ) -> Result<PinState, InputRegistersError> {
        if pin.address() != self.address {
            return Err(InputRegistersError::IncorrectAddress);
        }

        let reg = match pin.port() {
            Port::Port0 => self.port0,
            Port::Port1 => self.port1,
        };

        let bit = pin.pin().bit();

        let state = if reg & bit != 0 {
            PinState::Low
        } else {
            PinState::High
        };

        Ok(state)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputRegistersError {
    /// The address of the pin descriptor is not the address of the AW9523 that the registers were read from
    IncorrectAddress,
}
