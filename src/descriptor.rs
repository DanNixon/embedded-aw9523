use defmt::Format;
use num_enum::TryFromPrimitive;

pub trait DescriptorExt {
    fn address(&self) -> Address;
    fn port(&self) -> Port;
    fn pin(&self) -> Pin;
}

/// The i2c addresses that an AW9523 can be configured to be at.
#[derive(Format, Debug, Copy, Clone, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Address {
    Addr58 = 0x58,
    Addr59 = 0x59,
    Addr5A = 0x5A,
    Addr5B = 0x5B,
}

impl Address {
    pub(crate) fn address(&self) -> u8 {
        *self as u8
    }
}

/// Blocks of 8 pins.
#[derive(Format, Debug, Copy, Clone, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Port {
    Port0,
    Port1,
}

/// The pin in a port.
#[derive(Format, Debug, Copy, Clone, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Pin {
    Pin0 = 0b00000001,
    Pin1 = 0b00000010,
    Pin2 = 0b00000100,
    Pin3 = 0b00001000,
    Pin4 = 0b00010000,
    Pin5 = 0b00100000,
    Pin6 = 0b01000000,
    Pin7 = 0b10000000,
}

impl Pin {
    pub(crate) fn bit(&self) -> u8 {
        *self as u8
    }
}

#[derive(Format)]
pub(crate) struct PinDescriptor {
    address: Address,
    port: Port,
    pin: Pin,
}

impl PinDescriptor {
    pub(crate) fn new(address: Address, port: Port, pin: Pin) -> Self {
        Self { address, port, pin }
    }
}

impl DescriptorExt for PinDescriptor {
    fn address(&self) -> Address {
        self.address
    }

    fn port(&self) -> Port {
        self.port
    }

    fn pin(&self) -> Pin {
        self.pin
    }
}
