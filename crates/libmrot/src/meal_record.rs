use crate::Error;
use std::str::FromStr;

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
