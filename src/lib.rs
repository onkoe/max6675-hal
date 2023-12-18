#![cfg_attr(not(feature = "std"), no_std)]
//! # max6675-hal
//!
//! An embedded-hal driver for the MAX6675 digital thermocouple converter.
//!
//! ## Usage
//!
//! No matter which board you're using, you'll want to create an SPI representation
//! for the type to use internally.
//!
//! Your SPI settings should use MSB (most significant bit) first, target a clock speed of
//! at least 4mhz, and utilize SPI Mode 1.
//!
//! Below, you can see the general setup for an Arduino board.
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use arduino_hal::{prelude::*, spi::Spi};
//!
//!
//!
//! ```
//!
//! ## Note
//!
//! This crate re-exports a Temperature type from another crate, `simmer`.
//! You can change and play with the temperatures in various ways, so feel free
//! to [check out its docs](https://docs.rs/crate/simmer/latest) for more info.

// TODO: fix docs
// TODO: check naming n stuff for embedded-hal
// TODO: examples folder (with crates. use arduino, linux, etc.)

use core::marker::PhantomData;
use embedded_hal::{blocking::spi, digital::v2::OutputPin};

pub use simmer::Temperature;

/// Some problem with the MAX6675 or its connections
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Max6675Error<SpiError, CsError> {
    SpiError(SpiError),
    CsError(CsError),
    OpenCircuitError,
}

// implicit `?` syntax for SpiError to Max6675Error
impl<SpiError, CsError> core::convert::From<SpiError> for Max6675Error<SpiError, CsError> {
    fn from(value: SpiError) -> Self {
        Max6675Error::SpiError(value)
    }
}

// print... if you must
impl<SpiError, CsError> core::fmt::Display for Max6675Error<SpiError, CsError> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Max6675Error::SpiError(_) =>
                    "An error occured while attempting to reach the MAX6675 over SPI.",
                Max6675Error::OpenCircuitError => "The MAX6675 has detected an open circuit.",
                Max6675Error::CsError(_) => "Detected a chip select pin error.",
            }
        )
    }
}

// implement error if it's feasible
// TODO: check for core::error::Error stability in CI. if so, fail a test - i get an email :3
#[cfg(feature = "std")]
impl<SpiError: core::fmt::Debug> std::error::Error for Max6675Error<SpiError, CsError> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None // other types typically don't impl error :p
    }

    fn description(&self) -> &str {
        "error description is deprecated. use display instead!"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source() // (none)
    }

    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}

/// # Max6675
///
/// A representation of the MAX6675 digital thermocouple converter.
/// Maintains an SPI connection to the device.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Max6675<Cs, CsError, Spi, SpiError>
where
    Spi: spi::Transfer<u8, Error = SpiError> + spi::Write<u8, Error = SpiError>,
    Cs: OutputPin<Error = CsError>,
{
    /// SPI connection
    spi: Spi,

    /// Chip select pin
    chip_select: Cs,

    // we're using the generic spi error, but not here!
    _spi_err: PhantomData<SpiError>,
    _cs_err: PhantomData<CsError>,
}

