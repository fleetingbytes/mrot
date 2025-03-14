//! Implementation of tests for libmrot

use cucumber::{when, given, then};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::{NaiveDates, TextDates, Meals}};
use libmrot::Storage;
use tracing::trace;

#[given(regex = r"^an empty in-memory storage$")]
async fn a_storage(world: &mut World) -> Result<()> {
    let storage = Storage::open(":memory:")?;
    trace!(%storage, "Opened storage");
    world.storage = Some(storage);
    Ok(())
}

#[when(regex = r"^I add the meal (?P<meal>.*) on the dates? (?P<text_dates>.*) to the storage$")]
async fn add_meal_on_dates(world: &mut World, meal: String, text_dates: TextDates) -> Result<()> {
    let dates = text_dates.to_vec_string();
    trace!(?meal, ?dates);
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    storage.add_meal_on_dates(&meal, &dates)?;
    Ok(())
}

#[then(regex = r"^the storage, asked when (?P<meal>.*) was recorded, returns (?P<naive_dates>.*)$")]
async fn storage_when(world: &mut World, meal: String, expected_naive_dates: NaiveDates) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let actual_naive_dates = storage.when(&meal)?;
    trace!(?actual_naive_dates, ?expected_naive_dates);
    assert_eq!(actual_naive_dates, expected_naive_dates.to_vec_naivedate(), "storage.when returned {:?} but we expected {:?}", actual_naive_dates, expected_naive_dates);
    Ok(())
}

#[then(regex = r"^the storage, asked for the dates (?P<show_range>.*) returns (?P<meals>.*)$")]
async fn storage_show(world: &mut World, show_range: String, expected_meals: Meals) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let actual_meals: Vec<String> = storage.show(&show_range)?.iter().map(|meal_record| meal_record.meal.clone()).collect();
    trace!(?actual_meals, ?expected_meals);
    assert_eq!(actual_meals, expected_meals.to_vec_string(), "storage.show returned {:?} but we expected {:?}", actual_meals, expected_meals);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/add_when_show.feature").await;
}
