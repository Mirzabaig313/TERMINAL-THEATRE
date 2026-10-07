//! The SQLite database behind saves and settings: `<data dir>/theatre.db`.
//!
//! Tables:
//! - `saves`: one row per (story, slot); the playthrough state is a JSON column
//! - `kv`: small key/value pairs (settings, one-time migration markers)
//! - `seen`: which narration and dialogue lines the player has read (for skip)
//! - `endings`: which endings the player has reached (for the gallery)
//!
//! The schema version is kept in `PRAGMA user_version`; [`MIGRATIONS`] brings an
//! older database up to date when it is opened.

use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension, params};

pub const DB_FILE: &str = "theatre.db";

/// Applied in order; entry `i` moves the schema from version `i` to `i + 1`.
const MIGRATIONS: &[&str] = &[
    "
    CREATE TABLE saves (
        story_id          TEXT    NOT NULL,
        slot              INTEGER NOT NULL,
        name              TEXT    NOT NULL,
        story_title       TEXT    NOT NULL,
        timestamp         TEXT    NOT NULL,
        game_version      TEXT    NOT NULL,
        scene             TEXT    NOT NULL,
        scene_description TEXT    NOT NULL,
        playtime_ms       INTEGER NOT NULL,
        completion        REAL    NOT NULL,
        visited           INTEGER NOT NULL,
        choices_made      INTEGER NOT NULL,
        state             TEXT    NOT NULL,
        PRIMARY KEY (story_id, slot)
    );
    CREATE INDEX saves_by_time ON saves (timestamp);
    CREATE TABLE kv (
        key   TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );
",
    "
    -- text the player has read: line -1 is a scene's narration, 0.. its dialogue lines
    CREATE TABLE seen (
        story_id TEXT    NOT NULL,
        scene    TEXT    NOT NULL,
        line     INTEGER NOT NULL,
        PRIMARY KEY (story_id, scene, line)
    );
    -- endings the player has reached
    CREATE TABLE endings (
        story_id      TEXT    NOT NULL,
        scene         TEXT    NOT NULL,
        first_reached TEXT    NOT NULL,
        times         INTEGER NOT NULL,
        PRIMARY KEY (story_id, scene)
    );
",
];

pub struct Store {
    pub(crate) conn: Connection,
}

impl Store {
    /// Open (creating if needed) the database in `data_dir`.
    pub fn open(data_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(data_dir)
            .with_context(|| format!("creating {}", data_dir.display()))?;
        let path = data_dir.join(DB_FILE);
        let conn =
            Connection::open(&path).with_context(|| format!("opening {}", path.display()))?;
        Self::init(conn)
    }

    /// A throwaway database, for tests.
    pub fn in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Self> {
        // two games open at once wait for each other instead of failing
        conn.busy_timeout(Duration::from_secs(3))?;
        conn.pragma_update(None, "journal_mode", "WAL").ok();
        let mut store = Store { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&mut self) -> Result<()> {
        let version: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))?;
        for (i, sql) in MIGRATIONS.iter().enumerate().skip(version as usize) {
            let tx = self.conn.transaction()?;
            tx.execute_batch(sql)
                .with_context(|| format!("database migration {}", i + 1))?;
            tx.pragma_update(None, "user_version", i as i64 + 1)?;
            tx.commit()?;
        }
        Ok(())
    }

    /// Current schema version (number of migrations applied).
    pub fn schema_version(&self) -> Result<usize> {
        let v: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))?;
        Ok(v as usize)
    }

    /// Number of migrations this build knows; an up-to-date database has this version.
    pub fn latest_schema_version() -> usize {
        MIGRATIONS.len()
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row("SELECT value FROM kv WHERE key = ?1", params![key], |r| {
                r.get(0)
            })
            .optional()?)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO kv (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}
