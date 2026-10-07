//! Playing one story: stage, portraits, dialogue, choices and screen effects,
//! plus the story intro, pause menu, saving and autosave.
//! Story flow lives in the engine's Runner; this screen only presents it.

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use theatre_engine::color::{lerp, parse_hex, scale};
use theatre_engine::progress::{NARRATION, Seen};
use theatre_engine::runner::{Phase, Runner, revealed};
use theatre_engine::save::{AUTOSAVE_SLOT, MAX_SLOT, QUICKSAVE_SLOT, format_playtime};
use theatre_engine::scene::{LineFx, Mood};
use theatre_engine::settings::Effects;
use theatre_engine::sprite::Actor;
use theatre_engine::state::State;
use theatre_engine::{Rng, StoryPack};

use super::backlog::{Backlog, Entry};
use super::{Action, Ctx, Go};
use crate::render::fx::{self, Particles};
use crate::render::text::{continue_hint, pretty, shimmer, typed, wrapped_height};
use crate::render::ui::{Toast, centered, modal, option_line};
use crate::render::{Theme, rgb, sprite};

const SCENE_FADE_MS: u64 = 700;
const PORTRAIT_ENTER_MS: u64 = 350;
const PORTRAIT_W: u16 = 34;
const BOTTOM_H: u16 = 20;
const SAVE_NAME_MAX: usize = 32;

const PAUSE_ITEMS: [(&str, &str); 5] = [
    ("Resume", "Back to the story"),
    ("Save Game", "Keep this moment in a slot"),
    ("History", "Read everything again (h)"),
    ("Main Menu", "Leave this story"),
    ("Quit", "Close the theatre"),
];
/// Pause menu rows.
const P_RESUME: usize = 0;
const P_SAVE: usize = 1;
const P_HISTORY: usize = 2;
const P_MENU: usize = 3;

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

