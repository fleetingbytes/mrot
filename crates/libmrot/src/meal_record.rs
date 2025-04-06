use crate::{convert::convert_to_naive_date, convert_date_to_timestamp, parse_date, Error, Result};
use chrono::NaiveDate;
use std::{fmt, str::FromStr};

/// Container for a meal and a date on which it was recorded.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MealRecord {
    /// The meal.
    meal: String,
    /// The date on which this meal was recorded, formatted as Unix timestamp
    timestamp: i64,
}

impl MealRecord {
    /// Constructs a new MealRecord from a meal name and a parsable date expression.
    ///
    /// Error:
    /// - if the date expression cannot be parsed
    /// - if the date expression parses to more than one date
    ///
    /// Example:
    ///
    /// ```
    /// use libmrot::MealRecord;
    ///
    /// let meal_record = MealRecord::new("pizza", "today").unwrap();
    /// ```
    pub fn new(meal: &str, date: &str) -> Result<Self> {
        let mut dates = parse_date(date)?;
        if dates.len() > 1 {
            return Err(Error::MoreThanOneDate(date.to_string()));
        }
        let naive_date = dates.pop().unwrap();
        let timestamp = convert_date_to_timestamp(&naive_date);
        Ok(MealRecord {
            meal: meal.to_string(),
            timestamp,
        })
    }

    /// Constructs a MealRecord from a meal name and a [NaiveDate].
    ///
    /// Example:
    ///
    /// ```
    /// use libmrot::MealRecord;
    /// use chrono::NaiveDate;
    ///
    /// let nd = NaiveDate::from_ymd_opt(2025, 4, 2).unwrap();
    /// let meal_record = MealRecord::from_meal_and_naivedate("pizza", &nd);
    /// ```
    pub fn from_meal_and_naivedate(meal: &str, naive_date: &NaiveDate) -> Self {
        let timestamp = convert_date_to_timestamp(naive_date);
        MealRecord {
            meal: meal.to_string(),
            timestamp,
        }
    }

    /// Constructs a MealRecord from a meal name and a timestamp.
    /// The timestamp is quantized to the start of its day.
    ///
    /// Example:
    ///
    /// ```
    /// use libmrot::MealRecord;
    /// use chrono::{Offset, TimeZone, Utc};
    ///
    /// let unquantized: i64 = Utc
    ///     .with_ymd_and_hms(2025, 4, 2, 22, 2, 17)
    ///     .unwrap()
    ///     .timestamp();
    /// let quantized: i64 = Utc
    ///     .with_ymd_and_hms(2025, 4, 2, 0, 0, 0)
    ///     .unwrap()
    ///     .timestamp();
    ///
    /// assert_eq!(unquantized, 1743631337);
    /// assert_eq!(quantized, 1743552000);
    ///
    /// let meal_record = MealRecord::from_meal_and_timestamp(
    ///     "pizza",
    ///     unquantized,
    /// ).unwrap();
    ///
    /// assert_eq!(meal_record.timestamp(), quantized);
    /// ```
    ///
    /// Error:
    ///
    /// Returns an Error if the timestamp cannot be converted to a [`chrono::DateTime`].
    pub fn from_meal_and_timestamp(meal: &str, timestamp: i64) -> Result<Self> {
        let naive_date = convert_to_naive_date(timestamp)?;
        let timestamp = convert_date_to_timestamp(&naive_date);
        Ok(MealRecord {
            meal: meal.to_string(),
            timestamp,
        })
    }

    /// Get the meal name of the MealRecord.
    pub fn meal(&self) -> String {
        self.meal.clone()
    }

    /// Get the [NaiveDate] of the MealRecord.
    pub fn naive_date(&self) -> NaiveDate {
        convert_to_naive_date(self.timestamp)
            .expect("this MealRecord somehow bypassed the timestamp validity check")
    }

    /// Get the timestamp of the MealRecord.
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl FromStr for MealRecord {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split(", ");
        let timestamp = split
            .next()
            .ok_or(Error::ParseMealRecordError)?
            .parse::<i64>()?;
        let meal = String::from(split.next().ok_or(Error::ParseMealRecordError)?);
        Ok(MealRecord { meal, timestamp })
    }
}

impl fmt::Display for MealRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.meal(), self.naive_date())
    }
}
