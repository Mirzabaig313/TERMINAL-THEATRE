//! The pre-show cinematic: six captioned frames, skippable with any key.

use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};
use theatre_engine::Rgb;
use theatre_engine::color::scale;
use theatre_engine::runner::revealed;

use super::{Action, Go};
use crate::render::text::typed;
use crate::render::ui::centered;
use crate::render::{Theme, rgb};

struct Frame_ {
    art: &'static str,
    caption: &'static str,
    ms: u64,
    accent: Rgb,
}

const RED: Rgb = (255, 0, 64);
const BLUE: Rgb = (0, 212, 255);
const AMBER: Rgb = (255, 176, 0);
const ORANGE: Rgb = (255, 165, 0);

const FRAMES: [Frame_; 6] = [
    Frame_ {
        art: "\
╔══════════════════════════════════════════════════════════╗
║ ██╗    ██╗██████╗  ██████╗ ██╗███████╗   ╔════════════╗  ║
║ ██║    ██║██╔══██╗██╔════╝ ██║██╔════╝   ║  CURTAIN   ║  ║
║ ██║ █╗ ██║██████╔╝██║  ███╗██║█████╗     ║    CALL    ║  ║
║ ██║███╗██║██╔═══╝ ██║   ██║██║██╔══╝     ╚════════════╝  ║
║ ╚███╔███╔╝██║     ╚██████╔╝██║███████╗                   ║
╚══════════════════════════════════════════════════════════╝
          Velvet curtains draw back in rich crimson",
        caption: "The house lights fade. A hush settles over the theatre...",
        ms: 2400,
        accent: RED,
    },
    Frame_ {
        art: "\
            ⋱⋰        ☆      ⋱⋰        ☆       ⋱⋰

             ╔══════════════════════════════════╗
             ║       A CITY OF SHADOWS          ║
             ║    Rain-slick neon boulevards    ║
             ╚══════════════════════════════════╝

        Streetlights flicker. Footsteps echo in alleys.",
        caption: "Somewhere downtown, a saxophone cries against the rain.",
        ms: 2600,
        accent: BLUE,
    },
    Frame_ {
        art: "\
    ╭────────────────────────────────────────────────────╮
    │  MALONE INVESTIGATIONS                             │
    │  Venetian blinds cut the moonlight into ribbons    │
    ╰────────────────────────────────────────────────────╯

       A silhouette leans over a desk, fedora brim low.
            Cigarette smoke sketches anxious spirals.",
        caption: "Jack Malone watches the city breathe in secrets and exhale lies.",
        ms: 2500,
        accent: AMBER,
    },
    Frame_ {
        art: "\
        ╔════════════════════════════════════════════╗
        ║  PROJECT NIGHTFALL - TOP SECRET            ║
        ║  Names. Accounts. Deadline: Midnight.      ║
        ╚════════════════════════════════════════════╝

   Pages rustle like thunder. A storm is about to break.",
        caption: "Someone is tying off loose ends. You're next on the list.",
        ms: 2200,
        accent: ORANGE,
    },
    Frame_ {
        art: "\
              ╔══════════════════════════════╗
              ║  THE RITZ THEATRE - STAGE 47 ║
              ║  Velvet seats. Empty stage.  ║
              ╚══════════════════════════════╝

      A single spotlight. Dust dances like falling snow.",
        caption: "Someone waits in the dark with answers... or a bullet.",
        ms: 2400,
        accent: RED,
    },
    Frame_ {
        art: "\
         ╔══════════════════════════════════════════╗
         ║   ✦  TERMINAL THEATRE PRESENTS  ✦        ║
         ║   AN EVENING OF INTERACTIVE STORIES      ║
         ║    NOIR · NEON · NIGHTMARE               ║
         ╚══════════════════════════════════════════╝

        The curtain trembles. Somewhere, a revolver clicks.",
        caption: "Choose your story. The stage is waiting.",
        ms: 2600,
        accent: BLUE,
    },
];

const FADE_MS: u64 = 350;

pub struct Cinematic {
    start: u64,
    skipped_at: Option<u64>,
}

impl Cinematic {
    pub fn new(now: u64) -> Self {
        Cinematic {
            start: now,
            skipped_at: None,
        }
    }

    /// Which frame is showing and for how long; None once it's over.
    fn current(&self, now: u64) -> Option<(usize, u64)> {
        let mut t = now.saturating_sub(self.start);
        for (i, fr) in FRAMES.iter().enumerate() {
            if t < fr.ms {
                return Some((i, t));
            }
            t -= fr.ms;
        }
        None
    }

    /// Moves on by itself when the reel ends or after the "skipped" note.
    pub fn tick(&mut self, now: u64) -> Action {
        match self.skipped_at {
            Some(t) if now - t > 1200 => Go::StorySelect.into(),
            None if self.current(now).is_none() => Go::StorySelect.into(),
            _ => Action::Stay,
        }
    }

    pub fn key(&mut self, _k: KeyEvent, now: u64) -> Action {
        if self.skipped_at.is_some() {
            return Go::StorySelect.into();
        }
        self.skipped_at = Some(now);
        Action::Stay
    }

    pub fn draw(&self, f: &mut Frame, now: u64) {
        let th = Theme::theatre();
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        if self.skipped_at.is_some() {
            f.render_widget(
                Paragraph::new("Cinematic skipped. The show must go on!")
                    .style(Style::new().fg(rgb(scale(th.border, 0.7))))
                    .centered(),
                centered(area, area.width, 1),
            );
            return;
        }
        let Some((i, t)) = self.current(now) else {
            return;
        };
        let fr = &FRAMES[i];
        // fade in and out at the frame edges
        let k = (t as f32 / FADE_MS as f32)
            .min((fr.ms - t) as f32 / FADE_MS as f32)
            .clamp(0.0, 1.0);

        let art_h = fr.art.lines().count() as u16 + 4;
        let art_w = fr.art.lines().map(|l| l.chars().count()).max().unwrap_or(0) as u16 + 10;
        let [_, art, _, caption, _, prompt] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(art_h),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(area);
        let r = centered(art, art_w, art_h);
        let lines: Vec<Line> = fr
            .art
            .lines()
            .map(|l| {
                Line::styled(
                    l,
                    Style::new()
                        .fg(rgb(scale(fr.accent, k)))
                        .add_modifier(Modifier::BOLD),
                )
            })
            .collect();
        f.render_widget(
            Paragraph::new(lines).block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(rgb(scale(fr.accent, k * 0.8))))
                    .padding(Padding::new(4, 4, 1, 1)),
            ),
            r,
        );
        let n = revealed(fr.caption, t.saturating_sub(FADE_MS), 22);
        let text = typed(
            fr.caption,
            n,
            Style::new()
                .fg(rgb(scale((176, 176, 176), k)))
                .add_modifier(Modifier::ITALIC),
            Style::new().fg(rgb(th.bg)),
        );
        f.render_widget(Paragraph::new(text).centered(), caption);
        f.render_widget(
            Paragraph::new("Press any key to skip")
                .style(Style::new().fg(rgb(th.dim)))
                .centered(),
            prompt,
        );
    }
}
