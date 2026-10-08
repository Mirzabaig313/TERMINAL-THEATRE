//! Playing one story: stage, portraits, dialogue, choices and screen effects,
//! plus the story intro, pause menu, history, saving and autosave.
//! Story flow lives in the engine's Runner; this screen only presents it.
//!
//! - `mod.rs`: the screen's state, setup, per-frame update, key routing, story flow
//! - `modes.rs`: auto and skip, read-text tracking, quick save/load
//! - `overlays.rs`: intro card, pause menu, save dialog, history (keys and drawing)
//! - `stage.rs`: drawing the scene itself

mod modes;
mod overlays;
mod stage;

use super::backlog::{Backlog, Entry};
use super::{Action, Ctx, Go};
use crate::render::fx::{self, Particles};
use crate::render::text::{continue_hint, pretty, shimmer, typed, wrapped_height};
use crate::render::ui::{Toast, centered, modal, option_line};
use crate::render::{Theme, rgb, sprite};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use theatre_engine::color::{lerp, parse_hex, scale};
use theatre_engine::progress::{NARRATION, Seen};
use theatre_engine::runner::{Phase, Runner, revealed};
use theatre_engine::save::{AUTOSAVE_SLOT, MAX_SLOT, Metadata, QUICKSAVE_SLOT, format_playtime};
use theatre_engine::scene::{LineFx, Mood};
use theatre_engine::settings::Effects;
use theatre_engine::sprite::Actor;
use theatre_engine::state::State;
use theatre_engine::{Rng, StoryPack};

/// How the story moves on by itself.
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    /// waits for the player
    Normal,
    /// moves on after a pause long enough to read the line; waits at choices
    Auto,
    /// races through text already read; stops at unread text and choices
    Skip,
}

/// Something drawn over the story that takes the keys.
enum Overlay {
    None,
    /// title card before the first scene; Some(save name) when resuming
    Intro {
        resumed: Option<String>,
        since: u64,
    },
    Pause {
        selected: usize,
    },
    ConfirmLeave,
    SaveSlots {
        selected: usize,
    },
    SaveName {
        slot: u8,
        text: String,
    },
    /// rows scrolled up from the newest text
    Backlog {
        scroll: usize,
    },
}

pub struct Play {
    runner: Runner,
    overlay: Overlay,
    actor: Option<Actor>,
    actor_since: u64,
    fx_kind: Option<LineFx>,
    fx_start: u64,
    glitch_until: u64,
    toast: Option<Toast>,
    rng: Rng,
    particles: Particles,
    mode: Mode,
    /// when the current text finished typing (for auto mode)
    done_at: Option<u64>,
    last_skip: u64,
    /// lines read in any earlier session, plus this one
    seen: Seen,
    backlog: Backlog,
    /// save slots, read when the save dialog opens
    slots: Vec<Option<Metadata>>,
    /// set when an ending is reached: (first time?, endings found, endings in the story)
    ending: Option<(bool, usize, usize)>,
}

impl Play {
    /// A new game, or a saved one when `resume` holds its state and save name.
    pub fn new(pack: StoryPack, now: u64, resume: Option<(State, String)>, ctx: &Ctx) -> Self {
        let seen = ctx.store.seen(&pack.id);
        let mut rng = Rng::new(0x5EED_CAFE ^ now);
        let particles = Particles::new(70, &mut rng);
        let (runner, resumed) = match resume {
            Some((state, name)) => (Runner::resume(pack, state, now), Some(name)),
            None => (Runner::new(pack, now), None),
        };
        Play {
            runner,
            overlay: Overlay::Intro {
                resumed,
                since: now,
            },
            actor: None,
            actor_since: now,
            fx_kind: None,
            fx_start: 0,
            glitch_until: 0,
            toast: None,
            rng,
            particles,
            mode: Mode::Normal,
            done_at: None,
            last_skip: 0,
            seen,
            backlog: Backlog::default(),
            slots: Vec::new(),
            ending: None,
        }
    }

    /// Something on screen moves quickly right now (typing, a transition, an
    /// effect, auto/skip), so the app should draw at full frame rate.
    pub fn busy(&self, now: u64) -> bool {
        let recent = |t: u64, ms: u64| now.saturating_sub(t) < ms;
        if let Overlay::Intro { resumed, since } = &self.overlay {
            // the title card is busy while it fades in and types out
            return now.saturating_sub(*since) < 1000
                || self.intro_shown(resumed.as_deref(), *since, now) != usize::MAX;
        }
        !self.runner.typing_done(now)
            || recent(self.runner.scene_start(), 1000)
            || recent(self.runner.phase_start(), 800)
            || recent(self.actor_since, 600)
            || recent(self.fx_start, 600)
            || self.mode != Mode::Normal
    }

    pub fn tick(&mut self, now: u64, dt: f32, ctx: &Ctx) {
        self.runner.speed = ctx.settings.speed();
        self.particles.update(dt);
        if matches!(self.overlay, Overlay::None) {
            self.runner.state.playtime_ms += (dt * 1000.0) as u64;
        }
        let talking =
            matches!(self.runner.phase(), Phase::Line(_)) && !self.runner.typing_done(now);
        if let Some(actor) = &mut self.actor
            && let Some(sprite) = self.runner.pack.sprites.get(&actor.sprite)
        {
            actor.talking = talking;
            actor.update(sprite, now, &mut self.rng);
        }
        if matches!(self.overlay, Overlay::None) {
            self.run_mode(ctx, now);
        }
        // dangerous scenes flicker on their own now and then (full effects only)
        if ctx.settings.effects == Effects::Full
            && self.runner.mood() == Mood::Danger
            && now > self.glitch_until + 2500
            && self.rng.chance(0.01)
        {
            self.glitch_until = now + 120;
        }
    }

