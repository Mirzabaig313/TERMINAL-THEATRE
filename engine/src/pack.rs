//! Loading one story folder: metadata, scenes and characters, fully validated.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::color::{Rgb, parse_hex};
use crate::logic;
use crate::scene::{Mood, Scene, Transition};
use crate::sound::{AUDIO_EXTENSIONS, AmbienceRef, Audio};
use crate::sprite::Sprite;

/// Contents of `story.toml`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub title: String,
    /// typed out when a new game starts
    pub description: String,
    /// one-line tagline for the story-select screen
    #[serde(default)]
    pub summary: Option<String>,
    /// longer pitch for the story-select screen (defaults to the description)
    #[serde(default)]
    pub blurb: Option<String>,
    pub start: String,
    /// speaker whose portrait waits beside the choice menu
    #[serde(default)]
    pub player: Option<String>,
    /// character sprite shown on the story-select screen
    #[serde(default)]
    pub cover: Option<String>,
    #[serde(default = "default_cover_mood")]
    pub cover_mood: Mood,
    /// mood for scenes that neither set one nor match a keyword
    #[serde(default = "default_mood")]
    pub default_mood: Mood,
    /// how scenes come on screen when they don't say (dissolve, fade, sweep…)
    #[serde(default)]
    pub transition: Transition,
    /// background loop for every scene that doesn't choose its own
    #[serde(default)]
    pub ambience: Option<AmbienceRef>,
    /// background loop by mood: `[ambience_moods] danger = "drone"`
    #[serde(default)]
    pub ambience_moods: BTreeMap<Mood, AmbienceRef>,
    #[serde(default)]
    pub speakers: BTreeMap<String, Speaker>,
    /// per-mood color overrides: `[themes.danger] accent = "#ff2a6d"`
    #[serde(default)]
    pub themes: BTreeMap<Mood, ThemeOverride>,
}

fn default_mood() -> Mood {
    Mood::Noir
}

