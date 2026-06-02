mod input;
mod led;
mod output;
mod unconfigured;

pub use input::InputPin;
pub use led::LedPin;
pub use output::OutputPin;
pub use unconfigured::UnconfiguredPin;

#[allow(async_fn_in_trait)]
pub trait PinConfiguration<I2C, E> {
    async fn try_into_output(self) -> Result<OutputPin<I2C>, E>;
    async fn try_into_input(self) -> Result<InputPin<I2C>, E>;
    async fn try_into_led(self) -> Result<LedPin<I2C>, E>;
}
