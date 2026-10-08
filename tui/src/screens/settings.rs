//! Settings: color mode, cinematic intro, text speed, typewriter, sound.
//! Every change is saved immediately.

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use super::{Action, Ctx, Go};
use crate::render::ui::{Toast, centered, modal, option_line};
use crate::render::{Theme, rgb};

const ROWS: [&str; 9] = [
    "Color Mode",
    "Cinematic Intro",
    "Text Speed",
    "Typewriter Effect",
    "Screen Effects",
    "Skip Mode",
    "Scene Images",
    "Sound Effects",
    "Back",
];

pub struct SettingsScreen {
    selected: usize,
    toast: Option<Toast>,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsScreen {
    pub fn new() -> Self {
        SettingsScreen {
            selected: 0,
            toast: None,
        }
    }

    fn value(ctx: &Ctx, row: usize) -> String {
        let s = &ctx.settings;
        let on = |b: bool| if b { "Enabled" } else { "Disabled" }.to_string();
        match row {
            0 => if s.color { "Vivid" } else { "Monochrome" }.into(),
            1 => on(s.cinematics),
            2 => s.text_speed.label().into(),
            3 => on(s.typewriter),
            4 => format!("{} (flash, shake, glitch)", s.effects.label()),
            5 => if s.skip_unread {
                "All text"
            } else {
                "Read text only"
            }
            .into(),
            6 => if s.images {
                "On (pictures where a scene has one)"
            } else {
                "Off (ASCII art)"
            }
            .into(),
            7 => "Coming Soon".into(),
            _ => "Return".into(),
        }
    }

    pub fn key(&mut self, k: KeyEvent, ctx: &mut Ctx, now: u64) -> Action {
        let n = ROWS.len();
        match k.code {
            KeyCode::Up | KeyCode::Char('k') => self.selected = (self.selected + n - 1) % n,
            KeyCode::Down | KeyCode::Char('j') => self.selected = (self.selected + 1) % n,
            KeyCode::Esc | KeyCode::Char('q') => return Go::MainMenu.into(),
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Left | KeyCode::Right => {
                let s = &mut ctx.settings;
                match self.selected {
                    0 => s.color = !s.color,
                    1 => s.cinematics = !s.cinematics,
                    2 => s.cycle_text_speed(),
                    3 => s.toggle_typewriter(),
                    4 => s.effects = s.effects.next(),
                    5 => s.skip_unread = !s.skip_unread,
                    6 => s.images = !s.images,
                    7 => {
                        s.sound = !s.sound;
                        self.toast = Some(Toast {
                            life_ms: 4000,
                            ..Toast::new(
                                "Soundscapes are in composition! Thunder, jazz and the crackle of neon are coming. For now, imagine the soundtrack...",
                                (0, 255, 255),
                                now,
                            )
                        });
                    }
                    _ => return Go::MainMenu.into(),
                }
                ctx.save_settings();
            }
            _ => {}
        }
        Action::Stay
    }

    pub fn draw(&self, f: &mut Frame, ctx: &Ctx, now: u64) {
        let th = Theme::theatre();
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        let [_, body, desc, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(22),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);
        let inner = modal(
            f,
            centered(body, 76, 22),
            "SETTINGS",
            &th,
            th.border,
            "Enter to toggle · Esc to go back",
        );
        let mut lines = Vec::new();
        for (i, label) in ROWS.iter().enumerate() {
            lines.push(option_line(
                &th,
                None,
                label,
                &Self::value(ctx, i),
                i == self.selected,
                20,
            ));
            lines.push(Line::raw(""));
        }
        f.render_widget(Paragraph::new(lines), inner);
        f.render_widget(
            Paragraph::new(format!(
                "Adjust presentation preferences. Saved in {}",
                ctx.data.join(theatre_engine::store::DB_FILE).display()
            ))
            .style(Style::new().fg(rgb(th.dim)))
            .centered(),
            desc,
        );
        if let Some(t) = &self.toast {
            t.draw(f, area, &th, now);
        }
    }
}
