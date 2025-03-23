//! Implementation of tests for libmrot

use cucumber::when;
use mrot_test_utils::{normal_world as construct_world, World, Result, Error};
#[allow(unused_imports)]
use mrot_test_utils::common_steps::{check_result_vec_mealrecord, a_storage_with_records};

#[when(regex = r"^I ask for unique meals$")]
async fn ask_for_unique_meals(world: &mut World) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let result = storage.get_last_cooked_unique();
    world.result_vec_mealrecord = Some(result);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/unique_meals.feature").await;
}
