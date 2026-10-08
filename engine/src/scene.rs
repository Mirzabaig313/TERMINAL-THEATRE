//! Scene data, as written in `scenes/*.toml`.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub text: String,
    /// still ASCII art for the stage
    #[serde(default)]
    pub art: Option<String>,
    /// a picture for the stage (PNG, JPEG or SVG, path inside the story folder);
    /// shown instead of the ASCII art when the player has scene images on
    #[serde(default)]
    pub image: Option<String>,
    /// animated ASCII art: frames shown in turn every `art_ms`
    #[serde(default)]
    pub art_frames: Vec<String>,
    #[serde(default = "default_art_ms")]
    pub art_ms: u64,
    /// false = stop on the last frame
    #[serde(default = "yes")]
    pub art_loop: bool,
    #[serde(default)]
    pub mood: Option<Mood>,
    /// how the scene comes on screen (the story's default when missing)
    #[serde(default)]
    pub transition: Option<Transition>,
    /// screen effect played once the scene is on screen
    #[serde(default)]
    pub fx: Option<LineFx>,
    #[serde(default)]
    pub ending: bool,
    #[serde(default)]
    pub set_flags: Vec<String>,
    #[serde(default)]
    pub add_items: Vec<String>,
    /// change counters on entering: `add = { trust = 1, suspicion = -2 }`
    #[serde(default)]
    pub add: BTreeMap<String, i64>,
    /// set counters on entering: `set = { chapter = 2 }`
    #[serde(default)]
    pub set: BTreeMap<String, i64>,
    /// other openings: the first whose condition holds replaces `text`
    #[serde(default)]
    pub variants: Vec<Variant>,
    #[serde(default)]
    pub dialogue: Vec<Line>,
    #[serde(default)]
    pub choices: Vec<Choice>,
}

fn default_art_ms() -> u64 {
    150
}

fn yes() -> bool {
    true
}

impl Scene {
    pub fn has_art(&self) -> bool {
        self.art.is_some() || !self.art_frames.is_empty() || self.image.is_some()
    }

    /// The art to show `elapsed_ms` after the scene started.
    pub fn art_at(&self, elapsed_ms: u64) -> Option<&str> {
        if self.art_frames.is_empty() {
            return self.art.as_deref();
        }
        let n = self.art_frames.len() as u64;
        let i = elapsed_ms / self.art_ms.max(1);
        let i = if self.art_loop { i % n } else { i.min(n - 1) };
        Some(&self.art_frames[i as usize])
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Line {
    /// speaker id from story.toml `[speakers]`
    pub who: String,
    pub line: String,
    /// only said when this holds (see [`crate::logic`])
    #[serde(default, rename = "if")]
    pub cond: Option<String>,
    /// expression from the speaker's sprite (e.g. "afraid")
    #[serde(default)]
    pub mood: Option<String>,
    /// screen effect played when the line starts
    #[serde(default)]
    pub fx: Option<LineFx>,
}

/// A screen effect, played when a line starts (`fx` on a line) or when a scene
/// is on screen (`fx` on a scene).
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LineFx {
    /// the screen jolts sideways: gunshots, blows, slammed doors
    Shake,
    /// rows tear and noise flickers: static, drugs, broken implants
    Glitch,
    /// a white flash that fades: shocks and revelations
    Flash,
    /// a red wash that drains away: wounds, violence
    Blood,
    /// everything goes black, then slowly comes back: knocked out, lights cut
    Blackout,
    /// two quick white flashes: lightning, camera flashes
    Lightning,
    /// the screen throbs dark twice: fear, adrenaline
    Heartbeat,
    /// colors swim and shift hue: drugs, poison, vertigo
    Dizzy,
    /// colors drain to a cold blue: dread, the supernatural
    Chill,
    /// the screen breaks into noise and pulls itself back together
    Static,
    /// the screen is uncovered left to right: a reveal
    Reveal,
}

impl LineFx {
    pub const ALL: [LineFx; 11] = [
        LineFx::Shake,
        LineFx::Glitch,
        LineFx::Flash,
        LineFx::Blood,
        LineFx::Blackout,
        LineFx::Lightning,
        LineFx::Heartbeat,
        LineFx::Dizzy,
        LineFx::Chill,
        LineFx::Static,
        LineFx::Reveal,
    ];

    pub fn name(self) -> &'static str {
        match self {
            LineFx::Shake => "shake",
            LineFx::Glitch => "glitch",
            LineFx::Flash => "flash",
            LineFx::Blood => "blood",
            LineFx::Blackout => "blackout",
            LineFx::Lightning => "lightning",
            LineFx::Heartbeat => "heartbeat",
            LineFx::Dizzy => "dizzy",
            LineFx::Chill => "chill",
            LineFx::Static => "static",
            LineFx::Reveal => "reveal",
        }
    }
}

/// How a scene comes on screen.
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transition {
    /// cells appear in a scattered order out of the dark
    #[default]
    Dissolve,
    /// fades up from black
    Fade,
    /// uncovered left to right with a soft edge
    Sweep,
    /// uncovered bottom to top
    Rise,
    /// scattered cells gather into the picture
    Coalesce,
    /// no transition
    Cut,
}

impl Transition {
    pub const ALL: [Transition; 6] = [
        Transition::Dissolve,
        Transition::Fade,
        Transition::Sweep,
        Transition::Rise,
        Transition::Coalesce,
        Transition::Cut,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Transition::Dissolve => "dissolve",
            Transition::Fade => "fade",
            Transition::Sweep => "sweep",
            Transition::Rise => "rise",
            Transition::Coalesce => "coalesce",
            Transition::Cut => "cut",
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub text: String,
    pub goto: String,
    /// shown only if any of these flags is set (or any item below is held)
    #[serde(default)]
    pub any_flags: Vec<String>,
    #[serde(default)]
    pub any_items: Vec<String>,
    /// shown only when this holds (see [`crate::logic`]); combines with the above
    #[serde(default, rename = "if")]
    pub cond: Option<String>,
    /// applied when the player takes this choice
    #[serde(default)]
    pub set_flags: Vec<String>,
    #[serde(default)]
    pub add_items: Vec<String>,
    #[serde(default)]
    pub add: BTreeMap<String, i64>,
    #[serde(default)]
    pub set: BTreeMap<String, i64>,
}

/// An alternative opening for a scene.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    #[serde(rename = "if")]
    pub cond: String,
    pub text: String,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    Noir,
    Danger,
    Alert,
    Calm,
    Mystery,
}

impl Mood {
    pub const ALL: [Mood; 5] = [
        Mood::Noir,
        Mood::Danger,
        Mood::Alert,
        Mood::Calm,
        Mood::Mystery,
    ];

    /// Guess a mood from a scene id, like the original Python engine did.
    pub fn guess(scene_id: &str, default: Mood) -> Mood {
        let keywords: [(Mood, &[&str]); 4] = [
            (
                Mood::Danger,
                &["battle", "fight", "nightmare", "attack", "death", "blood"],
            ),
            (
                Mood::Alert,
                &["awakening", "spell", "warning", "danger", "threat"],
            ),
            (Mood::Calm, &["rest", "safe", "recovery", "memory", "dream"]),
            (
                Mood::Mystery,
                &["shadow", "darkness", "unknown", "secret", "revelation"],
            ),
        ];
        keywords
            .iter()
            .find(|(_, words)| words.iter().any(|w| scene_id.contains(w)))
            .map(|(m, _)| *m)
            .unwrap_or(default)
    }
}
