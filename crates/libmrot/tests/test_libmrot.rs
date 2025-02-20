//! Tests for libmrot

use cucumber::given;
use test_utils::{debug_world_no_cleanup as construct_world, World};
use libmrot::storage::SqliteStorage;

#[given("a storage")]
async fn storage(world: &mut World) {
    let storage = SqliteStorage::from(":memory");
    world.storage = Some(storage);
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features").await;
}
