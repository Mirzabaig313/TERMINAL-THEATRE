//! Text helpers shared by screens.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;
use theatre_engine::Rgb;
use theatre_engine::color::lerp;

use super::fx::rgb;

/// Text with the first `n` chars visible; the rest drawn invisibly so word-wrap never jumps.
pub fn typed(text: &str, n: usize, shown: Style, hidden: Style) -> Text<'static> {
    let mut count = 0;
    let lines = text
        .split('\n')
        .map(|l| {
            let len = l.chars().count();
            let vis = n.saturating_sub(count).min(len);
            count += len + 1;
            let (a, b): (String, String) =
                (l.chars().take(vis).collect(), l.chars().skip(vis).collect());
            Line::from(vec![Span::styled(a, shown), Span::styled(b, hidden)])
        })
        .collect::<Vec<_>>();
    Text::from(lines)
}

/// Rough wrapped line count (by characters) for sizing boxes.
pub fn wrapped_height(text: &str, width: usize) -> usize {
    text.split('\n')
        .map(|l| l.chars().count().max(1).div_ceil(width.max(1)))
        .sum()
}

/// "dark_archway" -> "Dark Archway"
pub fn pretty(id: &str) -> String {
    id.split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().chain(c).collect::<String>())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Letter-spaced title with a light sweep moving across it.
pub fn shimmer(title: &str, t_ms: u64, color: Rgb) -> Line<'static> {
    let spaced: Vec<char> = title.chars().flat_map(|c| [c, ' ']).collect();
    let spaced = &spaced[..spaced.len().saturating_sub(1)];
    let n = spaced.len() as f32;
    let sweep = (t_ms as f32 / 1800.0).fract() * (n + 20.0) - 10.0;
    let spans: Vec<Span> = spaced
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let glow = (1.0 - ((i as f32 - sweep).abs() / 6.0)).max(0.0);
            let col = lerp(color, (255, 245, 255), glow);
            Span::styled(
                c.to_string(),
                Style::new().fg(rgb(col)).add_modifier(Modifier::BOLD),
            )
        })
        .collect();
    Line::from(spans)
}

/// Bobbing ▼ in the bottom-right corner of a box.
pub fn continue_hint(f: &mut Frame, area: Rect, t_ms: u64, color: Rgb) {
    if area.width < 4 || area.height < 2 {
        return;
    }
    let bob = if (t_ms / 400).is_multiple_of(2) {
        "▼"
    } else {
        "▽"
    };
    let r = Rect {
        x: area.right() - 4,
        y: area.bottom() - 1,
        width: 1,
        height: 1,
    };
    f.render_widget(Paragraph::new(bob).style(Style::new().fg(rgb(color))), r);
}
