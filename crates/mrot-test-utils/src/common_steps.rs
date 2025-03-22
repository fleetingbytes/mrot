//! A collection of test steps used in the tests of [libmrot] which are shared among test targets

use crate::{World, Result, argument::DateString};
use cucumber::{given, gherkin::Step};
use tracing::debug;
use libmrot::Storage;

/// Provides a storages filled with the records specified in the feature file
#[given(regex = r"^an in-memory storage with the records$")]
pub async fn a_storage_with_records(world: &mut World, step: &Step) -> Result<()> {
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
