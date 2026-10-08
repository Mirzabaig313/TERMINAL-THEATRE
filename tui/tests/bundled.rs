//! The stories built into the binary: an installed `theatre` unpacks them
//! once into its data folder and plays them from there.

mod common;

use common::{stories, temp_dir};
use terminal_theatre::bundled::{FILES, ID, install};
use theatre_engine::StoryPack;

#[test]
fn every_story_file_is_bundled() {
    let mut on_disk = Vec::new();
    let mut todo = vec![stories()];
    while let Some(d) = todo.pop() {
        for e in std::fs::read_dir(d).unwrap().flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            if e.path().is_dir() {
                todo.push(e.path());
            } else {
                on_disk.push(e.path());
            }
        }
    }
    assert_eq!(FILES.len(), on_disk.len());
    for (rel, bytes) in FILES {
        let disk = std::fs::read(stories().join(rel)).unwrap();
        assert_eq!(&disk[..], *bytes, "{rel}");
    }
}

#[test]
fn unpacks_once_and_the_stories_load() {
    let dir = temp_dir("bundled");
    // a player's own story lives here too and must survive updates
    std::fs::create_dir_all(dir.join("my_story")).unwrap();
    std::fs::write(dir.join("my_story/story.toml"), "mine").unwrap();

    assert!(install(&dir).unwrap(), "first run unpacks");
    assert!(!install(&dir).unwrap(), "already there");
    for id in [
        "noir_detective",
        "blood_and_neon",
        "shadow_slave",
        "_template",
    ] {
        StoryPack::load(&dir.join(id)).unwrap_or_else(|e| panic!("{id}: {e:#}"));
    }

    // an update (a different bundle) unpacks again
    std::fs::write(dir.join(".bundled"), "older").unwrap();
    std::fs::remove_file(dir.join("noir_detective/story.toml")).unwrap();
    assert!(install(&dir).unwrap());
    assert!(dir.join("noir_detective/story.toml").is_file());
    assert_eq!(std::fs::read_to_string(dir.join(".bundled")).unwrap(), ID);
    assert_eq!(
        std::fs::read_to_string(dir.join("my_story/story.toml")).unwrap(),
        "mine"
    );
    std::fs::remove_dir_all(dir).ok();
}
