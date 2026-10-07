//! What the player has already seen across playthroughs: lines read (so
//! skip can fly past them) and endings reached (for the gallery).

use std::collections::HashSet;

use anyhow::Result;
use chrono::{DateTime, Local};
use rusqlite::params;

use crate::StoryPack;
use crate::store::Store;

/// Line index used for a scene's narration; dialogue lines are 0, 1, 2...
pub const NARRATION: i64 = -1;

/// An ending the player has reached.
#[derive(Debug, Clone)]
pub struct EndingRecord {
    pub scene: String,
    /// RFC 3339 local time
    pub first_reached: String,
    pub times: u32,
}

impl EndingRecord {
    /// "2026-10-08"
    pub fn date_label(&self) -> String {
        DateTime::parse_from_rfc3339(&self.first_reached)
            .map(|t| t.with_timezone(&Local).format("%Y-%m-%d").to_string())
            .unwrap_or_default()
    }
}

/// Lines read in one story, keyed by (scene, line).
pub type Seen = HashSet<(String, i64)>;

impl Store {
    pub fn mark_seen(&self, story_id: &str, scene: &str, line: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO seen (story_id, scene, line) VALUES (?1, ?2, ?3)",
            params![story_id, scene, line],
        )?;
        Ok(())
    }

    /// Everything read in a story, loaded once when a story starts.
    pub fn seen(&self, story_id: &str) -> Seen {
        let Ok(mut stmt) = self
            .conn
            .prepare("SELECT scene, line FROM seen WHERE story_id = ?1")
        else {
            return Seen::new();
        };
        stmt.query_map(params![story_id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        })
        .map(|rows| rows.filter_map(|r| r.ok()).collect())
        .unwrap_or_default()
    }

    /// Note that an ending was reached. Returns true the first time.
    pub fn record_ending(&self, story_id: &str, scene: &str) -> Result<bool> {
        let new = self.conn.execute(
            "INSERT OR IGNORE INTO endings (story_id, scene, first_reached, times) VALUES (?1, ?2, ?3, 1)",
            params![story_id, scene, Local::now().to_rfc3339()],
        )? == 1;
        if !new {
            self.conn.execute(
                "UPDATE endings SET times = times + 1 WHERE story_id = ?1 AND scene = ?2",
                params![story_id, scene],
            )?;
        }
        Ok(new)
    }

    /// Endings reached in a story, oldest first.
    pub fn endings(&self, story_id: &str) -> Vec<EndingRecord> {
        let Ok(mut stmt) = self.conn.prepare(
            "SELECT scene, first_reached, times FROM endings WHERE story_id = ?1 ORDER BY first_reached",
        ) else {
            return Vec::new();
        };
        stmt.query_map(params![story_id], |r| {
            Ok(EndingRecord {
                scene: r.get(0)?,
                first_reached: r.get(1)?,
                times: r.get(2)?,
            })
        })
        .map(|rows| rows.filter_map(|r| r.ok()).collect())
        .unwrap_or_default()
    }
}

impl StoryPack {
    /// Every ending scene of the story (marked `ending = true`, or with nowhere to go), sorted by id.
    pub fn ending_ids(&self) -> Vec<&str> {
        self.scenes
            .iter()
            .filter(|(_, s)| s.ending || s.choices.is_empty())
            .map(|(id, _)| id.as_str())
            .collect()
    }
}
