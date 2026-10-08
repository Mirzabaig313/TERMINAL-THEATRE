//! Sound: one-off effects, background loops and menu sounds.
//!
//! Sounds play on their own thread, so the game never waits for audio. The
//! thread starts the first time sound is wanted (sound is off by default),
//! and when there is no audio device (SSH, a headless machine) the game is
//! simply silent. `THEATRE_SOUND=off` keeps it silent everywhere.
//!
//! Built-in sounds are made by [`synth`]; stories can also name audio files.

pub mod synth;

use std::cell::{Cell, RefCell};
use std::path::PathBuf;

use theatre_engine::StoryPack;
use theatre_engine::sound::{AmbienceRef, Ambient, Audio as Named, Sfx, SoundRef};

pub use synth::Ui;

/// A one-off sound, ready to play.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cue {
    Builtin(Sfx),
    Ui(Ui),
    File(PathBuf),
}

/// A background loop, ready to play.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Loop {
    Builtin(Ambient),
    File(PathBuf),
}

impl Cue {
    pub fn of(s: &SoundRef, pack: &StoryPack) -> Option<Cue> {
        match s {
            Named::Builtin(Sfx::Silence) => None,
            Named::Builtin(b) => Some(Cue::Builtin(*b)),
            Named::File(f) => Some(Cue::File(pack.audio_path(f))),
        }
    }
}

impl Loop {
    pub fn of(a: &AmbienceRef, pack: &StoryPack) -> Option<Loop> {
        match a {
            Named::Builtin(Ambient::Silence) => None,
            Named::Builtin(b) => Some(Loop::Builtin(*b)),
            Named::File(f) => Some(Loop::File(pack.audio_path(f))),
        }
    }
}

/// The menu sound for a key, if it makes one.
pub fn menu_sound(code: ratatui::crossterm::event::KeyCode) -> Option<Ui> {
    use ratatui::crossterm::event::KeyCode;
    Some(match code {
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => Ui::Move,
        KeyCode::Char('j') | KeyCode::Char('k') => Ui::Move,
        KeyCode::Enter | KeyCode::Char(' ') => Ui::Select,
        KeyCode::Char(c) if c.is_ascii_digit() => Ui::Select,
        KeyCode::Esc => Ui::Back,
        _ => return None,
    })
}

// without the sound feature nothing reads the commands
#[cfg_attr(not(feature = "sound"), allow(dead_code))]
enum Cmd {
    Play(Cue),
    Ambience(Option<Loop>),
    Gain(f32),
}

/// Whether sound can be heard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// not started yet (sound has been off so far)
    Idle,
    Playing,
    /// no audio device, built without sound, or THEATRE_SOUND=off
    Unavailable(String),
}

/// The game's handle to the audio thread. Cheap to call every frame: it only
/// sends something when what should be heard changes.
pub struct Audio {
    tx: RefCell<Option<std::sync::mpsc::Sender<Cmd>>>,
    status: RefCell<Status>,
    gain: Cell<f32>,
    ambience: RefCell<Option<Loop>>,
    /// false in tests: never touch the audio device
    allowed: bool,
}

impl Audio {
    /// Sound that never plays (tests, and until the terminal app enables it).
    pub fn off() -> Self {
        Audio {
            tx: RefCell::new(None),
            status: RefCell::new(Status::Unavailable("sound is disabled".into())),
            gain: Cell::new(0.0),
            ambience: RefCell::new(None),
            allowed: false,
        }
    }

    /// Sound that starts when first wanted, unless THEATRE_SOUND=off.
    pub fn new() -> Self {
        let off = std::env::var("THEATRE_SOUND").is_ok_and(|v| v.eq_ignore_ascii_case("off"));
        Audio {
            allowed: !off,
            status: RefCell::new(if off {
                Status::Unavailable("THEATRE_SOUND=off".into())
            } else {
                Status::Idle
            }),
            ..Self::off()
        }
    }

    pub fn status(&self) -> Status {
        self.status.borrow().clone()
    }

    /// Volume, 0..1 (0 = silent). Starts the audio thread the first time it's above 0.
    pub fn set_gain(&self, gain: f32) {
        if gain > 0.0 {
            self.start();
        }
        if (gain - self.gain.get()).abs() > f32::EPSILON {
            self.gain.set(gain);
            self.send(Cmd::Gain(gain));
        }
    }

    pub fn gain(&self) -> f32 {
        self.gain.get()
    }

    pub fn play(&self, cue: Cue) {
        if self.gain.get() > 0.0 {
            self.send(Cmd::Play(cue));
        }
    }

    /// The loop that should be heard now (None = quiet). Changes cross-fade.
    pub fn set_ambience(&self, want: Option<Loop>) {
        if *self.ambience.borrow() != want {
            *self.ambience.borrow_mut() = want.clone();
            self.send(Cmd::Ambience(want));
        }
    }

    pub fn ambience(&self) -> Option<Loop> {
        self.ambience.borrow().clone()
    }

    fn send(&self, cmd: Cmd) {
        if let Some(tx) = self.tx.borrow().as_ref() {
            let _ = tx.send(cmd);
        }
    }

    fn start(&self) {
        if !self.allowed || self.tx.borrow().is_some() || *self.status.borrow() != Status::Idle {
            return;
        }
        match device::spawn() {
            Ok(tx) => {
                // the thread starts silent and with no loop: catch it up
                let _ = tx.send(Cmd::Gain(self.gain.get()));
                let _ = tx.send(Cmd::Ambience(self.ambience.borrow().clone()));
                *self.tx.borrow_mut() = Some(tx);
                *self.status.borrow_mut() = Status::Playing;
            }
            Err(why) => *self.status.borrow_mut() = Status::Unavailable(why),
        }
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "sound"))]
mod device {
    use super::Cmd;

