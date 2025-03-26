//! Meal Rotator Library
//!
//! A library for recording, planning and suggesting meals. It is the core of the meal rotator app [mrot][mrot].
//!
//! [mrot]: https://docs.rs/mrot

mod convert;
mod error;
mod meal_record;
mod storage;

use crate::convert::convert_date_to_timestamp;
use chrono::NaiveDate;
pub use convert::{convert_to_timestamps, parse_date};
pub use error::Error;
pub use meal_record::MealRecord;
pub use storage::Storage;

/// Type alias for results with libmrot's [Error].
pub type Result<T> = std::result::Result<T, Error>;

/// Holds data related to the look-ahead period in which to search for meals which the user
/// explicitly planned so that they can be excluded from meal suggestions.
/// See section [Getting Meal Suggestions](https://github.com/fleetingbytes/mrot/#getting-meal-suggestions) in the mrot readme.
///
/// In the libmrot API [LookAhead] is always used behind an [Option], i.e. `Option<LookAhead>`,
/// where [None] signals "no look-ahead" at all.
#[derive(Debug, Clone)]
pub struct LookAhead {
    first_day_timestamp: i64,
    last_day_timestamp: i64,
    first_date: NaiveDate,
    last_date: NaiveDate,
}

impl LookAhead {
    /// Construct a new `Option<LookAhead>`. If *date* is the `Some` variant, it should contain a parsable date expression
    /// (cf. [parse_date]).
    ///
    /// Examples of successfully constructed LookAheads:
    /// ```
    /// use libmrot::LookAhead;
    /// use chrono::Days;
    ///
    /// let no_look_ahead: Option<LookAhead> = LookAhead::new(None).unwrap();
    /// assert!(no_look_ahead.is_none());
    ///
    /// let one_day_look_ahead: Option<LookAhead> = LookAhead::new(
    ///     Some("tomorrow".to_string())
    ///     ).unwrap();
    /// assert!(one_day_look_ahead.is_some_and(|la| la.first_date() == la.last_date()));
    ///
    /// let multiple_day_look_ahead: Option<LookAhead> = LookAhead::new(
    ///     Some("from tomorrow through 11 days after tomorrow".to_string())
    ///     ).unwrap();
    /// assert!(multiple_day_look_ahead.is_some_and(
    ///     |la| la.first_date().checked_add_days(Days::new(11)).unwrap() == la.last_date())
    ///     );
    /// ```
    ///
    /// Error:
    /// ```
    /// use libmrot::{Error, LookAhead};
    ///
    /// let unparsable_date_string = "Christmas Eve 2025".to_string();
    /// let error_result = LookAhead::new(Some(unparsable_date_string)).unwrap_err();
    /// assert!(matches!(error_result, Error::TwoTimer(_)));
    /// ```
    pub fn new(date: Option<String>) -> Result<Option<Self>> {
        match date {
            None => Ok(None),
            Some(date_string) => {
                let dates = parse_date(&date_string)?;
                let first_date = dates.iter().next().unwrap();
                let last_date = dates.iter().last().unwrap();
                let first_day_timestamp = convert_date_to_timestamp(first_date);
                let last_day_timestamp = convert_date_to_timestamp(last_date);
                Ok(Some(Self {
                    first_day_timestamp,
                    last_day_timestamp,
                    first_date: *first_date,
                    last_date: *last_date,
                }))
            }
        }
    }

    /// Returns the [`NaiveDate`] of the first day of the look-ahead period.
    pub fn first_date(&self) -> NaiveDate {
        self.first_date
    }

    /// Returns the timestamp of the first day of the look-ahead period.
    pub fn first_day_timestamp(&self) -> i64 {
        self.first_day_timestamp
    }

    /// Return the [`NaiveDate`] of the last day of the look-ahead period.
    pub fn last_date(&self) -> NaiveDate {
        self.last_date
    }

    /// Returns the timestamp of the last day of the look-ahead period.
    pub fn last_day_timestamp(&self) -> i64 {
        self.last_day_timestamp
    }
}
