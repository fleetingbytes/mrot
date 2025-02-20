//! Data Storage Abstraction

use crate::Result;
use std::path::Path;

/// Defines the API of a data storage for the needs of [mrot].
pub trait Storage {
    /// Opens a storage.
    fn open<T: AsRef<Path>>(path: T) -> Result<Self>
    where
        Self: Sized;
    /// Adds a meal on the given dates to the storage.
    fn add_meal_on_dates(&self, meal: &str, dates: &Vec<String>) -> Result<()>;
}
