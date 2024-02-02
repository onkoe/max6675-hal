#![cfg_attr(not(feature = "std"), no_std)]
//! # max6675-hal
//!
//! An embedded-hal driver for the MAX6675 digital thermocouple converter.
//!
//! ## Usage
//!
//! This example code will change depending on which HAL device driver you're
//! using. An `arduino-hal` project's SPI isn't like that of an `esp32-hal`
//! project.
//!
//! However, you only have to focus on two parts:
//!
//! 1. A CS (chip select) pin as an `OutputPin`
//! 2. Some SPI representation that doesn't exclusively own the CS pin
//! (I'm looking at you, `linux-embedded-hal`!)
//!
//! Your SPI settings should use MSB (most significant bit) first, target a clock speed of
//! at least 4mhz, and utilize SPI Mode 1.
//!
//! ```ignore
//! // first, define what pins you're connecting to
//! let so_pin = pins.("your miso pin").into_pull_up_input();
//! let cs_pin = pins.("your cs pin").into_output();
//! let sck_pin = pins.("your sck/clock pin").into_output();
//!
//! // you may need a mosi pin for your device's SPI, though the max6675 doesn't use one.
//! // if so, just pick some pin that you're not using ☺️
//! let dummy_mosi = pins.("some pin you're not using").into_output();
//!
//! let (spi, cs) = device-hal::spi::Spi::new(
//!     sck_pin, dummy_mosi, so_pin, cs_pin,
//!     device-hal::spi::Settings {
//!         // pick some settings that roughly align like so:
//!         data_order: MostSignificantFirst,
//!         clock: 4MhzClockSpeed,
//!         mode: embedded_hal::spi::MODE_1,
//!     }
//! );
//! let mut max = Max6675::new(spi, cs)?; // your spi and chip select here
//!
//! let temp = max.read_celsius()? // ayo! we got the temperature
//! ```
//!
//! ## Note
//!
//! This crate re-exports a Temperature type from another crate, `simmer`.
//! You can change and play with the temperatures in various ways, so feel free
//! to [check out its docs](https://docs.rs/crate/simmer/latest) for more info.

use core::marker::PhantomData;
use embedded_hal::spi::{self, SpiDevice};

pub mod error;
pub use error::Max6675Error;

/// A Temperature type from [`simmer`](https://docs.rs/crate/simmer/latest).
pub use simmer::Temperature;

/// # Max6675
///
/// A representation of the MAX6675 digital thermocouple converter.
/// Maintains an SPI connection to the device.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Max6675<Spi, SpiError>
where
    Spi: SpiDevice,
    SpiError: spi::Error,
{
    /// SPI connection
    spi: Spi,

    // we're using the generic spi error, but not here!
    _spi_err: PhantomData<SpiError>,
}

