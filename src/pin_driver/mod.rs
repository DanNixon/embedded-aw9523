mod input;
mod led;
mod output;

pub use input::Input;
pub use led::Led;
pub use output::Output;

#[allow(async_fn_in_trait)]
pub trait PinConfiguration<I2C, E> {
    async fn try_into_output(self) -> Result<Output<I2C>, E>;
    async fn try_into_input(self) -> Result<Input<I2C>, E>;
    async fn try_into_led(self) -> Result<Led<I2C>, E>;
}
