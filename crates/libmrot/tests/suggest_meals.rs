//! Implementation of tests for libmrot

use cucumber::{given, when, then, gherkin::Step};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::{DateString, Meals, MealRecords}};
use libmrot::Storage;
use tracing::debug;

#[given(regex = r"^an in-memory storage with the records$")]
async fn a_storage_with_records(world: &mut World, step: &Step) -> Result<()> {
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
#[when(regex = r"^I ask for (?P<number>\d+) meal suggestions, ignoring (?P<ignore_list>.*) and look-ahead (?P<look_ahead>.*)$")]
async fn ask_for_suggestions(world: &mut World, number: usize, ignore_list: Meals, look_ahead: String) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let ignore = ignore_list.to_vec_string();
    let result = storage.what(number, &ignore, Some(&look_ahead));
    world.storage_what_result = Some(result);
    Ok(())
}

#[then(regex = r"^I get the meal records (?P<records>.*)$")]
async fn storage_when(world: &mut World, expected_records: MealRecords) -> Result<()> {
    let actual_records = world.storage_what_result.as_ref().ok_or(Error::UndefinedValue("storage_what_result".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert_eq!(*actual_records, expected_records.to_vec_mealrecord(), "storage.what returned {:?} but we expected {:?}", actual_records, expected_records);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/suggest_meals.feature").await;
}
