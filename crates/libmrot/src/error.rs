//! Mrot error

use confy::ConfyError;
use sqlite::Error as SqliteError;
use std::convert::From;
use std::ffi::OsString;
use std::fmt;
use std::io::Error as IoError;
use two_timer::TimeError;

/// Mrot error variants
#[derive(Debug)]
pub enum Error {
    /// wraps [std::io::Error]
    Io(IoError),
    /// wraps [confy::ConfyError]
    Confy(ConfyError),
    /// wraps [sqlite::Error]
    Sqlite(SqliteError),
    /// wraps [std::fmt::Error]
    Fmt(fmt::Error),
    /// when something could not be stored in the storage
    Storage,
    /// wraps [two_timer::TimeError]
    TwoTimer(TimeError),
    /// when a [chrono::NaiveDate] cannot be converted to [chrono::NaiveDateTime]
    TimeNotSupported,
    /// if the user input a time span instead of time
    TimeSpanNotSupported,
    /// when OsString does not contain valid Unicode
    InvalidUnicode(OsString),
    /// when [directories::ProjectDirs] is not found
    NoDirectory(String),
    /// when a path does not have a parent directory
    NoParentDirectory,
    /// Timestamp cannot be converted into [chrono::DateTime]
    InvalidTimestamp(i64),
    /// A date which is not an explicit range spans a time period longer than one day
    DateSpansMoreThanOneDay,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Io(io_error) => fmt::Display::fmt(io_error, f),
            Error::Confy(confy_error) => fmt::Display::fmt(confy_error, f),
            Error::Sqlite(sqlite_error) => fmt::Display::fmt(sqlite_error, f),
            Error::Fmt(fmt_error) => fmt::Display::fmt(fmt_error, f),
            Error::Storage => fmt::Display::fmt("cannot store values", f),
            Error::TwoTimer(time_error) => fmt::Display::fmt(time_error, f),
            Error::TimeNotSupported => fmt::Display::fmt("such time is not supported", f),
            Error::TimeSpanNotSupported => fmt::Display::fmt("time spans are not supported", f),
            Error::InvalidUnicode(os_string) => {
                fmt::Display::fmt(os_string.to_string_lossy().into_owned().as_str(), f)
            }
            Error::NoDirectory(group) => {
                fmt::Display::fmt(&format!("cannot find directory for {}", group), f)
            }
            Error::NoParentDirectory => fmt::Display::fmt("cannot find parent directory", f),
            Error::InvalidTimestamp(i) => fmt::Display::fmt(&format!("invalid timestamp {}", i), f),
            Error::DateSpansMoreThanOneDay => fmt::Display::fmt(
                "a date which is not an explicit range spans a time period longer than one day",
                f,
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Io(ref io_error) => Some(io_error),
            Error::Confy(ref confy_error) => Some(confy_error),
            Error::Sqlite(ref sqlite_error) => Some(sqlite_error),
            Error::Fmt(ref fmt_error) => Some(fmt_error),
            Error::Storage => None,
            Error::TwoTimer(ref time_error) => Some(time_error),
            Error::TimeNotSupported => None,
            Error::TimeSpanNotSupported => None,
            Error::InvalidUnicode(_) => None,
            Error::NoDirectory(_) => None,
            Error::NoParentDirectory => None,
            Error::InvalidTimestamp(_) => None,
            Error::DateSpansMoreThanOneDay => None,
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

impl From<SqliteError> for Error {
    fn from(value: SqliteError) -> Self {
        Error::Sqlite(value)
    }
}

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        Error::Fmt(value)
    }
}

impl From<TimeError> for Error {
    fn from(value: TimeError) -> Self {
        Error::TwoTimer(value)
    }
}
