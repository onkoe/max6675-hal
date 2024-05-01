#![no_std]
#![no_main]

use arduino_hal::spi::{Settings, Spi};

use max6675_hal::Max6675;
use panic_halt as _;
use ufmt::uwriteln;

// FIXME: since `avr-hal` is out of date, this isn't usable right now! 😖
//        ...but! when it's updated, remove this warning and test that mofo

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let so_pin = pins.d50.into_pull_up_input();
    let cs_pin = pins.d53.into_output();
    let sck_pin = pins.d52.into_output();
    let dummy_mosi = pins.d51.into_output(); // unused with the max6675

    // yeah, it's a lotta stuff. sorry!
    let (spi, _) = Spi::new(
        dp.SPI,
        sck_pin,
        dummy_mosi,
        so_pin,
        cs_pin,
        Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            clock: arduino_hal::spi::SerialClockRate::OscfOver4,
            mode: embedded_hal::spi::MODE_1,
        },
    );

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut max = Max6675::new(spi);

    arduino_hal::delay_ms(500);

    loop {
        arduino_hal::delay_ms(500); // delay between reads should be at least 220ms

        if let Ok(ref mut max) = max {
            match max.read_celsius() {
                Ok(t) => uwriteln!(
                    &mut serial,
                    "Read from MAX6675: {}° C",
                    ufmt_float::uFmt_f32::Three(t.into_inner())
                )
                .unwrap(),
                Err(e) => match e {
                    max6675_hal::Max6675Error::SpiError(_) => {
                        uwriteln!(&mut serial, "spi error").unwrap()
                    }
                    max6675_hal::Max6675Error::OpenCircuitError => {
                        uwriteln!(&mut serial, "open circuit!!!").unwrap()
                    }
                    max6675_hal::Max6675Error::CsError(_) => {
                        uwriteln!(&mut serial, "cs error ahhhh!!!").unwrap()
                    }
                },
            }
        }
    }
}
