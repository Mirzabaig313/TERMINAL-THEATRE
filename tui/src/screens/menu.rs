//! Story select: every folder in stories/ shows up here automatically.

use std::collections::BTreeMap;
use std::path::Path;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use theatre_engine::color::scale;
use theatre_engine::library::{self, Entry};
use theatre_engine::progress::EndingRecord;
use theatre_engine::scene::Mood;
use theatre_engine::sprite::{Actor, Sprite};
use theatre_engine::{Rng, Store, StoryPack};

use super::{Action, Go};
use crate::render::fx::{self, Particles};
use crate::render::ui::{centered, modal};
use crate::render::{Theme, rgb, sprite, text};

/// A story's endings: every ending scene id, and the ones the player reached.
struct Endings {
    all: Vec<String>,
    found: Vec<EndingRecord>,
}

pub struct Menu {
    entries: Vec<Entry>,
    covers: BTreeMap<String, (Sprite, Actor)>,
    /// problems with story folders, shown at the bottom
    notes: Vec<String>,
    selected: usize,
    changed_at: u64,
    opened_at: u64,
    rng: Rng,
    particles: Particles,
    endings: BTreeMap<String, Endings>,
    /// the endings gallery of the selected story is open
    gallery: bool,
}

impl Menu {
    pub fn new(root: &Path, store: &Store, now: u64, note: Option<String>) -> Self {
        let mut notes: Vec<String> = note.into_iter().collect();
        let entries = match library::discover(root) {
            Ok((entries, errors)) => {
                notes.extend(errors.iter().map(|e| format!("{e:#}")));
                entries
            }
            Err(e) => {
                notes.push(format!("{e:#}"));
                Vec::new()
            }
        };
        let mut covers = BTreeMap::new();
        for e in &entries {
            if let Some(name) = &e.meta.cover {
                match Sprite::load(&e.dir.join("characters").join(format!("{name}.toml"))) {
                    Ok(s) => {
                        covers.insert(e.id.clone(), (s, Actor::new(name, "neutral")));
                    }
                    Err(err) => notes.push(format!("story '{}': {err:#}", e.id)),
                }
            }
        }
        let mut endings = BTreeMap::new();
        for e in &entries {
            match StoryPack::load(&e.dir) {
                Ok(pack) => {
                    let all = pack.ending_ids().into_iter().map(String::from).collect();
                    endings.insert(
                        e.id.clone(),
                        Endings {
                            all,
                            found: store.endings(&e.id),
                        },
                    );
                }
                Err(err) => notes.push(format!("{err:#}")),
            }
        }
        let mut rng = Rng::new(0xC0FFEE);
        let particles = Particles::new(70, &mut rng);
        Menu {
            entries,
            covers,
            notes,
            selected: 0,
            changed_at: now,
            opened_at: now,
            rng,
            particles,
            endings,
            gallery: false,
        }
    }

    /// "3 / 24" for a story, if it loaded.
    fn progress(&self, id: &str) -> Option<String> {
        self.endings
            .get(id)
            .map(|e| format!("{} / {}", e.found.len(), e.all.len()))
    }

    pub fn tick(&mut self, now: u64, dt: f32) {
        self.particles.update(dt);
        if let Some(e) = self.entries.get(self.selected)
            && let Some((s, a)) = self.covers.get_mut(&e.id)
        {
            a.update(s, now, &mut self.rng);
        }
    }

