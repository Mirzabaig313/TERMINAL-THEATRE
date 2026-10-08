//! Scene data and playthrough state.

use theatre_engine::scene::{Choice, Mood, Scene};
use theatre_engine::state::State;

#[test]
fn art_frames_cycle_or_hold() {
    let mut s: Scene =
        toml::from_str("text = \"t\"\nart_frames = [\"a\", \"b\"]\nart_ms = 100").unwrap();
    assert_eq!(s.art_at(0), Some("a"));
    assert_eq!(s.art_at(150), Some("b"));
    assert_eq!(s.art_at(250), Some("a"));
    s.art_loop = false;
    assert_eq!(s.art_at(250), Some("b"));
}

#[test]
fn unknown_scene_fields_are_rejected() {
    assert!(toml::from_str::<Scene>("text = \"t\"\ntypo_field = 1").is_err());
}

#[test]
fn mood_is_guessed_from_scene_id() {
    assert_eq!(Mood::guess("final_battle", Mood::Noir), Mood::Danger);
    assert_eq!(Mood::guess("safe_house", Mood::Noir), Mood::Calm);
    assert_eq!(Mood::guess("pier_nineteen", Mood::Noir), Mood::Noir);
}

#[test]
fn conditional_choice() {
    let c = Choice {
        text: "x".into(),
        goto: "y".into(),
        any_flags: vec![],
        any_items: vec!["Rusty Blade".into()],
        ..Choice::default()
    };
    let mut s = State::default();
    assert!(!s.allows(&c));
    s.items.push("Rusty Blade".into());
    assert!(s.allows(&c));
}