    pub fn spawn() -> Result<std::sync::mpsc::Sender<Cmd>, String> {
        Err("built without sound".into())
    }
}

#[cfg(feature = "sound")]
mod device {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::num::NonZero;
    use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
    use std::time::{Duration, Instant};

    use rodio::buffer::SamplesBuffer;
    use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};

    use super::synth::{self, RATE};
    use super::{Cmd, Cue, Loop};

    /// How long a change of background loop takes to cross-fade.
    const FADE: Duration = Duration::from_millis(1500);
    /// Background loops sit under the effects.
    const AMBIENCE_LEVEL: f32 = 0.45;
    const SFX_LEVEL: f32 = 0.8;

    /// Open the audio device on a new thread; Err when there isn't one.
    pub fn spawn() -> Result<Sender<Cmd>, String> {
        let (tx, rx) = mpsc::channel();
        let (ready_tx, ready_rx) = mpsc::channel();
        std::thread::Builder::new()
            .name("theatre-audio".into())
            .spawn(move || match open() {
                Ok(sink) => {
                    let _ = ready_tx.send(Ok(()));
                    run(sink, rx);
                }
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                }
            })
            .map_err(|e| e.to_string())?;
        match ready_rx.recv_timeout(Duration::from_secs(2)) {
            Ok(Ok(())) => Ok(tx),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("the audio device didn't answer".into()),
        }
    }

    fn open() -> Result<MixerDeviceSink, String> {
        let mut sink = DeviceSinkBuilder::from_default_device()
            .map_err(|e| format!("no audio device ({e})"))?
            // never print: stderr is the game screen
            .with_error_callback(|_| {})
            .open_sink_or_fallback()
            .map_err(|e| format!("can't open the audio device ({e})"))?;
        sink.log_on_drop(false);
        Ok(sink)
    }

    fn buffer(samples: Vec<f32>) -> SamplesBuffer {
        SamplesBuffer::new(
            NonZero::new(1).unwrap(),
            NonZero::new(RATE).unwrap(),
            samples,
        )
    }

    /// A background loop playing, fading towards a volume.
    struct Fading {
        player: Player,
        from: f32,
        to: f32,
        since: Instant,
    }

    impl Fading {
        fn level(&self) -> f32 {
            let k = (self.since.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.0);
            self.from + (self.to - self.from) * k
        }
    }

    fn run(sink: MixerDeviceSink, rx: Receiver<Cmd>) {
        let mixer = sink.mixer();
        let mut sfx_cache: HashMap<Cue, Vec<f32>> = HashMap::new();
        let mut loop_cache: HashMap<Loop, Vec<f32>> = HashMap::new();
        let mut gain = 0.0f32;
        let mut current: Option<Fading> = None;
        let mut leaving: Vec<Fading> = Vec::new();
        loop {
            match rx.recv_timeout(Duration::from_millis(40)) {
                Ok(Cmd::Gain(g)) => gain = g,
                Ok(Cmd::Play(cue)) => {
                    let level = gain * SFX_LEVEL;
                    if let Cue::File(path) = &cue {
                        if let Ok(d) = File::open(path)
                            .map_err(|_| ())
                            .and_then(|f| Decoder::try_from(f).map_err(|_| ()))
                        {
                            mixer.add(d.amplify(level));
                        }
                    } else {
                        let samples = sfx_cache.entry(cue.clone()).or_insert_with(|| match &cue {
                            Cue::Builtin(s) => synth::sfx(*s),
                            Cue::Ui(u) => synth::ui(*u),
                            Cue::File(_) => Vec::new(),
                        });
                        if !samples.is_empty() {
                            mixer.add(buffer(samples.clone()).amplify(level));
                        }
                    }
                }
                Ok(Cmd::Ambience(want)) => {
                    if let Some(mut old) = current.take() {
                        old.from = old.level();
                        old.to = 0.0;
                        old.since = Instant::now();
                        leaving.push(old);
                    }
                    current = want.and_then(|l| start_loop(mixer, &l, &mut loop_cache));
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
            let level = gain * AMBIENCE_LEVEL;
            if let Some(c) = &mut current {
                c.to = 1.0;
                c.player.set_volume(c.level() * level);
            }
            leaving.retain(|f| {
                let v = f.level();
                f.player.set_volume(v * level);
                v > 0.0
            });
        }
    }

    fn start_loop(
        mixer: &rodio::mixer::Mixer,
        l: &Loop,
        cache: &mut HashMap<Loop, Vec<f32>>,
    ) -> Option<Fading> {
        let player = Player::connect_new(mixer);
        player.set_volume(0.0);
        match l {
            Loop::Builtin(a) => {
                let samples = cache
                    .entry(l.clone())
                    .or_insert_with(|| synth::ambience(*a));
                if samples.is_empty() {
                    return None;
                }
                player.append(buffer(samples.clone()).repeat_infinite());
            }
            Loop::File(path) => {
                let file = File::open(path).ok()?;
                let d = Decoder::new_looped(BufReader::new(file)).ok()?;
                player.append(d);
            }
        }
        Some(Fading {
            player,
            from: 0.0,
            to: 1.0,
            since: Instant::now(),
        })
    }
}
