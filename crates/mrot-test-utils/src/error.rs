//! test-utils error

use chrono::ParseError;
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
    /// wraps [chrono::ParseError]
    Chrono(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LibMrot(libmrot_error) => fmt::Display::fmt(libmrot_error, f),
            Error::UndefinedValue(field) => {
                fmt::Display::fmt(&format!("world has no value for {}", field), f)
            }
            Error::Chrono(parse_error) => fmt::Display::fmt(parse_error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::LibMrot(ref libmrot_error) => Some(libmrot_error),
            Error::UndefinedValue(_) => None,
            Error::Chrono(ref parse_error) => Some(parse_error),
        }
    }
}

impl From<LibMrotError> for Error {
    fn from(value: LibMrotError) -> Self {
        Error::LibMrot(value)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::Chrono(value)
    }
}
