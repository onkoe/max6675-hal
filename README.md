<!-- cargo-rdme start -->

# max6675-hal

An embedded-hal driver for the MAX6675 digital thermocouple converter.

[<img alt="license badge" src="https://img.shields.io/github/license/onkoe/max6675-hal">](https://github.com/onkoe/max6675-hal)
[<img alt="docs.rs badge" src="https://img.shields.io/docsrs/max6675-hal">](https://docs.rs/max6675-hal)
[<img alt="crates.io badge" src="https://img.shields.io/crates/dv/max6675-hal?label=crates.io">](https://crates.io/crates/max6675-hal)
[<img alt="GitHub badge" src="https://img.shields.io/badge/github-onkoe/max6675--hal-6e5494">](https://github.com/onkoe/max6675-hal)
[<img alt="GitHub Actions badge" src="https://img.shields.io/github/actions/workflow/status/onkoe/max6675-hal/ci.yml?branch=main">](https://github.com/onkoe/max6675-hal/actions)

## Usage

This example code will change depending on which HAL device driver you're
using. An `arduino-hal` project's SPI isn't like that of an `esp32-hal`
project.

However, you only need to focus on your device's SPI implementation!
(thanks, `embedded-hal` 1.0 ☺️)

Your SPI settings should use MSB (most significant bit) first, target a clock speed of
at least 4mhz, and utilize SPI Mode 1.

```rust
// first, define what pins you're connecting to
let so_pin = pins.("your miso pin").into_pull_up_input();
let cs_pin = pins.("your cs pin").into_output();
let sck_pin = pins.("your sck/clock pin").into_output();

// you may need a mosi pin for your device's SPI, though the max6675 doesn't use one.
// if so, just pick some pin that you're not using ☺️
let dummy_mosi = pins.("some pin you're not using").into_output();

let (spi, _) = device-hal::spi::Spi::new(
    sck_pin, dummy_mosi, so_pin, cs_pin,
    device-hal::spi::Settings {
        // pick some settings that roughly align like so:
        data_order: MostSignificantFirst,
        clock: 4MhzClockSpeed,
        mode: embedded_hal::spi::MODE_1,
    }
);
let mut max = Max6675::new(spi)?; // your spi and chip select here

let temp = max.read_celsius()? // ayo! we got the temperature
```

## Note

This crate re-exports a Temperature type from another crate, `simmer`.
You can change and play with the temperatures in various ways, so feel free
to [check out its docs](https://docs.rs/crate/simmer/latest) for more info.

## Contributions

Contributions are welcome to this project! Since it's pretty small, feel
free to submit a PR whenever. You can also make an issue - I'll likely get
to it soon!

## Help

Please don't hesitate to make an issue if you experience any problems!

If you can, please submit a [`hw-probe` report](https://linux-hardware.org/?view=howto)
alongside any error messages or useful logs you have!

<!-- cargo-rdme end -->
