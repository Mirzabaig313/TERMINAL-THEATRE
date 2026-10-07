//! Screen router: owns the shared context and the current screen, and switches
//! screens on their [`Action`]s.

use std::path::PathBuf;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use theatre_engine::StoryPack;

use crate::render::fx;
use crate::screens::cinematic::Cinematic;
use crate::screens::credits::Credits;
use crate::screens::load::Load;
use crate::screens::main_menu::MainMenu;
use crate::screens::menu::Menu;
use crate::screens::opening::Opening;
use crate::screens::play::Play;
use crate::screens::settings::SettingsScreen;
use crate::screens::{Action, Ctx, Go};

enum Screen {
    Opening(Opening),
    MainMenu(MainMenu),
    Cinematic(Cinematic),
    StorySelect(Box<Menu>),
    Settings(SettingsScreen),
    Credits(Credits),
    Load(Load),
    Play(Box<Play>),
}

/// Where the app starts.
pub enum Start {
    Opening,
    MainMenu,
    Story(String),
}

pub struct App {
    ctx: Ctx,
    screen: Screen,
    t: u64,
    pub quit: bool,
}

impl App {
    pub fn new(ctx: Ctx, start: Start) -> Self {
        let screen = Screen::Opening(Opening::new(0, seed()));
        let mut app = App {
            ctx,
            screen,
            t: 0,
            quit: false,
        };
        match start {
            Start::Opening => {}
            Start::MainMenu => app.go(Go::MainMenu),
            Start::Story(id) => {
                let dir = app.ctx.stories.join(id);
                app.go(Go::NewGame(dir));
            }
        }
        app
    }

    /// Shared state: settings, saves, folders.
    pub fn ctx(&self) -> &Ctx {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut Ctx {
        &mut self.ctx
    }

    pub fn tick(&mut self, now: u64) {
        let dt = now.saturating_sub(self.t) as f32 / 1000.0;
        self.t = now;
        let action = match &mut self.screen {
            Screen::Opening(s) => {
                s.tick(dt);
                Action::Stay
            }
            Screen::MainMenu(s) => {
                s.tick(dt);
                Action::Stay
            }
            Screen::Cinematic(s) => s.tick(now),
            Screen::StorySelect(s) => {
                s.tick(now, dt);
                Action::Stay
            }
            Screen::Play(p) => {
                p.tick(now, dt, &self.ctx);
                Action::Stay
            }
            Screen::Settings(_) | Screen::Credits(_) | Screen::Load(_) => Action::Stay,
        };
        self.apply(action);
    }

    pub fn key(&mut self, k: KeyEvent) {
        if k.code == KeyCode::Char('c') && k.modifiers.contains(KeyModifiers::CONTROL) {
            self.quit = true;
            return;
        }
        let now = self.t;
        let action = match &mut self.screen {
            Screen::Opening(s) => s.key(k, now),
            Screen::MainMenu(s) => s.key(k, &self.ctx, now),
            Screen::Cinematic(s) => s.key(k, now),
            Screen::StorySelect(s) => s.key(k, now),
            Screen::Settings(s) => s.key(k, &mut self.ctx, now),
            Screen::Credits(s) => s.key(k),
            Screen::Load(s) => s.key(k, &self.ctx, now),
            Screen::Play(p) => p.key(k, &self.ctx, now),
        };
        self.apply(action);
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Stay => {}
            Action::Quit => self.quit = true,
            Action::Go(g) => self.go(g),
        }
    }

    fn go(&mut self, g: Go) {
        let now = self.t;
        self.screen = match g {
            Go::MainMenu => Screen::MainMenu(MainMenu::new(now)),
            Go::StorySelect => Screen::StorySelect(Box::new(Menu::new(
                &self.ctx.stories,
                &self.ctx.store,
                now,
                None,
            ))),
            Go::Cinematic => Screen::Cinematic(Cinematic::new(now)),
            Go::Settings => Screen::Settings(SettingsScreen::new()),
            Go::Credits => Screen::Credits(Credits::new(now)),
            Go::Load => Screen::Load(Load::new(&self.ctx)),
            Go::NewGame(dir) => self.open(&dir, None),
            Go::Resume(dir, state, name) => self.open(&dir, Some((*state, name))),
        };
    }

    fn open(
        &self,
        dir: &std::path::Path,
        resume: Option<(theatre_engine::state::State, String)>,
    ) -> Screen {
        match StoryPack::load(dir) {
            Ok(pack) => Screen::Play(Box::new(Play::new(pack, self.t, resume, &self.ctx))),
            // a broken story never crashes the game; it is reported on the story list
            Err(e) => Screen::StorySelect(Box::new(Menu::new(
                &self.ctx.stories,
                &self.ctx.store,
                self.t,
                Some(format!("{e:#}")),
            ))),
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let now = self.t;
        match &mut self.screen {
            Screen::Opening(s) => s.draw(f, now),
            Screen::MainMenu(s) => s.draw(f, &self.ctx, now),
            Screen::Cinematic(s) => s.draw(f, now),
            Screen::StorySelect(s) => s.draw(f, now),
            Screen::Settings(s) => s.draw(f, &self.ctx, now),
            Screen::Credits(s) => s.draw(f, now),
            Screen::Load(s) => s.draw(f, now),
            Screen::Play(p) => p.draw(f, &self.ctx, now),
        }
        let area = f.area();
        if !self.ctx.settings.color {
            fx::grayscale(f.buffer_mut(), area);
        }
        if !self.ctx.truecolor {
            fx::downgrade_256(f.buffer_mut(), area);
        }
    }
}

fn seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(7)
}

pub fn default_stories_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("THEATRE_STORIES") {
        return dir.into();
    }
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("stories")))
        && dir.is_dir()
    {
        return dir;
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../stories")
}
