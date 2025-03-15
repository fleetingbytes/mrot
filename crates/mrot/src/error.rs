//! Error types for the mrot app

use confy::ConfyError;
use libmrot::Error as LibMrotError;
use std::{convert::From, ffi::OsString, fmt, io::Error as IoError};

#[derive(Debug)]
pub(crate) enum Error {
    /// Wraps [std::io::Error]
    Io(IoError),
    /// wraps [confy::ConfyError]
    Confy(ConfyError),
    /// Wraps [libmrot] error
    LibMrot(LibMrotError),
    /// A path contains an invalid Unicode character
    InvalidUnicode(OsString),
    /// When you try to set a value in the mrot config which is not allowed to be set. (`mrot
    /// config set look-ahead 0` would be such an example, for this setting only makes sense for
    /// values greater than 0).
    UnsupportedConfigValue(String, String),
    /// No suitable path for project directory could be found, see [directories::ProjectDirs]
    NoDirectory(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Io(io_error) => fmt::Display::fmt(io_error, f),
            Error::Confy(confy_error) => fmt::Display::fmt(confy_error, f),
            Error::LibMrot(libmrot_error) => fmt::Display::fmt(libmrot_error, f),
            Error::InvalidUnicode(os_string) => fmt::Display::fmt(
                &format!(
                    "invalid Unicode string: {}",
                    os_string.to_string_lossy().into_owned().as_str()
                ),
                f,
            ),
            Error::UnsupportedConfigValue(config_field, reason) => fmt::Display::fmt(
                &format!(
                    "unsupported configuration value for {}, reason: {}",
                    config_field, reason
                ),
                f,
            ),
            Error::NoDirectory(group) => {
                fmt::Display::fmt(&format!("cannot find directory for {}", group), f)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Io(ref io_error) => Some(io_error),
            Error::Confy(ref confy_error) => Some(confy_error),
            Error::LibMrot(ref libmrot_error) => Some(libmrot_error),
            Error::InvalidUnicode(_) => None,
            Error::UnsupportedConfigValue(_, _) => None,
            Error::NoDirectory(_) => None,
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

impl From<LibMrotError> for Error {
    fn from(value: LibMrotError) -> Self {
        Error::LibMrot(value)
    }
}

impl From<OsString> for Error {
    fn from(value: OsString) -> Self {
        Error::InvalidUnicode(value)
    }
}
