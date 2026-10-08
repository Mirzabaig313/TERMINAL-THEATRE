//! Tools for story writers: every route played through (`theatre check`),
//! the shortest route to a scene, the story map, starting mid-way, reloading.

mod common;

use common::{LATER, stories, temp_dir, template};
use theatre_engine::StoryPack;
use theatre_engine::map::{Format, render};
use theatre_engine::runner::{Phase, Runner};
use theatre_engine::state::State;
use theatre_engine::walk::{Walk, describe};

const LIMIT: usize = 2_000_000;

#[test]
fn every_bundled_story_survives_every_route() {
    for id in ["blood_and_neon", "noir_detective", "shadow_slave"] {
        let pack = StoryPack::load(&stories().join(id)).unwrap();
        let walk = Walk::run(&pack, LIMIT);
        assert!(!walk.truncated, "{id}: explored everything");
        let problems = walk.problems(&pack);
        assert!(problems.is_empty(), "{id}:\n{}", problems.join("\n"));
    }
}

#[test]
fn the_template_walks_cleanly_and_its_gates_are_noticed() {
    let pack = template();
    let walk = Walk::run(&pack, LIMIT);
    assert_eq!(walk.reached.len(), pack.scenes.len());
    assert!(
        walk.problems(&pack).is_empty(),
        "{:?}",
        walk.problems(&pack)
    );
    // the letter is always in hand at the door: the gate never closes
    let warnings = walk.warnings(&pack).join("\n");
    assert!(
        warnings.contains("\"Read the letter\" has a condition but is always open"),
        "{warnings}"
    );
}

#[test]
fn the_shortest_route_to_a_scene() {
    let pack = template();
    let walk = Walk::run(&pack, LIMIT);
    let route = walk.route("ending_letter").unwrap();
    assert_eq!(route.len(), 2);
    assert_eq!(route[0].scene, "opening");
    assert_eq!(
        describe(route),
        format!(
            "opening → \"{}\" → {} → \"{}\"",
            route[0].choice, route[1].scene, route[1].choice
        )
    );
    assert_eq!(walk.route("opening").unwrap().len(), 0);
}

/// The template copied to a temporary folder, with `edit` applied to its scenes.
fn template_with(name: &str, edit: impl Fn(String) -> String) -> StoryPack {
    let dir = temp_dir(name);
    let src = stories().join("_template");
    for sub in ["scenes", "characters", "images"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
        for f in std::fs::read_dir(src.join(sub)).unwrap() {
            let f = f.unwrap().path();
            std::fs::copy(&f, dir.join(sub).join(f.file_name().unwrap())).unwrap();
        }
    }
    std::fs::copy(src.join("story.toml"), dir.join("story.toml")).unwrap();
    let scenes = dir.join("scenes/01_beginning.toml");
    let text = std::fs::read_to_string(&scenes).unwrap();
    std::fs::write(&scenes, edit(text)).unwrap();
    let pack = StoryPack::load(&dir).unwrap();
    std::fs::remove_dir_all(dir).ok();
    pack
}

#[test]
fn mistakes_are_found() {
    // the only way out of `silence` needs courage the player can't have there,
    // and a line waits for a counter that only ever goes down
    let pack = template_with("walk-mistakes", |t| {
        t.replace(
            "[[silence.choices]]\ntext = \"Go back to sleep\"\ngoto = \"ending_letter\"",
            "[[silence.choices]]\ntext = \"Go back to sleep\"\ngoto = \"ending_letter\"\nif = \"courage >= 1\"",
        )
        .replace("if = \"caution >= 1 && !visited('door_open')\"", "if = \"caution <= -2\"")
        .replace(
            "{ who = \"narrator_voice\", line = \"Some doors should stay closed.\", fx = \"chill\" }",
            "{ who = \"narrator_voice\", line = \"Some doors should stay closed.\", fx = \"chill\", if = \"courage >= 5\" }",
        )
    });
    let walk = Walk::run(&pack, LIMIT);
    let problems = walk.problems(&pack).join("\n");
    assert!(
        problems.contains("scene 'silence': the player can be left with no choice"),
        "{problems}"
    );
    assert!(
        problems.contains("\"Open the door after all\" never opens"),
        "{problems}"
    );
    assert!(
        problems.contains("scene 'silence': line 1 is never said"),
        "{problems}"
    );
}

