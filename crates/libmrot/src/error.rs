//! Mrot error

#[allow(unused_imports)]
use crate::MealRecord;
use sqlite::Error as SqliteError;
use std::{convert::From, fmt, io::Error as IoError, num::ParseIntError};
use two_timer::TimeError;

/// Mrot error variants
#[derive(Debug)]
pub enum Error {
    /// Wraps [`std::num::ParseIntError`].
    StdNum(ParseIntError),
    /// Wraps [`std::io::Error`].
    Io(IoError),
    /// Wraps [`sqlite::Error`].
    Sqlite(SqliteError),
    /// Wraps [`two_timer::TimeError`].
    TwoTimer(TimeError),
    /// A path does not have a parent directory.
    NoParentDirectory,
    /// Timestamp cannot be converted into [`chrono::DateTime`].
    InvalidTimestamp(i64),
    /// [`MealRecord`] cannot be parsed.
    ParseMealRecordError,
    /// A date expression parses to more than one date when it should not.
    MoreThanOneDate(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::StdNum(parse_int_error) => fmt::Display::fmt(parse_int_error, f),
            Error::Io(io_error) => fmt::Display::fmt(io_error, f),
            Error::Sqlite(sqlite_error) => fmt::Display::fmt(sqlite_error, f),
            Error::TwoTimer(time_error) => fmt::Display::fmt(time_error, f),
            Error::NoParentDirectory => fmt::Display::fmt("cannot find parent directory", f),
            Error::InvalidTimestamp(i) => fmt::Display::fmt(&format!("invalid timestamp {}", i), f),
            Error::ParseMealRecordError => fmt::Display::fmt("cannot parse MealRecord", f),
            Error::MoreThanOneDate(s) => fmt::Display::fmt(
                &format!("date expression '{}' parses to more than one date ", s),
                f,
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::StdNum(ref parse_int_error) => Some(parse_int_error),
            Error::Io(ref io_error) => Some(io_error),
            Error::Sqlite(ref sqlite_error) => Some(sqlite_error),
            Error::TwoTimer(ref time_error) => Some(time_error),
            Error::NoParentDirectory => None,
            Error::InvalidTimestamp(_) => None,
            Error::ParseMealRecordError => None,
            Error::MoreThanOneDate(_) => None,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::StdNum(value)
    }
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Error::Io(value)
    }
}

impl From<SqliteError> for Error {
    fn from(value: SqliteError) -> Self {
        Error::Sqlite(value)
    }
}

impl From<TimeError> for Error {
    fn from(value: TimeError) -> Self {
        Error::TwoTimer(value)
    }
}
