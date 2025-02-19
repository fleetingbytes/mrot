//! Tests for libmrot

use cucumber::given;
use test_utils::{debug_world_no_cleanup as construct_world, World};

#[given("a storage")]
async fn storage(_world: &mut World) {
    assert!(true);
}

#[tokio::main]
async fn main() {
    let world = construct_world();
    world.run("tests/features").await;
}