    pub fn key(&mut self, k: KeyEvent, now: u64) -> Action {
        let n = self.entries.len();
        if self.gallery {
            if matches!(
                k.code,
                KeyCode::Esc | KeyCode::Char('e') | KeyCode::Enter | KeyCode::Char('q')
            ) {
                self.gallery = false;
            }
            return Action::Stay;
        }
        match k.code {
            KeyCode::Char('e') if n > 0 => self.gallery = true,
            KeyCode::Char('q') | KeyCode::Esc => return Go::MainMenu.into(),
            KeyCode::Char(c @ '1'..='9') => {
                let i = c as usize - '1' as usize;
                if let Some(e) = self.entries.get(i) {
                    return Go::NewGame(e.dir.clone()).into();
                }
            }
            KeyCode::Up | KeyCode::Char('k') if n > 0 => {
                self.selected = (self.selected + n - 1) % n;
                self.changed_at = now;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab if n > 0 => {
                self.selected = (self.selected + 1) % n;
                self.changed_at = now;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(e) = self.entries.get(self.selected) {
                    return Go::NewGame(e.dir.clone()).into();
                }
            }
            _ => {}
        }
        Action::Stay
    }

    pub fn draw(&mut self, f: &mut Frame, now: u64) {
        let entry = self.entries.get(self.selected);
        let (mood, themes) = match entry {
            Some(e) => (e.meta.cover_mood, Some(&e.meta.themes)),
            None => (Mood::Mystery, None),
        };
        let th = Theme::for_mood(mood, themes.unwrap_or(&BTreeMap::new()));
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        self.particles.render(f.buffer_mut(), area, th.mote);

        let [header, _, body, notes] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(self.notes.len().min(4) as u16),
        ])
        .areas(area);
        f.render_widget(
            Paragraph::new(vec![
                Line::raw(""),
                text::shimmer("TERMINAL THEATRE", now, th.dim),
            ])
            .alignment(Alignment::Center),
            header,
        );