    pub fn key(&mut self, k: KeyEvent, ctx: &Ctx, now: u64) -> Action {
        if let Some(action) = self.overlay_key(k, ctx, now) {
            return action;
        }

        let choosing = self.runner.phase() == Phase::Choose;
        // any other key takes control back from auto/skip; continue keys do only that
        let mode_key = matches!(k.code, KeyCode::Char('a') | KeyCode::Char('s'));
        if !mode_key && self.mode != Mode::Normal {
            self.mode = Mode::Normal;
            if matches!(k.code, KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right) {
                return Action::Stay;
            }
        }
        match k.code {
            KeyCode::Esc | KeyCode::Char('p') => self.overlay = Overlay::Pause { selected: 0 },
            KeyCode::Char('q') => return Action::Quit,
            KeyCode::Char('h') | KeyCode::PageUp => self.overlay = Overlay::Backlog { scroll: 0 },
            KeyCode::Char('a') => self.toggle_mode(Mode::Auto, now),
            KeyCode::Char('s') => self.toggle_mode(Mode::Skip, now),
            KeyCode::F(5) => self.quick_save(ctx, now),
            KeyCode::F(9) => self.quick_load(ctx, now),
            KeyCode::Up | KeyCode::Char('k') if choosing => self.runner.select_prev(),
            KeyCode::Down | KeyCode::Char('j') if choosing => self.runner.select_next(),
            KeyCode::Char(c @ '1'..='9') if choosing => {
                let i = c as usize - '1' as usize;
                if i < self.runner.choices().len() {
                    self.runner.select(i);
                    self.advance(ctx, now);
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right => {
                if self.runner.phase() == Phase::End && self.runner.typing_done(now) {
                    return Go::StorySelect.into();
                }
                self.advance(ctx, now);
            }
            _ => {}
        }
        Action::Stay
    }

    fn advance(&mut self, ctx: &Ctx, now: u64) {
        let leaving = self
            .runner
            .typing_done(now)
            .then(|| self.read_key())
            .flatten();
        if let Some(phase) = self.runner.advance(now) {
            if let Some(key) = leaving {
                self.remember(ctx, key);
            }
            self.done_at = None;
            self.on_phase(phase, now);
            match phase {
                Phase::Narration => self.autosave(ctx, now),
                Phase::End => self.reach_ending(ctx),
                _ => {}
            }
        }
    }

    /// Bring the right character on stage when the story moves on.
    fn on_phase(&mut self, phase: Phase, now: u64) {
        let pack = &self.runner.pack;
        let wanted = match phase {
            Phase::Line(_) => {
                let line = self.runner.line().expect("line phase has a line");
                self.fx_kind = line.fx;
                self.fx_start = now;
                let sprite = pack.speaker(&line.who).and_then(|s| s.sprite.clone());
                sprite.map(|s| (s, line.mood.clone().unwrap_or_else(|| "neutral".into())))
            }
            // the player's own character waits for the decision, keeping their expression
            Phase::Choose => {
                let player = pack
                    .meta
                    .player
                    .as_ref()
                    .and_then(|p| pack.speaker(p))
                    .and_then(|s| s.sprite.clone());
                match (&self.actor, player) {
                    (Some(a), Some(p)) if a.sprite == p => return,
                    (_, p) => p.map(|p| (p, "neutral".to_string())),
                }
            }
            Phase::Narration | Phase::End => None,
        };
        match (&mut self.actor, wanted) {
            (Some(a), Some((sprite, mood))) if a.sprite == sprite => a.pose.expression = mood,
            (_, Some((sprite, mood))) => {
                self.actor = Some(Actor::new(&sprite, &mood));
                self.actor_since = now;
            }
            (_, None) if phase == Phase::Narration => self.actor = None,
            _ => {}
        }
    }

    // ------------------------------------------------------------------ draw

    /// Slot 0 follows the player into every scene (except endings).
    fn autosave(&mut self, ctx: &Ctx, now: u64) {
        if self.runner.scene().ending {
            return;
        }
        if let Err(e) = ctx
            .store
            .save(&self.runner.pack, AUTOSAVE_SLOT, &self.runner.state, None)
        {
            self.toast = Some(Toast::new(
                format!("Autosave failed: {e:#}"),
                (255, 165, 0),
                now,
            ));
        } else {
            self.toast = Some(Toast {
                life_ms: 1400,
                ..Toast::new("Autosaved", (128, 128, 136), now)
            });
        }
    }

    fn reach_ending(&mut self, ctx: &Ctx) {
        let id = self.runner.pack.id.clone();
        let new = ctx
            .store
            .record_ending(&id, self.runner.scene_id())
            .unwrap_or(false);
        let found = ctx.store.endings(&id).len();
        self.ending = Some((new, found, self.runner.pack.ending_ids().len()));
    }
}
