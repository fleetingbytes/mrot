//! Functions to convert between various formats of dates.

use crate::{Error, Result};
use chrono::{DateTime, Days, NaiveDate, NaiveDateTime, TimeDelta};
use tracing::{instrument, Span};

/// Parses a given string into a vector of naive dates.
/// Implicit or explicit time ranges (see two_timer's [literal range](https://docs.rs/two_timer/latest/two_timer/)) may result in multiple dates,
/// if the range is longer than one full day.
/// See the [parse date feature file](https://github.com/fleetingbytes/mrot/tree/master/crates/libmrot/tests/features/parse_date.feature)
/// for detailed examples.
///
/// The result is guaranteed to contain at least one [`NaiveDate`].
///
/// Example:
/// ```
/// use libmrot::parse_date;
/// use chrono::NaiveDate;
///
/// let date: NaiveDate = parse_date("April 6th, 2025").unwrap().pop().unwrap();
/// let expected_date = NaiveDate::from_ymd_opt(2025, 4, 6).unwrap();
///
/// assert_eq!(date, expected_date);
///
/// let dates = parse_date("one day before and after March 1st, 2024").unwrap();
/// let expected_dates = vec![
///     NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
///     NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
///     NaiveDate::from_ymd_opt(2024, 3, 2).unwrap(),
/// ];
///
/// assert_eq!(dates, expected_dates);
/// ```
///
/// Error:
///
/// Returns an error if the date cannot be parsed.
/// ```
/// use libmrot::{parse_date, Error};
///
/// let error_result = parse_date("Christmas Eve 2025").unwrap_err();
/// assert!(matches!(error_result, Error::TwoTimer(_)));
/// ```
#[instrument]
pub fn parse_date(date: &str) -> Result<Vec<NaiveDate>> {
    let (start_datetime, end_datetime, range_is_explicit) = two_timer::parse(date, None)?;
    let mut result = Vec::new();

    add_date(start_datetime, &mut result);
    let (number_of_full_days, rest) = get_full_days_and_rest(start_datetime, end_datetime);
    add_end_dates_of_fully_included_day_periods(&mut result, start_datetime, number_of_full_days);
    if rest.is_zero()
        && ((range_is_explicit && number_of_full_days > 0)
            || (!range_is_explicit && number_of_full_days == 1))
    {
        remove_last_date(&mut result);
    }
    Ok(result)
}

#[instrument]
fn add_date(datetime: NaiveDateTime, vec: &mut Vec<NaiveDate>) {
    let date = datetime.date();
    vec.push(date);
}

#[instrument]
fn get_full_days_and_rest(start: NaiveDateTime, end: NaiveDateTime) -> (u64, TimeDelta) {
    let timedelta = end - start;
    let number_of_full_days = timedelta.num_days() as u64;
    let offset_start = start
        .checked_add_days(Days::new(number_of_full_days))
        .unwrap();
    let rest = end - offset_start;
    (number_of_full_days, rest)
}

#[instrument]
fn add_end_dates_of_fully_included_day_periods(
    vec: &mut Vec<NaiveDate>,
    start: NaiveDateTime,
    number_of_fully_included_days: u64,
) {
    for n in 1..number_of_fully_included_days + 1 {
        let datetime = generate_date_offset_by_n_days(start, n);
        add_date(datetime, vec);
    }
}

#[instrument]
fn generate_date_offset_by_n_days(start: NaiveDateTime, n: u64) -> NaiveDateTime {
    let n_days = Days::new(n);
    start.checked_add_days(n_days).unwrap()
}

#[instrument]
fn remove_last_date(vec: &mut Vec<NaiveDate>) {
    _ = vec.pop();
}

/// Convert multiple human-readable date strings to timestamps. The result vector is guaranteed to contain
/// at least one timestamp per string in the input vector.
///
/// Example:
/// ```
/// use libmrot::convert_to_timestamps;
///
/// let timestamps = convert_to_timestamps(
///     &vec![
///         "April 6th, 2025".to_string(),
///         "one day before and after March 1st, 2024".to_string(),
///     ]
/// ).unwrap();
/// let expected_timestamps = vec![
///     1743897600,
///     1709164800,
///     1709251200,
///     1709337600,
/// ];
///
/// assert_eq!(timestamps, expected_timestamps);
/// ```
///
/// Error:
///
/// Returns an error if a date cannot be parsed.
/// ```
/// use libmrot::{convert_to_timestamps, Error};
///
/// let error_result = convert_to_timestamps(
///     &vec!["Christmas Eve 2025".to_string()]
/// ).unwrap_err();
/// assert!(matches!(error_result, Error::TwoTimer(_)));
/// ```
#[instrument]
pub fn convert_to_timestamps(dates: &Vec<String>) -> Result<Vec<i64>> {
    dates
        .iter()
        .map(|date| {
            parse_date(date).map(|naive_dates| {
                naive_dates
                    .iter()
                    .map(|naive_date| convert_date_to_timestamp(naive_date))
                    .collect::<Vec<i64>>()
            })
        })
        .collect::<Result<Vec<Vec<i64>>>>()
        .map(|nested| nested.into_iter().flatten().collect())
}

/// Converts a NaiveDate to Unix timestamp
#[instrument(level = "debug", fields(result))]
pub(crate) fn convert_date_to_timestamp(date: &NaiveDate) -> i64 {
    let timestamp = date
        .and_hms_opt(0, 0, 0)
        .expect("invalid hour, minute, or second")
        .and_utc()
        .timestamp();
    Span::current().record("result", &timestamp);
    timestamp
}

pub(crate) fn convert_to_naive_date(i: i64) -> Result<NaiveDate> {
    let dt = DateTime::from_timestamp(i, 0).ok_or(Error::InvalidTimestamp(i))?;
    Ok(dt.date_naive())
}
