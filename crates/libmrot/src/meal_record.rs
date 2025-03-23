use crate::{convert::convert_to_naive_date, convert_date_to_timestamp, parse_date, Error, Result};
use std::{fmt, str::FromStr};

/// Container for a meal and a date on which it was recorded.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MealRecord {
    /// The meal.
    pub meal: String,
    /// The date on which this meal was recorded, formatted as Unix timestamp
    pub timestamp: i64,
}

impl MealRecord {
    /// Constructs a new MealRecord.
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
        let naive_date = convert_to_naive_date(self.timestamp).expect("cannot format meal record");
        write!(f, "{} ({})", self.meal, naive_date)
    }
}
