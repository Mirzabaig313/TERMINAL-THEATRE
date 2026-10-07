//! Save games in SQLite: slots, listing, export/import, the Python save import.

mod common;

use common::{stories, temp_dir, template_after_first_choice};
use theatre_engine::Store;
use theatre_engine::library::Entry;
use theatre_engine::pack::Meta;
use theatre_engine::save::{AUTOSAVE_SLOT, GAME_VERSION, format_playtime};

#[test]
fn save_load_list_delete() {
    let store = Store::in_memory().unwrap();
    let mut r = template_after_first_choice();
    r.state.playtime_ms = 90_000;

    let meta = store
        .save(&r.pack, 3, &r.state, Some("Before the letter"))
        .unwrap();
    assert_eq!(meta.scene_description, "MY NEW STORY - Door Open");
    assert_eq!(meta.choices_made, 1);
    store.save(&r.pack, AUTOSAVE_SLOT, &r.state, None).unwrap();
    // saving again replaces the slot
    store.save(&r.pack, 3, &r.state, Some("Renamed")).unwrap();

    let loaded = store.load("_template", 3).unwrap();
    assert_eq!(loaded.state.current_scene, "door_open");
    assert_eq!(loaded.state.items, vec!["Wet Letter".to_string()]);
    assert_eq!(loaded.state.playtime_ms, 90_000);
    assert_eq!(loaded.metadata.name, "Renamed");
    assert_eq!(
        store
            .load("_template", AUTOSAVE_SLOT)
            .unwrap()
            .metadata
            .name,
        "Autosave"
    );
    assert_eq!(store.slots("_template").iter().flatten().count(), 2);
    assert_eq!(store.all().len(), 2);
    assert_eq!(store.latest().unwrap().story_id, "_template");

    store.delete("_template", 3).unwrap();
    assert!(store.load("_template", 3).is_err());
    assert!(store.delete("_template", 3).is_err());
    assert!(store.save(&r.pack, 11, &r.state, None).is_err());
}

#[test]
fn blank_names_get_a_default() {
    let store = Store::in_memory().unwrap();
    let r = template_after_first_choice();
    assert_eq!(
        store.save(&r.pack, 4, &r.state, Some("   ")).unwrap().name,
        "Save 4"
    );
}

#[test]
fn export_and_import() {
    let store = Store::in_memory().unwrap();
    let r = template_after_first_choice();
    store.save(&r.pack, 3, &r.state, Some("Keep me")).unwrap();

    let dir = temp_dir("export");
    let file = dir.join("backup.json");
    store.export("_template", 3, &file).unwrap();

    // into another database, a different slot
    let other = Store::in_memory().unwrap();
    let m = other.import(&file, 7).unwrap();
    assert_eq!((m.slot, m.name.as_str()), (7, "Keep me"));
    assert_eq!(
        other.load("_template", 7).unwrap().state.current_scene,
        "door_open"
    );
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn rejects_garbage_and_other_versions() {
    let store = Store::in_memory().unwrap();
    let dir = temp_dir("bad");
    let bad = dir.join("bad.json");
    std::fs::write(&bad, "{\"nope\": 1}").unwrap();
    assert!(store.import(&bad, 1).is_err());

    // a save from an incompatible future version
    let r = template_after_first_choice();
    store.save(&r.pack, 1, &r.state, None).unwrap();
    let good = dir.join("good.json");
    store.export("_template", 1, &good).unwrap();
    let future = std::fs::read_to_string(&good)
        .unwrap()
        .replace(GAME_VERSION, "99.0.0");
    std::fs::write(&bad, future).unwrap();
    assert!(
        store
            .import(&bad, 2)
            .unwrap_err()
            .to_string()
            .contains("99.0.0")
    );
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn imports_python_saves_once() {
    let dir = temp_dir("legacy");
    std::fs::create_dir_all(dir.join("saves")).unwrap();
    std::fs::write(
        dir.join("saves/save_slot_1.json"),
        r#"{"metadata": {"slot": 1, "save_name": "3rd choise", "timestamp": "2025-10-24T15:20:25.672107",
            "game_version": "1.0.0", "current_scene": "door_open", "scene_description": "x",
            "playtime": 313.0, "completion_percentage": 3.5, "visited_scenes_count": 2, "total_choices_made": 1},
           "game_state": {"current_scene": "door_open", "visited_scenes": ["opening", "door_open"],
            "flags": {"heard_knock": true, "nope": false}, "inventory": ["Wet Letter"], "variables": {},
            "choice_history": [["opening", 0, "Open the door"]], "playtime": 313.0,
            "story_title": "MY NEW STORY", "story_class": "X", "total_scenes": 4}}"#,
    )
    .unwrap();
    // a save for a story that isn't installed is skipped
    std::fs::write(
        dir.join("saves/autosave.json"),
        r#"{"metadata": {}, "game_state": {"story_title": "UNKNOWN"}}"#,
    )
    .unwrap();
    let template = stories().join("_template");
    let entries = vec![Entry {
        id: "_template".into(),
        meta: Meta::load(&template).unwrap(),
        dir: template,
    }];

    let store = Store::in_memory().unwrap();
    assert_eq!(store.import_python_saves(&dir, &entries).unwrap(), 1);
    let f = store.load("_template", 1).unwrap();
    assert_eq!(f.metadata.name, "3rd choise");
    assert_eq!(f.state.playtime_ms, 313_000);
    assert_eq!(
        f.state.flags.iter().collect::<Vec<_>>(),
        vec!["heard_knock"]
    );
    assert_eq!(f.state.choices[0].text, "Open the door");
    assert!(f.metadata.timestamp.starts_with("2025-10-24T15:20:25"));
    // runs only once
    assert_eq!(store.import_python_saves(&dir, &entries).unwrap(), 0);
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn playtime_format() {
    assert_eq!(format_playtime(5 * 60_000), "5m");
    assert_eq!(format_playtime(65 * 60_000), "1h 05m");
}
