//! Screen router: owns the shared context and the current screen, and switches
//! screens on their [`Action`]s.

use std::path::PathBuf;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use theatre_engine::StoryPack;

use crate::audio::{Cue, menu_sound};
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
    MainMenu(Box<MainMenu>),
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
    /// try out a story while writing it, from this state (`theatre rehearse`)
    Rehearse(String, Box<theatre_engine::state::State>),
}

pub struct App {
    ctx: Ctx,
    screen: Screen,
    t: u64,
    /// last key press or screen change, for the frame rate
    last_input: u64,
    pub quit: bool,
}

impl App {
    pub fn new(ctx: Ctx, start: Start) -> Self {
        let screen = Screen::Opening(Opening::new(0, seed()));
        let mut app = App {
            last_input: 0,
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
            Start::Rehearse(id, state) => {
                let dir = app.ctx.stories.join(id);
                app.screen = match app.open(&dir, Some((*state, "Rehearsal".into()))) {
                    Screen::Play(p) => Screen::Play(Box::new(p.rehearse(dir))),
                    other => other,
                };
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

    /// Draw at full frame rate? True right after input and while the current
    /// screen animates quickly; otherwise the main loop slows to the idle rate,
    /// which keeps terminals (especially editor terminals) responsive.
    pub fn busy(&self) -> bool {
        let now = self.t;
        now.saturating_sub(self.last_input) < 1000
            || match &self.screen {
                Screen::Opening(s) => s.busy(now),
                Screen::Cinematic(_) => true,
                Screen::Play(p) => p.busy(now),
                _ => false,
            }
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
        // sound follows the settings; background loops only play in a story
        self.ctx.audio.set_gain(self.ctx.settings.gain());
        if !matches!(self.screen, Screen::Play(_)) {
            self.ctx.audio.set_ambience(None);
        }
        self.apply(action);
    }

    pub fn key(&mut self, k: KeyEvent) {
        if k.code == KeyCode::Char('c') && k.modifiers.contains(KeyModifiers::CONTROL) {
            self.quit = true;
            return;
        }
        let now = self.t;
        self.last_input = now;
        self.menu_sound(k);
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

    /// Soft clicks for moving through menus, choosing and going back. Stories
    /// make their own sounds; the title sequence and cinematic stay silent.
    fn menu_sound(&self, k: KeyEvent) {
        if matches!(
            self.screen,
            Screen::Opening(_) | Screen::Cinematic(_) | Screen::Play(_)
        ) {
            return;
        }
        let Some(ui) = menu_sound(k.code) else {
            return;
        };
        self.ctx.audio.play(Cue::Ui(ui));
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
        self.last_input = now;
        self.screen = match g {
            Go::MainMenu => Screen::MainMenu(Box::new(MainMenu::new(now, &self.ctx))),
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
            Screen::MainMenu(s) => s.draw(f, now),
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
