//! Title sequence: the marquee is revealed left to right, flickers between neon
//! red and blue, then waits for any key.

use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use theatre_engine::Rng;
use theatre_engine::color::{lerp, scale};
use theatre_engine::save::GAME_VERSION;

use super::{Action, Go};
use crate::render::fx::{self, Particles};
use crate::render::{Theme, rgb};

pub const TAGLINES: [&str; 8] = [
    "Where every choice steals the spotlight",
    "An interactive ASCII experience",
    "Stories written in neon and noir",
    "Lights up. Curtains rise. Your turn.",
    "Drama unfolds in every line",
    "Choose wisely. The city is watching.",
    "From terminal to theatre – let the show begin",
    "The stage is yours, detective",
];

pub const QUOTES: [&str; 8] = [
    "“In the dark, every whisper is a confession.”",
    "“Tonight the rain writes the prologue.”",
    "“Every flicker of neon hides a secret.”",
    "“Curtains up. Fate takes a bow.”",
    "“Some stories you watch. This one, you live.”",
    "“A single choice can rewrite the whole act.”",
    "“Spotlights reveal more than they hide.”",
    "“Let the theatre of shadows begin.”",
];

const TITLE_ART: &str = "\
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║  ████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗ █████╗    ║
║  ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██╔══██╗   ║
║     ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║███████║   ║
║     ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██╔══██║   ║
║     ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║██║  ██║   ║
║     ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝   ║
║                                                              ║
║  ████████╗██╗  ██╗███████╗ █████╗ ████████╗██████╗ ███████╗  ║
║  ╚══██╔══╝██║  ██║██╔════╝██╔══██╗╚══██╔══╝██╔══██╗██╔════╝  ║
║     ██║   ███████║█████╗  ███████║   ██║   ██████╔╝█████╗    ║
║     ██║   ██╔══██║██╔══╝  ██╔══██║   ██║   ██╔══██╗██╔══╝    ║
║     ██║   ██║  ██║███████╗██║  ██║   ██║   ██║  ██║███████╗  ║
║     ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝  ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝";

const BACKDROP: &str = "\
  ☆                     ☆        ☆        ☆
        ╔════════════════════════════════════════════╗
        ║ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓ ║
        ║ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓▓▓ ▓ ║
        ╚════════════════════════════════════════════╝
  ☆            The orchestra tunes to a noir overture";

const REVEAL_MS_PER_COL: u64 = 30;
const FLICKER_MS: u64 = 120;
const FLICKERS: u64 = 6;

const NEON_RED: (u8, u8, u8) = (255, 0, 64);
const NEON_BLUE: (u8, u8, u8) = (0, 212, 255);
const AMBER: (u8, u8, u8) = (255, 176, 0);

pub struct Opening {
    start: u64,
    quote: &'static str,
    tagline: &'static str,
    skipped: bool,
    particles: Particles,
}

impl Opening {
    pub fn new(now: u64, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let quote = QUOTES[rng.range(0, QUOTES.len() as u64) as usize];
        let tagline = TAGLINES[rng.range(0, TAGLINES.len() as u64) as usize];
        Opening {
            start: now,
            quote,
            tagline,
            skipped: false,
            particles: Particles::new(60, &mut rng),
        }
    }

