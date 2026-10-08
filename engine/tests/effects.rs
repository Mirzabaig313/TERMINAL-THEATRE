//! Screen effects and scene transitions in the story format: every name loads,
//! a misspelt one is refused, and scenes fall back to the story's transition.

mod common;

use common::template;
use theatre_engine::scene::{LineFx, Scene, Transition};

#[test]
fn every_effect_and_transition_name_loads() {
    for fx in LineFx::ALL {
        for t in Transition::ALL {
            let src = format!(
                "text = \"x\"\ntransition = \"{}\"\nfx = \"{}\"\ndialogue = [{{ who = \"a\", line = \"b\", fx = \"{}\" }}]",
                t.name(),
                fx.name(),
                fx.name()
            );
            let scene: Scene = toml::from_str(&src).unwrap_or_else(|e| panic!("{src}: {e}"));
            assert_eq!(scene.transition, Some(t));
            assert_eq!(scene.fx, Some(fx));
            assert_eq!(scene.dialogue[0].fx, Some(fx));
        }
    }
}

#[test]
fn a_misspelt_effect_is_refused_with_the_choices() {
    let err = toml::from_str::<Scene>("text = \"x\"\nfx = \"lightening\"")
        .expect_err("should fail")
        .to_string();
    assert!(
        err.contains("lightening") && err.contains("lightning"),
        "{err}"
    );
    let err = toml::from_str::<Scene>("text = \"x\"\ntransition = \"wipe\"")
        .expect_err("should fail")
        .to_string();
    assert!(err.contains("wipe") && err.contains("dissolve"), "{err}");
}

#[test]
fn scenes_use_the_story_transition_unless_they_say() {
    let pack = template();
    assert_eq!(pack.meta.transition, Transition::Dissolve);
    assert_eq!(pack.transition_of("opening"), Transition::Dissolve);
    assert_eq!(pack.transition_of("door_open"), Transition::Sweep);
    assert_eq!(pack.transition_of("silence"), Transition::Fade);
    assert_eq!(pack.scenes["door_open"].fx, Some(LineFx::Lightning));
}
