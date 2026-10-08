//! Rehearsal: trying out a story while writing it (`theatre rehearse`).
//! Story files reload when they change, `d` shows the story state, and
//! nothing is saved: no autosave, no read-text or endings records, no saves.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::*;

/// How often the story folder is checked for changes.
const WATCH_MS: u64 = 700;

pub(super) struct Rehearsal {
    dir: PathBuf,
    stamp: Stamp,
    checked: u64,
    /// the state panel is open
    pub(super) panel: bool,
}

/// Newest change time and number of files in the story folder.
#[derive(PartialEq)]
struct Stamp(Option<SystemTime>, usize);

fn stamp(dir: &Path) -> Stamp {
    let mut newest = None;
    let mut count = 0;
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let Ok(meta) = e.metadata() else { continue };
            if meta.is_dir() {
                todo.push(e.path());
            } else {
                count += 1;
                newest = newest.max(meta.modified().ok());
            }
        }
    }
    Stamp(newest, count)
}

impl Rehearsal {
    pub(super) fn new(dir: PathBuf) -> Self {
        Rehearsal {
            stamp: stamp(&dir),
            dir,
            checked: 0,
            panel: false,
        }
    }
}

impl Play {
    /// Turn this game into a rehearsal of the story in `dir`.
    pub fn rehearse(mut self, dir: PathBuf) -> Self {
        self.rehearsal = Some(Rehearsal::new(dir));
        self
    }

    pub fn rehearsing(&self) -> bool {
        self.rehearsal.is_some()
    }

    /// Reload the story when its files change. A broken edit keeps the old
    /// story running and says what's wrong.
    pub(super) fn watch(&mut self, now: u64) {
        let Some(r) = &mut self.rehearsal else { return };
        if now.saturating_sub(r.checked) < WATCH_MS {
            return;
        }
        r.checked = now;
        let fresh = stamp(&r.dir);
        if fresh == r.stamp {
            return;
        }
        r.stamp = fresh;
        let result = StoryPack::load(&r.dir)
            .map_err(|e| format!("{e:#}"))
            .and_then(|pack| self.runner.reload(pack, now));
        self.toast = Some(match result {
            Ok(()) => {
                // the scene starts over: its picture, effects and sounds too
                self.image = None;
                self.image_error = None;
                self.scene_fx_for = None;
                self.transition_for = None;
                self.on_phase(Phase::Narration, now);
                Toast::new("↻ Story reloaded", (0, 255, 127), now)
            }
            Err(e) => Toast {
                life_ms: 6000,
                ..Toast::new(format!("✗ Not reloaded: {e}"), (255, 80, 80), now)
            },
        });
    }

    /// Saving is off in a rehearsal; say so instead.
    pub(super) fn no_saving(&mut self, now: u64) {
        self.toast = Some(Toast::new(
            "Rehearsal: nothing is saved",
            (255, 176, 0),
            now,
        ));
    }

    /// Scene, counters, flags and items, in a box at the top right.
    pub(super) fn draw_state_panel(&self, f: &mut Frame, th: &Theme) {
        if !self.rehearsal.as_ref().is_some_and(|r| r.panel) {
            return;
        }
        let s = &self.runner.state;
        let label = |t: &str| Span::styled(t.to_string(), Style::new().fg(rgb(th.dim)));
        let value = |t: String| Span::styled(t, Style::new().fg(rgb(th.text)));
        let list = |v: Vec<String>| {
            if v.is_empty() {
                "-".to_string()
            } else {
                v.join(", ")
            }
        };
        let lines = vec![
            Line::from(vec![
                label("scene    "),
                value(self.runner.scene_id().into()),
            ]),
            Line::from(vec![
                label("counters "),
                value(list(
                    s.vars.iter().map(|(k, v)| format!("{k}={v}")).collect(),
                )),
            ]),
            Line::from(vec![
                label("flags    "),
                value(list(s.flags.iter().cloned().collect())),
            ]),
            Line::from(vec![label("items    "), value(list(s.items.clone()))]),
            Line::from(vec![
                label("visited  "),
                value(format!("{} scenes", s.visited.len())),
            ]),
        ];
        let area = f.area();
        let w = 54.min(area.width.saturating_sub(2));
        let rect = Rect {
            x: area.right().saturating_sub(w + 1),
            y: area.y + 2,
            width: w,
            height: 9.min(area.height),
        };
        f.render_widget(ratatui::widgets::Clear, rect);
        f.render_widget(
            Paragraph::new(lines).wrap(Wrap { trim: true }).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(rgb(th.border)))
                    .title(" STATE · d to hide ")
                    .style(Style::new().bg(rgb(th.panel))),
            ),
            rect,
        );
    }
}
