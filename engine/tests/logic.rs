//! Story logic: conditions, counters, conditional lines, narration variants and
//! `{counter}` text, checked on their own and through real playthroughs.

mod common;

use common::{LATER, stories, temp_dir, template};
use theatre_engine::logic::{check, check_text, fill, parse};
use theatre_engine::runner::{Phase, Runner};
use theatre_engine::state::State;
use theatre_engine::{Store, StoryPack};

fn state() -> State {
    let mut s = State::default();
    s.vars.insert("trust".into(), 3);
    s.vars.insert("suspicion".into(), -1);
    s.flags.insert("met_chen".into());
    s.items.push("Photograph".into());
    s.visited.push("alley".into());
    s
}

#[test]
fn conditions_evaluate() {
    let s = state();
    let yes = [
        "trust >= 2",
        "trust == 3 && flag('met_chen')",
        "item(\"Photograph\") || false",
        "!flag('betrayed')",
        "visited('alley') && !visited('pier')",
        "trust + suspicion == 2",
        "(trust > 5 || suspicion < 0) && true",
        "unknown_counter == 0",
        "trust",
        "-1 < suspicion + 1",
    ];
    for c in yes {
        assert!(check(Some(c), &s), "should hold: {c}");
    }
    let no = [
        "trust < 3",
        "flag('betrayed')",
        "item('Gun')",
        "!(trust >= 3)",
        "unknown_counter",
        "trust >= 2 && false",
    ];
    for c in no {
        assert!(!check(Some(c), &s), "should not hold: {c}");
    }
    assert!(check(None, &s), "no condition means yes");
}

#[test]
fn bad_conditions_are_explained() {
    for (src, hint) in [
        ("trust >=", "ends too early"),
        ("trust >= 2 &&& x", "unexpected"),
        ("flagg('x')", "unknown function"),
        ("flag(x)", "quoted name"),
        ("item('Gun'", "missing ')'"),
        ("(trust > 1", "missing ')'"),
        ("trust > 1)", "unexpected"),
        ("'open", "unclosed quote"),
        ("trust @ 2", "unexpected '@'"),
        ("", "empty"),
    ] {
        let err = parse(src)
            .err()
            .unwrap_or_else(|| panic!("should fail: {src}"))
            .to_string();
        assert!(err.contains(hint), "{src:?}: {err}");
    }
}

#[test]
fn counters_fill_text() {
    let s = state();
    assert_eq!(
        fill("Trust: {trust}, doubt: {suspicion}, none: {nope}", &s),
        "Trust: 3, doubt: -1, none: 0"
    );
    assert_eq!(fill("Braces {{like this}}", &s), "Braces {like this}");
    assert_eq!(fill("no placeholders", &s), "no placeholders");
    assert!(check_text("Trust: {trust} and {{literal}}").is_ok());
    for bad in ["{}", "{two words}", "{trust", "stray }", "{tr-ust}"] {
        assert!(check_text(bad).is_err(), "should be rejected: {bad}");
    }
}

/// Play the template from the start: `picks` are choice numbers (1-based).
fn play(picks: &[usize]) -> Runner {
    let mut r = Runner::new(template(), 0);
    let mut now = 0;
    let mut press = |r: &mut Runner| {
        now += LATER;
        r.advance(now);
    };
    for &pick in picks {
        while r.phase() != Phase::Choose {
            press(&mut r);
        }
        r.select(pick - 1);
        press(&mut r);
    }
    r
}

#[test]
fn choices_change_counters() {
    let r = play(&[1]); // open the door
    assert_eq!(r.state.vars.get("courage"), Some(&1));
    assert_eq!(r.state.vars.get("caution"), None);
    let r = play(&[2]); // stay silent
    assert_eq!(r.state.vars.get("caution"), Some(&1));
}

#[test]
fn conditional_choice_appears_only_when_it_holds() {
    let r = play(&[2]); // stay silent → silence
    let mut r = r;
    let mut now = LATER;
    while r.phase() != Phase::Choose {
        now += LATER;
        r.advance(now);
    }
    let texts: Vec<_> = r.choices().iter().map(|c| c.text.clone()).collect();
    assert!(
        texts.contains(&"Open the door after all".to_string()),
        "{texts:?}"
    );
}

#[test]
fn variants_and_conditional_lines() {
    // silence → open the door after all → read the letter: hesitated, then brave
    let mut r = play(&[2, 2, 1]);
    assert_eq!(r.scene_id(), "ending_letter");
    assert!(r.narration().contains("Courage: 1."), "{}", r.narration());
    // that path has courage, so the "should have kept the door shut" line is skipped
    let mut now = 10 * LATER;
    r.advance(now);
    now += LATER;
    r.advance(now);
    assert_eq!(r.phase(), Phase::End, "the conditional line was skipped");

    // silence → go back to sleep: caution 1, courage 0 → the line is said
    let mut r = play(&[2, 1]);
    assert!(
        r.narration().starts_with("Whatever happens next"),
        "plain text: {}",
        r.narration()
    );
    // the narration is fully typed by now, so one press moves to the line
    assert_eq!(r.advance(10 * LATER), Some(Phase::Line(0)));
    assert_eq!(r.line_text(0), "Should have kept the door shut.");

    // open the door → read the letter: the letter variant
    let r = play(&[1, 1]);
    assert!(
        r.narration().starts_with("The letter is damp"),
        "{}",
        r.narration()
    );
}

#[test]
fn counters_survive_saving() {
    let store = Store::in_memory().unwrap();
    let r = play(&[2, 2]);
    store.save(&r.pack, 1, &r.state, None).unwrap();
    let loaded = store.load("_template", 1).unwrap();
    assert_eq!(loaded.state.vars, r.state.vars);
    assert_eq!(loaded.state.vars.get("courage"), Some(&1));
}

#[test]
fn story_with_a_typo_in_a_condition_does_not_load() {
    let dir = temp_dir("logic-typo");
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
    let text = std::fs::read_to_string(&scenes)
        .unwrap()
        // Windows checkouts may have CRLF line endings
        .replace("\r\n", "\n");
    std::fs::write(
        &scenes,
        text.replace("caution >= 1 && !visited('door_open')", "caution >= && x"),
    )
    .unwrap();
    let err = format!("{:#}", StoryPack::load(&dir).err().expect("should fail"));
    assert!(
        err.contains("scene 'silence'") && err.contains("Open the door after all"),
        "{err}"
    );
    std::fs::remove_dir_all(dir).ok();
}
