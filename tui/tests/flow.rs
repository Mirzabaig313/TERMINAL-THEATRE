//! The whole game driven by key presses, the way a player would.
//! Set `SNAP_DIR=/some/dir` to also save each step as a picture.

mod common;

use common::{Harness, sprite_cells};
use ratatui::crossterm::event::KeyCode;
use ratatui::style::Color;
use terminal_theatre::app::Start;
use theatre_engine::settings::TextSpeed;

/// Opening → menu → cinematic → story → dialogue → save → main menu → load → continue.
#[test]
fn new_game_save_and_continue() {
    let mut h = Harness::new("flow", Start::Opening);

    h.wait(900);
    h.snap("01_opening_reveal");
    h.char('x'); // skips the reveal
    assert!(h.screen().contains("Press any key to start"));
    h.snap("02_opening_done");

    h.char('x');
    assert!(h.screen().contains("MAIN MENU"));
    h.snap("03_main_menu");

    h.press(KeyCode::Enter); // New Game plays the cinematic
    h.wait(1500);
    assert!(h.screen().contains("Press any key to skip"));
    h.snap("04_cinematic");
    h.char('x');
    h.char('x');
    assert!(h.screen().contains("STORIES"));
    h.snap("05_story_select");

    h.char('3'); // Shadow Slave
    h.wait(6000);
    assert!(h.screen().contains("Press ENTER to continue"));
    h.snap("06_intro");

    h.press(KeyCode::Enter);
    assert_eq!(
        h.app.ctx().store.all().len(),
        1,
        "autosaved on entering the first scene"
    );
    for _ in 0..4 {
        h.press(KeyCode::Enter); // finish typing...
        h.wait(4000); // ...and read
        h.press(KeyCode::Enter);
    }
    h.wait(1000);
    assert!(
        sprite_cells(h.draw()) > 200,
        "a character portrait is on screen"
    );
    h.snap("07_dialogue");

    // pause menu → Save Game → slot 2 → keep the default name
    h.press(KeyCode::Esc);
    assert!(h.screen().contains("PAUSED"));
    h.snap("08_pause");
    h.press(KeyCode::Down);
    h.press(KeyCode::Enter);
    assert!(h.screen().contains("SAVE GAME"));
    h.snap("09_save_slots");
    h.char('2');
    assert!(h.screen().contains("SAVE TO SLOT 2"));
    h.snap("10_save_name");
    h.press(KeyCode::Backspace);
    h.char('X');
    h.press(KeyCode::Enter);
    assert!(h.screen().contains("Game saved to slot 2"));
    h.snap("11_saved");
    let saved = h.app.ctx().store.load("shadow_slave", 2).unwrap();
    assert_eq!(saved.metadata.name, "Save X");

    // back to the main menu (asks first): Resume, Save Game, History, Main Menu
    h.press(KeyCode::Esc);
    for _ in 0..3 {
        h.press(KeyCode::Down);
    }
    h.press(KeyCode::Enter);
    assert!(h.screen().contains("RETURN TO MAIN MENU"));
    h.char('y');
    assert!(h.screen().contains("MAIN MENU"));

    // Load Game lists both saves
    h.char('3');
    let load = h.screen();
    assert!(load.contains("LOAD GAME") && load.contains("Save X") && load.contains("Autosave"));
    h.snap("12_load");
    h.press(KeyCode::Esc);

    // Continue resumes the newest save
    h.char('2');
    h.wait(3000);
    assert!(h.screen().contains("Resuming your story"));
    h.snap("13_resume");
}

