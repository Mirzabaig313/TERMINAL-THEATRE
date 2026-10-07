//! Credits.

use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use theatre_engine::color::scale;

use super::{Action, Go};
use crate::render::text::shimmer;
use crate::render::ui::{centered, modal};
use crate::render::{Theme, rgb};

pub struct Credits {
    opened_at: u64,
}

impl Credits {
    pub fn new(now: u64) -> Self {
        Credits { opened_at: now }
    }

    pub fn key(&mut self, _k: KeyEvent) -> Action {
        Go::MainMenu.into()
    }

    pub fn draw(&self, f: &mut Frame, now: u64) {
        let th = Theme::theatre();
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        let inner = modal(
            f,
            centered(area, 64, 22),
            "CREDITS",
            &th,
            th.border,
            "press any key to return",
        );
        let k = (now.saturating_sub(self.opened_at) as f32 / 600.0).min(1.0);
        let text = Style::new().fg(rgb(scale(th.text, k)));
        let head = Style::new()
            .fg(rgb(scale((0, 255, 127), k)))
            .add_modifier(Modifier::BOLD);
        let lines = vec![
            shimmer("TERMINAL THEATRE", now, th.border),
            Line::styled(
                "An Interactive ASCII Experience",
                Style::new().fg(rgb(scale(th.mote, k))),
            ),
            Line::raw(""),
            Line::styled("Directed by: Terminal Theatre Development Team", text),
            Line::styled("Lead Story Architect: Noir Narrative Collective", text),
            Line::styled("Engine & Renderer: Rust + Ratatui", text),
            Line::styled("ASCII & Pixel Art Direction: Retro Terminal Artists", text),
            Line::raw(""),
            Line::styled("Special Thanks:", head),
            Line::styled("— Film noir classics for eternal inspiration", text),
            Line::styled("— The players who keep the spotlight alive", text),
            Line::styled("— You, for stepping onto this stage", text),
        ];
        f.render_widget(Paragraph::new(lines).centered(), inner);
    }
}
