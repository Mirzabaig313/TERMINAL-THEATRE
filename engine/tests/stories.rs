//! The bundled stories: every one must load, validate and be finishable.

mod common;

use common::{LATER, stories};
use theatre_engine::StoryPack;
use theatre_engine::library::discover;
use theatre_engine::runner::{Phase, Runner};

#[test]
fn all_bundled_stories_load() {
    let (entries, errors) = discover(&stories()).unwrap();
    assert!(errors.is_empty(), "{errors:?}");
    assert!(entries.len() >= 3);
    for e in entries {
        let pack = StoryPack::load(&e.dir).unwrap_or_else(|err| panic!("{err:#}"));
        assert!(!pack.scenes.is_empty());
    }
}

#[test]
fn template_is_hidden_but_valid() {
    let (entries, _) = discover(&stories()).unwrap();
    assert!(entries.iter().all(|e| e.id != "_template"));
    StoryPack::load(&stories().join("_template")).unwrap_or_else(|err| panic!("{err:#}"));
}

/// Always picking the first choice must reach an ending in every bundled story.
#[test]
fn first_choice_path_reaches_an_end() {
    for entry in discover(&stories()).unwrap().0 {
        let mut r = Runner::new(StoryPack::load(&entry.dir).unwrap(), 0);
        let mut steps = 0;
        while r.phase() != Phase::End {
            r.advance(LATER);
            steps += 1;
            assert!(steps < 10_000, "{}: looped at '{}'", entry.id, r.scene_id());
        }
    }
}

/// No story may leave a scene that nothing can reach.
#[test]
fn every_scene_is_reachable() {
    for entry in discover(&stories()).unwrap().0 {
        let pack = StoryPack::load(&entry.dir).unwrap();
        assert!(
            pack.unreachable().is_empty(),
            "{}: unreachable {:?}",
            entry.id,
            pack.unreachable()
        );
    }
}
