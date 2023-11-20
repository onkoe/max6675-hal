//! # max6675-hal
//!
//! An [embedded-hal](https://github.com/rust-embedded/embedded-hal) implementation of the MAX6675 for Rust embedded users.
//!
//! ## Usage
//!
//! You can see how to use this library by checking out either the [hal] or [linux] modules!

#![no_std]
#![feature(error_in_core)]
// TODO: check HAL DESIGN PATTERNS CHECKLIST: https://doc.rust-lang.org/beta/embedded-book/design-patterns/hal/checklist.html
// TODO: USE TEMPERATURE IN `hal.rs`
// TODO: add Fahrenheit/Kelvin functions
// TODO: all of linux_embedded_hal lmao

//use connection::Connection;
use onlyerror::Error;
use temperature::Temperature;

pub mod hal;
pub mod temperature;

#[cfg(any(feature = "linux", test, doc))]
pub mod linux;

#[derive(Debug, Error)]
pub enum Max6675Error {
    #[error("Failed to connect over SPI.")]
    SpiError(),
    #[error("The MAX6675 detected an open circuit (bit D2 was high). Please check the thermocouple connection and try again.")]
    OpenCircuitError,
}

// #[derive(Debug)]
// pub struct Max6675<C: Connection> {
//     connection: C,
// }

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