#[test]
fn settings_persist_and_monochrome_applies() {
    let mut h = Harness::new("settings", Start::MainMenu);
    h.char('5');
    assert!(h.screen().contains("SETTINGS"));
    h.press(KeyCode::Enter); // Color Mode → Monochrome
    h.snap("14_monochrome");
    assert!(!h.app.ctx().settings.color);
    assert!(!h.app.ctx().store.settings().color, "saved to the database");

    let buf = h.draw().clone();
    for cell in buf.content.iter() {
        for c in [cell.fg, cell.bg] {
            if let Color::Rgb(r, g, b) = c {
                assert!(r == g && g == b, "color left in monochrome mode: {c:?}");
            }
        }
    }

    // Text Speed cycles and saves too
    h.press(KeyCode::Down);
    h.press(KeyCode::Down);
    h.press(KeyCode::Enter);
    assert_eq!(h.app.ctx().store.settings().text_speed, TextSpeed::Fast);
}

#[test]
fn continue_without_saves_explains() {
    let mut h = Harness::new("nosaves", Start::MainMenu);
    h.char('2');
    assert!(h.screen().contains("No saved story yet"));
}

#[test]
fn konami_code_unlocks_the_secret() {
    let mut h = Harness::new("konami", Start::MainMenu);
    for k in [
        KeyCode::Up,
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Left,
        KeyCode::Right,
    ] {
        h.press(k);
    }
    h.char('b');
    h.char('a');
    assert!(h.screen().contains("Phantom Stage Entrance"));
}

#[test]
fn credits_and_back() {
    let mut h = Harness::new("credits", Start::MainMenu);
    h.char('4');
    assert!(h.screen().contains("Rust + Ratatui"));
    h.char('x');
    assert!(h.screen().contains("MAIN MENU"));
}

#[test]
fn every_story_starts_from_the_command_line() {
    for id in [
        "shadow_slave",
        "noir_detective",
        "blood_and_neon",
        "_template",
    ] {
        let mut h = Harness::new(&format!("start-{id}"), Start::Story(id.into()));
        h.wait(20_000); // long descriptions take a while to type out
        let screen = h.screen();
        assert!(
            screen.contains("Press ENTER to continue"),
            "{id} intro never finished:\n{screen}"
        );
    }
}

/// Every screen at small sizes: nothing may panic, however cramped.
#[test]
fn small_terminals_never_crash() {
    for (w, h) in [(80, 24), (60, 20), (30, 10), (12, 5)] {
        let mut h_ = Harness::with_size(&format!("small-{w}x{h}"), Start::Opening, w, h);
        let h = &mut h_;
        h.wait(700);
        h.draw();
        h.char('x');
        h.char('x'); // main menu
        h.draw();
        for screen in ['4', '5', '3'] {
            h.char(screen); // credits, settings, load
            h.draw();
            h.press(KeyCode::Esc);
        }
        h.press(KeyCode::Enter); // cinematic
        h.wait(3000);
        h.draw();
        h.char('x');
        h.char('x'); // story select
        h.draw();
        h.char('e'); // endings gallery
        h.draw();
        h.char('e');
        h.char('1'); // a story's intro
        h.wait(2000);
        h.draw();
        h.press(KeyCode::Enter);
        for _ in 0..6 {
            h.press(KeyCode::Enter);
            h.wait(300);
            h.draw();
        }
        h.char('h'); // history
        h.draw();
        h.press(KeyCode::Esc);
        h.char('a'); // auto
        h.wait(2000);
        h.draw();
        h.press(KeyCode::Esc); // pause, save slots, name
        h.draw();
        h.press(KeyCode::Down);
        h.press(KeyCode::Enter);
        h.draw();
        h.char('1');
        h.draw();
        h.press(KeyCode::Enter);
        h.draw();
    }
}

/// Terminals without 24-bit color get the 256-color palette instead.
#[test]
fn colors_fall_back_to_256() {
    let mut h = Harness::new("256", Start::MainMenu);
    h.app.ctx_mut().truecolor = false;
    let buf = h.draw().clone();
    let mut indexed = 0;
    for cell in buf.content.iter() {
        for c in [cell.fg, cell.bg] {
            assert!(!matches!(c, Color::Rgb(..)), "24-bit color left: {c:?}");
            indexed += matches!(c, Color::Indexed(_)) as usize;
        }
    }
    assert!(indexed > 1000);
}
