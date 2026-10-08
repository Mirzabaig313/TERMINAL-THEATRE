//! Keeping the terminal light: an idle screen must barely change between
//! frames, and the app must only ask for the fast frame rate while something
//! moves. Editor terminals (VS Code, Kiro…) lag badly when flooded with output.

mod common;

use common::Harness;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyCode;
use terminal_theatre::app::Start;
use terminal_theatre::render::fx::ANIM_STEP_MS;

/// Share of cells that differ between two frames.
fn changed(a: &Buffer, b: &Buffer) -> f32 {
    let n = a
        .content
        .iter()
        .zip(&b.content)
        .filter(|(x, y)| x != y)
        .count();
    n as f32 / a.content.len() as f32
}

/// Average share of cells changing per idle frame over two seconds.
fn idle_churn(h: &mut Harness) -> f32 {
    let mut prev = h.draw().clone();
    let mut total = 0.0;
    let frames = 16;
    for _ in 0..frames {
        h.wait(ANIM_STEP_MS);
        let next = h.draw().clone();
        total += changed(&prev, &next);
        prev = next;
    }
    total / frames as f32
}

#[test]
fn idle_main_menu_is_nearly_static() {
    let mut h = Harness::new("perf-menu", Start::MainMenu);
    h.wait(2000);
    assert!(!h.app.busy(), "an idle menu should use the slow frame rate");
    let churn = idle_churn(&mut h);
    assert!(
        churn < 0.02,
        "{:.1}% of cells change per idle frame",
        churn * 100.0
    );
}

#[test]
fn idle_story_scene_is_nearly_static() {
    let mut h = Harness::new("perf-play", Start::Story("shadow_slave".into()));
    h.wait(20_000);
    h.press(KeyCode::Enter); // past the title card
    h.press(KeyCode::Enter); // finish typing the narration
    h.wait(3000);
    assert!(
        !h.app.busy(),
        "a finished scene waiting for the player should be idle"
    );
    let churn = idle_churn(&mut h);
    assert!(
        churn < 0.03,
        "{:.1}% of cells change per idle frame",
        churn * 100.0
    );
}

#[test]
fn idle_scene_with_a_picture_is_static() {
    let mut h = Harness::new("perf-picture", Start::Story("_template".into()));
    h.wait(20_000);
    h.press(KeyCode::Enter); // the opening has a picture
    h.press(KeyCode::Enter); // finish typing the narration
    h.wait(3000);
    assert!(!h.app.busy());
    let churn = idle_churn(&mut h);
    assert!(
        churn < 0.03,
        "{:.1}% of cells change per idle frame",
        churn * 100.0
    );
}

#[test]
fn fast_frames_only_while_something_moves() {
    let mut h = Harness::new("perf-busy", Start::Story("_template".into()));
    h.wait(200);
    assert!(h.app.busy(), "the title card types out");
    h.wait(20_000);
    assert!(!h.app.busy(), "the title card has finished");
    h.press(KeyCode::Enter);
    assert!(h.app.busy(), "a key press and a new scene are busy");
    h.wait(10);
    assert!(h.app.busy(), "narration is typing");
}
