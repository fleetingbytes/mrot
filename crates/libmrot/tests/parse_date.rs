//! Implementation of tests for libmrot

use cucumber::{when, then};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::NaiveDates};
use libmrot::parse_date;

#[when(regex = "^I parse the date \"(?P<text_date>.*)\"$")]
async fn parse_the_date(world: &mut World, date: String) -> Result<()> {
    world.two_timer_parse_result = Some(format!("{:?}", two_timer::parse(&date, None)?));
    world.parse_result = Some(parse_date(&date));
    Ok(())
}

#[then(regex = r"^two_timer's intermediate parse result is (?P<intermediate>.*)$")]
async fn check_intermediate_parse_result(world: &mut World, expected: String) -> Result<()> {
    let actual = world.two_timer_parse_result.clone().ok_or(Error::UndefinedValue("two_timer_parse_result".to_string()))?;
    assert_eq!(actual, expected, "parse result was {} but we expected {}", actual, expected);
    Ok(())
}

#[then(regex = r"^our own parse result is (?P<naive_dates>.*)$")]
async fn check_parse_result(world: &mut World, expected_dates: NaiveDates) -> Result<()> {
    let actual_dates = world.parse_result.as_ref().ok_or(Error::UndefinedValue("parse_result".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(*actual_dates, expected_dates.to_vec_naivedate(), "parse result was {:?} but we expected {:?}", actual_dates, expected_dates);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/parse_date.feature").await;
}
