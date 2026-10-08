//! Things drawn over the story that take the keys: the intro card, pause
//! menu, save dialog and history.

use super::*;

const SAVE_NAME_MAX: usize = 32;

const PAUSE_ITEMS: [(&str, &str); 5] = [
    ("Resume", "Back to the story"),
    ("Save Game", "Keep this moment in a slot"),
    ("History", "Read everything again (h)"),
    ("Main Menu", "Leave this story"),
    ("Quit", "Close the theatre"),
];
/// Pause menu rows.
const P_RESUME: usize = 0;
const P_SAVE: usize = 1;
const P_HISTORY: usize = 2;
const P_MENU: usize = 3;

impl Play {
    /// Keys while an overlay is open. `None` when no overlay took the key.
    pub(super) fn overlay_key(&mut self, k: KeyEvent, ctx: &Ctx, now: u64) -> Option<Action> {
        let rehearsing = self.rehearsing();
        match &mut self.overlay {
            Overlay::None => None,
            Overlay::Intro { .. } => {
                if matches!(k.code, KeyCode::Esc) {
                    return Some(Go::StorySelect.into());
                }
                self.overlay = Overlay::None;
                // restart the scene clock so its entrance plays now
                let scene = self.runner.scene_id().to_string();
                self.runner.goto(&scene, now);
                self.on_phase(Phase::Narration, now);
                self.autosave(ctx, now);
                Some(Action::Stay)
            }
            Overlay::Pause { selected } => {
                let n = PAUSE_ITEMS.len();
                match k.code {
                    KeyCode::Up | KeyCode::Char('k') => *selected = (*selected + n - 1) % n,
                    KeyCode::Down | KeyCode::Char('j') => *selected = (*selected + 1) % n,
                    KeyCode::Esc => self.overlay = Overlay::None,
                    KeyCode::Enter | KeyCode::Char(' ') => match *selected {
                        P_RESUME => self.overlay = Overlay::None,
                        P_SAVE if rehearsing => {
                            self.overlay = Overlay::None;
                            self.no_saving(now);
                        }
                        P_SAVE => {
                            // read once: drawing must not query the database every frame
                            self.slots = ctx.store.slots(&self.runner.pack.id);
                            self.overlay = Overlay::SaveSlots { selected: 0 };
                        }
                        P_HISTORY => self.overlay = Overlay::Backlog { scroll: 0 },
                        P_MENU => self.overlay = Overlay::ConfirmLeave,
                        _ => return Some(Action::Quit),
                    },
                    _ => {}
                }
                Some(Action::Stay)
            }
            Overlay::ConfirmLeave => {
                if matches!(k.code, KeyCode::Char('y') | KeyCode::Char('Y')) {
                    return Some(Go::MainMenu.into());
                }
                self.overlay = Overlay::Pause { selected: P_MENU };
                Some(Action::Stay)
            }
            Overlay::Backlog { scroll } => {
                match k.code {
                    KeyCode::Up | KeyCode::Char('k') => *scroll += 1,
                    KeyCode::Down | KeyCode::Char('j') => *scroll = scroll.saturating_sub(1),
                    KeyCode::PageUp => *scroll += 10,
                    KeyCode::PageDown => *scroll = scroll.saturating_sub(10),
                    KeyCode::Esc | KeyCode::Char('h') | KeyCode::Enter | KeyCode::Char('q') => {
                        self.overlay = Overlay::None
                    }
                    _ => {}
                }
                Some(Action::Stay)
            }
            Overlay::SaveSlots { selected } => {
                let n = MAX_SLOT as usize;
                let mut sel = *selected;
                match k.code {
                    KeyCode::Up | KeyCode::Char('k') => sel = (sel + n - 1) % n,
                    KeyCode::Down | KeyCode::Char('j') => sel = (sel + 1) % n,
                    KeyCode::Char(c @ '1'..='9') => sel = c as usize - '1' as usize,
                    KeyCode::Esc => {
                        self.overlay = Overlay::Pause { selected: P_SAVE };
                        return Some(Action::Stay);
                    }
                    _ => {}
                }
                *selected = sel;
                if matches!(
                    k.code,
                    KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('1'..='9')
                ) {
                    let slot = sel as u8 + 1;
                    let existing = ctx
                        .store
                        .load(&self.runner.pack.id, slot)
                        .ok()
                        .map(|f| f.metadata.name);
                    let text = existing.unwrap_or_else(|| format!("Save {slot}"));
                    self.overlay = Overlay::SaveName { slot, text };
                }
                Some(Action::Stay)
            }
            Overlay::SaveName { slot, text } => {
                match k.code {
                    KeyCode::Esc => {
                        self.overlay = Overlay::SaveSlots {
                            selected: *slot as usize - 1,
                        }
                    }
                    KeyCode::Backspace => {
                        text.pop();
                    }
                    KeyCode::Char(c) if text.chars().count() < SAVE_NAME_MAX => text.push(c),
                    KeyCode::Enter => {
                        let (slot, name) = (*slot, text.clone());
                        self.toast = Some(
                            match ctx.store.save(
                                &self.runner.pack,
                                slot,
                                &self.runner.state,
                                Some(&name),
                            ) {
                                Ok(m) => Toast::new(
                                    format!("✓ Game saved to slot {slot}: {}", m.name),
                                    (0, 255, 127),
                                    now,
                                ),
                                Err(e) => {
                                    Toast::new(format!("✗ Failed to save: {e:#}"), (255, 0, 0), now)
                                }
                            },
                        );
                        self.overlay = Overlay::None;
                    }
                    _ => {}
                }
                Some(Action::Stay)
            }
        }
    }

