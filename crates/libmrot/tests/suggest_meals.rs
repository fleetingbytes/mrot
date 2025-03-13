//! Implementation of tests for libmrot

use cucumber::{given, when, then, gherkin::Step};
use mrot_test_utils::{normal_world as construct_world, World, Result, Error, argument::{NaiveDates, DateString}};
use libmrot::{Storage, parse_date};
use tracing::debug;

#[given(regex = r"^an in-memory storage with the records$")]
async fn a_storage(world: &mut World, step: &Step) -> Result<()> {
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
#[when(regex = "^I parse the date \"(?P<text_date>.*)\"$")]
async fn parse_the_date(world: &mut World, date: String) -> Result<()> {
    world.two_timer_parse_result = Some(format!("{:?}", two_timer::parse(&date, None)?));
    world.parse_result = Some(parse_date(&date));
    Ok(())
}

#[then(regex = r"^the storage, asked when (?P<meal>.*) was recorded, returns (?P<naive_dates>.*)$")]
async fn storage_when(world: &mut World, meal: String, expected_naive_dates: NaiveDates) -> Result<()> {
    let storage = world.storage.as_ref().ok_or(Error::UndefinedValue("storage".to_string()))?;
    let actual_naive_dates = storage.when(&meal)?;
    assert_eq!(actual_naive_dates, expected_naive_dates.to_vec_naivedate(), "storage.when returned {:?} but we expected {:?}", actual_naive_dates, expected_naive_dates);
    Ok(())
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features/suggest_meals.feature").await;
}
