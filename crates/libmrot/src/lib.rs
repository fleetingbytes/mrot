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

/// Holds data about a date or a date range. This can be for example a period
/// in  which to search for meals which the user
/// explicitly planned so that these can be excluded from meal suggestions.
/// See section [Getting Meal Suggestions](https://github.com/fleetingbytes/mrot/#getting-meal-suggestions) in the mrot readme.
#[derive(Debug, Clone)]
pub struct Period {
    first_day_timestamp: i64,
    last_day_timestamp: i64,
    first_date: NaiveDate,
    last_date: NaiveDate,
}

impl Period {
    /// Construct a new `Period`. The string argument should be a parsable date expression
    /// (cf. [parse_date]).
    ///
    /// Examples of successfully constructed [Period]s:
    /// ```
    /// use libmrot::Period;
    /// use chrono::Days;
    ///
    /// let one_day_period = Period::new("tomorrow").unwrap();
    /// assert_eq!(one_day_period.first_date(), one_day_period.last_date());
    ///
    /// let multiple_day_period = Period::new(
    ///     "from tomorrow through 11 days after tomorrow"
    ///     ).unwrap();
    /// assert_eq!(multiple_day_period.first_date()
    ///     .checked_add_days(Days::new(11))
    ///     .unwrap(),
    ///     multiple_day_period.last_date()
    ///     );
    /// ```
    ///
    /// Error:
    /// ```
    /// use libmrot::{Error, Period};
    ///
    /// let unparsable_date = "Christmas Eve 2025";
    /// let error_result = Period::new(unparsable_date).unwrap_err();
    /// assert!(matches!(error_result, Error::TwoTimer(_)));
    /// ```
    pub fn new(s: &str) -> Result<Self> {
        let dates = parse_date(s)?;
        let first_date = dates.iter().next().unwrap();
        let last_date = dates.iter().last().unwrap();
        let first_day_timestamp = convert_date_to_timestamp(first_date);
        let last_day_timestamp = convert_date_to_timestamp(last_date);
        Ok(Self {
            first_day_timestamp,
            last_day_timestamp,
            first_date: *first_date,
            last_date: *last_date,
        })
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