    /// Text of the title card: the description, or a resume summary.
    fn intro_body(&self, resumed: Option<&str>) -> String {
        match resumed {
            Some(name) => format!(
                "Resuming your story...\n\n\"{name}\" · {} · {} played",
                pretty(self.runner.scene_id()),
                format_playtime(self.runner.state.playtime_ms)
            ),
            None => self.runner.pack.meta.description.clone(),
        }
    }

    /// Characters of the title card shown at `now` (usize::MAX when done).
    pub(super) fn intro_shown(&self, resumed: Option<&str>, since: u64, now: u64) -> usize {
        if self.runner.speed <= 0.0 {
            return usize::MAX;
        }
        let elapsed = now.saturating_sub(since).saturating_sub(400);
        let ms = ((20.0 * self.runner.speed).round() as u64).max(1);
        revealed(&self.intro_body(resumed), elapsed, ms)
    }

    /// Story title card: the description types out for a new game.
    pub(super) fn draw_intro(
        &self,
        f: &mut Frame,
        th: &Theme,
        resumed: Option<&str>,
        since: u64,
        now: u64,
    ) {
        let area = f.area();
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), area);
        let meta = &self.runner.pack.meta;
        let body = self.intro_body(resumed);
        let w = area.width.min(84);
        let body_h = wrapped_height(&body, w as usize) as u16;
        let [_, rule1, title, rule2, _, text, _, prompt, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(body_h),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);
        let rule = "═".repeat(60.min(area.width as usize));
        for r in [rule1, rule2] {
            f.render_widget(
                Paragraph::new(rule.as_str())
                    .style(Style::new().fg(rgb(th.border)))
                    .centered(),
                r,
            );
        }
        f.render_widget(
            Paragraph::new(shimmer(&meta.title, now, th.accent)).centered(),
            title,
        );
        let shown = self.intro_shown(resumed, since, now);
        let text_area = Rect {
            x: text.x + (text.width - w) / 2,
            width: w,
            ..text
        };
        f.render_widget(
            Paragraph::new(typed(
                &body,
                shown,
                Style::new().fg(rgb(th.text)),
                Style::new().fg(rgb(th.bg)),
            ))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
            text_area,
        );
        if shown == usize::MAX {
            let pulse = 0.55 + 0.45 * fx::wave(now, 450.0, 4);
            f.render_widget(
                Paragraph::new("Press ENTER to continue...  ·  Esc to go back")
                    .style(Style::new().fg(rgb(scale(th.dim, pulse * 1.3))))
                    .centered(),
                prompt,
            );
        }
        fx::fade(
            f.buffer_mut(),
            area,
            (now.saturating_sub(since) as f32 / 600.0).min(1.0),
        );
    }

    pub(super) fn draw_overlay(&self, f: &mut Frame, th: &Theme, now: u64) {
        let area = f.area();
        match &self.overlay {
            Overlay::None | Overlay::Intro { .. } => {}
            Overlay::Backlog { scroll } => self.backlog.draw(f, th, *scroll),
            Overlay::Pause { selected } => {
                let inner = modal(
                    f,
                    centered(area, 58, 13),
                    "PAUSED",
                    th,
                    th.border,
                    "↑/↓ · Enter · Esc resume",
                );
                let mut lines = vec![Line::styled(
                    format!(
                        "{} · {} · {}",
                        self.runner.pack.meta.title,
                        pretty(self.runner.scene_id()),
                        format_playtime(self.runner.state.playtime_ms)
                    ),
                    Style::new().fg(rgb(th.dim)),
                )];
                lines.push(Line::raw(""));
                for (i, (label, desc)) in PAUSE_ITEMS.iter().enumerate() {
                    lines.push(option_line(th, None, label, desc, i == *selected, 10));
                    lines.push(Line::raw(""));
                }
                f.render_widget(Paragraph::new(lines), inner);
            }
            Overlay::ConfirmLeave => {
                let inner = modal(
                    f,
                    centered(area, 58, 9),
                    "RETURN TO MAIN MENU",
                    th,
                    (255, 165, 0),
                    "Y / N",
                );
                f.render_widget(
                    Paragraph::new(vec![
                        Line::raw("Are you sure?"),
                        Line::raw(""),
                        Line::styled(
                            "Progress since your last save is lost. The autosave keeps the start of this scene.",
                            Style::new().fg(rgb(th.dim)),
                        ),
                    ])
                    .centered()
                    .wrap(Wrap { trim: true }),
                    inner,
                );
            }
            Overlay::SaveSlots { selected } => {
                let inner = modal(
                    f,
                    centered(area, 90, 24),
                    "SAVE GAME",
                    th,
                    th.border,
                    "↑/↓ or 1-9 · Enter choose · Esc back",
                );
                let slots = &self.slots;
                let mut lines = Vec::new();
                for slot in 1..=MAX_SLOT {
                    let i = slot as usize - 1;
                    let (label, detail) = match slots.get(slot as usize).and_then(Option::as_ref) {
                        Some(m) => (
                            m.name.clone(),
                            format!(
                                "{} · {} · {}",
                                m.scene_description,
                                m.time_label(),
                                format_playtime(m.playtime_ms)
                            ),
                        ),
                        None => ("[Empty Slot]".to_string(), String::new()),
                    };
                    lines.push(option_line(
                        th,
                        Some(i),
                        &label,
                        &detail,
                        i == *selected,
                        24,
                    ));
                    lines.push(Line::raw(""));
                }
                f.render_widget(Paragraph::new(lines), inner);
            }
            Overlay::SaveName { slot, text } => {
                let inner = modal(
                    f,
                    centered(area, 60, 9),
                    &format!("SAVE TO SLOT {slot}"),
                    th,
                    th.border,
                    "Enter save · Esc back",
                );
                let cursor = if (now / 500).is_multiple_of(2) {
                    "▌"
                } else {
                    " "
                };
                f.render_widget(
                    Paragraph::new(vec![
                        Line::styled("Name this save:", Style::new().fg(rgb(th.dim))),
                        Line::raw(""),
                        Line::from(vec![
                            Span::styled(
                                text.clone(),
                                Style::new().fg(rgb(th.text)).add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(cursor, Style::new().fg(rgb(th.accent))),
                        ]),
                    ]),
                    inner,
                );
            }
        }
    }
}
