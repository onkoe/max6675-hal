# `max6675_hal`

![Crates.io](https://img.shields.io/crates/l/max6675_hal)
![docs.rs](https://img.shields.io/docsrs/max6675_hal)
![GitHub](https://img.shields.io/badge/github-onkoe/max6675__hal-6e5494?)
![Actions](https://img.shields.io/github/actions/workflow/status/onkoe/max6675_hal/ci.yml?branch=main)

An embedded-hal driver for the MAX6675 digital thermocouple converter.

## Usage

This example code will change depending on which HAL device driver you're using. An `arduino-hal` project's SPI isn't like that of an `esp32-hal` project.

However, you only have to focus on two parts:

1. A CS (chip select) pin as an `OutputPin`
2. Some SPI representation that doesn't exclusively own the CS pin (I'm looking at you, `linux-embedded-hal`!)

Your SPI settings should use MSB (most significant bit) first, target a clock speed of at least 4mhz, and utilize SPI Mode 1.

After both are good, pass them into the `Max6675::new(spi, chip_select)` constructor. Ta-da! Your MAX6675 gets put to good use.

```rust
// first, define what pins you're connecting to
let so_pin = pins.("your miso pin").into_pull_up_input();
let cs_pin = pins.("your cs pin").into_output();
let sck_pin = pins.("your sck/clock pin").into_output();

// you may need a mosi pin for your device's SPI, though the max6675 doesn't use one.
// if so, just pick some pin that you're not using ☺️
let dummy_mosi = pins.("some pin you're not using").into_output();

let (spi, cs) = device-hal::spi::Spi::new(
    sck_pin, dummy_mosi, so_pin, cs_pin,
    device-hal::spi::Settings {
        // pick some settings that roughly align like so:
        data_order: MostSignificantFirst,
        clock: 4MhzClockSpeed,
        mode: embedded_hal::spi::MODE_1,
    }
);
let mut max = Max6675::new(spi, cs)?; // your spi and chip select here

let temp = max.read_celsius()? // ayo! we got the temperature
```

## Contributions

Contributions are welcome to this project! Since it's pretty small, feel free to submit a PR whenever.

## Help

Please feel free to make an issue if you experience any problems!

If you can, please submit a [`hw-probe` report](https://linux-hardware.org/?view=howto) alongside any error messages or useful logs you have!
