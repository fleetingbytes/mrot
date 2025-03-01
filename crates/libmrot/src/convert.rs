//! Functions to convert between various formats of dates.

use crate::Result;
use chrono::NaiveDate;
use tracing::{field, instrument, trace, Span};

/// Parses a given string into a vector of naive dates.
/// If the string describes a time range, it will return all the dates included in the time range,
/// if it spans across multiple days.
#[instrument]
pub fn parse_date(date: &str) -> Result<Vec<NaiveDate>> {
    let (start_datetime, end_datetime, range) = two_timer::parse(date, None)?;
    trace!(%start_datetime, %end_datetime, %range);
    let start_date = start_datetime.date();
    let end_date = end_datetime.date();
    let result = dates_from_range(start_date, end_date, range);
    Span::current().record("result", field::display(&format!("{:?}", result)));
    Ok(result)
}

fn dates_from_range(start_date: NaiveDate, end_date: NaiveDate, range: bool) -> Vec<NaiveDate> {
    trace!(%start_date, %end_date, %range);
    let mut result = Vec::new();
    result.push(start_date);
    result.push(end_date);
    result
}
