//! Finding story folders on disk.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::pack::Meta;

/// A story folder found on disk; only `story.toml` has been read.
pub struct Entry {
    pub id: String,
    pub dir: PathBuf,
    pub meta: Meta,
}

/// Every `<root>/<id>/story.toml`, sorted by id. Broken folders are returned as errors
/// alongside the good ones so one bad story never hides the others.
pub fn discover(root: &Path) -> Result<(Vec<Entry>, Vec<anyhow::Error>)> {
    let mut entries = Vec::new();
    let mut errors = Vec::new();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(root)
        .with_context(|| format!("reading stories folder {}", root.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("story.toml").is_file())
        // folders starting with '_' (like _template) are not listed
        .filter(|p| {
            !p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('_'))
        })
        .collect();
    dirs.sort();
    for dir in dirs {
        let id = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        match Meta::load(&dir) {
            Ok(meta) => entries.push(Entry { id, dir, meta }),
            Err(e) => errors.push(e.context(format!("story '{id}'"))),
        }
    }
    Ok((entries, errors))
}
