//! Story flow: typewriter timing, choices, resuming.

mod common;

use common::{LATER, template, template_after_first_choice};
use theatre_engine::runner::{Phase, Runner, revealed};

#[test]
fn typewriter_pauses_after_sentences() {
    assert_eq!(revealed("ab", 1000, 10), usize::MAX);
    // "a." then a pause: 'b' needs 10 + 10 + 80 ms
    assert_eq!(revealed("a.b", 25, 10), 2);
    assert_eq!(revealed("a.b", 111, 10), usize::MAX);
}

#[test]
fn narration_then_lines_then_choices() {
    let mut r = Runner::new(template(), 0);
    assert_eq!(r.phase(), Phase::Narration);
    // the first press only finishes the typing
    assert_eq!(r.advance(0), None);
    assert_eq!(r.advance(0), Some(Phase::Line(0)));
    r.advance(LATER);
    assert_eq!(r.phase(), Phase::Line(1));
    r.advance(2 * LATER);
    assert_eq!(r.phase(), Phase::Choose);
    assert_eq!(r.choices().len(), 2);
}

#[test]
fn records_choices_and_resumes() {
    let r = template_after_first_choice();
    assert_eq!(r.state.choices.len(), 1);
    assert_eq!(r.state.choices[0].text, "Open the door");
    assert_eq!(r.state.current_scene, "door_open");

    let resumed = Runner::resume(template(), r.state.clone(), 0);
    assert_eq!(resumed.scene_id(), "door_open");
    assert_eq!(resumed.state.items, vec!["Wet Letter".to_string()]);
}

#[test]
fn resume_with_unknown_scene_starts_over() {
    let mut state = template_after_first_choice().state;
    state.current_scene = "no_such_scene".into();
    let r = Runner::resume(template(), state, 0);
    assert_eq!(r.scene_id(), "opening");
}

#[test]
fn instant_speed_shows_everything() {
    let mut r = Runner::new(template(), 0);
    assert!(!r.typing_done(0));
    r.speed = 0.0;
    assert!(r.typing_done(0));
}

#[test]
fn ending_stops_the_story() {
    let mut r = template_after_first_choice();
    // each press happens well after the previous text finished typing
    let mut now = LATER;
    let mut press = |r: &mut theatre_engine::runner::Runner| {
        now += LATER;
        r.advance(now)
    };
    // door_open: narration, two lines, then a choice gated on the letter
    while r.phase() != Phase::Choose {
        press(&mut r);
    }
    press(&mut r);
    assert_eq!(r.scene_id(), "ending_letter");
    press(&mut r);
    assert_eq!(r.phase(), Phase::End);
    assert_eq!(press(&mut r), None);
}
