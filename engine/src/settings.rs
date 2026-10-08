//! Player preferences, kept in the database (see [`Store`]).

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::store::Store;

/// `$THEATRE_HOME`, or `~/.terminal_theatre`. Saves and settings live here.
pub fn data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("THEATRE_HOME") {
        return dir.into();
    }
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    Path::new(&home).join(".terminal_theatre")
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextSpeed {
    Relaxed,
    Normal,
    Fast,
    Instant,
}

impl TextSpeed {
    pub const ALL: [TextSpeed; 4] = [
        TextSpeed::Relaxed,
        TextSpeed::Normal,
        TextSpeed::Fast,
        TextSpeed::Instant,
    ];

    pub fn label(self) -> &'static str {
        match self {
            TextSpeed::Relaxed => "Relaxed",
            TextSpeed::Normal => "Normal",
            TextSpeed::Fast => "Fast",
            TextSpeed::Instant => "Instant",
        }
    }

    /// Typewriter delay multiplier.
    pub fn multiplier(self) -> f32 {
        match self {
            TextSpeed::Relaxed => 1.35,
            TextSpeed::Normal => 1.0,
            TextSpeed::Fast => 0.65,
            TextSpeed::Instant => 0.0,
        }
    }
}

/// How strong screen effects (flash, shake, glitch) are.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Effects {
    Full,
    /// gentler: short, small shakes; dim flashes; light glitches
    Reduced,
    /// none at all, for photosensitivity and motion sickness
    Off,
}

impl Effects {
    pub fn label(self) -> &'static str {
        match self {
            Effects::Full => "Full",
            Effects::Reduced => "Reduced",
            Effects::Off => "Off",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Effects::Full => Effects::Reduced,
            Effects::Reduced => Effects::Off,
            Effects::Off => Effects::Full,
        }
    }

    /// Multiplier for effect strength: 1, 0.35 or 0.
    pub fn strength(self) -> f32 {
        match self {
            Effects::Full => 1.0,
            Effects::Reduced => 0.35,
            Effects::Off => 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// false = monochrome
    pub color: bool,
    /// play the cinematic before choosing a story
    pub cinematics: bool,
    pub text_speed: TextSpeed,
    pub typewriter: bool,
    /// sound effects, background ambience and menu sounds
    pub sound: bool,
    /// loudness in percent (0..=100)
    pub volume: u8,
    pub effects: Effects,
    /// skip mode also skips text never read before
    pub skip_unread: bool,
    /// show scene pictures (false = ASCII art only)
    pub images: bool,
    /// pause after a fully shown line in auto mode, in ms (scaled by line length)
    pub auto_delay_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            color: true,
            cinematics: true,
            text_speed: TextSpeed::Normal,
            typewriter: true,
            sound: false,
            volume: 70,
            effects: Effects::Full,
            skip_unread: false,
            images: true,
            auto_delay_ms: 1500,
        }
    }
}

const KEY: &str = "settings";

impl Store {
    /// Saved settings, or defaults when there are none (or they are unreadable).
    pub fn settings(&self) -> Settings {
        self.get(KEY)
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save_settings(&self, s: &Settings) -> Result<()> {
        self.set(KEY, &serde_json::to_string(s)?)
    }
}

impl Settings {
    /// Effective typewriter multiplier (0 = instant).
    pub fn speed(&self) -> f32 {
        if self.typewriter {
            self.text_speed.multiplier()
        } else {
            0.0
        }
    }

    /// Next speed preset; "Instant" turns the typewriter off, anything else on.
    pub fn cycle_text_speed(&mut self) {
        let i = TextSpeed::ALL
            .iter()
            .position(|s| *s == self.text_speed)
            .unwrap_or(0);
        self.text_speed = TextSpeed::ALL[(i + 1) % TextSpeed::ALL.len()];
        self.typewriter = self.text_speed != TextSpeed::Instant;
    }

    /// Volume as a 0..1 gain, 0 when sound is off.
    pub fn gain(&self) -> f32 {
        if self.sound {
            self.volume.min(100) as f32 / 100.0
        } else {
            0.0
        }
    }

    /// Louder by `step` percent (or quieter when negative), within 0..=100.
    pub fn change_volume(&mut self, step: i32) {
        self.volume = (self.volume as i32 + step).clamp(0, 100) as u8;
    }

    pub fn toggle_typewriter(&mut self) {
        self.typewriter = !self.typewriter;
        if !self.typewriter {
            self.text_speed = TextSpeed::Instant;
        } else if self.text_speed == TextSpeed::Instant {
            self.text_speed = TextSpeed::Normal;
        }
    }
}
