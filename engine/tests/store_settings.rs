//! The SQLite database file and player settings.

mod common;

use common::temp_dir;
use theatre_engine::Store;
use theatre_engine::settings::{Settings, TextSpeed};

#[test]
fn database_migrates_once_and_keeps_data() {
    let dir = temp_dir("db");
    {
        let s = Store::open(&dir).unwrap();
        assert_eq!(s.schema_version().unwrap(), Store::latest_schema_version());
        s.set("hello", "world").unwrap();
        s.set("hello", "again").unwrap();
    }
    // reopening an up-to-date database changes nothing
    let s = Store::open(&dir).unwrap();
    assert_eq!(s.get("hello").unwrap().as_deref(), Some("again"));
    assert_eq!(s.get("missing").unwrap(), None);
    assert_eq!(s.schema_version().unwrap(), Store::latest_schema_version());
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn settings_round_trip() {
    let store = Store::in_memory().unwrap();
    assert!(store.settings().color, "defaults when nothing is saved");
    store
        .save_settings(&Settings {
            color: false,
            text_speed: TextSpeed::Fast,
            ..Settings::default()
        })
        .unwrap();
    let s = store.settings();
    assert!(!s.color);
    assert_eq!(s.text_speed, TextSpeed::Fast);
}

#[test]
fn speed_and_typewriter_stay_in_sync() {
    let mut s = Settings::default();
    s.cycle_text_speed();
    assert_eq!(s.text_speed, TextSpeed::Fast);
    s.cycle_text_speed();
    assert_eq!(s.text_speed, TextSpeed::Instant);
    assert!(!s.typewriter);
    assert_eq!(s.speed(), 0.0);
    s.toggle_typewriter();
    assert!(s.typewriter);
    assert_eq!(s.text_speed, TextSpeed::Normal);
    s.cycle_text_speed();
    s.cycle_text_speed();
    s.cycle_text_speed();
    assert_eq!(s.text_speed, TextSpeed::Relaxed);
    assert!(s.speed() > 1.0);
}
