//! `theatre rehearse`: start anywhere with any state, see the state, reload
//! the story when its files change, and never save anything.

mod common;

use std::path::Path;

use common::{Harness, stories, temp_dir};
use ratatui::crossterm::event::KeyCode;
use terminal_theatre::app::Start;
use theatre_engine::state::State;

fn rehearse(name: &str, dir: &Path, scene: &str, sets: &[&str]) -> Harness {
    let mut state = State::default();
    for s in sets {
        state.set_from(s).unwrap();
    }
    state.current_scene = scene.into();
    let start = Start::Rehearse("_template".into(), Box::new(state));
    let mut h = Harness::with_stories(name, dir, start, 120, 48);
    h.wait(20_000);
    h.press(KeyCode::Enter); // past the title card
    h
}

/// Finish the text on screen and move on.
fn next(h: &mut Harness) {
    h.press(KeyCode::Enter);
    h.wait(200);
    h.press(KeyCode::Enter);
}

#[test]
fn start_anywhere_and_see_the_state() {
    let mut h = rehearse(
        "reh-state",
        &stories(),
        "door_open",
        &["courage=4", "flag:met_chen"],
    );
    let screen = h.screen();
    assert!(screen.contains("REHEARSAL"), "{screen}");
    assert!(
        screen.contains("stumbles inside"),
        "starts in the chosen scene"
    );
    h.char('d');
    let screen = h.screen();
    assert!(screen.contains("STATE"), "{screen}");
    assert!(screen.contains("courage=4"));
    assert!(screen.contains("met_chen"));
    assert!(
        screen.contains("Wet Letter"),
        "the scene's own effects apply on entering"
    );
    h.char('d');
    assert!(!h.screen().contains("STATE ·"));
}

#[test]
fn nothing_is_saved() {
    let mut h = rehearse("reh-nosave", &stories(), "door_open", &[]);
    h.press(KeyCode::F(5));
    assert!(h.screen().contains("nothing is saved"));
    next(&mut h);
    next(&mut h);
    h.char('1'); // read the letter: an ending
    h.wait(300);
    next(&mut h);
    next(&mut h);
    h.wait(1500);
    let screen = h.screen();
    assert!(
        screen.contains("Ending Letter"),
        "reached the ending:\n{screen}"
    );
    let store = &h.app.ctx().store;
    assert!(store.all().is_empty(), "no autosave, no quicksave");
    assert!(
        store.endings("_template").is_empty(),
        "endings aren't recorded"
    );
    assert!(
        store.seen("_template").is_empty(),
        "read text isn't recorded"
    );
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            copy_dir(&e.path(), &to.join(e.file_name()));
        } else {
            std::fs::copy(e.path(), to.join(e.file_name())).unwrap();
        }
    }
}

#[test]
fn story_files_reload_when_saved() {
    let lib = temp_dir("reh-reload-stories");
    copy_dir(&stories().join("_template"), &lib.join("_template"));
    let scenes = lib.join("_template/scenes/01_beginning.toml");
    let mut h = rehearse("reh-reload", &lib, "silence", &[]);
    assert!(h.screen().contains("The knocking stops"));

    let text = std::fs::read_to_string(&scenes).unwrap();
    std::fs::write(
        &scenes,
        text.replace("The knocking stops.", "The knocking STOPS, rewritten."),
    )
    .unwrap();
    h.wait(1000);
    h.press(KeyCode::Enter); // finish typing the scene again
    let screen = h.screen();
    assert!(screen.contains("Story reloaded"), "{screen}");
    assert!(screen.contains("knocking STOPS, rewritten"), "{screen}");

    // a broken edit keeps the last good story running and says why
    std::fs::write(&scenes, "this is [not toml").unwrap();
    h.wait(1000);
    let screen = h.screen();
    assert!(screen.contains("Not reloaded"), "{screen}");
    assert!(screen.contains("knocking STOPS, rewritten"));
    std::fs::remove_dir_all(lib).ok();
}
