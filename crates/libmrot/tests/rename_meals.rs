//! Implementation of tests for libmrot

use cucumber::when;
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::WrappedPeriod};
#[allow(unused_imports)]
use mrot_test_utils::common_steps::{check_result_vec_mealrecord, a_storage_with_records, storage_when_meal};

#[when(regex = r"^I rename the meal (?P<old_name>.*) to (?P<new_name>.*) in the period (?P<period>.*)$")]
async fn rename_meal(world: &mut World, old_name: String, new_name: String, period: WrappedPeriod) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let option_period = period.to_option_period();
    let result = storage.rename(&old_name, &new_name, option_period);
    world.result_vec_mealrecord = Some(result);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/rename_meals.feature").await;
}
