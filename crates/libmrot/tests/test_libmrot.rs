//! Tests for libmrot

use cucumber::{given, World as _};
use libmrot::Storage;

/// World for cucumber tests
#[derive(Debug, Default, cucumber::World)]
pub struct World {
    storage: Storage,
}

#[given("a storage")]
async fn storage(_world: &mut World) {
    assert!(true);
}

#[tokio::main]
async fn main() {
    World::run("tests/features").await;
}
