//! Save games, stored in the SQLite [`Store`]. Saves are keyed by story, so
//! stories never share or overwrite each other's slots.
//!
//! Slot 0 is the autosave (written on every scene); slots 1..=9 are the player's.
//! A save can be exported to / imported from a JSON file to move it between machines.

use std::path::Path;

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use rusqlite::{OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};

use crate::StoryPack;
use crate::library::Entry;
use crate::state::{ChoiceRecord, State};
use crate::store::Store;

pub const AUTOSAVE_SLOT: u8 = 0;
/// Slots 1..=MAX_SLOT are for the player.
pub const MAX_SLOT: u8 = 9;
/// Written by the quick-save key, read by quick load.
pub const QUICKSAVE_SLOT: u8 = 10;
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub slot: u8,
    pub name: String,
    pub story_id: String,
    pub story_title: String,
    /// RFC 3339 local time
    pub timestamp: String,
    pub game_version: String,
    pub scene: String,
    /// e.g. "SHADOW SLAVE - Dark Archway"
    pub scene_description: String,
    pub playtime_ms: u64,
    pub completion: f32,
    pub visited: usize,
    pub choices_made: usize,
}

impl Metadata {
    pub fn time(&self) -> Option<DateTime<Local>> {
        DateTime::parse_from_rfc3339(&self.timestamp)
            .ok()
            .map(|t| t.with_timezone(&Local))
    }

    /// "2026-10-08 21:14"
    pub fn time_label(&self) -> String {
        self.time()
            .map(|t| t.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| self.timestamp.clone())
    }
}

/// A save as exported to / imported from a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFile {
    pub metadata: Metadata,
    pub state: State,
}

/// "1h 05m" / "12m"
pub fn format_playtime(ms: u64) -> String {
    let mins = ms / 60_000;
    if mins >= 60 {
        format!("{}h {:02}m", mins / 60, mins % 60)
    } else {
        format!("{mins}m")
    }
}

fn title_case(id: &str) -> String {
    id.split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().chain(c).collect::<String>())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn check_slot(slot: u8) -> Result<()> {
    if slot > QUICKSAVE_SLOT {
        bail!("slot {slot} does not exist (0..={QUICKSAVE_SLOT})");
    }
    Ok(())
}

const COLUMNS: &str = "slot, name, story_id, story_title, timestamp, game_version, scene, scene_description, \
                       playtime_ms, completion, visited, choices_made";

fn metadata(r: &Row) -> rusqlite::Result<Metadata> {
    Ok(Metadata {
        slot: r.get(0)?,
        name: r.get(1)?,
        story_id: r.get(2)?,
        story_title: r.get(3)?,
        timestamp: r.get(4)?,
        game_version: r.get(5)?,
        scene: r.get(6)?,
        scene_description: r.get(7)?,
        playtime_ms: r.get::<_, i64>(8)? as u64,
        completion: r.get::<_, f64>(9)? as f32,
        visited: r.get::<_, i64>(10)? as usize,
        choices_made: r.get::<_, i64>(11)? as usize,
    })
}

impl Store {
    pub fn save(
        &self,
        pack: &StoryPack,
        slot: u8,
        state: &State,
        name: Option<&str>,
    ) -> Result<Metadata> {
        check_slot(slot)?;
        let default_name = if slot == QUICKSAVE_SLOT {
            "Quicksave".to_string()
        } else if slot == AUTOSAVE_SLOT {
            "Autosave".to_string()
        } else {
            format!("Save {slot}")
        };
        let name = name
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .map(str::to_string)
            .unwrap_or(default_name);
        let total = pack.scenes.len().max(1);
        let metadata = Metadata {
            slot,
            name,
            story_id: pack.id.clone(),
            story_title: pack.meta.title.clone(),
            timestamp: Local::now().to_rfc3339(),
            game_version: GAME_VERSION.to_string(),
            scene: state.current_scene.clone(),
            scene_description: format!(
                "{} - {}",
                pack.meta.title,
                title_case(&state.current_scene)
            ),
            playtime_ms: state.playtime_ms,
            completion: (state.visited.len() as f32 / total as f32 * 100.0).min(100.0),
            visited: state.visited.len(),
            choices_made: state.choices.len(),
        };
        self.put(&SaveFile {
            metadata: metadata.clone(),
            state: state.clone(),
        })?;
        Ok(metadata)
    }

