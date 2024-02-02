#![cfg(test)]
extern crate alloc;

use assert_approx_eq::assert_approx_eq;
use embedded_hal::spi::ErrorKind;
use embedded_hal_mock::common::Generic;
use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction};
use max6675_hal::{error::Max6675Error, *};

#[test]
fn test_make() {
    let mut max = make_max6675().unwrap();
    max.read_celsius().unwrap();
    max.free().done()
}

#[test]
fn test_celsius() {
    let mut max = make_max6675().unwrap();
    assert_approx_eq!(max.read_celsius().unwrap().into_inner(), 37_f32);
    max.free().done()
}

#[test]
fn test_fahrenheit() {
    let mut max = make_max6675().unwrap();
    assert_approx_eq!(max.read_fahrenheit().unwrap().into_inner(), 98.6_f32);
    max.free().done()
}

#[test]
fn test_kelvin() {
    let mut max = make_max6675().unwrap();
    assert_approx_eq!(max.read_kelvin().unwrap().into_inner(), 310.15_f32);
    max.free().done()
}

#[test]
fn test_raw() {
    let mut max = make_max6675().unwrap();
    assert_eq!(max.read_raw().unwrap(), [0x04, 0xA0]);
    max.free().done()
}

fn make_max6675() -> Result<Max6675<Generic<Transaction<u8>>, ErrorKind>, Max6675Error<ErrorKind>> {
    // expects 37° C - around body temp
    let spi_exp = [
        Transaction::transaction_start(),
        Transaction::read_vec(((148 << 3) as u16).to_be_bytes().to_vec()),
        Transaction::transaction_end(),
    ];

    let spi = SpiMock::new(&spi_exp);

    Max6675::new(spi)
}
