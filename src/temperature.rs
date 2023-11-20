//! # Temperature
//!
//! Encapsulates conversions between temperatures so crate users can get to work.

/// A representation of some common temperature unit.
#[derive(Clone, Debug, PartialEq)]
pub enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
    Kelvin(f64),
}

impl Temperature {
    /// Return a Temperature in Fahrenheit based off of Self.
    pub fn to_fahrenheit(&self) -> Temperature {
        match self {
            Self::Fahrenheit(_) => self.clone(),
            Self::Celsius(c) => Self::Fahrenheit((c * 1.8) + 32_f64),
            Self::Kelvin(k) => Self::Fahrenheit(((k - 273.15) * 1.8) + 32_f64),
        }
    }

    /// Return a Temperature in Fahrenheit based off of Self.
    pub fn to_celsius(&self) -> Temperature {
        match self {
            Temperature::Fahrenheit(f) => Self::Celsius((f - 32_f64) / 1.8),
            Temperature::Celsius(_) => self.clone(),
            Temperature::Kelvin(k) => Self::Celsius(k - 273.15),
        }
    }

    /// Return a Temperature in Fahrenheit based off of Self.
    pub fn to_kelvin(&self) -> Temperature {
        match self {
            Temperature::Fahrenheit(f) => Self::Kelvin(((f - 32_f64) / 1.8) + 273.15),
            Temperature::Celsius(c) => Self::Kelvin(c + 273.15),
            Temperature::Kelvin(_) => self.clone(),
        }
    }

    /// A discovery function that returns the inner type. Use `my_temp.into()` when possible.
    pub fn into_inner(&self) -> f64 {
        Into::<f64>::into(self.clone())
    }
}

// we can't go from an f64 to a temperature of unknown type... :p
#[allow(clippy::from_over_into)]
impl Into<f64> for Temperature {
    fn into(self) -> f64 {
        match self {
            Temperature::Fahrenheit(f) => f,
            Temperature::Celsius(c) => c,
            Temperature::Kelvin(k) => k,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Temperature;
    use approx_eq::assert_approx_eq;

    /// This macro expects an argument order of (Fahrenheit, Celsius, Kelvin).
    /// If that order isn't correct, you'll find that things don't work properly...
    macro_rules! test_all {
        ($temp_f:expr, $temp_c:expr, $temp_k:expr) => {
            // test temp_f
            assert_approx_eq!(
                $temp_f,
                Temperature::Celsius($temp_c).to_fahrenheit().into()
            );
            assert_approx_eq!($temp_f, Temperature::Kelvin($temp_k).to_fahrenheit().into());
            assert_approx_eq!(
                $temp_f,
                Temperature::Fahrenheit($temp_f).to_fahrenheit().into()
            );

            // ok now temp_c
            assert_approx_eq!(
                $temp_c,
                Temperature::Fahrenheit($temp_f).to_celsius().into()
            );
            assert_approx_eq!($temp_c, Temperature::Kelvin($temp_k).to_celsius().into());
            assert_approx_eq!($temp_c, Temperature::Celsius($temp_c).to_celsius().into());

            // annnnd temp_k
            assert_approx_eq!($temp_k, Temperature::Fahrenheit($temp_f).to_kelvin().into());
            assert_approx_eq!($temp_k, Temperature::Celsius($temp_c).to_kelvin().into());
            assert_approx_eq!($temp_k, Temperature::Kelvin($temp_k).to_kelvin().into());
        };
    }

    #[test]
    fn surface_of_sun() {
        let sun_f = 9941_f64;
        let sun_c = 5505_f64;
        let sun_k = 5778.15_f64;

        test_all!(sun_f, sun_c, sun_k);
    }

    #[test]
    fn water_boils() {
        let water_f = 212_f64;
        let water_c = 100_f64;
        let water_k = 373.15_f64;

        test_all!(water_f, water_c, water_k);
    }

    #[test]
    fn water_freezes() {
        let ice_f = 32_f64;
        let ice_c = 0_f64;
        let ice_k = 273.15_f64;

        test_all!(ice_f, ice_c, ice_k);
    }
}