    /// Write a save row as is (replacing whatever was in that story's slot).
    fn put(&self, f: &SaveFile) -> Result<()> {
        let m = &f.metadata;
        self.conn.execute(
            &format!("INSERT OR REPLACE INTO saves ({COLUMNS}, state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"),
            params![
                m.slot,
                m.name,
                m.story_id,
                m.story_title,
                m.timestamp,
                m.game_version,
                m.scene,
                m.scene_description,
                m.playtime_ms as i64,
                m.completion as f64,
                m.visited as i64,
                m.choices_made as i64,
                serde_json::to_string(&f.state)?,
            ],
        )?;
        Ok(())
    }

    pub fn load(&self, story_id: &str, slot: u8) -> Result<SaveFile> {
        let row = self
            .conn
            .query_row(
                &format!("SELECT {COLUMNS}, state FROM saves WHERE story_id = ?1 AND slot = ?2"),
                params![story_id, slot],
                |r| Ok((metadata(r)?, r.get::<_, String>(12)?)),
            )
            .optional()?;
        let Some((metadata, state)) = row else {
            bail!("{story_id} has nothing in slot {slot}")
        };
        let state: State = serde_json::from_str(&state)
            .with_context(|| format!("{story_id} slot {slot} is damaged"))?;
        let file = SaveFile { metadata, state };
        validate(&file)?;
        Ok(file)
    }

    pub fn delete(&self, story_id: &str, slot: u8) -> Result<()> {
        let n = self.conn.execute(
            "DELETE FROM saves WHERE story_id = ?1 AND slot = ?2",
            params![story_id, slot],
        )?;
        if n == 0 {
            bail!("{story_id} has nothing in slot {slot}");
        }
        Ok(())
    }

    /// Slots 0..=MAX_SLOT for a story; `None` where empty.
    pub fn slots(&self, story_id: &str) -> Vec<Option<Metadata>> {
        let mut out = vec![None; MAX_SLOT as usize + 1];
        for m in self.query(
            &format!("SELECT {COLUMNS} FROM saves WHERE story_id = ?1"),
            params![story_id],
        ) {
            if let Some(o) = out.get_mut(m.slot as usize) {
                *o = Some(m);
            }
        }
        out
    }

    /// Every save of every story, newest first.
    pub fn all(&self) -> Vec<Metadata> {
        let mut out = self.query(&format!("SELECT {COLUMNS} FROM saves"), params![]);
        // compare as times: timestamps may carry different UTC offsets
        out.sort_by_key(|m| std::cmp::Reverse(m.time()));
        out
    }

    /// The most recent save of any story (what "Continue" resumes).
    pub fn latest(&self) -> Option<Metadata> {
        self.all().into_iter().next()
    }

    fn query(&self, sql: &str, p: impl rusqlite::Params) -> Vec<Metadata> {
        let Ok(mut stmt) = self.conn.prepare(sql) else {
            return Vec::new();
        };
        stmt.query_map(p, metadata)
            .map(|rows| rows.filter_map(|r| r.ok()).collect())
            .unwrap_or_default()
    }

    /// Write a save to a JSON file.
    pub fn export(&self, story_id: &str, slot: u8, to: &Path) -> Result<()> {
        let file = self.load(story_id, slot)?;
        std::fs::write(to, serde_json::to_string_pretty(&file)?)
            .with_context(|| format!("writing {}", to.display()))
    }

    /// Put a save file into a slot of the story it belongs to. Returns its metadata.
    pub fn import(&self, from: &Path, slot: u8) -> Result<Metadata> {
        check_slot(slot)?;
        let text =
            std::fs::read_to_string(from).with_context(|| format!("reading {}", from.display()))?;
        let mut file: SaveFile = serde_json::from_str(&text)
            .with_context(|| format!("{} is not a Terminal Theatre save", from.display()))?;
        validate(&file)?;
        file.metadata.slot = slot;
        self.put(&file)?;
        Ok(file.metadata)
    }

