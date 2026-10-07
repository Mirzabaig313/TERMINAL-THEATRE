//! Read-text tracking, endings found, and the quick-save slot.

mod common;

use common::{template, template_after_first_choice};
use theatre_engine::Store;
use theatre_engine::progress::NARRATION;
use theatre_engine::save::{MAX_SLOT, QUICKSAVE_SLOT};
use theatre_engine::settings::{Effects, Settings};

#[test]
fn seen_lines_are_remembered_per_story() {
    let store = Store::in_memory().unwrap();
    store.mark_seen("_template", "opening", NARRATION).unwrap();
    store.mark_seen("_template", "opening", 0).unwrap();
    store.mark_seen("_template", "opening", 0).unwrap(); // twice is fine
    store.mark_seen("other", "opening", 1).unwrap();

    let seen = store.seen("_template");
    assert_eq!(seen.len(), 2);
    assert!(seen.contains(&("opening".to_string(), NARRATION)));
    assert!(!seen.contains(&("opening".to_string(), 1)));
    assert!(store.seen("nobody").is_empty());
}

#[test]
fn endings_count_first_and_repeat_visits() {
    let store = Store::in_memory().unwrap();
    assert!(store.record_ending("_template", "ending_letter").unwrap());
    assert!(!store.record_ending("_template", "ending_letter").unwrap());
    let found = store.endings("_template");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].times, 2);
    assert_eq!(found[0].date_label().len(), 10);
    assert!(store.endings("other").is_empty());
}

#[test]
fn story_lists_its_endings() {
    assert_eq!(template().ending_ids(), vec!["ending_letter"]);
}

#[test]
fn quicksave_has_its_own_slot() {
    let store = Store::in_memory().unwrap();
    let r = template_after_first_choice();
    let m = store.save(&r.pack, QUICKSAVE_SLOT, &r.state, None).unwrap();
    assert_eq!(m.name, "Quicksave");
    // not one of the player's numbered slots
    assert!(store.slots("_template").iter().all(Option::is_none));
    const { assert!(QUICKSAVE_SLOT > MAX_SLOT) };
    assert!(
        store
            .save(&r.pack, QUICKSAVE_SLOT + 1, &r.state, None)
            .is_err()
    );
}

#[test]
fn effects_cycle_and_scale() {
    let mut e = Settings::default().effects;
    assert_eq!(e, Effects::Full);
    e = e.next();
    assert_eq!((e, e.strength()), (Effects::Reduced, 0.35));
    e = e.next();
    assert_eq!(e.strength(), 0.0);
    assert_eq!(e.next(), Effects::Full);
}
