//! Meal Rotator Library
//!
//! A library for recording, planning and suggesting meals. It is the core of the meal rotator app [mrot][mrot].
//!
//! [mrot]: https://docs.rs/mrot

pub mod error;
mod meal;
pub mod storage;

use chrono::naive::NaiveDate;
pub use error::Error;
use meal::Meal;
pub use storage::Storage;
use tracing::{instrument, Span};
use two_timer::parse;

/// Type alias for results with libmrot's [Error].
pub type Result<T> = core::result::Result<T, Error>;

/// Opens the storage for reading or writing data.
#[instrument]
pub fn open_storage() -> Result<Storage> {
    Ok(Storage::open()?)
}

/// Adds a meal on several dates to the storage.
#[instrument]
pub fn add_meal_on_dates(meal: &str, dates: &Vec<String>, storage: &Storage) -> Result<()> {
    for date in dates {
        add_meal(&meal, &date, &storage)?;
    }
    Ok(())
}

/// Adds a meal to the storage.
#[instrument]
pub fn add_meal(meal: &str, date: &str, storage: &Storage) -> Result<()> {
    let naive_date = parse_date(date)?;
    storage.add_meal(naive_date, meal)?;
    Ok(())
}

/// Parses a date string into NaiveDate.
#[instrument(level = "debug", fields(result))]
fn parse_date(date: &str) -> Result<NaiveDate> {
    let (naive_datetime, _end_date, range) = parse(date, None)?;
    if range {
        return Err(Error::TimeSpanNotSupported);
    }
    let naive_date = naive_datetime.date();
    Span::current().record("result", &naive_date.to_string());
    Ok(naive_date)
}

/// Looks up meals between the given dates.
#[instrument]
pub fn meals_between_dates(range: &str, storage: &Storage) -> Result<Vec<Meal>> {
    let (start, end, _real_range) = parse(range, None)?;
    Ok(storage.meals_between_dates(start.date(), end.date())?)
}
