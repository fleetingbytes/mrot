//!~Meal Rotator Library
//!
//!
#![deny(missing_docs)]

pub mod cli;
mod config;
mod error;
mod storage;

use chrono::NaiveDateTime;
use error::Error;
use storage::Storage;
use two_timer::parse;

/// Opens the storage for data
pub fn open_storage() -> Result<Storage, Error> {
    Ok(Storage::open()?)
}

/// shared between adding a meal and a meal plan
fn add_meal_core(
    meal: &str,
    date: NaiveDateTime,
    planned: bool,
    storage: &Storage,
) -> Result<(), Error> {
    storage.add_meal(date, meal, planned)?;
    Ok(())
}
/// Adds a meal to the storage
pub fn add_meal(meal: &str, date: &str, storage: &Storage) -> Result<(), Error> {
    let (date, end_date, range) = parse(date, None)?;
    if range {
        return Err(Error::UnacceptableDate(
            date.to_string(),
            end_date.to_string(),
        ));
    }
    add_meal_core(meal, date, false, storage)?;
    Ok(())
}
/// Adds a plan to the storage
pub fn add_plan(meal: &str, date: &str, storage: &Storage) -> Result<(), Error> {
    let (date, end_date, range) = parse(date, None)?;
    if range {
        return Err(Error::UnacceptableDate(
            date.to_string(),
            end_date.to_string(),
        ));
    }
    add_meal_core(meal, date, true, storage)?;
    Ok(())
}
