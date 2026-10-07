//! History of everything read in this session, shown over the story on `h`.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};
use theatre_engine::Rgb;
use theatre_engine::color::scale;

use crate::render::text::wrapped_height;
use crate::render::ui::{centered, modal};
use crate::render::{Theme, rgb};

/// Oldest entries are dropped past this many.
const KEEP: usize = 500;

pub struct Entry {
    /// speaker name; None for narration
    pub who: Option<String>,
    pub text: String,
    pub thought: bool,
    pub color: Option<Rgb>,
}

#[derive(Default)]
pub struct Backlog {
    entries: Vec<Entry>,
}

impl Backlog {
    pub fn push(&mut self, e: Entry) {
        self.entries.push(e);
        if self.entries.len() > KEEP {
            self.entries.remove(0);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Draw the history; `scroll` counts rows up from the newest text.
    pub fn draw(&self, f: &mut Frame, th: &Theme, scroll: usize) {
        let area = f.area();
        let box_area = centered(
            area,
            area.width.saturating_sub(8).min(100),
            area.height.saturating_sub(4),
        );
        let inner = modal(
            f,
            box_area,
            "HISTORY",
            th,
            th.border,
            "↑/↓ PgUp/PgDn scroll · h or Esc close",
        );
        if self.entries.is_empty() {
            f.render_widget(
                Paragraph::new("Nothing read yet.").style(Style::new().fg(rgb(th.dim))),
                inner,
            );
            return;
        }
        let width = inner.width.max(1) as usize;
        let mut lines: Vec<Line> = Vec::new();
        let mut rows = 0;
        for e in &self.entries {
            let body = match (&e.who, e.thought) {
                (None, _) => e.text.clone(),
                (Some(_), true) => format!("({})", e.text),
                (Some(_), false) => format!("“{}”", e.text),
            };
            if let Some(who) = &e.who {
                let c = e.color.unwrap_or(th.accent);
                lines.push(Line::styled(
                    who.clone(),
                    Style::new().fg(rgb(c)).add_modifier(Modifier::BOLD),
                ));
                rows += 1;
            }
            let style = match (&e.who, e.thought) {
                (None, _) => Style::new().fg(rgb(scale(th.text, 0.8))),
                (Some(_), true) => Style::new().fg(rgb(th.text)).add_modifier(Modifier::ITALIC),
                _ => Style::new().fg(rgb(th.text)),
            };
            for l in body.split('\n') {
                lines.push(Line::from(Span::styled(l.to_string(), style)));
            }
            rows += wrapped_height(&body, width);
            lines.push(Line::raw(""));
            rows += 1;
        }
        let max_top = rows.saturating_sub(inner.height as usize);
        let top = max_top.saturating_sub(scroll.min(max_top));
        f.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .scroll((top as u16, 0)),
            inner,
        );
        if top > 0 {
            let r = Rect {
                y: inner.y,
                height: 1,
                x: inner.right().saturating_sub(3),
                width: 1,
            };
            f.render_widget(
                Paragraph::new("▲").style(Style::new().fg(rgb(th.accent))),
                r,
            );
        }
    }
}
