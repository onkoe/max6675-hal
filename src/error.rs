/// Some problem with the MAX6675 or its connections
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Max6675Error<SpiError> {
    SpiError(SpiError),
    OpenCircuitError,
}

impl<SpiError> Max6675Error<SpiError> {
    fn message(&self) -> &'static str {
        match *self {
            Max6675Error::SpiError(_) => {
                "An error occured while attempting to reach the MAX6675 over SPI."
            }
            Max6675Error::OpenCircuitError => {
                "The MAX6675 detected an open circuit (bit D2 was high). \
                Please check the thermocouple connection and try again."
            }
        }
    }
}

// implicit `?` syntax for SpiError to Max6675Error
impl<SpiError> core::convert::From<SpiError> for Max6675Error<SpiError> {
    fn from(value: SpiError) -> Self {
        Max6675Error::SpiError(value)
    }
}

// print... if you must
impl<SpiError> core::fmt::Display for Max6675Error<SpiError> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<SpiError> ufmt::uDisplay for Max6675Error<SpiError> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(self.message())
    }
}

// debug impls
impl<SpiError> core::fmt::Debug for Max6675Error<SpiError> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<SpiError> ufmt::uDebug for Max6675Error<SpiError> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(self.message())
    }
}

// implement error if it's feasible
// FIXME: use core::error::Error once stable! <3
#[cfg(feature = "std")]
impl<SpiError: std::fmt::Debug> std::error::Error for Max6675Error<SpiError> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None // other types typically don't impl error :p
    }

    fn description(&self) -> &str {
        "error description is deprecated. use display instead!"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source() // (none)
    }
}