impl<Cs, CsError, Spi, SpiError> Max6675<Cs, CsError, Spi, SpiError>
where
    Spi: spi::Transfer<u8, Error = SpiError> + spi::Write<u8, Error = SpiError>,
    Cs: OutputPin<Error = CsError>,
{
    /// Creates a new Max6675 representation.
    ///
    /// For the `spi` argument, you should pass in your `embedded-hal` device's
    /// SPI implementation filled with appropriate details.
    ///
    /// # Usage
    ///
    /// ```ignore
    /// // first, define what pins you're connecting to
    /// let so_pin = pins.("your miso pin").into_pull_up_input();
    /// let cs_pin = pins.("your cs pin").into_output();
    /// let sck_pin = pins.("your sck/clock pin").into_output;
    ///
    /// // you may need a mosi pin for your device's SPI, though the max6675 doesn't have one.
    /// // if so, just pick some pin that you're not using ☺️
    /// let dummy_mosi = pins.("some pin you're not using").into_output();
    ///
    /// let spi = device-hal::spi::Spi::new(
    ///     sck_pin, dummy_mosi, so_pin, cs_pin,
    ///     device-hal::spi::Settings {
    ///         // pick some settings that roughly align like so:
    ///         data_order: MostSignificantFirst,
    ///         clock: 4MhzClockSpeed,
    ///         mode: embedded_hal::spi::MODE_1,
    ///     }
    /// );
    /// let mut max = Max6675::new(spi); // your spi here
    /// ```
    pub fn new(spi: Spi, mut chip_select: Cs) -> Result<Self, Max6675Error<SpiError, CsError>> {
        if let Err(e) = chip_select.set_high() {
            return Err(Max6675Error::CsError(e));
        }

        Ok(Self {
            spi,
            chip_select,
            _spi_err: PhantomData,
            _cs_err: PhantomData,
        })
    }

    /// Tries to read thermocouple temperature, leaving it as a raw ADC count.
    ///
    /// ```ignore
    /// let mut max = Max6675::new(spi)?;
    /// let adc_ct: [u8; 2] = max.read_raw()?;
    /// ```
    pub fn read_raw(&mut self) -> Result<[u8; 2], Max6675Error<SpiError, CsError>> {
        let mut buf: [u8; 2] = [0_u8; 2];

        if let Err(e) = self.chip_select.set_low() {
            return Err(Max6675Error::CsError(e));
        }

        self.spi.transfer(&mut buf)?;

        if let Err(e) = self.chip_select.set_high() {
            return Err(Max6675Error::CsError(e));
        }

        Ok(buf)
    }

    /// Internal function to convert a `read_raw()` into a parsable `u16`.
    fn process_raw(&mut self) -> Result<u16, Max6675Error<SpiError, CsError>> {
        Ok(u16::from_be_bytes(self.read_raw()?))
    }

    /// Tries to read the thermocouple's temperature in Celsius.
    ///
    /// ```ignore
    /// let mut max = Max6675::new(spi)?;
    /// let temp_c = max.read_celsius()?;
    /// ```
    pub fn read_celsius(&mut self) -> Result<Temperature, Max6675Error<SpiError, CsError>> {
        let raw = self.process_raw()?;

        if raw & 0x04 != 0 {
            return Err(Max6675Error::OpenCircuitError);
        }

        let temp = ((raw >> 3) & 0x1FFF) as f32 * 0.25_f32;
        Ok(Temperature::Celsius(temp))
    }

    /// Tries to read the thermocouple's temperature in Fahrenheit.
    ///
    /// ```ignore
    /// let mut max = Max6675::new(spi)?;
    /// let temp_c = max.read_fahrenheit()?;
    /// ```
    pub fn read_fahrenheit(&mut self) -> Result<Temperature, Max6675Error<SpiError, CsError>> {
        Ok(self.read_celsius()?.to_fahrenheit())
    }

    /// Tries to read the thermocouple's temperature in Kelvin.
    ///
    /// ```ignore
    /// let mut max = Max6675::new(spi)?;
    /// let temp_c = max.read_kelvin()?;
    /// ```
    pub fn read_kelvin(&mut self) -> Result<Temperature, Max6675Error<SpiError, CsError>> {
        Ok(self.read_celsius()?.to_kelvin())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use embedded_hal_mock::{
        pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
        spi::{Mock as SpiMock, Transaction as SpiTransaction},
        MockError,
    };

    #[test]
    fn test_make() {
        let _ = make_max6675().unwrap();
    }

    #[test]
    fn test_celsius() {
        let mut max = make_max6675().unwrap();
        assert_approx_eq!(max.read_celsius().unwrap().into_inner(), 37_f32);
    }

    #[test]
    fn test_fahrenheit() {
        let mut max = make_max6675().unwrap();
        assert_approx_eq!(max.read_fahrenheit().unwrap().into_inner(), 98.6_f32);
    }

    #[test]
    fn test_kelvin() {
        let mut max = make_max6675().unwrap();
        assert_approx_eq!(max.read_kelvin().unwrap().into_inner(), 310.15_f32);
    }

    #[test]
    fn test_raw() {
        let mut max = make_max6675().unwrap();
        assert_eq!(max.read_raw().unwrap(), [0x04, 0xA0])
    }

    fn make_max6675() -> Result<
        Max6675<
            embedded_hal_mock::common::Generic<PinTransaction>,
            MockError,
            embedded_hal_mock::common::Generic<SpiTransaction>,
            MockError,
        >,
        Max6675Error<MockError, MockError>,
    > {
        // expects 37° C - around body temp
        let resp = ((148 << 3) as u16).to_be_bytes();

        let cs_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];

        // note: transfer()'s `expected` is 0x00 because we don't really send anything... 🫣️
        let spi = SpiMock::new(&[SpiTransaction::transfer(
            alloc::vec![0x00, 0x00],
            resp.to_vec(),
        )]);

        let cs = PinMock::new(&cs_transactions);

        Max6675::new(spi, cs)
    }
}
