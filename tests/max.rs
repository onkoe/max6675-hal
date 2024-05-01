#![cfg(test)]
extern crate alloc;

use assert_approx_eq::assert_approx_eq;
use embedded_hal_mock::{
    common::Generic,
    eh1::spi::{Mock, Transaction},
};
use max6675_hal::{error::Max6675Error, *};
use once_cell::sync::Lazy;

// expects 37° C - around body temp
static EXP_TEMP: Lazy<[Transaction<u8>; 3]> = Lazy::new(|| {
    [
        Transaction::transaction_start(),
        Transaction::read_vec(((148 << 3) as u16).to_be_bytes().to_vec()),
        Transaction::transaction_end(),
    ]
});

#[test]
fn test_make() {
    let max = make_max6675([]).unwrap();
    max.free().done();
}

#[test]
fn test_celsius() {
    let mut max = make_max6675(EXP_TEMP.clone()).unwrap();
    assert_approx_eq!(max.read_celsius().unwrap().into_inner(), 37_f32);
    max.free().done();
}

#[test]
fn test_fahrenheit() {
    let mut max = make_max6675(EXP_TEMP.clone()).unwrap();
    assert_approx_eq!(max.read_fahrenheit().unwrap().into_inner(), 98.6_f32);
    max.free().done();
}

#[test]
fn test_kelvin() {
    let mut max = make_max6675(EXP_TEMP.clone()).unwrap();
    assert_approx_eq!(max.read_kelvin().unwrap().into_inner(), 310.15_f32);
    max.free().done();
}

#[test]
fn test_raw() {
    let mut max = make_max6675(EXP_TEMP.clone()).unwrap();
    assert_eq!(max.read_raw().unwrap(), [0x04, 0xA0]);
    max.free().done();
}

// long ahh types
type Max = Max6675<Generic<Transaction<u8>>, embedded_hal::spi::ErrorKind>;
type MaxError = Max6675Error<embedded_hal::spi::ErrorKind>;

fn make_max6675(expected: impl Into<Vec<Transaction<u8>>>) -> Result<Max, MaxError> {
    let expected = expected.into();
    let spi = Mock::new(&expected);

    Max6675::new(spi)
}
