//! Error types for the mrot app

use libmrot::Error as LibMrotError;
use std::{convert::From, fmt};

#[derive(Debug)]
pub(crate) enum Error {
    /// Wraps [libmrot] error
    LibMrot(LibMrotError),
    /// When you try to set a value in the mrot config which is not allowed to be set. (`mrot
    /// config set look-ahead 0` would be such an example, for this setting only makes sense for
    /// values greater than 0).
    UnsupportedConfigValue(String, String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::LibMrot(ref libmrot_error) => Some(libmrot_error),
            Error::UnsupportedConfigValue(_, _) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LibMrot(libmrot_error) => fmt::Display::fmt(libmrot_error, f),
            Error::UnsupportedConfigValue(config_field, reason) => fmt::Display::fmt(
                &format!(
                    "unsupported configuration value for {}, reason: {}",
                    config_field, reason
                ),
                f,
            ),
        }
    }
}

impl From<LibMrotError> for Error {
    fn from(value: LibMrotError) -> Self {
        Error::LibMrotError(value)
    }
}