    fn reveal_end(&self) -> u64 {
        let cols = TITLE_ART
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0) as u64;
        cols * REVEAL_MS_PER_COL
    }

    fn done(&self, now: u64) -> bool {
        self.skipped
            || now.saturating_sub(self.start) > self.reveal_end() + FLICKERS * FLICKER_MS + 300
    }

    /// The marquee is still being revealed or flickering.
    pub fn busy(&self, now: u64) -> bool {
        !self.done(now)
    }

    pub fn tick(&mut self, dt: f32) {
        self.particles.update(dt);
    }

    pub fn key(&mut self, _k: KeyEvent, now: u64) -> Action {
        if self.done(now) {
            return Go::MainMenu.into();
        }
        self.skipped = true;
        Action::Stay
    }

    pub fn draw(&mut self, f: &mut Frame, now: u64) {
        let th = Theme::theatre();
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        self.particles
            .render(f.buffer_mut(), area, scale(AMBER, 0.6));

        let elapsed = if self.skipped {
            u64::MAX / 2
        } else {
            now.saturating_sub(self.start)
        };
        let title_h = TITLE_ART.lines().count() as u16;
        let backdrop_h = BACKDROP.lines().count() as u16;
        let show_backdrop = area.height >= title_h + backdrop_h + 10;
        let [top, _, backdrop, _, title, _, tagline, _, prompt, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(if show_backdrop { backdrop_h } else { 0 }),
            Constraint::Length(1),
            Constraint::Length(title_h),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        f.render_widget(
            Paragraph::new(format!("v{GAME_VERSION}   |   {}", self.quote))
                .style(Style::new().fg(rgb(th.dim)))
                .centered(),
            top,
        );
        if show_backdrop {
            let lines: Vec<Line> = BACKDROP
                .lines()
                .map(|l| Line::styled(l, Style::new().fg(rgb((74, 74, 84)))))
                .collect();
            f.render_widget(Paragraph::new(lines), center_block(backdrop, BACKDROP));
        }

        // reveal column by column, alternating neon colors while it draws, then flicker
        let col = (elapsed / REVEAL_MS_PER_COL) as usize;
        let after = elapsed.saturating_sub(self.reveal_end());
        let color = if after == 0 {
            if col.is_multiple_of(2) {
                NEON_RED
            } else {
                NEON_BLUE
            }
        } else if after < FLICKERS * FLICKER_MS {
            if (after / FLICKER_MS).is_multiple_of(2) {
                NEON_RED
            } else {
                NEON_BLUE
            }
        } else {
            // settled: a slow glow
            lerp(
                NEON_RED,
                (255, 120, 150),
                0.3 + 0.4 * fx::wave(now, 700.0, 5),
            )
        };
        let lines: Vec<Line> = TITLE_ART
            .lines()
            .map(|l| {
                let shown: String = l
                    .chars()
                    .enumerate()
                    .map(|(i, c)| if i <= col || c == ' ' { c } else { ' ' })
                    .collect();
                let frame = l.starts_with('╔') || l.starts_with('╚');
                let c = if frame { NEON_BLUE } else { color };
                Line::styled(shown, Style::new().fg(rgb(c)).add_modifier(Modifier::BOLD))
            })
            .collect();
        f.render_widget(Paragraph::new(lines), center_block(title, TITLE_ART));

        f.render_widget(
            Paragraph::new(self.tagline)
                .style(Style::new().fg(rgb(AMBER)).add_modifier(Modifier::ITALIC))
                .centered(),
            tagline,
        );
        if self.done(now) {
            let phase = (now / 500) % 3;
            let (c, m) = match phase {
                0 => ((0, 255, 127), Modifier::BOLD),
                1 => (NEON_RED, Modifier::BOLD),
                _ => ((128, 128, 128), Modifier::DIM),
            };
            f.render_widget(
                Paragraph::new("Press any key to start...")
                    .style(Style::new().fg(rgb(c)).add_modifier(m))
                    .centered(),
                prompt,
            );
        }
        let k = (now.saturating_sub(self.start) as f32 / 500.0).min(1.0);
        fx::fade(f.buffer_mut(), area, k);
    }
}

/// Rect for a block of text, horizontally centered in `area`.
fn center_block(area: Rect, text: &str) -> Rect {
    let w = text.lines().map(|l| l.chars().count()).max().unwrap_or(0) as u16;
    let w = w.min(area.width);
    Rect {
        x: area.x + (area.width - w) / 2,
        width: w,
        ..area
    }
}
