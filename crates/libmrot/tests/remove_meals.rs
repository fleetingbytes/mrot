//! Implementation of tests for libmrot

use cucumber::when;
use mrot_test_utils::{normal_world as construct_world, World, Result, Error};
#[allow(unused_imports)]
use mrot_test_utils::common_steps::{check_result_vec_mealrecord, a_storage_with_records, storage_show_meal_records};

#[when(regex = r"^I remove all meals in the period (?P<period>.*)$")]
async fn remove_all_meals(world: &mut World, period: String) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let result = storage.remove_all(&period);
    world.result_vec_mealrecord = Some(result);
    Ok(())
}

#[when(regex = r"^I remove the meal (?P<meal>.*) in the period (?P<period>.*)$")]
async fn remove_meal_in_period(world: &mut World, meal: String, period: String) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let result = storage.remove(&meal, &period);
    world.result_vec_mealrecord = Some(result);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/remove_meals.feature").await;
}
