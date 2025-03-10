//! test-utils error

use chrono::ParseError;
use libmrot::Error as LibMrotError;
use std::convert::From;
use std::fmt;
use two_timer::TimeError;

/// Mrot error variants
#[derive(Debug)]
pub enum Error {
    /// wraps [libmrot::Error]
    LibMrot(LibMrotError),
    /// Bug in the test code. A value in the [`World`](crate::World) was supposed to be defined but wasn't.
    UndefinedValue(String),
    /// wraps [chrono::ParseError]
    Chrono(ParseError),
    /// Unexpected Err Result
    UnexpectedErrResult(String),
    /// Wraps [two_timer::TimeError]
    TwoTimer(TimeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LibMrot(libmrot_error) => fmt::Display::fmt(libmrot_error, f),
            Error::UndefinedValue(field) => {
                fmt::Display::fmt(&format!("world has no value for {}", field), f)
            }
            Error::Chrono(parse_error) => fmt::Display::fmt(parse_error, f),
            Error::UnexpectedErrResult(error) => fmt::Display::fmt(&error, f),
            Error::TwoTimer(time_error) => fmt::Display::fmt(time_error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::LibMrot(ref libmrot_error) => Some(libmrot_error),
            Error::UndefinedValue(_) => None,
            Error::Chrono(ref parse_error) => Some(parse_error),
            Error::UnexpectedErrResult(_) => None,
            Error::TwoTimer(ref time_error) => Some(time_error),
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

impl From<TimeError> for Error {
    fn from(value: TimeError) -> Self {
        Error::TwoTimer(value)
    }
}