impl<Spi, SpiError> Max6675<Spi, SpiError>
where
    Spi: SpiDevice,
    SpiError: spi::Error,
    Max6675Error<SpiError>: From<<Spi as spi::ErrorType>::Error>,
{
    /// Creates a new Max6675 representation.
    ///
    /// For the `spi` argument, you should pass in your `embedded-hal` device's
    /// SPI implementation filled with appropriate details.
    ///
    /// # Usage
    ///
    /// Since the `Spi` (SPI) and `Cs` (chip select) arguments are generic,
    /// you'll have to make some decisions based on the hardware you're using!
    ///
    /// Please follow this general template:
    ///
    /// ```ignore
    /// // first, define what pins you're connecting to
    /// let so_pin = pins.("your miso pin").into_pull_up_input();
    /// let cs_pin = pins.("your cs pin").into_output();
    /// let sck_pin = pins.("your sck/clock pin").into_output();
    ///
    /// // you may need a mosi pin for your device's SPI, though the max6675 doesn't use one.
    /// // if so, just pick some pin that you're not using ☺️
    /// let dummy_mosi = pins.("some pin you're not using").into_output();
    ///
    /// let (spi, cs) = device-hal::spi::Spi::new(
    ///     sck_pin, dummy_mosi, so_pin, cs_pin,
    ///     device-hal::spi::Settings {
    ///         // pick some settings that roughly align like so:
    ///         data_order: MostSignificantFirst,
    ///         clock: 4MhzClockSpeed,
    ///         mode: embedded_hal::spi::MODE_1,
    ///     }
    /// );
    /// let mut max = Max6675::new(spi, cs)?; // your spi and chip select here
    /// ```
    pub fn new(spi: Spi) -> Result<Self, Max6675Error<SpiError>> {
        Ok(Self {
            spi,
            _spi_err: PhantomData,
        })
    }

    /// Destructs the `MAX6675` into its bare components, as recommended by the
    /// [HAL Design Patterns](https://doc.rust-lang.org/beta/embedded-book/design-patterns/hal/interoperability.html).
    ///
    /// ```
    /// use max6675_hal::Max6675;
    ///# use embedded_hal_mock::{
    /// #     pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
    /// #     spi::{Mock as SpiMock, Transaction as SpiTransaction},
    /// # };
    /// #
    /// # let temp = ((400 << 3) as u16).to_be_bytes().to_vec(); // 100 degrees celsius
    /// #
    /// # let cs_exp = [
    /// #    PinTransaction::set(PinState::High)
    /// # ];
    /// #
    /// # let spi = SpiMock::new(&[SpiTransaction::transfer(vec![0, 0], temp)]);
    /// # let cs = PinMock::new(&cs_exp);
    /// // (pretend there's some spi/chip select setup code)
    ///
    /// let mut max = Max6675::new(spi, cs).unwrap();
    /// let (spi, cs) = max.free();
    /// # spi.done();
    pub fn free(self) -> Spi {
        self.spi
    }

    /// Tries to read thermocouple temperature, leaving it as a raw ADC count.
    ///
    /// ```
    /// use max6675_hal::Max6675;
    ///# use embedded_hal_mock::{
    /// #     pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
    /// #     spi::{Mock as SpiMock, Transaction as SpiTransaction},
    /// # };
    /// #
    /// let temp = ((400 << 3) as u16).to_be_bytes().to_vec(); // 100 degrees celsius
    ///
    /// # let cs_exp = [
    /// #    PinTransaction::set(PinState::High),
    /// #     PinTransaction::set(PinState::Low),
    /// #     PinTransaction::set(PinState::High),
    /// # ];
    /// #
    /// # let spi = SpiMock::new(&[SpiTransaction::transfer(vec![0, 0], temp)]);
    /// # let cs = PinMock::new(&cs_exp);
    /// // (pretend there's some spi/chip select setup code)
    ///
    /// let mut max = Max6675::new(spi, cs).unwrap();
    /// assert_eq!(max.read_raw().unwrap(), [0xc, 0x80]);
    /// ```
    pub fn read_raw(&mut self) -> Result<[u8; 2], Max6675Error<SpiError>> {
        let mut buf: [u8; 2] = [0_u8; 2];
        self.spi.read(&mut buf)?;

        Ok(buf)
    }

    /// Internal function to convert a `read_raw()` into a parsable `u16`.
    fn process_raw(&mut self) -> Result<u16, Max6675Error<SpiError>> {
        Ok(u16::from_be_bytes(self.read_raw()?))
    }

    /// Tries to read the thermocouple's temperature in Celsius.
    ///
    /// ```
    /// use max6675_hal::Max6675;
    /// # use assert_approx_eq::assert_approx_eq;
    /// # use embedded_hal_mock::{
    /// #     pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
    /// #     spi::{Mock as SpiMock, Transaction as SpiTransaction},
    /// # };
    /// #
    /// # let temp = ((400 << 3) as u16).to_be_bytes().to_vec(); // 100 degrees celsius
    ///
    /// # let cs_exp = [
    /// #    PinTransaction::set(PinState::High),
    /// #     PinTransaction::set(PinState::Low),
    /// #     PinTransaction::set(PinState::High),
    /// # ];
    /// #
    /// # let spi = SpiMock::new(&[SpiTransaction::transfer(vec![0, 0], temp)]);
    /// # let cs = PinMock::new(&cs_exp);
    /// // (pretend there's some spi/chip select setup code)
    ///
    /// let mut max = Max6675::new(spi, cs).unwrap();
    /// assert_approx_eq!(max.read_celsius().unwrap().into_inner(), 100_f32);
    /// ```
    pub fn read_celsius(&mut self) -> Result<Temperature, Max6675Error<SpiError>> {
        let raw = self.process_raw()?;

        if raw & 0x04 != 0 {
            return Err(Max6675Error::OpenCircuitError);
        }

        let temp = ((raw >> 3) & 0x1FFF) as f32 * 0.25_f32;
        Ok(Temperature::Celsius(temp))
    }

    /// Tries to read the thermocouple's temperature in Fahrenheit.
    ///
    /// ```
    /// use max6675_hal::Max6675;
    /// # use assert_approx_eq::assert_approx_eq;
    /// # use embedded_hal_mock::{
    /// #     pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
    /// #     spi::{Mock as SpiMock, Transaction as SpiTransaction},
    /// # };
    /// #
    /// # let temp = ((80 << 3) as u16).to_be_bytes().to_vec(); // 68 degrees fahrenheit
    ///
    /// # let cs_exp = [
    /// #    PinTransaction::set(PinState::High),
    /// #     PinTransaction::set(PinState::Low),
    /// #     PinTransaction::set(PinState::High),
    /// # ];
    /// #
    /// # let spi = SpiMock::new(&[SpiTransaction::transfer(vec![0, 0], temp)]);
    /// # let cs = PinMock::new(&cs_exp);
    /// // (pretend there's some spi/chip select setup code)
    ///
    /// let mut max = Max6675::new(spi, cs).unwrap();
    /// assert_approx_eq!(max.read_fahrenheit().unwrap().into_inner(), 68_f32);
    /// ```
    pub fn read_fahrenheit(&mut self) -> Result<Temperature, Max6675Error<SpiError>> {
        Ok(self.read_celsius()?.to_fahrenheit())
    }

    /// Tries to read the thermocouple's temperature in Kelvin.
    ///
    /// ```
    /// use max6675_hal::Max6675;
    /// # use assert_approx_eq::assert_approx_eq;
    /// # use embedded_hal_mock::{
    /// #     pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
    /// #     spi::{Mock as SpiMock, Transaction as SpiTransaction},
    /// # };
    /// #
    /// # let temp = ((400 << 3) as u16).to_be_bytes().to_vec(); // 68 degrees fahrenheit
    ///
    /// # let cs_exp = [
    /// #    PinTransaction::set(PinState::High),
    /// #     PinTransaction::set(PinState::Low),
    /// #     PinTransaction::set(PinState::High),
    /// # ];
    /// #
    /// # let spi = SpiMock::new(&[SpiTransaction::transfer(vec![0, 0], temp)]);
    /// # let cs = PinMock::new(&cs_exp);
    /// // (pretend there's some spi/chip select setup code)
    ///
    /// let mut max = Max6675::new(spi, cs).unwrap();
    /// assert_approx_eq!(max.read_kelvin().unwrap().into_inner(), 373.15_f32);
    /// ```
    pub fn read_kelvin(&mut self) -> Result<Temperature, Max6675Error<SpiError>> {
        Ok(self.read_celsius()?.to_kelvin())
    }
}
