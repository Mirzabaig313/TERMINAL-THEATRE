//! Small widgets shared by screens: panels, centered boxes, toasts, option lists.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph};
use theatre_engine::Rgb;
use theatre_engine::color::scale;

use super::{Theme, rgb};

/// A `w`×`h` box centered in `area` (clamped to it).
pub fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let (w, h) = (w.min(area.width), h.min(area.height));
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 2,
        width: w,
        height: h,
    }
}

/// Rounded panel with a title in the accent color.
pub fn panel<'a>(title: &'a str, th: &Theme, border: Rgb) -> Block<'a> {
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(rgb(border)))
        .title(Span::styled(
            format!(" {title} "),
            Style::new().fg(rgb(th.accent)).add_modifier(Modifier::BOLD),
        ))
        .style(Style::new().bg(rgb(th.panel)).fg(rgb(th.text)))
        .padding(Padding::new(2, 2, 1, 1))
}

/// A modal box: clears what's under it and draws the panel. Returns the inner area.
pub fn modal(
    f: &mut Frame,
    area: Rect,
    title: &str,
    th: &Theme,
    border: Rgb,
    footer: &str,
) -> Rect {
    f.render_widget(Clear, area);
    let block = panel(title, th, border).title_bottom(Line::styled(
        format!(" {footer} "),
        Style::new().fg(rgb(th.dim)),
    ));
    let inner = block.inner(area);
    f.render_widget(block, area);
    inner
}

/// A short message near the bottom of `area` that fades out over `life_ms`.
pub struct Toast {
    pub text: String,
    pub color: Rgb,
    pub at: u64,
    pub life_ms: u64,
}

impl Toast {
    pub fn new(text: impl Into<String>, color: Rgb, now: u64) -> Self {
        Toast {
            text: text.into(),
            color,
            at: now,
            life_ms: 2200,
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let age = now.saturating_sub(self.at);
        if age >= self.life_ms {
            return;
        }
        let k = 1.0 - (age as f32 / self.life_ms as f32).powi(3);
        let w = (self.text.chars().count() as u16 + 6).min(area.width);
        let r = Rect {
            x: area.x + (area.width - w) / 2,
            y: area.bottom().saturating_sub(3),
            width: w,
            height: 3,
        };
        f.render_widget(Clear, r);
        f.render_widget(
            Paragraph::new(self.text.as_str())
                .style(Style::new().fg(rgb(scale(self.color, k))).bg(rgb(th.panel)))
                .centered()
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(rgb(scale(self.color, k * 0.7)))),
                ),
            r,
        );
    }
}

/// One row of a menu: `▶ 1. Label        description`.
pub fn option_line(
    th: &Theme,
    index: Option<usize>,
    label: &str,
    detail: &str,
    selected: bool,
    label_w: usize,
) -> Line<'static> {
    let pointer = if selected { "▶ " } else { "  " };
    let num = index.map(|i| format!("{}. ", i + 1)).unwrap_or_default();
    let (ls, ds) = if selected {
        (
            Style::new().fg(rgb(th.text)).add_modifier(Modifier::BOLD),
            Style::new()
                .fg(rgb(th.border))
                .add_modifier(Modifier::ITALIC),
        )
    } else {
        (
            Style::new().fg(rgb(scale(th.text, 0.75))),
            Style::new().fg(rgb(th.dim)),
        )
    };
    Line::from(vec![
        Span::styled(pointer, Style::new().fg(rgb(th.accent))),
        Span::styled(num, Style::new().fg(rgb((255, 215, 0)))),
        Span::styled(format!("{label:<label_w$}"), ls),
        Span::raw("  "),
        Span::styled(detail.to_string(), ds),
    ])
}