/// Skip mode moves on this often.
const SKIP_STEP_MS: u64 = 70;

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
            ending: None,
        }
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
        match &mut self.overlay {
            Overlay::None => {}
            Overlay::Intro { .. } => {
                if matches!(k.code, KeyCode::Esc) {
                    return Go::StorySelect.into();
                }
                self.overlay = Overlay::None;
                // restart the scene clock so its entrance plays now
                let scene = self.runner.scene_id().to_string();
                self.runner.goto(&scene, now);
                self.on_phase(Phase::Narration, now);
                self.autosave(ctx, now);
                return Action::Stay;
            }
            Overlay::Pause { selected } => {
                let n = PAUSE_ITEMS.len();
                match k.code {
                    KeyCode::Up | KeyCode::Char('k') => *selected = (*selected + n - 1) % n,
                    KeyCode::Down | KeyCode::Char('j') => *selected = (*selected + 1) % n,
                    KeyCode::Esc => self.overlay = Overlay::None,
                    KeyCode::Enter | KeyCode::Char(' ') => match *selected {
                        P_RESUME => self.overlay = Overlay::None,
                        P_SAVE => self.overlay = Overlay::SaveSlots { selected: 0 },
                        P_HISTORY => self.overlay = Overlay::Backlog { scroll: 0 },
                        P_MENU => self.overlay = Overlay::ConfirmLeave,
                        _ => return Action::Quit,
                    },
                    _ => {}
                }
                return Action::Stay;
            }
            Overlay::ConfirmLeave => {
                if matches!(k.code, KeyCode::Char('y') | KeyCode::Char('Y')) {
                    return Go::MainMenu.into();
                }
                self.overlay = Overlay::Pause { selected: P_MENU };
                return Action::Stay;
            }
            Overlay::Backlog { scroll } => {
                match k.code {
                    KeyCode::Up | KeyCode::Char('k') => *scroll += 1,
                    KeyCode::Down | KeyCode::Char('j') => *scroll = scroll.saturating_sub(1),
                    KeyCode::PageUp => *scroll += 10,
                    KeyCode::PageDown => *scroll = scroll.saturating_sub(10),
                    KeyCode::Esc | KeyCode::Char('h') | KeyCode::Enter | KeyCode::Char('q') => {
                        self.overlay = Overlay::None
                    }
                    _ => {}
                }
                return Action::Stay;
            }
            Overlay::SaveSlots { selected } => {
                let n = MAX_SLOT as usize;
                let mut sel = *selected;
                match k.code {
                    KeyCode::Up | KeyCode::Char('k') => sel = (sel + n - 1) % n,
                    KeyCode::Down | KeyCode::Char('j') => sel = (sel + 1) % n,
                    KeyCode::Char(c @ '1'..='9') => sel = c as usize - '1' as usize,
                    KeyCode::Esc => {
                        self.overlay = Overlay::Pause { selected: P_SAVE };
                        return Action::Stay;
                    }
                    _ => {}
                }
                *selected = sel;
                if matches!(
                    k.code,
                    KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('1'..='9')
                ) {
                    let slot = sel as u8 + 1;
                    let existing = ctx
                        .store
                        .load(&self.runner.pack.id, slot)
                        .ok()
                        .map(|f| f.metadata.name);
                    let text = existing.unwrap_or_else(|| format!("Save {slot}"));
                    self.overlay = Overlay::SaveName { slot, text };
                }
                return Action::Stay;
            }
            Overlay::SaveName { slot, text } => {
                match k.code {
                    KeyCode::Esc => {
                        self.overlay = Overlay::SaveSlots {
                            selected: *slot as usize - 1,
                        }
                    }
                    KeyCode::Backspace => {
                        text.pop();
                    }
                    KeyCode::Char(c) if text.chars().count() < SAVE_NAME_MAX => text.push(c),
                    KeyCode::Enter => {
                        let (slot, name) = (*slot, text.clone());
                        self.toast = Some(
                            match ctx.store.save(
                                &self.runner.pack,
                                slot,
                                &self.runner.state,
                                Some(&name),
                            ) {
                                Ok(m) => Toast::new(
                                    format!("✓ Game saved to slot {slot}: {}", m.name),
                                    (0, 255, 127),
                                    now,
                                ),
                                Err(e) => {
                                    Toast::new(format!("✗ Failed to save: {e:#}"), (255, 0, 0), now)
                                }
                            },
                        );
                        self.overlay = Overlay::None;
                    }
                    _ => {}
                }
                return Action::Stay;
            }
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

    /// (scene, line) of the text on screen; line -1 is the narration.
    fn read_key(&self) -> Option<(String, i64)> {
        let scene = self.runner.scene_id().to_string();
        match self.runner.phase() {
            Phase::Narration => Some((scene, NARRATION)),
            Phase::Line(i) => Some((scene, i as i64)),
            _ => None,
        }
    }

    /// The player has read this text: keep it for skip and the history.
    fn remember(&mut self, ctx: &Ctx, key: (String, i64)) {
        let scene = &self.runner.pack.scenes[&key.0];
        let entry = if key.1 == NARRATION {
            Entry {
                who: None,
                text: scene.text.clone(),
                thought: false,
                color: None,
            }
        } else {
            let line = &scene.dialogue[key.1 as usize];
            let speaker = self.runner.pack.speaker(&line.who);
            Entry {
                who: Some(self.runner.pack.speaker_name(&line.who).to_string()),
                text: line.line.clone(),
                thought: speaker.is_some_and(|s| s.thought),
                color: speaker
                    .and_then(|s| s.color.as_deref())
                    .and_then(|c| parse_hex(c).ok()),
            }
        };
        self.backlog.push(entry);
        let _ = ctx.store.mark_seen(&self.runner.pack.id, &key.0, key.1);
        self.seen.insert(key);
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

    fn toggle_mode(&mut self, m: Mode, now: u64) {
        self.mode = if self.mode == m { Mode::Normal } else { m };
        self.done_at = None;
        let label = match self.mode {
            Mode::Normal => "Auto and skip off",
            Mode::Auto => "Auto: lines move on by themselves",
            Mode::Skip => "Skip: racing through text you've read",
        };
        self.toast = Some(Toast {
            life_ms: 1200,
            ..Toast::new(label, (0, 212, 255), now)
        });
    }

    /// Auto and skip modes move the story along from `tick`.
    fn run_mode(&mut self, ctx: &Ctx, now: u64) {
        let reading = matches!(self.runner.phase(), Phase::Narration | Phase::Line(_));
        match self.mode {
            Mode::Normal => {}
            Mode::Skip if !reading => self.mode = Mode::Normal,
            Mode::Skip => {
                if now.saturating_sub(self.last_skip) < SKIP_STEP_MS {
                    return;
                }
                let read = self.read_key().is_some_and(|k| self.seen.contains(&k));
                if !read && !ctx.settings.skip_unread {
                    self.mode = Mode::Normal;
                    self.toast = Some(Toast::new("Skip stopped: new text", (255, 176, 0), now));
                    return;
                }
                self.last_skip = now;
                if !self.runner.typing_done(now) {
                    self.runner.advance(now); // finish the line first
                }
                self.advance(ctx, now);
            }
            Mode::Auto if !reading => {}
            Mode::Auto => {
                if !self.runner.typing_done(now) {
                    return;
                }
                let done = *self.done_at.get_or_insert(now);
                let len = self.read_key().map(|k| self.text_len(&k)).unwrap_or(0) as u64;
                let wait = ctx.settings.auto_delay_ms + len * 25;
                if now.saturating_sub(done) >= wait {
                    self.advance(ctx, now);
                }
            }
        }
    }

    fn text_len(&self, key: &(String, i64)) -> usize {
        let scene = &self.runner.pack.scenes[&key.0];
        if key.1 == NARRATION {
            scene.text.chars().count()
        } else {
            scene.dialogue[key.1 as usize].line.chars().count()
        }
    }

    fn quick_save(&mut self, ctx: &Ctx, now: u64) {
        let result = ctx
            .store
            .save(&self.runner.pack, QUICKSAVE_SLOT, &self.runner.state, None);
        self.toast = Some(match result {
            Ok(_) => Toast::new("✓ Quicksaved (F9 to load)", (0, 255, 127), now),
            Err(e) => Toast::new(format!("✗ Quicksave failed: {e:#}"), (255, 0, 0), now),
        });
    }

    /// Jump back to this story's quicksave without leaving the screen.
    fn quick_load(&mut self, ctx: &Ctx, now: u64) {
        let Ok(file) = ctx.store.load(&self.runner.pack.id, QUICKSAVE_SLOT) else {
            self.toast = Some(Toast::new(
                "No quicksave yet (F5 to make one)",
                (255, 165, 0),
                now,
            ));
            return;
        };
        let scene = file.state.current_scene.clone();
        if !self.runner.pack.scenes.contains_key(&scene) {
            self.toast = Some(Toast::new(
                "The quicksave's scene no longer exists",
                (255, 0, 0),
                now,
            ));
            return;
        }
        self.runner.state = file.state;
        self.runner.goto(&scene, now);
        self.actor = None;
        self.ending = None;
        self.mode = Mode::Normal;
        self.on_phase(Phase::Narration, now);
        self.toast = Some(Toast::new("✓ Quickloaded", (0, 255, 127), now));
    }

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

    pub fn draw(&mut self, f: &mut Frame, ctx: &Ctx, now: u64) {
        let th = Theme::for_mood(self.runner.mood(), &self.runner.pack.meta.themes);
        if let Overlay::Intro { resumed, since } = &self.overlay {
            self.draw_intro(f, &th, resumed.as_deref(), *since, now);
            return;
        }
        let full = f.area();
        let strength = ctx.settings.effects.strength();
        let area = self.shaken(full, now, strength);
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), full);
        self.particles.render(f.buffer_mut(), full, th.mote);

        let [header, stage, bottom] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(BOTTOM_H),
        ])
        .areas(area);
        self.draw_header(f, header, &th);
        self.draw_stage(f, stage, &th, now);
        self.draw_bottom(f, bottom, &th, now);

        // line effects, scaled by the Screen Effects setting
        let since = now.saturating_sub(self.fx_start);
        match self.fx_kind {
            Some(LineFx::Glitch) if since < 450 && strength > 0.0 => {
                let k = (1.0 - since as f32 / 450.0) * strength;
                fx::glitch(f.buffer_mut(), full, &mut self.rng, k, th.accent);
            }
            Some(LineFx::Flash) if since < 500 && strength > 0.0 => {
                let k = (1.0 - since as f32 / 500.0) * strength;
                fx::flash(f.buffer_mut(), full, (255, 255, 255), k);
            }
            _ => {}
        }
        if now < self.glitch_until && strength >= 1.0 {
            fx::glitch(f.buffer_mut(), full, &mut self.rng, 0.5, th.accent);
        }
        // scene entrance
        let since_scene = now.saturating_sub(self.runner.scene_start());
        if since_scene < SCENE_FADE_MS {
            let k = since_scene as f32 / SCENE_FADE_MS as f32;
            fx::dissolve(
                f.buffer_mut(),
                full,
                k * 1.2,
                th.bg,
                self.runner.scene_start(),
            );
            fx::fade(f.buffer_mut(), full, 0.3 + 0.7 * k);
        }
        self.draw_overlay(f, ctx, &th, now);
        if let Some(t) = &self.toast {
            t.draw(f, full, &th, now);
        }
    }

    /// Story title card: the description types out for a new game.
    fn draw_intro(&self, f: &mut Frame, th: &Theme, resumed: Option<&str>, since: u64, now: u64) {
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        let meta = &self.runner.pack.meta;
        let body = match resumed {
            Some(name) => format!(
                "Resuming your story...\n\n\"{name}\" · {} · {} played",
                pretty(self.runner.scene_id()),
                format_playtime(self.runner.state.playtime_ms)
            ),
            None => meta.description.clone(),
        };
        let w = area.width.min(84);
        let body_h = wrapped_height(&body, w as usize) as u16;
        let [_, rule1, title, rule2, _, text, _, prompt, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(body_h),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);
        let rule = "═".repeat(60.min(area.width as usize));
        for r in [rule1, rule2] {
            f.render_widget(
                Paragraph::new(rule.as_str())
                    .style(Style::new().fg(rgb(th.border)))
                    .centered(),
                r,
            );
        }
        f.render_widget(
            Paragraph::new(shimmer(&meta.title, now, th.accent)).centered(),
            title,
        );
        let elapsed = now.saturating_sub(since).saturating_sub(400);
        let ms = ((20.0 * self.runner.speed).round() as u64).max(1);
        let shown = if self.runner.speed <= 0.0 {
            usize::MAX
        } else {
            revealed(&body, elapsed, ms)
        };
        let text_area = Rect {
            x: text.x + (text.width - w) / 2,
            width: w,
            ..text
        };
        f.render_widget(
            Paragraph::new(typed(
                &body,
                shown,
                Style::new().fg(rgb(th.text)),
                Style::new().fg(rgb(th.bg)),
            ))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
            text_area,
        );
        if shown == usize::MAX {
            let pulse = 0.55 + 0.45 * ((now as f32 / 450.0).sin() * 0.5 + 0.5);
            f.render_widget(
                Paragraph::new("Press ENTER to continue...  ·  Esc to go back")
                    .style(Style::new().fg(rgb(scale(th.dim, pulse * 1.3))))
                    .centered(),
                prompt,
            );
        }
        fx::fade(
            f.buffer_mut(),
            area,
            (now.saturating_sub(since) as f32 / 600.0).min(1.0),
        );
    }

    fn draw_overlay(&self, f: &mut Frame, ctx: &Ctx, th: &Theme, now: u64) {
        let area = f.area();
        match &self.overlay {
            Overlay::None | Overlay::Intro { .. } => {}
            Overlay::Backlog { scroll } => self.backlog.draw(f, th, *scroll),
            Overlay::Pause { selected } => {
                let inner = modal(
                    f,
                    centered(area, 58, 13),
                    "PAUSED",
                    th,
                    th.border,
                    "↑/↓ · Enter · Esc resume",
                );
                let mut lines = vec![Line::styled(
                    format!(
                        "{} · {} · {}",
                        self.runner.pack.meta.title,
                        pretty(self.runner.scene_id()),
                        format_playtime(self.runner.state.playtime_ms)
                    ),
                    Style::new().fg(rgb(th.dim)),
                )];
                lines.push(Line::raw(""));
                for (i, (label, desc)) in PAUSE_ITEMS.iter().enumerate() {
                    lines.push(option_line(th, None, label, desc, i == *selected, 10));
                    lines.push(Line::raw(""));
                }
                f.render_widget(Paragraph::new(lines), inner);
            }
            Overlay::ConfirmLeave => {
                let inner = modal(
                    f,
                    centered(area, 58, 9),
                    "RETURN TO MAIN MENU",
                    th,
                    (255, 165, 0),
                    "Y / N",
                );
                f.render_widget(
                    Paragraph::new(vec![
                        Line::raw("Are you sure?"),
                        Line::raw(""),
                        Line::styled(
                            "Progress since your last save is lost. The autosave keeps the start of this scene.",
                            Style::new().fg(rgb(th.dim)),
                        ),
                    ])
                    .centered()
                    .wrap(Wrap { trim: true }),
                    inner,
                );
            }
            Overlay::SaveSlots { selected } => {
                let inner = modal(
                    f,
                    centered(area, 90, 24),
                    "SAVE GAME",
                    th,
                    th.border,
                    "↑/↓ or 1-9 · Enter choose · Esc back",
                );
                let slots = ctx.store.slots(&self.runner.pack.id);
                let mut lines = Vec::new();
                for slot in 1..=MAX_SLOT {
                    let i = slot as usize - 1;
                    let (label, detail) = match &slots[slot as usize] {
                        Some(m) => (
                            m.name.clone(),
                            format!(
                                "{} · {} · {}",
                                m.scene_description,
                                m.time_label(),
                                format_playtime(m.playtime_ms)
                            ),
                        ),
                        None => ("[Empty Slot]".to_string(), String::new()),
                    };
                    lines.push(option_line(
                        th,
                        Some(i),
                        &label,
                        &detail,
                        i == *selected,
                        24,
                    ));
                    lines.push(Line::raw(""));
                }
                f.render_widget(Paragraph::new(lines), inner);
            }
            Overlay::SaveName { slot, text } => {
                let inner = modal(
                    f,
                    centered(area, 60, 9),
                    &format!("SAVE TO SLOT {slot}"),
                    th,
                    th.border,
                    "Enter save · Esc back",
                );
                let cursor = if (now / 500).is_multiple_of(2) {
                    "▌"
                } else {
                    " "
                };
                f.render_widget(
                    Paragraph::new(vec![
                        Line::styled("Name this save:", Style::new().fg(rgb(th.dim))),
                        Line::raw(""),
                        Line::from(vec![
                            Span::styled(
                                text.clone(),
                                Style::new().fg(rgb(th.text)).add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(cursor, Style::new().fg(rgb(th.accent))),
                        ]),
                    ]),
                    inner,
                );
            }
        }
    }

    /// Screen shake: full is 2 cells for 400 ms, reduced 1 cell for 150 ms, off none.
    fn shaken(&mut self, area: Rect, now: u64, strength: f32) -> Rect {
        let since = now.saturating_sub(self.fx_start);
        let length = if strength >= 1.0 { 400 } else { 150 };
        if self.fx_kind != Some(LineFx::Shake)
            || strength <= 0.0
            || since > length
            || area.width < 4
        {
            return area;
        }
        let amp = if since < 200 && strength >= 1.0 { 2 } else { 1 };
        let dx = self.rng.range(0, amp * 2 + 1) as i32 - amp as i32;
        let x = (area.x as i32 + dx).max(0) as u16;
        Rect {
            x,
            width: area.width - amp as u16,
            ..area
        }
    }

    fn draw_header(&self, f: &mut Frame, area: Rect, th: &Theme) {
        let line = Line::from(vec![
            Span::styled(
                format!(" {} ", self.runner.pack.meta.title),
                Style::new()
                    .fg(rgb(th.bg))
                    .bg(rgb(th.accent))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}", pretty(self.runner.scene_id())),
                Style::new().fg(rgb(th.dim)).add_modifier(Modifier::ITALIC),
            ),
        ]);
        f.render_widget(Paragraph::new(line), area);
        let mut right = Vec::new();
        let badge = match self.mode {
            Mode::Normal => None,
            Mode::Auto => Some(" AUTO ▶ "),
            Mode::Skip => Some(" SKIP ▶▶ "),
        };
        if let Some(b) = badge {
            right.push(Span::styled(
                b,
                Style::new()
                    .fg(rgb(th.bg))
                    .bg(rgb(th.border))
                    .add_modifier(Modifier::BOLD),
            ));
            right.push(Span::raw("  "));
        }
        if !self.runner.state.items.is_empty() {
            right.push(Span::raw(format!(
                "⚔ {}  ",
                self.runner.state.items.join(" · ")
            )));
        }
        right.push(Span::raw(
            "h history · a auto · s skip · F5/F9 quick · esc menu ",
        ));
        f.render_widget(
            Paragraph::new(Line::from(right))
                .style(Style::new().fg(rgb(th.dim)))
                .alignment(Alignment::Right),
            area,
        );
    }

    fn draw_stage(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let scene = self.runner.scene();
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(rgb(th.border)))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::horizontal(3));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let phase = self.runner.phase();

        let Some(art) = scene.art_at(now.saturating_sub(self.runner.scene_start())) else {
            // no art: the narration lives on the stage and stays (dimmed) during dialogue
            let w = inner.width.min(90);
            let h = (wrapped_height(&scene.text, w as usize) as u16).min(inner.height);
            let r = Rect {
                x: inner.x + (inner.width - w) / 2,
                y: inner.y + (inner.height - h) / 2,
                width: w,
                height: h,
            };
            let (shown, col) = match phase {
                Phase::Narration => (self.runner.shown_chars(now), th.text),
                _ => (usize::MAX, lerp(th.text, th.panel, 0.45)),
            };
            let text = typed(
                &scene.text,
                shown,
                Style::new().fg(rgb(col)),
                Style::new().fg(rgb(th.panel)),
            );
            f.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), r);
            if phase == Phase::Narration && self.runner.typing_done(now) {
                continue_hint(f, area, now, th.accent);
            }
            return;
        };

        let lines: Vec<&str> = art.trim_matches('\n').lines().collect();
        // crop tall art around its middle
        let skip = lines.len().saturating_sub(inner.height as usize) / 2;
        let lines = &lines[skip..lines.len().min(skip + inner.height as usize)];
        let art_h = lines.len() as u16;
        let since = now.saturating_sub(self.runner.scene_start()) as f32;
        let styled: Vec<Line> = lines
            .iter()
            .enumerate()
            .map(|(i, l)| {
                // vertical gradient, slowly breathing
                let k = i as f32 / art_h.max(1) as f32;
                let pulse = 0.8 + 0.2 * ((now as f32 / 900.0) + k * 3.0).sin();
                let col = scale(
                    lerp(th.accent, th.dim, k),
                    pulse * (since / 1200.0).min(1.0),
                );
                Line::styled(*l, Style::new().fg(rgb(col)))
            })
            .collect();
        let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as u16;
        let r = Rect {
            x: inner.x + inner.width.saturating_sub(w) / 2,
            y: inner.y + (inner.height - art_h) / 2,
            width: w.min(inner.width),
            height: art_h,
        };
        f.render_widget(Paragraph::new(styled), r);
    }

    fn draw_bottom(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let phase = self.runner.phase();
        let has_portrait = self.actor.is_some() && matches!(phase, Phase::Line(_) | Phase::Choose);
        let [portrait, box_col] = Layout::horizontal([
            Constraint::Length(if has_portrait { PORTRAIT_W } else { 0 }),
            Constraint::Fill(1),
        ])
        .areas(area);

        if has_portrait
            && let Some(actor) = &self.actor
            && let Some(s) = self.runner.pack.sprites.get(&actor.sprite)
        {
            let k =
                (now.saturating_sub(self.actor_since) as f32 / PORTRAIT_ENTER_MS as f32).min(1.0);
            let ease = 1.0 - (1.0 - k).powi(3);
            let dx = ((1.0 - ease) * -12.0) as i32;
            sprite::draw(f.buffer_mut(), portrait, s, &actor.pose, now, dx, ease);
        }

        let box_h = 9.min(area.height);
        let dbox = Rect {
            y: box_col.bottom() - box_h,
            height: box_h,
            ..box_col
        };
        match phase {
            Phase::Line(_) => self.draw_line(f, dbox, th, now),
            Phase::Choose => self.draw_choices(f, box_col, th, now),
            Phase::End => self.draw_end(f, box_col, th, now),
            Phase::Narration if self.runner.scene().has_art() => {
                self.draw_narration(f, area, th, now)
            }
            Phase::Narration => {}
        }
    }

    /// Narration box at the bottom, used when the stage is showing art.
    fn draw_narration(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let scene = self.runner.scene();
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(rgb(th.border)))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(3, 3, 1, 1));
        let w = block.inner(area).width as usize;
        let h = (wrapped_height(&scene.text, w) as u16 + 4).min(area.height);
        let area = Rect {
            y: area.bottom() - h,
            height: h,
            ..area
        };
        let inner = block.inner(area);
        f.render_widget(block, area);
        let text = typed(
            &scene.text,
            self.runner.shown_chars(now),
            Style::new().fg(rgb(th.text)),
            Style::new().fg(rgb(th.panel)),
        );
        f.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), inner);
        if self.runner.typing_done(now) {
            continue_hint(f, area, now, th.accent);
        }
    }

    fn draw_line(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let Some(line) = self.runner.line() else {
            return;
        };
        let pack = &self.runner.pack;
        let speaker = pack.speaker(&line.who);
        let thought = speaker.is_some_and(|s| s.thought);
        let own = speaker
            .and_then(|s| s.color.as_deref())
            .and_then(|c| parse_hex(c).ok());
        let accent = match (thought, own) {
            (_, Some(c)) if !thought => c,
            (true, Some(c)) => scale(c, 0.7),
            (true, None) => th.dim,
            _ => th.accent,
        };
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::new().fg(rgb(accent)))
            .title(Span::styled(
                format!(" {} ", pack.speaker_name(&line.who)),
                Style::new()
                    .fg(rgb(th.bg))
                    .bg(rgb(accent))
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(2, 2, 1, 0));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let mut style = Style::new().fg(rgb(th.text));
        if thought {
            style = style.add_modifier(Modifier::ITALIC);
        }
        let quoted = if thought {
            format!("({})", line.line)
        } else {
            format!("“{}”", line.line)
        };
        // +1 for the opening quote/paren
        let shown = self.runner.shown_chars(now).saturating_add(1);
        f.render_widget(
            Paragraph::new(typed(&quoted, shown, style, Style::new().fg(rgb(th.panel))))
                .wrap(Wrap { trim: false }),
            inner,
        );
        if self.runner.typing_done(now) {
            continue_hint(f, area, now, th.accent);
        }
    }

    fn draw_choices(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let choices = self.runner.choices();
        let h = (choices.len() as u16 * 2 + 3).min(area.height);
        let area = Rect {
            y: area.bottom() - h,
            height: h,
            ..area
        };
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(rgb(th.accent)))
            .title(Span::styled(
                " WHAT DO YOU DO? ",
                Style::new().fg(rgb(th.accent)).add_modifier(Modifier::BOLD),
            ))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(2, 2, 1, 0));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let since = now.saturating_sub(self.runner.phase_start());
        let mut lines = Vec::new();
        for (i, c) in choices.iter().enumerate() {
            // choices slide in one after another
            let appear = ((since as f32 - i as f32 * 90.0) / 250.0).clamp(0.0, 1.0);
            let selected = i == self.runner.selected();
            let pulse = 0.75 + 0.25 * (now as f32 / 300.0).sin();
            let col = if selected {
                scale(th.accent, pulse)
            } else {
                th.dim
            };
            let mut style =
                Style::new().fg(rgb(scale(if selected { th.text } else { col }, appear)));
            if selected {
                style = style.add_modifier(Modifier::BOLD);
            }
            lines.push(Line::from(vec![
                Span::raw(" ".repeat(((1.0 - appear) * 6.0) as usize)),
                Span::styled(
                    if selected { "▶ " } else { "  " },
                    Style::new().fg(rgb(th.accent)),
                ),
                Span::styled(
                    format!("{}. ", i + 1),
                    Style::new().fg(rgb(scale(th.dim, appear))),
                ),
                Span::styled(c.text.clone(), style),
            ]));
            lines.push(Line::raw(""));
        }
        f.render_widget(Paragraph::new(lines), inner);
    }

    fn draw_end(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let k = (now.saturating_sub(self.runner.phase_start()) as f32 / 1200.0).min(1.0);
        let st = &self.runner.state;
        let total = self.runner.pack.scenes.len().max(1);
        let lines = vec![
            Line::styled(
                "·  T H E   E N D  ·",
                Style::new()
                    .fg(rgb(scale(th.accent, k)))
                    .add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
            Line::styled(
                "Thank you for playing!",
                Style::new().fg(rgb(scale(th.text, k))),
            ),
            Line::styled(
                format!("Ending: {}", pretty(self.runner.scene_id())),
                Style::new()
                    .fg(rgb(scale(th.mote, k)))
                    .add_modifier(Modifier::ITALIC),
            ),
            match self.ending {
                Some((new, found, all)) => Line::styled(
                    format!(
                        "{}Endings found: {found} / {all}",
                        if new {
                            "✦ New ending unlocked!  ·  "
                        } else {
                            ""
                        }
                    ),
                    Style::new()
                        .fg(rgb(scale(th.border, k)))
                        .add_modifier(Modifier::BOLD),
                ),
                None => Line::raw(""),
            },
            Line::styled(
                format!(
                    "{} played · {} choices · {:.0}% of the story seen",
                    format_playtime(st.playtime_ms),
                    st.choices.len(),
                    st.visited.len() as f32 / total as f32 * 100.0
                ),
                Style::new().fg(rgb(scale(th.dim, k))),
            ),
            Line::raw(""),
            Line::styled(
                "press ENTER to choose another story",
                Style::new().fg(rgb(scale(th.dim, k))),
            ),
        ];
        let h = lines.len() as u16;
        let r = Rect {
            y: area.y + area.height.saturating_sub(h) / 2,
            height: h.min(area.height),
            ..area
        };
        f.render_widget(Paragraph::new(lines).alignment(Alignment::Center), r);
    }
}
