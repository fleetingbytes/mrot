//!~Meal Rotator Library
//!
//!
#![deny(missing_docs)]

pub mod cli;
mod config;
pub mod error;
mod meal;
mod storage;

use error::Error;
use meal::Meal;
use storage::Storage;
use two_timer::parse;

/// Opens the storage for data
pub fn open_storage() -> Result<Storage, Error> {
    Ok(Storage::open()?)
}

/// Adds a meal to the storage
pub fn add_meal(meal: &str, date: &str, storage: &Storage) -> Result<(), Error> {
    let (date, _end_date, range) = parse(date, None)?;
    if range {
        return Err(Error::TimeSpanNotSupported);
    }
    storage.add_meal(date.date(), meal)?;
    Ok(())
}

/// Looks up meals between the given dates
pub fn meals_between_dates(range: &str, storage: &Storage) -> Result<Vec<Meal>, Error> {
    let (start, end, _real_range) = parse(range, None)?;
    Ok(storage.meals_between_dates(start.date(), end.date())?)
}
