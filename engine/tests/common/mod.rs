//! Helpers shared by the engine's integration tests.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use theatre_engine::StoryPack;
use theatre_engine::runner::{Phase, Runner};

/// The repository's `stories/` folder.
pub fn stories() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../stories")
}

/// The `_template` story: small, stable, and covers every format feature.
pub fn template() -> StoryPack {
    StoryPack::load(&stories().join("_template")).expect("template story loads")
}

/// An empty, unique temp folder for one test.
pub fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("theatre-test-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Far enough in the future that any text has finished typing.
pub const LATER: u64 = 1_000_000;

/// Play the template up to its first choice and take it ("Open the door").
pub fn template_after_first_choice() -> Runner {
    let mut r = Runner::new(template(), 0);
    while r.phase() != Phase::Choose {
        r.advance(LATER);
    }
    r.advance(LATER);
    r
}