    /// Bring saves from the Python version (`<data dir>/saves/*.json`) into the
    /// database, once. Each is matched to an installed story by title and keeps
    /// its slot. Returns how many were imported; the old files are left alone.
    pub fn import_python_saves(&self, data_dir: &Path, stories: &[Entry]) -> Result<usize> {
        const DONE: &str = "python_saves_imported";
        if self.get(DONE)?.is_some() {
            return Ok(0);
        }
        let mut count = 0;
        let files = std::fs::read_dir(data_dir.join("saves"))
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok());
        for path in files
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
        {
            if let Some(file) = std::fs::read_to_string(&path)
                .ok()
                .and_then(|t| serde_json::from_str::<PySave>(&t).ok())
                .and_then(|py| py.convert(stories))
            {
                // never overwrite a save made in this version
                if self
                    .load(&file.metadata.story_id, file.metadata.slot)
                    .is_err()
                {
                    self.put(&file)?;
                    count += 1;
                }
            }
        }
        self.set(DONE, &count.to_string())?;
        Ok(count)
    }
}

fn validate(file: &SaveFile) -> Result<()> {
    if !compatible(&file.metadata.game_version) {
        bail!(
            "save from version {} can't be loaded by {GAME_VERSION}",
            file.metadata.game_version
        );
    }
    if file.state.current_scene.is_empty() {
        bail!("save \"{}\" has no current scene", file.metadata.name);
    }
    Ok(())
}

/// Same major version (or, before 1.0, same minor version).
fn compatible(saved: &str) -> bool {
    let key = |v: &str| {
        let mut parts = v.split('.').map(|p| p.parse::<u32>().unwrap_or(u32::MAX));
        let (major, minor) = (
            parts.next().unwrap_or(u32::MAX),
            parts.next().unwrap_or(u32::MAX),
        );
        if major == 0 { (0, minor) } else { (major, 0) }
    };
    key(saved) == key(GAME_VERSION)
}

/// A save file written by the Python version.
#[derive(Deserialize)]
struct PySave {
    metadata: PyMeta,
    game_state: PyState,
}

#[derive(Deserialize)]
struct PyMeta {
    slot: u8,
    save_name: String,
    timestamp: String,
    #[serde(default)]
    completion_percentage: f32,
}

#[derive(Deserialize)]
struct PyState {
    current_scene: String,
    #[serde(default)]
    visited_scenes: Vec<String>,
    #[serde(default)]
    flags: std::collections::BTreeMap<String, bool>,
    #[serde(default)]
    inventory: Vec<String>,
    #[serde(default)]
    choice_history: Vec<(String, usize, String)>,
    #[serde(default)]
    playtime: f64,
    story_title: String,
}

impl PySave {
    fn convert(self, stories: &[Entry]) -> Option<SaveFile> {
        let (g, m) = (self.game_state, self.metadata);
        let story = stories
            .iter()
            .find(|e| e.meta.title.eq_ignore_ascii_case(&g.story_title))?;
        if m.slot > MAX_SLOT || g.current_scene.is_empty() {
            return None;
        }
        let timestamp = NaiveDateTime::parse_from_str(&m.timestamp, "%Y-%m-%dT%H:%M:%S%.f")
            .ok()
            .and_then(|t| Local.from_local_datetime(&t).single())
            .unwrap_or_else(Local::now)
            .to_rfc3339();
        let state = State {
            current_scene: g.current_scene.clone(),
            flags: g
                .flags
                .into_iter()
                .filter(|(_, on)| *on)
                .map(|(k, _)| k)
                .collect(),
            items: g.inventory,
            visited: g.visited_scenes,
            choices: g
                .choice_history
                .into_iter()
                .map(|(scene, index, text)| ChoiceRecord { scene, index, text })
                .collect(),
            playtime_ms: (g.playtime * 1000.0) as u64,
        };
        let metadata = Metadata {
            slot: m.slot,
            name: m.save_name,
            story_id: story.id.clone(),
            story_title: story.meta.title.clone(),
            timestamp,
            game_version: GAME_VERSION.to_string(),
            scene: g.current_scene.clone(),
            scene_description: format!("{} - {}", story.meta.title, title_case(&g.current_scene)),
            playtime_ms: state.playtime_ms,
            completion: m.completion_percentage,
            visited: state.visited.len(),
            choices_made: state.choices.len(),
        };
        Some(SaveFile { metadata, state })
    }
}