fn default_cover_mood() -> Mood {
    Mood::Mystery
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Speaker {
    pub name: String,
    /// file name (without .toml) in this story's characters/ folder
    #[serde(default)]
    pub sprite: Option<String>,
    /// inner voice: drawn in italics, in parentheses
    #[serde(default)]
    pub thought: bool,
    /// name tag color, "#rrggbb" (defaults to the scene's accent)
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeOverride {
    pub bg: Option<String>,
    pub panel: Option<String>,
    pub border: Option<String>,
    pub text: Option<String>,
    pub dim: Option<String>,
    pub accent: Option<String>,
    pub mote: Option<String>,
}

impl ThemeOverride {
    /// Parsed colors in the order bg, panel, border, text, dim, accent, mote.
    pub fn colors(&self) -> [Option<Rgb>; 7] {
        [
            &self.bg,
            &self.panel,
            &self.border,
            &self.text,
            &self.dim,
            &self.accent,
            &self.mote,
        ]
        .map(|c| c.as_deref().and_then(|s| parse_hex(s).ok()))
    }
}

pub struct StoryPack {
    /// folder name, used as the story id (and for save files later)
    pub id: String,
    pub dir: PathBuf,
    pub meta: Meta,
    pub scenes: BTreeMap<String, Scene>,
    pub sprites: BTreeMap<String, Sprite>,
}

impl Meta {
    pub fn load(dir: &Path) -> Result<Self> {
        let path = dir.join("story.toml");
        let src = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        toml::from_str(&src).with_context(|| format!("parsing {}", path.display()))
    }
}

impl StoryPack {
    pub fn load(dir: &Path) -> Result<Self> {
        let id = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("story")
            .to_string();
        let meta = Meta::load(dir)?;

        let mut scenes = BTreeMap::new();
        for path in toml_files(&dir.join("scenes"))? {
            let src = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let file: BTreeMap<String, Scene> =
                toml::from_str(&src).with_context(|| format!("parsing {}", path.display()))?;
            for (sid, scene) in file {
                if scenes.insert(sid.clone(), scene).is_some() {
                    bail!(
                        "scene '{sid}' is defined twice (again in {})",
                        path.display()
                    );
                }
            }
        }

        let mut sprites = BTreeMap::new();
        let wanted = meta
            .speakers
            .values()
            .filter_map(|s| s.sprite.as_ref())
            .chain(meta.cover.as_ref());
        for name in wanted {
            if !sprites.contains_key(name) {
                let path = dir.join("characters").join(format!("{name}.toml"));
                sprites.insert(name.clone(), Sprite::load(&path)?);
            }
        }

        let pack = StoryPack {
            id,
            dir: dir.to_path_buf(),
            meta,
            scenes,
            sprites,
        };
        pack.validate()
            .with_context(|| format!("story '{}'", pack.id))?;
        Ok(pack)
    }

    fn validate(&self) -> Result<()> {
        let m = &self.meta;
        if !self.scenes.contains_key(&m.start) {
            bail!("start scene '{}' does not exist", m.start);
        }
        for (id, s) in &m.speakers {
            if let Some(c) = &s.color {
                parse_hex(c).with_context(|| format!("speaker '{id}'"))?;
            }
        }
        if let Some(p) = &m.player
            && !m.speakers.contains_key(p)
        {
            bail!("player '{p}' is not a speaker");
        }
        // audio files must exist and be a kind the game can play
        let audio_file = |what: &str, file: &str| -> Result<()> {
            let path = self.dir.join(file);
            if !path.is_file() {
                bail!("{what}: sound '{file}' not found at {}", path.display());
            }
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if !AUDIO_EXTENSIONS.contains(&ext.as_str()) {
                bail!(
                    "{what}: sound '{file}' must be one of .{}",
                    AUDIO_EXTENSIONS.join(", .")
                );
            }
            Ok(())
        };
        if let Some(Audio::File(f)) = &m.ambience {
            audio_file("story ambience", f)?;
        }
        for (mood, a) in &m.ambience_moods {
            if let Audio::File(f) = a {
                audio_file(&format!("ambience for {mood:?}"), f)?;
            }
        }
        for (id, scene) in &self.scenes {
            if let Some(Audio::File(f)) = &scene.ambience {
                audio_file(&format!("scene '{id}' ambience"), f)?;
            }
            if let Some(Audio::File(f)) = &scene.sound {
                audio_file(&format!("scene '{id}'"), f)?;
            }
            for (n, line) in scene.dialogue.iter().enumerate() {
                if let Some(Audio::File(f)) = &line.sound {
                    audio_file(&format!("scene '{id}' line {}", n + 1), f)?;
                }
            }
            if let Some(img) = &scene.image {
                let path = self.dir.join(img);
                if !path.is_file() {
                    bail!(
                        "scene '{id}': image '{img}' not found at {}",
                        path.display()
                    );
                }
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "svg") {
                    bail!("scene '{id}': image '{img}' must be a .png, .jpg or .svg");
                }
            }
            // story logic: conditions must parse and {placeholders} be well formed
            let ctx = |what: String| {
                move |e: anyhow::Error| anyhow::anyhow!("scene '{id}': {what}: {e:#}")
            };
            logic::check_text(&scene.text).map_err(ctx("text".into()))?;
            for (n, v) in scene.variants.iter().enumerate() {
                logic::parse(&v.cond).map_err(ctx(format!("variant {}", n + 1)))?;
                logic::check_text(&v.text).map_err(ctx(format!("variant {} text", n + 1)))?;
            }
            for (n, line) in scene.dialogue.iter().enumerate() {
                if let Some(c) = &line.cond {
                    logic::parse(c).map_err(ctx(format!("line {}", n + 1)))?;
                }
                logic::check_text(&line.line).map_err(ctx(format!("line {}", n + 1)))?;
            }
            for c in &scene.choices {
                if let Some(cond) = &c.cond {
                    logic::parse(cond).map_err(ctx(format!("choice '{}'", c.text)))?;
                }
            }
            for c in &scene.choices {
                if !self.scenes.contains_key(&c.goto) {
                    bail!(
                        "scene '{id}': choice '{}' goes to missing scene '{}'",
                        c.text,
                        c.goto
                    );
                }
            }
            for line in &scene.dialogue {
                let Some(speaker) = m.speakers.get(&line.who) else {
                    bail!(
                        "scene '{id}': unknown speaker '{}' (add it to [speakers])",
                        line.who
                    );
                };
                if let (Some(mood), Some(sprite)) = (
                    &line.mood,
                    speaker.sprite.as_ref().map(|s| &self.sprites[s]),
                ) && !sprite.has_expression(mood)
                {
                    let all: Vec<_> = sprite.expressions().collect();
                    bail!(
                        "scene '{id}': '{}' has no expression '{mood}' (has: {})",
                        line.who,
                        all.join(", ")
                    );
                }
            }
        }
        Ok(())
    }

    /// Full path of a scene's image, if it has one.
    pub fn image_path(&self, scene_id: &str) -> Option<PathBuf> {
        self.scenes
            .get(scene_id)?
            .image
            .as_ref()
            .map(|i| self.dir.join(i))
    }

    pub fn speaker_name<'a>(&'a self, who: &'a str) -> &'a str {
        self.meta
            .speakers
            .get(who)
            .map(|s| s.name.as_str())
            .unwrap_or(who)
    }

    pub fn speaker(&self, who: &str) -> Option<&Speaker> {
        self.meta.speakers.get(who)
    }

    pub fn mood_of(&self, scene_id: &str) -> Mood {
        self.scenes[scene_id]
            .mood
            .unwrap_or_else(|| Mood::guess(scene_id, self.meta.default_mood))
    }

    /// The background loop for a scene: its own, else its mood's, else the story's.
    pub fn ambience_of(&self, scene_id: &str) -> Option<&AmbienceRef> {
        self.scenes[scene_id]
            .ambience
            .as_ref()
            .or_else(|| self.meta.ambience_moods.get(&self.mood_of(scene_id)))
            .or(self.meta.ambience.as_ref())
    }

    /// Where an audio file named in the story lives.
    pub fn audio_path(&self, file: &str) -> PathBuf {
        self.dir.join(file)
    }

    pub fn transition_of(&self, scene_id: &str) -> Transition {
        self.scenes[scene_id]
            .transition
            .unwrap_or(self.meta.transition)
    }

    /// Scenes no path from the start can reach (useful when writing).
    pub fn unreachable(&self) -> Vec<&str> {
        let mut seen = std::collections::BTreeSet::new();
        let mut todo = vec![self.meta.start.as_str()];
        while let Some(id) = todo.pop() {
            if seen.insert(id) {
                todo.extend(self.scenes[id].choices.iter().map(|c| c.goto.as_str()));
            }
        }
        self.scenes
            .keys()
            .map(String::as_str)
            .filter(|id| !seen.contains(id))
            .collect()
    }
}

fn toml_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect();
    files.sort();
    Ok(files)
}
