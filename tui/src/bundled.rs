//! The stories that ship inside the binary.
//!
//! An installed `theatre` has no `stories/` folder beside it, so on first run
//! (and after an update changes them) the bundled stories are unpacked into
//! `~/.terminal_theatre/stories/`. Players can add their own story folders
//! there too; unpacking only ever writes the bundled files.

use std::path::Path;

use anyhow::{Context, Result};

mod files {
    include!(concat!(env!("OUT_DIR"), "/bundled.rs"));
}

pub use files::{FILES, ID};

/// Marks which bundle a stories folder was last unpacked from.
const MARKER: &str = ".bundled";

/// Unpack the bundled stories into `dir` unless this bundle is already there.
/// Returns true when files were written.
pub fn install(dir: &Path) -> Result<bool> {
    let marker = dir.join(MARKER);
    if std::fs::read_to_string(&marker).is_ok_and(|m| m.trim() == ID) {
        return Ok(false);
    }
    for (rel, bytes) in FILES {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        std::fs::write(&path, bytes).with_context(|| format!("writing {}", path.display()))?;
    }
    std::fs::write(&marker, ID)?;
    Ok(true)
}
