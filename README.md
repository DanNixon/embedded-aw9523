# `embedded-aw9523`

[![Crates.io Version](https://img.shields.io/crates/v/embedded-aw9523)](https://crates.io/crates/embedded-aw9523)
[![docs.rs](https://img.shields.io/docsrs/embedded-aw9523)](https://docs.rs/embedded-aw9523/latest/embedded_aw9523/)

An embedded-hal first async AW9523 driver.

Unlike other drivers, this exposes pins via appropriate `embedded-hal` and `embedded-hal-async` traits, so pins can be used exactly as they would otherwise be.
e.g. [need to use a digital output as the chip select for an SPI device](https://github.com/DanNixon/ethernet-hexpansion)? No problem.

## Resources

- [AW9523 datasheet](https://cdn-shop.adafruit.com/product-files/4886/AW9523+English+Datasheet.pdf)
