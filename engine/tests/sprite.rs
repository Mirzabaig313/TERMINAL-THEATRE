//! Pixel-art sprites: parsing, validation, expressions and animation.

mod common;

use theatre_engine::Rng;
use theatre_engine::library::discover;
use theatre_engine::sprite::{Actor, Pose, Sprite};

const TINY: &str = r##"
name = "Tiny"
size = [2, 2]
base = """
ab
.b
"""
[palette]
a = "#ff0000"
b = "#00ff00"
[anim]
blink_eyes = "shut"
blink_ms = [1000, 2000]
breathe_ms = 0
breathe_from_row = 0
talk_ms = 100
[parts.eyes.open]
at = [0, 0]
grid = "a"
[parts.eyes.shut]
at = [0, 0]
grid = "b"
[expressions.neutral]
eyes = "open"
"##;

fn error(src: &str) -> String {
    Sprite::parse(src)
        .err()
        .expect("should be rejected")
        .to_string()
}

#[test]
fn compose_and_blink() {
    let s = Sprite::parse(TINY).unwrap();
    let mut pose = Pose {
        expression: "neutral".into(),
        ..Pose::default()
    };
    assert_eq!(s.compose(&pose, 0)[0][0], Some((255, 0, 0)));
    assert_eq!(s.compose(&pose, 0)[1][0], None);
    pose.blinking = true;
    assert_eq!(s.compose(&pose, 0)[0][0], Some((0, 255, 0)));
}

#[test]
fn rejects_bad_sprites() {
    assert!(error(&TINY.replace("ab\n.b", "ax\n.b")).contains("'x'"));
    assert!(error(&TINY.replace("ab\n.b", "abc\n.b")).contains("2x2"));
    assert!(
        error(&TINY.replace("[expressions.neutral]", "[expressions.calm]")).contains("neutral")
    );
    assert!(error(&TINY.replace("eyes = \"open\"", "eyes = \"missing\"")).contains("eyes.missing"));
}

#[test]
fn actor_blinks_and_talks() {
    let s = Sprite::parse(TINY).unwrap();
    let mut a = Actor::new("tiny", "neutral");
    let mut rng = Rng::new(1);
    a.talking = true;
    let mut blinked = false;
    let mut mouth = [false, false];
    for t in (0..10_000).step_by(20) {
        a.update(&s, t, &mut rng);
        blinked |= a.pose.blinking;
        mouth[a.pose.mouth_open as usize] = true;
    }
    assert!(blinked, "never blinked in 10s");
    assert_eq!(mouth, [true, true], "mouth never moved while talking");
}

/// Every character file of every story (and the template) parses, and every
/// expression composes at full size.
#[test]
fn all_story_sprites_compose() {
    let mut dirs: Vec<_> = discover(&common::stories())
        .unwrap()
        .0
        .into_iter()
        .map(|e| e.dir)
        .collect();
    dirs.push(common::stories().join("_template"));
    let mut count = 0;
    for dir in dirs {
        for f in std::fs::read_dir(dir.join("characters")).unwrap() {
            let path = f.unwrap().path();
            let s = Sprite::load(&path).unwrap_or_else(|e| panic!("{e:#}"));
            for expr in s.expressions() {
                let pose = Pose {
                    expression: expr.into(),
                    blinking: true,
                    mouth_open: true,
                    exhale: true,
                };
                assert_eq!(s.compose(&pose, 1234).len(), s.height, "{}", path.display());
            }
            count += 1;
        }
    }
    assert!(count >= 15);
}
