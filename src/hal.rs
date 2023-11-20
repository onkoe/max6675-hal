//! # hal
//!
//! A general implementation of the MAX6675 thermocouple-to-digital converter for `embedded-hal`
//! devices, assuming they use the `embedded_hal` SPI.
//!
//! Note: if using `linux_embedded_hal`, you'll need to enable the `linux` feature!
//!
//! ## Usage
//!
//! No matter which board you're using, you'll want to create an "SPI" representation
//! and a chip select pin.
//!
//! Your SPI settings should use MSB (most significant bit) first, target a clock speed of
//! at least 4mhz, and utilize SPI Mode 1.
//!
//! You can see an example of using it below:
//!
//! ```no_run
//! // #![no_std] // uncomment this
//! // #![no_main] // uncomment this
//!
//! use arduino_hal::{prelude::*, spi::Spi};
//! use embedded_hal::{blocking::serial, spi::MODE_1}; // TODO
//! use max6675_hal::hal::Max6675;
//! // use panic_halt as _; // uncomment this
//! use ufmt::uwriteln;
//!
//! // #[arduino_hal::entry] // uncomment this
//! fn main() -> ! {
//!     let dp = arduino_hal::Peripherals::take().unwrap();
//!     let pins = arduino_hal::pins!(dp);
//!
//!     let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
//!
//!     let so_pin = pins.d50.into_pull_up_input();
//!     let cs_pin = pins.d53.into_output();
//!     let sck_pin = pins.d52.into_output();
//!     let dummy_mosi = pins.d51.into_output();
//!
//!     uwriteln!(
//!         &mut serial,
//!         "Welcome! Starting SPI connection in three seconds..."
//!     )
//!     .void_unwrap();
//!
//!     arduino_hal::delay_ms(3000);
//!
//!     let (mut spi, mut chip_select) = Spi::new(
//!         dp.SPI,
//!         sck_pin,
//!         dummy_mosi,
//!         so_pin,
//!         cs_pin,
//!         arduino_hal::spi::Settings {
//!             data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
//!             clock: arduino_hal::spi::SerialClockRate::OscfOver4,
//!             mode: embedded_hal::spi::MODE_1,
//!         },
//!     );
//!
//!     arduino_hal::delay_ms(3000);
//!
//!     // if your house isn't freezing, you'll know it's working ;)
//!     let temp = spi.read_celsius(&mut chip_select).unwrap_or(0_f32);
//!
//!     uwriteln!(
//!             &mut serial,
//!             "Read from MAX6675: {}° C",
//!             ufmt_float::uFmt_f32::Three(temp)
//!         )
//!         .void_unwrap();
//!     
//!     loop {
//!         arduino_hal::delay_ms(1000);
//!     }
//! }
//! ```

use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};

/// Errors from low-level SPI interactions.
#[derive(Debug)]
pub enum HalError<SpiE, CsE> {
    OpenCircuitError,
    SpiError(SpiE),
    ChipSelectError(CsE),
}

impl<SpiE: core::fmt::Debug, CsE: core::fmt::Debug> core::error::Error for HalError<SpiE, CsE> {}

impl<SpiE, CsE> core::fmt::Display for HalError<SpiE, CsE> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::OpenCircuitError => {
                write!(
                    f,
                    "The MAX6675 detected an open circuit (bit D2 was high). 
                Please check the thermocouple connection and try again."
                )
            }
            Self::ChipSelectError(_) => {
                write!(f, "A Chip Select error has occured.")
            }
            Self::SpiError(_) => {
                write!(f, "An SPI error has occured.")
            }
        }
    }
}

enum CsState {
    High,
    Low,
}

use CsState::*;

fn set_cs<CS, SpiE, CsE>(cs: &mut CS, state: CsState) -> Result<(), HalError<SpiE, CsE>>
where
    CS: OutputPin<Error = CsE>,
{
    let result = match state {
        CsState::High => cs.set_high(),
        CsState::Low => cs.set_low(),
    };

    result.map_err(|e| HalError::ChipSelectError(e))
}

fn transfer<CS, SPI, SpiE, CsE>(
    spi: &mut SPI,
    chip_select: &mut CS,
    buffer: &mut [u8],
) -> Result<(), HalError<SpiE, CsE>>
where
    CS: OutputPin<Error = CsE>,
    SPI: Transfer<u8, Error = SpiE>,
{
    set_cs(chip_select, Low)?;

    spi.transfer(buffer).map_err(|e| HalError::SpiError(e))?;

    set_cs(chip_select, High)?;

    Ok(())
}

/// Trait enabling using the Max6675
pub trait Max6675<SpiE, CsE, CS> {
    /// Reads the thermocouple temperature and leave it as a raw ADC count. Checks if there is a fault but doesn't detect what kind of fault it is
    fn read_raw(&mut self, chip_select: &mut CS) -> Result<[u8; 2], HalError<SpiE, CsE>>;
    /// Reads the thermocouple temperature and converts it into degrees in the provided unit. Checks if there is a fault but doesn't detect what kind of fault it is
    fn read_celsius(&mut self, chip_select: &mut CS) -> Result<f32, HalError<SpiE, CsE>>;
}

impl<CS, SPI, SpiE, CsE> Max6675<SpiE, CsE, CS> for SPI
where
    CS: OutputPin<Error = CsE>,
    SPI: Transfer<u8, Error = SpiE>,
{
    /// Reads the thermocouple temperature and leave it as a buffer of two bytes.
    fn read_raw(&mut self, chip_select: &mut CS) -> Result<[u8; 2], HalError<SpiE, CsE>> {
        let mut data = [0_u8; 2];

        transfer(self, chip_select, &mut data)?;

        Ok(data)
    }

    /// Reads the thermocouple temperature and converts it into Celsius.
    fn read_celsius(&mut self, chip_select: &mut CS) -> Result<f32, HalError<SpiE, CsE>> {
        let raw = u16::from_be_bytes(self.read_raw(chip_select)?);

        // check for Bit D2 being high, indicating that the thermocouple input is open
        // (see MAX6675 datasheet, p. 5)
        if raw & 0x04 != 0 {
            return Err(HalError::OpenCircuitError);
        }

        // ripped from the Arduino library (see: https://github.com/RobTillaart/MAX6675)
        // note: using f32 over f64 due to f64 not working properly on avr.
        let temp = ((raw >> 3) & 0x1FFF) as f32 * 0.25_f32;
        Ok(temp)
    }
}
