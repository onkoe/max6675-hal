/// Some problem with the MAX6675 or its connections
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Max6675Error<SpiError, CsError> {
    SpiError(SpiError),
    CsError(CsError),
    OpenCircuitError,
}

impl<SpiError, CsError> Max6675Error<SpiError, CsError> {
    fn message(&self) -> &'static str {
        match *self {
            Max6675Error::SpiError(_) => {
                "An error occured while attempting to reach the MAX6675 over SPI."
            }
            Max6675Error::CsError(_) => "The MAX6675 has encountered a chip select pin error.",
            Max6675Error::OpenCircuitError => {
                "The MAX6675 detected an open circuit (bit D2 was high). \
                Please check the thermocouple connection and try again."
            }
        }
    }
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
        write!(f, "{}", self.message())
    }
}

impl<SpiError, CsError> ufmt::uDisplay for Max6675Error<SpiError, CsError> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(self.message())
    }
}

// debug impls
impl<SpiError, CsError> core::fmt::Debug for Max6675Error<SpiError, CsError> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<SpiError, CsError> ufmt::uDebug for Max6675Error<SpiError, CsError> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(self.message())
    }
}

// implement error if it's feasible
// TODO: check for core::error::Error stability in CI. if so, fail a test - i get an email :3
#[cfg(feature = "std")]
impl<SpiError: std::fmt::Debug, CsError: std::fmt::Debug> std::error::Error
    for Max6675Error<SpiError, CsError>
{
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
