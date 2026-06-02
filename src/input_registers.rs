use crate::{
    descriptor::{Address, DescriptorExt, Port},
    operations::read_input_registers,
};
use defmt::Format;

#[derive(Debug, Format, PartialEq, Eq, Clone)]
pub struct InputRegisters {
    addr58_port0: u8,
    addr58_port1: u8,
    addr59_port0: u8,
    addr59_port1: u8,
    addr5a_port0: u8,
    addr5a_port1: u8,
    addr5b_port0: u8,
    addr5b_port1: u8,
}

impl InputRegisters {
    pub async fn read<I2C, E>(i2c: &mut I2C, addrs: &[Address]) -> Result<Self, E>
    where
        I2C: embedded_hal_async::i2c::I2c<Error = E>,
    {
        let (addr58_port0, addr58_port1) = if addrs.contains(&Address::Addr58) {
            read_input_registers(i2c, Address::Addr58).await?
        } else {
            (0u8, 0u8)
        };

        let (addr59_port0, addr59_port1) = if addrs.contains(&Address::Addr59) {
            read_input_registers(i2c, Address::Addr59).await?
        } else {
            (0u8, 0u8)
        };

        let (addr5a_port0, addr5a_port1) = if addrs.contains(&Address::Addr5A) {
            read_input_registers(i2c, Address::Addr5A).await?
        } else {
            (0u8, 0u8)
        };

        let (addr5b_port0, addr5b_port1) = if addrs.contains(&Address::Addr5B) {
            read_input_registers(i2c, Address::Addr5B).await?
        } else {
            (0u8, 0u8)
        };

        Ok(Self {
            addr58_port0,
            addr58_port1,
            addr59_port0,
            addr59_port1,
            addr5a_port0,
            addr5a_port1,
            addr5b_port0,
            addr5b_port1,
        })
    }

    pub fn pin_state<PIN: DescriptorExt>(&self, pin: &PIN) -> bool {
        let reg = match pin.address() {
            Address::Addr58 => match pin.port() {
                Port::Port0 => self.addr58_port0,
                Port::Port1 => self.addr58_port1,
            },
            Address::Addr59 => match pin.port() {
                Port::Port0 => self.addr59_port0,
                Port::Port1 => self.addr59_port1,
            },
            Address::Addr5A => match pin.port() {
                Port::Port0 => self.addr5a_port0,
                Port::Port1 => self.addr5a_port1,
            },
            Address::Addr5B => match pin.port() {
                Port::Port0 => self.addr5b_port0,
                Port::Port1 => self.addr5b_port1,
            },
        };

        let bit = pin.pin().bit();

        reg & bit != 0
    }
}
