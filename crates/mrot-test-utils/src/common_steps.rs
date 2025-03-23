//! A collection of test steps used in the tests of [libmrot] which are shared among test targets

use crate::{World, Result, Error, argument::{DateString, MealRecords}};
use cucumber::{given, then, gherkin::Step};
use tracing::debug;
use libmrot::Storage;

/// Provides a storages filled with the records specified in the feature file
#[given(regex = r"^an in-memory storage with the records$")]
pub async fn a_storage_with_records(world: &mut World, step: &Step) -> Result<()> {
    if let Some(table) = step.table.as_ref() {
        let storage = Storage::open(":memory:")?;
        for row in table.rows.iter().skip(1) {
            let date_string = row[0].parse::<DateString>()?;
            let meal = &row[1];
            let dates: Vec<String> = vec![format!("{}", date_string)];
            debug!(meal, ?dates, "adding to storage");
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
    assert_eq!(*actual_records, expected_records.to_vec_mealrecord(), "storage.what returned {:?} but we expected {:?}", actual_records, expected_records);
    Ok(())
}
