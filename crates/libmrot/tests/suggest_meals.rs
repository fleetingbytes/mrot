//! Implementation of tests for libmrot

use cucumber::{when, then};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::{TextLookAhead, Meals, MealRecords}};
#[allow(unused_imports)]
use mrot_test_utils::common_steps::a_storage_with_records;

#[when(regex = r"^I ask for (?P<number>\d+) meal suggestions, ignoring (?P<ignore_list>.*) and look-ahead (?P<look_ahead>.*)$")]
async fn ask_for_suggestions(world: &mut World, number: u64, ignore_list: Meals, look_ahead: TextLookAhead) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let ignore = ignore_list.to_vec_string();
    let look_ahead = look_ahead.to_option_lookahead();
    let result = storage.what(number, look_ahead, ignore);
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
