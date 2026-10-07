//! Reading comfort and replay: history, auto, skip, quick save/load,
//! screen-effect levels, endings found and the endings gallery.

mod common;

use common::{Harness, stories};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Color;
use terminal_theatre::app::Start;
use terminal_theatre::screens::menu::Menu;
use theatre_engine::Store;
use theatre_engine::settings::Effects;

/// Start a story and get past its title card.
fn playing(name: &str, story: &str) -> Harness {
    let mut h = Harness::new(name, Start::Story(story.into()));
    h.wait(20_000);
    h.press(KeyCode::Enter);
    h
}

/// Finish the text on screen and move to the next.
fn next(h: &mut Harness) {
    h.press(KeyCode::Enter);
    h.wait(200);
    h.press(KeyCode::Enter);
}

#[test]
fn history_shows_what_was_read() {
    let mut h = playing("history", "shadow_slave");
    for _ in 0..3 {
        next(&mut h);
    }
    h.char('h');
    let screen = h.screen();
    assert!(screen.contains("HISTORY"));
    assert!(
        screen.contains("Your eyes snap open"),
        "narration missing:\n{screen}"
    );
    assert!(screen.contains("THE SPELL"), "speaker missing");
    h.press(KeyCode::Esc);
    assert!(!h.screen().contains("HISTORY"));
}

#[test]
fn auto_mode_moves_on_by_itself_and_waits_at_choices() {
    let mut h = playing("auto", "_template");
    h.char('a');
    h.wait(800); // past the scene's entrance dissolve
    let first_row = h.screen().lines().next().unwrap_or_default().to_string();
    assert!(first_row.contains("AUTO"), "header: {first_row:?}");
    h.wait(60_000);
    // narration and both lines went by; it waits at the choice
    assert!(h.screen().contains("WHAT DO YOU DO?"));
    assert!(h.screen().contains("AUTO"), "auto stays on at a choice");
}

#[test]
fn skip_stops_at_text_never_read() {
    let mut h = playing("skip", "_template");
    for _ in 0..3 {
        next(&mut h);
    }
    h.char('1'); // open the door: a scene not read before
    h.wait(300);
    h.char('s');
    h.wait(500);
    assert!(h.screen().contains("Skip stopped: new text"));
}

#[test]
fn skip_races_through_text_already_read() {
    let mut h = playing("skip-read", "_template");
    h.press(KeyCode::F(5)); // quicksave at the very start
    for _ in 0..3 {
        next(&mut h);
    }
    assert!(h.screen().contains("WHAT DO YOU DO?"));
    h.press(KeyCode::F(9)); // back to the start: everything here is read now
    h.char('s');
    h.wait(1000);
    let screen = h.screen();
    assert!(
        screen.contains("WHAT DO YOU DO?"),
        "skip should reach the choice:\n{screen}"
    );
}

#[test]
fn quick_load_without_quicksave_explains() {
    let mut h = playing("noquick", "_template");
    h.press(KeyCode::F(9));
    assert!(h.screen().contains("No quicksave yet"));
}

#[test]
fn reaching_an_ending_is_remembered() {
    let mut h = playing("ending", "_template");
    for _ in 0..3 {
        next(&mut h);
    }
    h.char('1'); // open the door
    for _ in 0..3 {
        next(&mut h);
    }
    h.char('1'); // read the letter
    h.wait(300);
    next(&mut h);
    h.wait(1500);
    let screen = h.screen();
    assert!(screen.contains("New ending unlocked"), "{screen}");
    assert!(screen.contains("Endings found: 1 / 1"));
    assert_eq!(h.app.ctx().store.endings("_template").len(), 1);
}

#[test]
fn gallery_lists_found_and_locked_endings() {
    let store = Store::in_memory().unwrap();
    store
        .record_ending("noir_detective", "ending_heroic_victory")
        .unwrap();
    let mut menu = Menu::new(&stories(), &store, 0, None);
    // stories are listed alphabetically: blood_and_neon, noir_detective, shadow_slave
    menu.key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), 0);
    menu.key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE), 0);
    let mut term = Terminal::new(TestBackend::new(120, 48)).unwrap();
    term.draw(|f| menu.draw(f, 3000)).unwrap();
    let screen: String = term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(screen.contains("ENDINGS"));
    assert!(screen.contains("Ending Heroic Victory"));
    assert!(screen.contains("1 / 24"));
    assert!(screen.contains("? ? ?"));
}

/// Average background brightness (0..765): a flash lifts the whole screen.
fn brightness(h: &mut Harness) -> u32 {
    let buf = h.draw().clone();
    let values: Vec<u32> = buf
        .content
        .iter()
        .filter_map(|c| match c.bg {
            Color::Rgb(r, g, b) => Some(r as u32 + g as u32 + b as u32),
            _ => None,
        })
        .collect();
    values.iter().sum::<u32>() / values.len().max(1) as u32
}

#[test]
fn screen_effects_can_be_reduced_or_turned_off() {
    // Shadow Slave's first line flashes the screen white
    let mut peaks = Vec::new();
    for effects in [Effects::Full, Effects::Reduced, Effects::Off] {
        let mut h = playing(&format!("fx-{}", effects.label()), "shadow_slave");
        h.app.ctx_mut().settings.effects = effects;
        h.press(KeyCode::Enter); // finish the narration
        h.wait(600); // past the scene's entrance fade
        h.app.key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)); // first line: flash
        h.t += 33;
        h.app.tick(h.t);
        let during = brightness(&mut h);
        h.wait(1000); // the flash is long over
        peaks.push((during, brightness(&mut h)));
    }
    let [full, reduced, off] = [peaks[0], peaks[1], peaks[2]];
    assert!(
        full.0 > full.1 + 300,
        "full: the screen flashes near white: {peaks:?}"
    );
    assert!(
        reduced.0 > reduced.1 && reduced.0 < full.0,
        "reduced: a dimmer flash: {peaks:?}"
    );
    assert!(off.0 <= off.1 + 10, "off: no flash at all: {peaks:?}");
}

#[test]
fn new_settings_are_saved() {
    let mut h = Harness::new("newsettings", Start::MainMenu);
    h.char('5');
    for _ in 0..4 {
        h.press(KeyCode::Down);
    }
    h.press(KeyCode::Enter); // Screen Effects: Full → Reduced
    h.press(KeyCode::Down);
    h.press(KeyCode::Enter); // Skip Mode: all text
    let s = h.app.ctx().store.settings();
    assert_eq!(s.effects, Effects::Reduced);
    assert!(s.skip_unread);
    assert!(h.screen().contains("Reduced"));
}
