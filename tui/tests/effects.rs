//! Screen effects (tachyonfx) and scene transitions: each one changes the
//! screen while it plays, leaves it untouched when it's over, and respects the
//! Screen Effects setting.

mod common;

use common::Harness;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::style::Color;
use terminal_theatre::app::Start;
use terminal_theatre::render::Theme;
use terminal_theatre::render::effects::{Running, TRANSITION_MS, screen_effect, transition};
use theatre_engine::scene::{LineFx, Transition};
use theatre_engine::settings::Effects;

/// A small screen of colored text, like a scene.
fn scene() -> Buffer {
    let area = Rect::new(0, 0, 40, 12);
    let mut buf = Buffer::empty(area);
    for y in 0..area.height {
        for x in 0..area.width {
            let ch = if (x + y) % 3 == 0 { '#' } else { 'a' };
            buf[(x, y)]
                .set_char(ch)
                .set_fg(Color::Rgb(200, 180, 120))
                .set_bg(Color::Rgb(20, 18, 30));
        }
    }
    buf
}

fn changed(a: &Buffer, b: &Buffer) -> usize {
    a.content
        .iter()
        .zip(&b.content)
        .filter(|(x, y)| x != y)
        .count()
}

/// Play an effect over fresh frames; returns (most cells changed in one frame,
/// cells changed in the frame after it ended).
fn play(running: &mut Running, ms: u64) -> (usize, usize) {
    let clean = scene();
    let mut most = 0;
    let mut t = 0;
    while t <= ms {
        t += 33;
        let mut buf = scene();
        let area = buf.area;
        running.apply(&mut buf, area, t);
        most = most.max(changed(&clean, &buf));
    }
    let mut after = scene();
    let area = after.area;
    running.apply(&mut after, area, t + 33);
    (most, changed(&clean, &after))
}

#[test]
fn every_screen_effect_shows_then_clears() {
    let th = Theme::theatre();
    for kind in LineFx::ALL {
        let Some(effect) = screen_effect(kind, &th, 1.0) else {
            // drawn by hand: the screen jolts or tears
            assert!(matches!(kind, LineFx::Shake | LineFx::Glitch), "{kind:?}");
            continue;
        };
        let mut running = Running::new(effect, 0);
        let (most, after) = play(&mut running, 3000);
        assert!(
            most > 100,
            "{kind:?} should visibly change the screen: {most}"
        );
        assert_eq!(after, 0, "{kind:?} should leave the screen untouched");
        assert!(!running.running(), "{kind:?} should end within 3 s");
    }
}

#[test]
fn effects_follow_the_setting() {
    let th = Theme::theatre();
    for kind in LineFx::ALL {
        assert!(
            screen_effect(kind, &th, Effects::Off.strength()).is_none(),
            "{kind:?}"
        );
    }
    // reduced: still there, but gentler than full
    let peak = |strength: f32| {
        let mut r = Running::new(screen_effect(LineFx::Flash, &th, strength).unwrap(), 0);
        let mut buf = scene();
        let area = buf.area;
        r.apply(&mut buf, area, 33);
        match buf[(1, 0)].bg {
            Color::Rgb(r, g, b) => r as u32 + g as u32 + b as u32,
            _ => 0,
        }
    };
    let (full, reduced) = (peak(1.0), peak(Effects::Reduced.strength()));
    assert!(
        full > reduced && reduced > 20 + 18 + 30,
        "full {full}, reduced {reduced}"
    );
}

#[test]
fn every_transition_uncovers_the_scene() {
    let th = Theme::theatre();
    for kind in Transition::ALL {
        let Some(effect) = transition(kind, &th) else {
            assert_eq!(kind, Transition::Cut);
            continue;
        };
        let mut running = Running::new(effect, 0);
        let mut first = scene();
        let area = first.area;
        running.apply(&mut first, area, 33);
        assert!(
            changed(&scene(), &first) > 100,
            "{kind:?} should hide most of the scene at first"
        );
        let (_, after) = play(&mut running, TRANSITION_MS);
        assert_eq!(after, 0, "{kind:?} should be over after {TRANSITION_MS} ms");
    }
}

#[test]
fn a_late_first_draw_catches_up() {
    // effects are timed from when they started, not from when they are first drawn
    let th = Theme::theatre();
    let mut running = Running::new(transition(Transition::Sweep, &th).unwrap(), 0);
    let mut buf = scene();
    let area = buf.area;
    assert!(!running.apply(&mut buf, area, 5000), "long over");
    assert_eq!(changed(&scene(), &buf), 0);
}

#[test]
fn a_scene_effect_plays_after_the_transition() {
    // the template's door_open sweeps in, then lightning strikes
    let mut h = Harness::new("fx-scene", Start::Story("_template".into()));
    h.wait(20_000);
    h.press(KeyCode::Enter); // past the title card
    for _ in 0..3 {
        h.press(KeyCode::Enter);
        h.wait(200);
        h.press(KeyCode::Enter);
    }
    h.char('1'); // open the door
    h.draw();
    assert!(h.app.busy(), "the transition is playing");
    let lit = |b: &Buffer| {
        b.content
            .iter()
            .filter(|c| matches!(c.bg, Color::Rgb(r, g, b) if r as u32 + g as u32 + b as u32 > 400))
            .count()
    };
    let mut peak = 0;
    for _ in 0..30 {
        h.t += 33;
        h.app.tick(h.t);
        peak = peak.max(lit(h.draw()));
    }
    assert!(
        peak > 1000,
        "lightning lights up the screen: {peak} bright cells"
    );
    h.wait(3000);
    let after = lit(h.draw());
    assert!(
        after * 5 < peak,
        "and is gone ({after} bright cells left: the picture)"
    );
    assert!(!h.app.busy(), "idle again");
}
