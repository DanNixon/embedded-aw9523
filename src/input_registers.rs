use crate::{
    descriptor::{Address, DescriptorExt, Port},
    operations::read_input_registers,
};
use embedded_hal::digital::PinState;
use heapless::index_map::FnvIndexMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputRegisters {
    registers: FnvIndexMap<(Address, Port), u8, 8>,
}

impl InputRegisters {
    pub async fn read<I2C, E>(i2c: &mut I2C, addresses: &[Address]) -> Result<Self, E>
    where
        I2C: embedded_hal_async::i2c::I2c<Error = E>,
    {
        let mut registers = FnvIndexMap::new();

        for addr in addresses {
            let (port0, port1) = read_input_registers(i2c, *addr).await?;

            let _ = registers.insert((*addr, Port::Port0), port0);
            let _ = registers.insert((*addr, Port::Port1), port1);
        }

        Ok(Self { registers })
    }

    pub fn pin_state<PIN: DescriptorExt>(
        &self,
        pin: &PIN,
    ) -> Result<PinState, InputRegistersError> {
        match self.registers.get(&(pin.address(), pin.port())) {
            Some(reg) => {
                let bit = pin.pin().bit();

                let state = if reg & bit == 0 {
                    PinState::Low
                } else {
                    PinState::High
                };

                Ok(state)
            }
            None => Err(InputRegistersError::IncorrectAddress),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputRegistersError {
    /// The address of the pin descriptor is not the address of the AW9523 that the registers were read from
    IncorrectAddress,
}
