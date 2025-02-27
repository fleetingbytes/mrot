//! Contains the arguments used in test implementation and the instructions how to parse them from
//! a string.

use crate::Error;
use chrono::naive::NaiveDate;
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
