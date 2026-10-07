//! Load Game: every save of every story, newest first. Enter loads, D deletes.

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use theatre_engine::color::scale;
use theatre_engine::save::{AUTOSAVE_SLOT, Metadata, QUICKSAVE_SLOT, format_playtime};

use super::{Action, Ctx, Go};
use crate::render::ui::{Toast, centered, modal};
use crate::render::{Theme, rgb};

pub struct Load {
    saves: Vec<Metadata>,
    selected: usize,
    confirm_delete: bool,
    toast: Option<Toast>,
}

impl Load {
    pub fn new(ctx: &Ctx) -> Self {
        Load {
            saves: ctx.store.all(),
            selected: 0,
            confirm_delete: false,
            toast: None,
        }
    }

    pub fn key(&mut self, k: KeyEvent, ctx: &Ctx, now: u64) -> Action {
        if self.confirm_delete {
            self.confirm_delete = false;
            if matches!(k.code, KeyCode::Char('y') | KeyCode::Char('Y')) {
                let m = &self.saves[self.selected];
                match ctx.store.delete(&m.story_id, m.slot) {
                    Ok(()) => {
                        self.toast = Some(Toast::new(
                            format!("Deleted \"{}\"", m.name),
                            (255, 165, 0),
                            now,
                        ))
                    }
                    Err(e) => self.toast = Some(Toast::new(format!("{e:#}"), (255, 0, 0), now)),
                }
                self.saves = ctx.store.all();
                self.selected = self.selected.min(self.saves.len().saturating_sub(1));
            }
            return Action::Stay;
        }
        let n = self.saves.len();
        match k.code {
            KeyCode::Esc | KeyCode::Char('q') => return Go::MainMenu.into(),
            KeyCode::Up | KeyCode::Char('k') if n > 0 => {
                self.selected = (self.selected + n - 1) % n
            }
            KeyCode::Down | KeyCode::Char('j') if n > 0 => self.selected = (self.selected + 1) % n,
            KeyCode::Char('d') | KeyCode::Delete if n > 0 => self.confirm_delete = true,
            KeyCode::Enter | KeyCode::Char(' ') if n > 0 => {
                let m = &self.saves[self.selected];
                match ctx.store.load(&m.story_id, m.slot) {
                    Ok(file) => {
                        return Go::Resume(
                            ctx.stories.join(&m.story_id),
                            Box::new(file.state),
                            m.name.clone(),
                        )
                        .into();
                    }
                    Err(e) => self.toast = Some(Toast::new(format!("{e:#}"), (255, 0, 0), now)),
                }
            }
            _ => {}
        }
        Action::Stay
    }

    pub fn draw(&self, f: &mut Frame, now: u64) {
        let th = Theme::theatre();
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        let footer = if self.confirm_delete {
            "Delete this save? Y / N"
        } else {
            "Enter load · D delete · Esc back"
        };
        let inner = modal(
            f,
            centered(area, 96, 30),
            "LOAD GAME",
            &th,
            th.border,
            footer,
        );
        if self.saves.is_empty() {
            f.render_widget(
                Paragraph::new("No saves yet. Your progress is autosaved at every scene once you start a story.")
                    .style(Style::new().fg(rgb(th.dim)))
                    .centered(),
                inner,
            );
            return;
        }
        // keep the selection visible: three lines per save
        let per_page = (inner.height / 3).max(1) as usize;
        let first = self.selected.saturating_sub(per_page - 1);
        let mut lines = Vec::new();
        for (i, m) in self.saves.iter().enumerate().skip(first).take(per_page) {
            let sel = i == self.selected;
            let slot = if m.slot == QUICKSAVE_SLOT {
                "quick".to_string()
            } else if m.slot == AUTOSAVE_SLOT {
                "auto".to_string()
            } else {
                format!("slot {}", m.slot)
            };
            let name_style = if sel {
                Style::new().fg(rgb(th.text)).add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(rgb(scale(th.text, 0.75)))
            };
            lines.push(Line::from(vec![
                Span::styled(
                    if sel { "▶ " } else { "  " },
                    Style::new().fg(rgb(th.accent)),
                ),
                Span::styled(format!("{:<26}", m.name), name_style),
                Span::styled(
                    format!("{:<20}", m.story_title),
                    Style::new().fg(rgb(th.mote)),
                ),
                Span::styled(format!("{slot:<8}"), Style::new().fg(rgb(th.dim))),
                Span::styled(m.time_label(), Style::new().fg(rgb(th.dim))),
            ]));
            lines.push(Line::styled(
                format!(
                    "    {} · {} played · {:.0}% seen · {} choices",
                    m.scene_description,
                    format_playtime(m.playtime_ms),
                    m.completion,
                    m.choices_made
                ),
                Style::new().fg(rgb(if sel { th.border } else { th.dim })),
            ));
            lines.push(Line::raw(""));
        }
        f.render_widget(Paragraph::new(lines), inner);
        if let Some(t) = &self.toast {
            t.draw(f, area, &th, now);
        }
    }
}
