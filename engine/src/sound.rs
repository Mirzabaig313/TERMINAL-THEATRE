//! Sounds in the story format: one-off sounds (`sound = "gunshot"`) and
//! background loops (`ambience = "rain"`). The game makes the built-in sounds
//! itself; a story can also name an audio file in its folder
//! (`sound = "sounds/door.ogg"`).

use std::fmt;

use serde::Deserialize;

use crate::scene::LineFx;

/// A built-in one-off sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sfx {
    Gunshot,
    Thunder,
    Knock,
    Phone,
    Siren,
    Glass,
    Heartbeat,
    Impact,
    Static,
    Sting,
    Whoosh,
    Warble,
    Chill,
    Chime,
    Footsteps,
    /// no sound (to keep an effect quiet)
    Silence,
}

impl Sfx {
    pub const ALL: [Sfx; 16] = [
        Sfx::Gunshot,
        Sfx::Thunder,
        Sfx::Knock,
        Sfx::Phone,
        Sfx::Siren,
        Sfx::Glass,
        Sfx::Heartbeat,
        Sfx::Impact,
        Sfx::Static,
        Sfx::Sting,
        Sfx::Whoosh,
        Sfx::Warble,
        Sfx::Chill,
        Sfx::Chime,
        Sfx::Footsteps,
        Sfx::Silence,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Sfx::Gunshot => "gunshot",
            Sfx::Thunder => "thunder",
            Sfx::Knock => "knock",
            Sfx::Phone => "phone",
            Sfx::Siren => "siren",
            Sfx::Glass => "glass",
            Sfx::Heartbeat => "heartbeat",
            Sfx::Impact => "impact",
            Sfx::Static => "static",
            Sfx::Sting => "sting",
            Sfx::Whoosh => "whoosh",
            Sfx::Warble => "warble",
            Sfx::Chill => "chill",
            Sfx::Chime => "chime",
            Sfx::Footsteps => "footsteps",
            Sfx::Silence => "silence",
        }
    }
}

/// A built-in background loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ambient {
    Rain,
    Storm,
    City,
    Neon,
    Wind,
    Drone,
    Dream,
    Silence,
}

impl Ambient {
    pub const ALL: [Ambient; 8] = [
        Ambient::Rain,
        Ambient::Storm,
        Ambient::City,
        Ambient::Neon,
        Ambient::Wind,
        Ambient::Drone,
        Ambient::Dream,
        Ambient::Silence,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Ambient::Rain => "rain",
            Ambient::Storm => "storm",
            Ambient::City => "city",
            Ambient::Neon => "neon",
            Ambient::Wind => "wind",
            Ambient::Drone => "drone",
            Ambient::Dream => "dream",
            Ambient::Silence => "silence",
        }
    }
}

/// Names that can be written in a story file.
pub trait Named: Sized + Copy + 'static {
    const KIND: &'static str;
    fn all() -> &'static [Self];
    fn label(self) -> &'static str;
}

impl Named for Sfx {
    const KIND: &'static str = "sound";
    fn all() -> &'static [Self] {
        &Sfx::ALL
    }
    fn label(self) -> &'static str {
        self.name()
    }
}

impl Named for Ambient {
    const KIND: &'static str = "ambience";
    fn all() -> &'static [Self] {
        &Ambient::ALL
    }
    fn label(self) -> &'static str {
        self.name()
    }
}

/// A built-in sound by name, or an audio file inside the story folder (any
/// value with a `/` or a `.` in it).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String", bound = "")]
pub enum Audio<T: Named> {
    Builtin(T),
    File(String),
}

pub type SoundRef = Audio<Sfx>;
pub type AmbienceRef = Audio<Ambient>;

/// Audio files a story may use.
pub const AUDIO_EXTENSIONS: [&str; 4] = ["ogg", "wav", "mp3", "flac"];

impl<T: Named> TryFrom<String> for Audio<T> {
    type Error = String;

    fn try_from(s: String) -> Result<Self, String> {
        if s.contains('/') || s.contains('.') {
            return Ok(Audio::File(s));
        }
        T::all()
            .iter()
            .find(|t| t.label() == s)
            .map(|t| Audio::Builtin(*t))
            .ok_or_else(|| {
                let names: Vec<_> = T::all().iter().map(|t| t.label()).collect();
                format!(
                    "unknown {} '{s}' (use one of {}, or a file like \"sounds/x.ogg\")",
                    T::KIND,
                    names.join(", ")
                )
            })
    }
}

impl<T: Named> fmt::Display for Audio<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Audio::Builtin(t) => f.write_str(t.label()),
            Audio::File(p) => f.write_str(p),
        }
    }
}

impl LineFx {
    /// The sound an effect makes when the line or scene doesn't name one.
    pub fn sound(self) -> Sfx {
        match self {
            LineFx::Shake | LineFx::Blood => Sfx::Impact,
            LineFx::Glitch | LineFx::Static => Sfx::Static,
            LineFx::Flash | LineFx::Reveal => Sfx::Sting,
            LineFx::Blackout => Sfx::Whoosh,
            LineFx::Lightning => Sfx::Thunder,
            LineFx::Heartbeat => Sfx::Heartbeat,
            LineFx::Dizzy => Sfx::Warble,
            LineFx::Chill => Sfx::Chill,
        }
    }
}

/// What to play for a line or scene: its own sound, else its effect's sound.
pub fn sound_for(sound: Option<&SoundRef>, fx: Option<LineFx>) -> Option<SoundRef> {
    match sound {
        Some(Audio::Builtin(Sfx::Silence)) => None,
        Some(s) => Some(s.clone()),
        None => fx.map(|f| Audio::Builtin(f.sound())),
    }
}
