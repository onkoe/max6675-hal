#![cfg(test)]
extern crate alloc;

use assert_approx_eq::assert_approx_eq;
use embedded_hal_mock::{
    common::Generic,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
    spi::{Mock as SpiMock, Transaction as SpiTransaction},
    MockError,
};
use max6675_hal::{error::Max6675Error, *};

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

// long ahh types
type Max = Max6675<Generic<PinTransaction>, MockError, Generic<SpiTransaction>, MockError>;
type MaxError = Max6675Error<MockError, MockError>;

fn make_max6675() -> Result<Max, MaxError> {
    // expects 37° C - around body temp
    let resp = ((148 << 3) as u16).to_be_bytes();

    let cs_transactions = [
        PinTransaction::set(PinState::High),
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
