//! test-utils error

use libmrot::Error as LibMrotError;
use std::convert::From;
use std::fmt;

/// Mrot error variants
#[derive(Debug)]
pub enum Error {
    /// wraps [libmrot::Error]
    LibMrot(LibMrotError),
    /// Bug in the test code
    UndefinedValue(String),
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LibMrot(libmrot_error) => fmt::Display::fmt(libmrot_error, f),
            Error::UndefinedValue(field) => {
                fmt::Display::fmt(&format!("world has no value for {}", field), f)
            }
            Error::NotImplemented => fmt::Display::fmt("not implemented", f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::LibMrot(ref libmrot_error) => Some(libmrot_error),
            Error::UndefinedValue(_) => None,
            Error::NotImplemented => None,
        }
    }
}

impl From<LibMrotError> for Error {
    fn from(value: LibMrotError) -> Self {
        Error::LibMrot(value)
    }
}