#[test]
fn negative_thresholds_count() {
    // `-2` is written as `0 - 2` inside a condition; it must still count as -2
    let pack = template_with("walk-negative", |t| {
        t.replace("add = { caution = 1 }", "add = { caution = -2 }")
            .replace(
                "if = \"caution >= 1 && !visited('door_open')\"",
                "if = \"caution <= -2\"",
            )
    });
    let walk = Walk::run(&pack, LIMIT);
    let problems = walk.problems(&pack).join("\n");
    assert!(!problems.contains("Open the door after all"), "{problems}");
    assert!(walk.reached.contains("door_open"));
}

#[test]
fn a_big_story_stops_at_the_limit() {
    let pack = StoryPack::load(&stories().join("shadow_slave")).unwrap();
    let walk = Walk::run(&pack, 500);
    assert!(walk.truncated);
    assert!(walk.warnings(&pack)[0].contains("may be incomplete"));
}

#[test]
fn the_story_map() {
    let pack = template();
    let mermaid = render(&pack, Format::Mermaid);
    assert!(mermaid.contains("flowchart TD"));
    assert!(mermaid.contains("s_opening[\"Opening\"]:::start"));
    assert!(mermaid.contains("s_ending_letter([\"Ending Letter\"]):::ending"));
    assert!(mermaid.contains("s_opening -->|\"Open the door\"| s_door_open"));
    assert!(
        mermaid.contains("s_silence -.->|\"Open the door after all\"| s_door_open"),
        "gated choices are dashed"
    );
    let edges = pack.scenes.values().map(|s| s.choices.len()).sum::<usize>();
    assert_eq!(mermaid.matches("|\"").count(), edges);

    let dot = render(&pack, Format::Dot);
    assert!(dot.starts_with("digraph"));
    assert!(dot.contains("\"opening\" -> \"door_open\" [label=\"Open the door\"]"));
    assert!(dot.trim_end().ends_with('}'));
}

#[test]
fn starting_state_from_text() {
    let mut s = State::default();
    s.set_from("trust=3").unwrap();
    s.set_from(" mercy = -2 ").unwrap();
    s.set_from("flag:met_chen").unwrap();
    s.set_from("item:Wet Letter").unwrap();
    s.set_from("item:Wet Letter").unwrap();
    assert_eq!(s.vars["trust"], 3);
    assert_eq!(s.vars["mercy"], -2);
    assert!(s.flags.contains("met_chen"));
    assert_eq!(s.items, vec!["Wet Letter".to_string()]);
    for bad in ["trust", "trust=lots", ""] {
        assert!(s.set_from(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn reloading_keeps_the_state_and_the_scene() {
    let mut r = Runner::new(template(), 0);
    let mut now = 0;
    while r.phase() != Phase::Choose {
        now += LATER;
        r.advance(now);
    }
    r.select(0); // open the door: courage 1
    now += LATER;
    r.advance(now);
    assert_eq!(r.scene_id(), "door_open");
    let courage = r.state.vars["courage"];
    let items = r.state.items.clone();

    r.reload(template(), now + 10).unwrap();
    assert_eq!(r.scene_id(), "door_open");
    assert_eq!(r.phase(), Phase::Narration, "the scene starts over");
    assert_eq!(r.state.vars["courage"], courage);
    assert_eq!(r.state.items, items, "scene effects are not applied again");

    let gone = template_with("reload-gone", |t| {
        t.replace("[door_open]", "[door_opened]")
            .replace("goto = \"door_open\"", "goto = \"door_opened\"")
            .replace("[[door_open.choices]]", "[[door_opened.choices]]")
            .replace("visited('door_open')", "visited('door_opened')")
    });
    let err = r.reload(gone, now + 20).unwrap_err();
    assert!(err.contains("door_open"), "{err}");
    assert_eq!(r.scene_id(), "door_open", "the old story keeps running");
}
