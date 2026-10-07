//! Screens. Each one handles its own keys and drawing and tells the app
//! where to go next through [`Action`].

pub mod backlog;
pub mod cinematic;
pub mod credits;
pub mod load;
pub mod main_menu;
pub mod menu;
pub mod opening;
pub mod play;
pub mod settings;

use std::path::PathBuf;

use anyhow::Result;
use theatre_engine::Store;
use theatre_engine::library;
use theatre_engine::settings::Settings;
use theatre_engine::state::State;

/// Shared by every screen.
pub struct Ctx {
    /// folder holding the story folders
    pub stories: PathBuf,
    /// where the database lives
    pub data: PathBuf,
    pub settings: Settings,
    /// saves and settings (SQLite)
    pub store: Store,
    /// the terminal shows 24-bit color; otherwise colors are reduced to 256
    pub truecolor: bool,
}

/// Whether the terminal handles 24-bit color. `THEATRE_COLOR=256` or
/// `THEATRE_COLOR=truecolor` overrides the guess.
pub fn detect_truecolor() -> bool {
    let var = |k: &str| std::env::var(k).unwrap_or_default().to_lowercase();
    match var("THEATRE_COLOR").as_str() {
        "256" => return false,
        "truecolor" | "24bit" => return true,
        _ => {}
    }
    let colorterm = var("COLORTERM");
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return true;
    }
    // terminals known to support it even when COLORTERM isn't set
    let program = var("TERM_PROGRAM");
    let known = [
        "iterm.app",
        "wezterm",
        "vscode",
        "ghostty",
        "hyper",
        "tabby",
        "kiro",
    ];
    known.contains(&program.as_str())
        || std::env::var("WT_SESSION").is_ok()
        || var("TERM").contains("kitty")
        || var("TERM").contains("direct")
}

impl Ctx {
    /// Open the database in `data`, bringing over saves from the Python version
    /// the first time.
    pub fn new(stories: PathBuf, data: PathBuf) -> Result<Self> {
        let store = Store::open(&data)?;
        if let Ok((entries, _)) = library::discover(&stories) {
            store.import_python_saves(&data, &entries)?;
        }
        Ok(Ctx {
            settings: store.settings(),
            store,
            stories,
            data,
            truecolor: detect_truecolor(),
        })
    }

    /// Persist settings; failing to write them is not worth interrupting play.
    pub fn save_settings(&self) {
        let _ = self.store.save_settings(&self.settings);
    }
}

pub enum Go {
    MainMenu,
    StorySelect,
    Cinematic,
    Settings,
    Credits,
    Load,
    /// start the story in this folder from the beginning
    NewGame(PathBuf),
    /// continue the story in this folder from a saved state
    Resume(PathBuf, Box<State>, String),
}

pub enum Action {
    Stay,
    Quit,
    Go(Go),
}

impl From<Go> for Action {
    fn from(g: Go) -> Self {
        Action::Go(g)
    }
}
