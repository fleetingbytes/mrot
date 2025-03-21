use crate::{convert::convert_to_naive_date, Error};
use std::{fmt, str::FromStr};

/// Container for a meal and a date on which it was recorded.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MealRecord {
    /// The meal.
    pub meal: String,
    /// The date on which this meal was recorded, formatted as Unix timestamp
    pub timestamp: i64,
}

impl FromStr for MealRecord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
