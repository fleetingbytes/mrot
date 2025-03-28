//! Contains the arguments used in test implementation and the instructions how to parse them from
//! a string.

use crate::Error;
use chrono::{naive::NaiveDate, DateTime};
use libmrot::{LookAhead, MealRecord, Period};
use std::{fmt, str::FromStr};

const NAIVE_DATE_PARSE_FROM_STRING_FORMAT: &str = "%Y-%m-%d";

#[derive(Default)]
pub struct TextDate(String);

impl FromStr for TextDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TextDate(String::from(s)))
    }
}

impl fmt::Debug for TextDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TextDate({})", self.0)
    }
}

#[derive(Default)]
pub struct TextDates(Vec<TextDate>);

impl TextDates {
    pub fn to_vec_string(&self) -> Vec<String> {
        self.0.iter().map(|date| date.0.clone()).collect()
    }
}

impl FromStr for TextDates {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dates: Result<Vec<TextDate>, _> = s.split("; ").map(TextDate::from_str).collect();
        dates.map(TextDates)
    }
}

impl fmt::Debug for TextDates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

#[derive(Default)]
pub struct NaiveDates(Vec<NaiveDate>);

impl NaiveDates {
    pub fn to_vec_naivedate(&self) -> Vec<NaiveDate> {
        self.0.clone()
    }
}

impl FromStr for NaiveDates {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dates: Result<Vec<NaiveDate>, _> = s
            .split(", ")
            .map(|date_str| {
                NaiveDate::parse_from_str(date_str, NAIVE_DATE_PARSE_FROM_STRING_FORMAT)
            })
            .collect();
        Ok(dates.map(NaiveDates)?)
    }
}

impl fmt::Debug for NaiveDates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

#[derive(Default)]
pub struct Meals(Vec<String>);

impl Meals {
    pub fn to_vec_string(&self) -> Vec<String> {
        self.0.clone()
    }
}

impl FromStr for Meals {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let meals: Vec<String> = s.split(", ").map(String::from).collect();
        Ok(Meals(meals))
    }
}

impl fmt::Debug for Meals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

#[derive(Default)]
pub struct DateString(String);

impl FromStr for DateString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timestamp = i64::from_str(s)?;
        let dt = DateTime::from_timestamp(timestamp, 0)
            .ok_or(libmrot::Error::InvalidTimestamp(timestamp))?;
        let naive_date = dt.date_naive();
        let result_string = naive_date
            .format(NAIVE_DATE_PARSE_FROM_STRING_FORMAT)
            .to_string();
        Ok(DateString(result_string))
    }
}

impl fmt::Debug for DateString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DateString({})", self.0)
    }
}

impl fmt::Display for DateString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default)]
pub struct MealRecords(Vec<MealRecord>);

impl MealRecords {
    pub fn to_vec_mealrecord(&self) -> Vec<MealRecord> {
        self.0
            .iter()
            .map(|meal_record| meal_record.clone())
            .collect()
    }
}

impl FromStr for MealRecords {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Ok(MealRecords::default());
        }
        let meal_records: Result<Vec<MealRecord>, _> =
            s.split("; ").map(MealRecord::from_str).collect();
        Ok(meal_records.map(MealRecords)?)
    }
}

impl fmt::Debug for MealRecords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

#[derive(Default)]
pub struct TextLookAhead(Option<LookAhead>);

impl TextLookAhead {
    pub fn to_option_lookahead(&self) -> Option<LookAhead> {
        match self.0 {
            None => None,
            Some(ref look_ahead) => Some(look_ahead.clone()),
        }
    }
}

impl FromStr for TextLookAhead {
    type Err = Error;

    /// Construct a TextLookAhead from a string.
    /// The string `"None"` constructs TextLookAhead containing the [None] variant of
    /// `[Option]<[LookAhead]>`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let option_look_ahead = match s {
            "None" => LookAhead::new(None)?,
            _ => LookAhead::new(Some(String::from(s)))?,
        };
        Ok(TextLookAhead(option_look_ahead))
    }
}

#[derive(Default)]
pub struct WrappedPeriod(Option<Period>);

impl WrappedPeriod {
    pub fn to_option_period(&self) -> Option<Period> {
        match self.0 {
            None => None,
            Some(ref period) => Some(period.clone()),
        }
    }
}

impl FromStr for WrappedPeriod {
    type Err = Error;

    /// Construct a WrappedPeriod from a string.
    /// The string `"None"` constructs WrappedPeriod containing `None`.
    /// Other strings try to construct a WrappedPeriod containing `Some(Period {...})`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let option_period = match s {
            "None" => None,
            _ => Some(Period::new(s)?),
        };
        Ok(WrappedPeriod(option_period))
    }
}
