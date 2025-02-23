//! Tests for libmrot

use cucumber::{when, given, then};
use mrot_test_utils::{debug_world as construct_world, World, Result, Error};
use libmrot::Storage;
use tracing::trace;

#[given(regex = "^a storage$")]
async fn a_storage(world: &mut World) -> Result<()> {
    let storage = Storage::open(":memory:")?;
    trace!(%storage, "Opened storage");
    world.storage = Some(storage);
    Ok(())
}

#[given(regex = "^some dates$")]
async fn some_dates(world: &mut World) -> Result<()> {
    world.dates = Some(vec![
        String::from("2025-02-22"),
        String::from("2025-02-23"),
        String::from("2025-02-24"),
    ]);
    Ok(())
}

#[given(regex = "^a meal$")]
async fn a_meal(world: &mut World) -> Result<()> {
    world.meal = Some(String::from("porridge"));
    Ok(())
}

#[when(regex = "^I add the meal on those dates to the storage$")]
async fn add_meal_on_dates(world: &mut World) -> Result<()> {
    let meal = world.meal.as_ref().ok_or(Error::UndefinedValue("meal".to_string()))?;
    let dates = world.dates.as_ref().ok_or(Error::UndefinedValue("dates".to_string()))?;
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    storage.add_meal_on_dates(meal, dates)?;
    Ok(())
}

#[then(regex = "^the storage contains these records$")]
async fn storage_contains_records(world: &mut World) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    Err(Error::NotImplemented)
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features").await;
}