        let [list, _, show] = Layout::horizontal([
            Constraint::Length(34),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .areas(body);
        self.draw_list(f, list, &th);
        if let Some(e) = entry {
            self.draw_cover(f, show, &th, e, now);
        } else {
            f.render_widget(
                Paragraph::new("No stories found. Add a folder with a story.toml to stories/.")
                    .style(Style::new().fg(rgb(th.dim)))
                    .alignment(Alignment::Center),
                show,
            );
        }
        let lines: Vec<Line> = self
            .notes
            .iter()
            .map(|n| Line::styled(format!(" ⚠ {n}"), Style::new().fg(rgb(th.accent))))
            .collect();
        f.render_widget(Paragraph::new(lines), notes);

        if self.gallery
            && let Some(e) = self.entries.get(self.selected)
        {
            self.draw_gallery(f, &th, e);
        }
        let k = (now.saturating_sub(self.opened_at) as f32 / 700.0).min(1.0);
        fx::fade(f.buffer_mut(), area, k);
    }

    /// Every ending of the selected story: reached ones by name, the rest locked.
    fn draw_gallery(&self, f: &mut Frame, th: &Theme, e: &Entry) {
        let Some(endings) = self.endings.get(&e.id) else {
            return;
        };
        let area = f.area();
        let title = format!(
            "ENDINGS · {} · {} / {}",
            e.meta.title,
            endings.found.len(),
            endings.all.len()
        );
        let inner = modal(
            f,
            centered(
                area,
                area.width.saturating_sub(6).min(110),
                area.height.saturating_sub(4),
            ),
            &title,
            th,
            th.border,
            "e or Esc close",
        );
        let rows = inner.height.max(1) as usize;
        let columns = endings.all.len().div_ceil(rows).max(1);
        let col_w = inner.width / columns as u16;
        for (i, id) in endings.all.iter().enumerate() {
            let (col, row) = (i / rows, i % rows);
            let r = Rect {
                x: inner.x + col as u16 * col_w,
                y: inner.y + row as u16,
                width: col_w,
                height: 1,
            };
            let line = match endings.found.iter().find(|f| &f.scene == id) {
                Some(rec) => Line::from(vec![
                    Span::styled("✦ ", Style::new().fg(rgb(th.mote))),
                    Span::styled(
                        text::pretty(id),
                        Style::new().fg(rgb(th.text)).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(
                            "  {}{}",
                            rec.date_label(),
                            if rec.times > 1 {
                                format!(" · ×{}", rec.times)
                            } else {
                                String::new()
                            }
                        ),
                        Style::new().fg(rgb(th.dim)),
                    ),
                ]),
                None => Line::styled("·  ? ? ?", Style::new().fg(rgb(scale(th.dim, 0.6)))),
            };
            f.render_widget(Paragraph::new(line), r);
        }
    }

    fn draw_list(&self, f: &mut Frame, area: Rect, th: &Theme) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(rgb(th.border)))
            .title(Span::styled(
                " STORIES ",
                Style::new().fg(rgb(th.accent)).add_modifier(Modifier::BOLD),
            ))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(1, 1, 1, 0));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let mut lines = Vec::new();
        for (i, e) in self.entries.iter().enumerate() {
            let sel = i == self.selected;
            let style = if sel {
                Style::new().fg(rgb(th.text)).add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(rgb(th.dim))
            };
            lines.push(Line::from(vec![
                Span::styled(
                    if sel { "▶ " } else { "  " },
                    Style::new().fg(rgb(th.accent)),
                ),
                Span::styled(format!("[{}] ", i + 1), Style::new().fg(rgb((255, 215, 0)))),
                Span::styled(e.meta.title.clone(), style),
                Span::styled(
                    self.progress(&e.id)
                        .map(|p| format!("  ✦ {p}"))
                        .unwrap_or_default(),
                    Style::new().fg(rgb(scale(th.mote, if sel { 1.0 } else { 0.6 }))),
                ),
            ]));
            lines.push(Line::raw(""));
        }
        f.render_widget(Paragraph::new(lines), inner);
        let help = Rect {
            y: area.bottom().saturating_sub(2),
            height: 1,
            ..inner
        };
        f.render_widget(
            Paragraph::new("↑↓ 1-9 · enter play · e endings · esc")
                .style(Style::new().fg(rgb(scale(th.dim, 0.8)))),
            help,
        );
    }

    fn draw_cover(&self, f: &mut Frame, area: Rect, th: &Theme, e: &Entry, now: u64) {
        let blurb = e.meta.blurb.as_deref().unwrap_or(&e.meta.description);
        let desc_h = text::wrapped_height(blurb, area.width.min(80) as usize) as u16;
        let [_, art, _, title, summary, _, desc, _, prompt, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(20),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(desc_h),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);
        let k = (now.saturating_sub(self.changed_at) as f32 / 600.0).min(1.0);
        if let Some((s, a)) = self.covers.get(&e.id) {
            sprite::draw(f.buffer_mut(), art, s, &a.pose, now, 0, k);
        }
        f.render_widget(
            Paragraph::new(text::shimmer(&e.meta.title, now, th.accent))
                .alignment(Alignment::Center),
            title,
        );
        if let Some(sum) = &e.meta.summary {
            f.render_widget(
                Paragraph::new(sum.as_str())
                    .style(
                        Style::new()
                            .fg(rgb(scale(th.mote, k)))
                            .add_modifier(Modifier::ITALIC),
                    )
                    .alignment(Alignment::Center),
                summary,
            );
        }
        let w = area.width.min(80);
        let desc = Rect {
            x: desc.x + (desc.width - w) / 2,
            width: w,
            ..desc
        };
        f.render_widget(
            Paragraph::new(blurb)
                .style(Style::new().fg(rgb(scale(th.text, k * 0.8))))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            desc,
        );
        let pulse = 0.55 + 0.45 * fx::wave(now, 450.0, 4);
        f.render_widget(
            Paragraph::new(match self.progress(&e.id) {
                Some(p) => format!("▶  press ENTER to begin  ·  E endings found ({p})"),
                None => "▶  press ENTER to begin".to_string(),
            })
            .style(Style::new().fg(rgb(scale(th.text, pulse))))
            .alignment(Alignment::Center),
            prompt,
        );
    }
}
