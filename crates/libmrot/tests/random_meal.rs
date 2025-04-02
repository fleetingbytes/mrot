//! Implementation of tests for libmrot

use cucumber::{when, then};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::Meals};
#[allow(unused_imports)]
use mrot_test_utils::common_steps::a_storage_with_records;

#[when(regex = r"^I ask for a random meal")]
async fn get_random_meal(world: &mut World) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let result = storage.random();
    world.result_option_mealrecord = Some(result);
    Ok(())
}

#[then(regex = r"^I get no meal record")]
async fn get_no_meal_record(world: &mut World) -> Result<()> {
    let actual_record = world.result_option_mealrecord.as_ref().ok_or(Error::UndefinedValue("storage_random_result".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?;
    assert!(actual_record.is_none(), "meal record from storage.random was not None");
    Ok(())
}

#[then(regex = r"^I get the meal (?P<meals>.*)$")]
async fn verify_one_random_meal(world: &mut World, meals: Meals) -> Result<()> {
    let actual_record = world.result_option_mealrecord.as_ref().ok_or(Error::UndefinedValue("storage_random_result".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?.clone().ok_or(Error::UndefinedValue("the Option in storage_random_result was None".to_string()))?;
    let mut expected_meals_vec = meals.to_vec_string();
    let expected_meal = expected_meals_vec.pop().ok_or(Error::UndefinedValue("did not find any expected meal in the feature file".to_string()))?;
    assert_eq!(*actual_record.meal(), expected_meal, "storage.random returned {:?} but we expected {:?}", actual_record.meal(), expected_meal);
    Ok(())
}

#[then(regex = r"^the meal is one of (?P<meals>.*)$")]
async fn verify_random_is_one_of_expected_meals(world: &mut World, meals: Meals) -> Result<()> {
    let actual_record = world.result_option_mealrecord.as_ref().ok_or(Error::UndefinedValue("storage_random_result".to_string()))?.as_ref().map_err(|e| Error::UnexpectedErrResult(format!("{:?}", e)))?.clone().ok_or(Error::UndefinedValue("the Option in storage_random_result was None".to_string()))?;
    let expected_meals_vec = meals.to_vec_string();
    assert!(expected_meals_vec.contains(&actual_record.meal()), "storage.random returned an unexpected meal record");
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/random_meal.feature").await;
}
