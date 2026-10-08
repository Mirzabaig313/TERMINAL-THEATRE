//! Build the bundled stories into the binary, so a downloaded `theatre` works
//! on its own. Every file under `../stories` becomes an entry in
//! `$OUT_DIR/bundled.rs`; `terminal_theatre::bundled` unpacks them on first run.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        let hidden = path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with('.'));
        if hidden {
            continue;
        }
        if path.is_dir() {
            println!("cargo:rerun-if-changed={}", path.display());
            files(&path, out);
        } else {
            out.push(path);
        }
    }
}

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../stories");
    let root = root.canonicalize().unwrap_or(root);
    println!("cargo:rerun-if-changed={}", root.display());
    let mut list = Vec::new();
    files(&root, &mut list);

    let mut hasher = DefaultHasher::new();
    let mut entries = String::new();
    for path in &list {
        let rel = path
            .strip_prefix(&root)
            .unwrap()
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        println!("cargo:rerun-if-changed={}", path.display());
        rel.hash(&mut hasher);
        std::fs::read(path).unwrap_or_default().hash(&mut hasher);
        entries += &format!(
            "    ({rel:?}, include_bytes!({:?})),\n",
            path.display().to_string()
        );
    }
    let code = format!(
        "/// Every bundled story file: (path inside the stories folder, contents).\n\
         pub static FILES: &[(&str, &[u8])] = &[\n{entries}];\n\
         /// Changes whenever any bundled file does.\n\
         pub const ID: &str = \"{:016x}\";\n",
        hasher.finish()
    );
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bundled.rs");
    std::fs::write(out, code).unwrap();
}
