//! Implementation of tests for libmrot

use cucumber::when;
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::{WrappedPeriod, Meals}};
#[allow(unused_imports)]
use mrot_test_utils::common_steps::{check_result_vec_mealrecord, a_storage_with_records};

#[when(regex = r"^I ask for (?P<number>\d+) meal suggestions, ignoring (?P<ignore_list>.*) and ignore period (?P<ignore_period>.*)$")]
async fn ask_for_suggestions(world: &mut World, number: u64, ignore_list: Meals, ignore_period: WrappedPeriod) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let ignore = ignore_list.to_vec_string();
    let option_period = ignore_period.to_option_period();
    let result = storage.what(number, option_period, ignore);
    world.result_vec_mealrecord = Some(result);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/suggest_meals.feature").await;
}
