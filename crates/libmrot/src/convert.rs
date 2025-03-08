//! Functions to convert between various formats of dates.

use crate::Result;
use chrono::{Days, NaiveDate, NaiveDateTime, TimeDelta};
use tracing::instrument;

/// Parses a given string into a vector of naive dates.
/// Implicit or explicit time ranges (see [literal range][two-timer]) may result in multiple dates,
/// if the range is longer than one full day.
/// See the [parse date feature file][parse-date-feature] for detailed examples.
/// [parse-date-feature]: https://github.com/fleetingbytes/mrot/tree/master/crates/libmrot/tests/features/parse_date.feature
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
