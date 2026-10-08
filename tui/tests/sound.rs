//! Sound: the built-in sounds are well formed, background loops have no seam,
//! the game asks for the right loop in each scene, and the settings work.
//! Tests never open the audio device (`Audio::off`).

mod common;

use common::Harness;
use ratatui::crossterm::event::KeyCode;
use terminal_theatre::app::Start;
use terminal_theatre::audio::synth::{self, RATE, Ui};
use terminal_theatre::audio::{Audio, Loop, Status};
use theatre_engine::sound::{Ambient, Sfx};

fn peak(s: &[f32]) -> f32 {
    s.iter().fold(0.0, |m, x| m.max(x.abs()))
}

#[test]
fn built_in_sounds_are_well_formed() {
    for s in Sfx::ALL {
        let samples = synth::sfx(s);
        if s == Sfx::Silence {
            assert!(samples.is_empty());
            continue;
        }
        let secs = samples.len() as f32 / RATE as f32;
        assert!((0.3..5.0).contains(&secs), "{s:?}: {secs} s");
        assert!(samples.iter().all(|x| x.is_finite()), "{s:?}");
        let p = peak(&samples);
        assert!((0.4..=0.9001).contains(&p), "{s:?}: peak {p}");
        // dies away (or fades out) instead of stopping with a click
        let end = &samples[samples.len() - RATE as usize / 50..];
        assert!(peak(end) < 0.3, "{s:?} ends abruptly: {}", peak(end));
    }
    for u in [Ui::Move, Ui::Select, Ui::Back] {
        let samples = synth::ui(u);
        assert!(samples.len() < RATE as usize / 4, "{u:?} is short");
        assert!(peak(&samples) <= 0.4001, "{u:?} is soft");
    }
}

#[test]
fn background_loops_have_no_seam() {
    for a in Ambient::ALL {
        let s = synth::ambience(a);
        if a == Ambient::Silence {
            assert!(s.is_empty());
            continue;
        }
        assert!(
            s.len() >= 8 * RATE as usize,
            "{a:?} long enough not to repeat obviously"
        );
        assert!(s.iter().all(|x| x.is_finite()), "{a:?}");
        assert!(peak(&s) <= 0.5501, "{a:?} sits under the effects");
        // the jump from the last sample back to the first is like any other step
        let steps: f32 = s.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>() / s.len() as f32;
        let seam = (s[0] - s[s.len() - 1]).abs();
        assert!(
            seam < steps * 8.0 + 0.01,
            "{a:?}: seam {seam}, usual step {steps}"
        );
    }
}

#[test]
fn silent_audio_never_starts() {
    let audio = Audio::off();
    audio.set_gain(0.7);
    assert!(matches!(audio.status(), Status::Unavailable(_)));
    audio.set_ambience(Some(Loop::Builtin(Ambient::Rain)));
    assert_eq!(audio.ambience(), Some(Loop::Builtin(Ambient::Rain)));
}

#[test]
fn each_scene_asks_for_its_background() {
    let mut h = Harness::new("sound-amb", Start::Story("_template".into()));
    h.wait(20_000);
    h.press(KeyCode::Enter);
    assert_eq!(
        h.app.ctx().audio.ambience(),
        Some(Loop::Builtin(Ambient::Rain)),
        "the story's rain"
    );
    for _ in 0..3 {
        h.press(KeyCode::Enter);
        h.wait(200);
        h.press(KeyCode::Enter);
    }
    h.char('2'); // stay silent: a scene with its own wind
    h.wait(100);
    assert_eq!(
        h.app.ctx().audio.ambience(),
        Some(Loop::Builtin(Ambient::Wind))
    );
    h.press(KeyCode::Esc); // pause menu
    h.press(KeyCode::Down);
    h.press(KeyCode::Down);
    h.press(KeyCode::Down); // Main Menu
    h.press(KeyCode::Enter);
    h.char('y'); // confirm leaving
    h.wait(100);
    assert!(
        h.screen().contains("Begin a fresh performance"),
        "back at the main menu"
    );
    assert_eq!(h.app.ctx().audio.ambience(), None, "quiet outside stories");
}

#[test]
fn sound_and_volume_settings() {
    let mut h = Harness::new("sound-settings", Start::MainMenu);
    h.char('5'); // settings
    for _ in 0..7 {
        h.press(KeyCode::Down);
    }
    assert!(h.screen().contains("Sound"));
    h.press(KeyCode::Enter); // sound on
    assert!(h.app.ctx().settings.sound);
    assert!(
        h.screen().contains("silent"),
        "tests have no audio device, and say so"
    );
    h.press(KeyCode::Down); // volume
    h.press(KeyCode::Left);
    h.press(KeyCode::Left);
    assert_eq!(h.app.ctx().settings.volume, 50);
    assert!(h.screen().contains("50%"));
    let saved = h.app.ctx().store.settings();
    assert!(saved.sound);
    assert_eq!(saved.volume, 50);
    assert!(
        (h.app.ctx().audio.gain() - 0.5).abs() < 1e-6,
        "the game follows the setting"
    );
}
