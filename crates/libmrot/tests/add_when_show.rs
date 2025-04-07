//! Implementation of tests for libmrot

use cucumber::{when, given};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::TextDates};
use libmrot::Storage;
#[allow(unused_imports)]
use mrot_test_utils::common_steps::{a_storage_with_records, storage_show_meal_records, storage_when_meal};

#[given(regex = r"^an empty in-memory storage$")]
async fn a_storage(world: &mut World) -> Result<()> {
    let storage = Storage::open(":memory:")?;
    world.storage = Some(storage);
    Ok(())
}

#[when(regex = r"^I add the meal (?P<meal>.*) on the dates? (?P<text_dates>.*) to the storage$")]
async fn add_meal_on_dates(world: &mut World, meal: String, text_dates: TextDates) -> Result<()> {
    let dates = text_dates.to_vec_string();
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    storage.add_meal_on_dates(&meal, &dates)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/add_when_show.feature").await;
}
