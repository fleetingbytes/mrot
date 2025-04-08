//! A collection of test steps used in the tests of [libmrot] which are shared among test targets

use crate::{World, Result, Error, argument::{DateString, MealRecords, NaiveDates}};
use cucumber::{given, then, gherkin::Step};
use libmrot::Storage;

/// Provides a storage filled with the records specified in the feature file (in the step table)
#[given(regex = r"^an in-memory storage with the records$")]
pub async fn a_storage_with_records(world: &mut World, step: &Step) -> Result<()> {
    if let Some(table) = step.table.as_ref() {
        let storage = Storage::open(":memory:")?;
        for row in table.rows.iter().skip(1) {
            let date_string = row[0].parse::<DateString>()?;
            let meal = &row[1];
            let dates: Vec<String> = vec![format!("{}", date_string)];
            storage.add_meal_on_dates(meal, &dates)?;
        }
        world.storage = Some(storage);
    }
    Ok(())
}

/// Checks the `Result<Vec<MealRecord>>`
#[then(regex = r"^I get the meal records (?P<records>.*)$")]
pub async fn check_result_vec_mealrecord(world: &mut World, expected_records: MealRecords) -> Result<()> {
    let actual_records = world.result_vec_mealrecord.as_ref().ok_or(Error::UndefinedValue("storage_what_result".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(*actual_records, expected_records.to_vec_mealrecord(), "found {:?} but we expected {:?}", actual_records, expected_records);
    Ok(())
}

/// Checks the content of the storage
#[then(regex = r"^the storage, asked to show the meal records in the period (?P<show_range>.*), returns (?P<meal_records>.*)$")]
pub async fn storage_show_meal_records(world: &mut World, show_range: String, expected_meal_records: MealRecords) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let actual_meal_records = storage.show(&show_range)?;
    assert_eq!(actual_meal_records, expected_meal_records.to_vec_mealrecord(), "storage.show returned {:?} but we expected {:?}", actual_meal_records, expected_meal_records);
    Ok(())
}

/// Checks the dates of when a meal was consumed
#[then(regex = r"^the storage, asked when (?P<meal>.*) was consumed, returns (?P<naive_dates>.*)$")]
pub async fn storage_when_meal(world: &mut World, meal: String, expected_naive_dates: NaiveDates) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let actual_naive_dates = storage.when(&meal)?;
    assert_eq!(actual_naive_dates, expected_naive_dates.to_vec_naivedate(), "storage.when returned {:?} but we expected {:?}", actual_naive_dates, expected_naive_dates);
    Ok(())
}
