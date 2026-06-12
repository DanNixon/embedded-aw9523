use crate::{
    Pins,
    descriptor::Address,
    operations::{Register, read_register, write_register},
};
use defmt::{debug, info};

pub struct Aw9523<I2C: 'static> {
    bus: I2C,
    address: Address,
    pins: Option<Pins<I2C>>,
}

impl<I2C, E> Aw9523<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E> + Clone,
{
    pub async fn new(bus: I2C, address: Address) -> Result<Self, E> {
        let pins = Some(Pins::new(bus.clone(), address).await?);
        Ok(Self { bus, address, pins })
    }

    pub async fn init(&mut self) -> Result<(), E> {
        let id = read_register(&mut self.bus, self.address.address(), Register::ID).await?;
        debug!("AW9523B with id {} found at address {}", id, self.address);

        // Set port 0 to push pull mode and LED current to Imax
        write_register(
            &mut self.bus,
            self.address.address(),
            Register::CTL,
            0b00010000,
        )
        .await?;

        // Disable all interrupts
        write_register(
            &mut self.bus,
            self.address.address(),
            Register::INT_P0,
            0b11111111,
        )
        .await?;
        write_register(
            &mut self.bus,
            self.address.address(),
            Register::INT_P1,
            0b11111111,
        )
        .await?;

        info!("IO expander init");
        Ok(())
    }

    pub async fn reset(&mut self) -> Result<(), E> {
        write_register(&mut self.bus, self.address.address(), Register::SW_RSTN, 0).await?;
        info!("IO expander reset");
        Ok(())
    }

    pub fn pins(&mut self) -> Pins<I2C> {
        self.pins.take().expect("can only get pins once")
    }
}
