//! Implementation of tests for libmrot

use cucumber::{when, then};
use libmrot::MealRecord;
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::NaiveDates};

#[when(regex = r"^I create a meal record from meal (?P<meal>.*) and date (?P<date>.*)$")]
async fn create_meal_record(world: &mut World, meal: String, date_string: String) -> Result<()> {
    let result = MealRecord::new(&meal, &date_string);
    world.result_mealrecord = Some(result);
    Ok(())
}

#[then(regex = r"^I get the meal record (?P<meal_record>.*)$")]
async fn check_meal_record(world: &mut World, expected_mealrecord: MealRecord) -> Result<()> {
    let actual_mealrecord = world.result_mealrecord.as_ref().ok_or(Error::UndefinedValue("result_mealrecord".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(*actual_mealrecord, expected_mealrecord, "found {:?} but we expected {:?}", actual_mealrecord, expected_mealrecord);
    Ok(())
}

#[then(regex = r"^the meal record has meal (?P<meal>.*)$")]
async fn check_meal_record_meal(world: &mut World, expected_meal: String) -> Result<()> {
    let actual_mealrecord = world.result_mealrecord.as_ref().ok_or(Error::UndefinedValue("result_mealrecord".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(actual_mealrecord.meal(), expected_meal, "found {:?} but we expected {:?}", actual_mealrecord.meal(), expected_meal);
    Ok(())
}

#[then(regex = r"^the meal record has naive date (?P<naive_date>.*)$")]
async fn check_meal_record_naive_date(world: &mut World, expected_naive_date: String) -> Result<()> {
    let actual_mealrecord = world.result_mealrecord.as_ref().ok_or(Error::UndefinedValue("result_mealrecord".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(format!("{}", actual_mealrecord.naive_date()), expected_naive_date, "found {:?} but we expected {:?}", actual_mealrecord.naive_date(), expected_naive_date);
    Ok(())
}

#[then(regex = r"^the meal record has timestamp (?P<timestamp>.*)$")]
async fn check_meal_record_timestamp(world: &mut World, expected_timestamp: i64) -> Result<()> {
    let actual_mealrecord = world.result_mealrecord.as_ref().ok_or(Error::UndefinedValue("result_mealrecord".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(actual_mealrecord.timestamp(), expected_timestamp, "found {:?} but we expected {:?}", actual_mealrecord.timestamp(), expected_timestamp);
    Ok(())
}

#[when(regex = r"^I create a meal record from meal (?P<meal>.*) and naive date (?P<naive_date>.*)$")]
async fn create_meal_record_with_naive_date(world: &mut World, meal: String, naive_dates: NaiveDates) -> Result<()> {
    let nd = naive_dates.to_vec_naivedate().pop().ok_or(Error::UndefinedValue("did not find any NaiveDate".to_string()))?;
    let meal_record = MealRecord::from_meal_and_naivedate(&meal, &nd);
    world.result_mealrecord = Some(Ok(meal_record));
    Ok(())
}

#[when(regex = r"^I create a meal record from meal (?P<meal>.*) and timestamp (?P<timestamp>.*)$")]
async fn create_meal_record_with_timestamp(world: &mut World, meal: String, unquantized_ts: i64) -> Result<()> {
    let result = MealRecord::from_meal_and_timestamp(&meal, unquantized_ts);
    world.result_mealrecord = Some(result);
    Ok(())
}


#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/meal_records.feature").await;
}
