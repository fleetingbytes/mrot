//! Library with helper functions and constructs used in the mrot test code

mod error;

pub use error::Error;
use cucumber::{parser, runner, writer, Cucumber, World as _, WriterExt};
use futures::FutureExt as _;
use tracing::info;
use tracing_subscriber::{
    filter,
    fmt::format,
    layer::{Layer, SubscriberExt as _},
};
use std::{io, path::Path};
use libmrot::Storage;

pub type Result<T> = std::result::Result<T, Error>;

/// World for cucumber tests
#[derive(Debug, Default, cucumber::World)]
#[world(init = Self::default)]
pub struct World {
    pub storage: Option<Storage>,
    pub dates: Option<Vec<String>>,
    pub meal: Option<String>,
}

/// Clean-up procedure after each scenario
fn cleanup(world: Option<&mut World>) {
    if let Some(_w) = world {
        info!("Cleaning up the World");
    }
}

/// Normal World
pub fn normal_world<I: AsRef<Path>>() -> Cucumber<
    World,
    parser::Basic,
    I,
    runner::Basic<World>,
    writer::Summarize<writer::Normalize<World, writer::Basic>>,
> {
    World::cucumber()
        .after(|_feature, _rule, _scenario, _event, world| async { cleanup(world) }.boxed_local())
}

/// World where Scenarios are run in series
pub fn serial_world<I: AsRef<Path>>() -> Cucumber<
    World,
    parser::Basic,
    I,
    runner::Basic<World>,
    writer::Summarize<writer::Normalize<World, writer::Basic>>,
> {
    World::cucumber()
        .max_concurrent_scenarios(1)
        .after(|_feature, _rule, _scenario, _event, world| async { cleanup(world) }.boxed_local())
}

/// World where Scenarios are run in series and traces for debugging tests are output
pub fn debug_world<I: AsRef<Path>>() -> Cucumber<
    World,
    parser::Basic,
    I,
    runner::Basic<World>,
    writer::AssertNormalized<writer::Summarize<writer::Basic>>,
> {
    World::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(
            writer::Basic::raw(io::stdout(), writer::Coloring::Never, 0)
                .summarized()
                .assert_normalized(),
        )
        .configure_and_init_tracing(
            format::DefaultFields::new(),
            format::Format::default(),
            |layer| tracing_subscriber::registry().with(filter::LevelFilter::TRACE.and_then(layer)),
        )
        .after(|_feature, _rule, _scenario, _event, world| async { cleanup(world) }.boxed_local())
}

/// World where Scenarios are run in series and traces for debugging tests are output. Cleanup
/// procedure is not run after the scenario.
pub fn debug_world_no_cleanup<I: AsRef<Path>>() -> Cucumber<
    World,
    parser::Basic,
    I,
    runner::Basic<World>,
    writer::AssertNormalized<writer::Summarize<writer::Basic>>,
> {
    World::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(
            writer::Basic::raw(io::stdout(), writer::Coloring::Never, 0)
                .summarized()
                .assert_normalized(),
        )
        .configure_and_init_tracing(
            format::DefaultFields::new(),
            format::Format::default(),
            |layer| tracing_subscriber::registry().with(filter::LevelFilter::INFO.and_then(layer)),
        )
}
