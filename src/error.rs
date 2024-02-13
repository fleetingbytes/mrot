//! Mrot error

use confy::ConfyError;
use std::convert::From;
use std::ffi::OsString;
use std::fmt;
use std::io::Error as IoError;

/// Mrot error variants
#[derive(Debug)]
pub enum Error {
    /// wraps std::io::Error
    Io(IoError),
    /// wraps confy::Error
    Confy(ConfyError),
    /// when OsString does not contain valid Unicode
    InvalidUnicode(OsString),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Io(io_error) => fmt::Display::fmt(io_error, f),
            Error::Confy(confy_error) => fmt::Display::fmt(confy_error, f),
            Error::InvalidUnicode(os_string) => {
                fmt::Display::fmt(os_string.to_string_lossy().into_owned().as_str(), f)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Io(ref io_error) => Some(io_error),
            Error::Confy(ref confy_error) => Some(confy_error),
            Error::InvalidUnicode(_) => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Error::Io(value)
    }
}

impl From<ConfyError> for Error {
    fn from(value: ConfyError) -> Self {
        Error::Confy(value)
    }
}

impl From<OsString> for Error {
    fn from(value: OsString) -> Self {
        Error::InvalidUnicode(value)
    }
}
