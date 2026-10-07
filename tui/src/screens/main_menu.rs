//! Main menu: New Game, Continue, Load Game, Credits, Settings, Exit.

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph};
use theatre_engine::Rng;
use theatre_engine::color::scale;
use theatre_engine::save::format_playtime;

use super::{Action, Ctx, Go};
use crate::render::fx::{self, Particles};
use crate::render::text::shimmer;
use crate::render::ui::{Toast, centered, modal, option_line, panel};
use crate::render::{Theme, rgb};

const KONAMI: [KeyCode; 10] = [
    KeyCode::Up,
    KeyCode::Up,
    KeyCode::Down,
    KeyCode::Down,
    KeyCode::Left,
    KeyCode::Right,
    KeyCode::Left,
    KeyCode::Right,
    KeyCode::Char('b'),
    KeyCode::Char('a'),
];

#[derive(Clone, Copy, PartialEq)]
enum Item {
    NewGame,
    Continue,
    Load,
    Credits,
    Settings,
    Exit,
}

const ITEMS: [(Item, &str, &str); 6] = [
    (Item::NewGame, "New Game", "Begin a fresh performance"),
    (Item::Continue, "Continue", "Resume your last investigation"),
    (Item::Load, "Load Game", "Choose a saved spotlight"),
    (Item::Credits, "Credits", "Meet the cast & crew"),
    (Item::Settings, "Settings", "Tailor your experience"),
    (Item::Exit, "Exit", "Leave the theatre"),
];

pub struct MainMenu {
    selected: usize,
    konami: usize,
    secret_at: Option<u64>,
    toast: Option<Toast>,
    opened_at: u64,
    particles: Particles,
}

impl MainMenu {
    pub fn new(now: u64) -> Self {
        let mut rng = Rng::new(0xBADA55 ^ now);
        MainMenu {
            selected: 0,
            konami: 0,
            secret_at: None,
            toast: None,
            opened_at: now,
            particles: Particles::new(50, &mut rng),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.particles.update(dt);
    }

    pub fn key(&mut self, k: KeyEvent, ctx: &Ctx, now: u64) -> Action {
        // classic code: up up down down left right left right b a
        self.konami = if k.code == KONAMI[self.konami] {
            self.konami + 1
        } else if k.code == KONAMI[0] {
            1
        } else {
            0
        };
        if self.konami == KONAMI.len() {
            self.konami = 0;
            self.secret_at = Some(now);
            return Action::Stay;
        }
        if self.secret_at.is_some_and(|t| now - t < 2200) {
            self.secret_at = None;
            return Action::Stay;
        }

        let n = ITEMS.len();
        match k.code {
            KeyCode::Up | KeyCode::Char('k') => self.selected = (self.selected + n - 1) % n,
            KeyCode::Down | KeyCode::Char('j') => self.selected = (self.selected + 1) % n,
            KeyCode::Char('q') | KeyCode::Esc => return Action::Quit,
            KeyCode::Char(c @ '1'..='6') => {
                self.selected = c as usize - '1' as usize;
                return self.activate(ctx, now);
            }
            KeyCode::Enter | KeyCode::Char(' ') => return self.activate(ctx, now),
            _ => {}
        }
        Action::Stay
    }

    fn activate(&mut self, ctx: &Ctx, now: u64) -> Action {
        match ITEMS[self.selected].0 {
            Item::NewGame if ctx.settings.cinematics => Go::Cinematic.into(),
            Item::NewGame => Go::StorySelect.into(),
            Item::Continue => match ctx.store.latest() {
                Some(m) => match ctx.store.load(&m.story_id, m.slot) {
                    Ok(file) => {
                        Go::Resume(ctx.stories.join(&m.story_id), Box::new(file.state), m.name)
                            .into()
                    }
                    Err(e) => {
                        self.toast = Some(Toast::new(
                            format!("Can't continue: {e:#}"),
                            (255, 165, 0),
                            now,
                        ));
                        Action::Stay
                    }
                },
                None => {
                    self.toast = Some(Toast::new(
                        "No saved story yet. Start a New Game!",
                        (255, 165, 0),
                        now,
                    ));
                    Action::Stay
                }
            },
            Item::Load => Go::Load.into(),
            Item::Credits => Go::Credits.into(),
            Item::Settings => Go::Settings.into(),
            Item::Exit => Action::Quit,
        }
    }

    pub fn draw(&mut self, f: &mut Frame, ctx: &Ctx, now: u64) {
        let th = Theme::theatre();
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        self.particles
            .render(f.buffer_mut(), area, scale(th.mote, 0.6));

        let [_, title, _, body, _, footer] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(16),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(area);
        f.render_widget(
            Paragraph::new(shimmer("TERMINAL THEATRE", now, th.accent)).centered(),
            title,
        );

        let latest = ctx.store.latest();
        let box_area = centered(body, 76, 16);
        let block = panel("MAIN MENU", &th, th.border)
            .border_type(BorderType::Rounded)
            .title_bottom(Line::styled(
                " ↑/↓ or 1-6 · Enter to confirm · Q to exit ",
                Style::new().fg(rgb(th.dim)),
            ));
        let inner = block.inner(box_area);
        f.render_widget(block, box_area);
        let mut lines = Vec::new();
        for (i, (item, label, desc)) in ITEMS.iter().enumerate() {
            let detail = match (item, &latest) {
                (Item::Continue, Some(m)) => format!(
                    "{} · {}",
                    m.scene_description,
                    format_playtime(m.playtime_ms)
                ),
                (Item::Continue, None) => "No saved story yet".to_string(),
                _ => desc.to_string(),
            };
            lines.push(option_line(
                &th,
                Some(i),
                label,
                &detail,
                i == self.selected,
                10,
            ));
            lines.push(Line::raw(""));
        }
        f.render_widget(Paragraph::new(lines), inner);

        f.render_widget(
            Paragraph::new("Tip: Discover hidden surprises with classic codes...")
                .style(
                    Style::new()
                        .fg(rgb(scale(th.dim, 0.8)))
                        .add_modifier(Modifier::ITALIC),
                )
                .centered(),
            footer,
        );

        if let Some(at) = self.secret_at {
            if now - at < 2200 {
                let r = centered(area, 56, 9);
                let inner = modal(f, r, "SECRET UNLOCKED", &th, th.accent, "any key");
                let pulse = 0.7 + 0.3 * (now as f32 / 150.0).sin();
                f.render_widget(
                    Paragraph::new(vec![
                        Line::styled(
                            "✦ Secret Unlocked! ✦",
                            Style::new()
                                .fg(rgb(scale(th.border, pulse)))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Line::raw(""),
                        Line::raw("You found the Phantom Stage Entrance."),
                        Line::raw("In future updates, hidden stories await..."),
                    ])
                    .centered(),
                    inner,
                );
            } else {
                self.secret_at = None;
            }
        }
        if let Some(t) = &self.toast {
            t.draw(f, area, &th, now);
        }
        fx::fade(
            f.buffer_mut(),
            area,
            (now.saturating_sub(self.opened_at) as f32 / 400.0).min(1.0),
        );
    }
}
