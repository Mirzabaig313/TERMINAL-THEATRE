//! Drawing the scene: header, stage art or narration, portraits, dialogue,
//! choices, the ending card, and screen effects.

use super::*;

const PORTRAIT_ENTER_MS: u64 = 350;
const PORTRAIT_W: u16 = 34;
const BOTTOM_H: u16 = 20;

impl Play {
    pub fn draw(&mut self, f: &mut Frame, ctx: &Ctx, now: u64) {
        let th = Theme::for_mood(self.runner.mood(), &self.runner.pack.meta.themes);
        if let Overlay::Intro { resumed, since } = &self.overlay {
            self.draw_intro(f, &th, resumed.as_deref(), *since, now);
            return;
        }
        let full = f.area();
        let strength = ctx.settings.effects.strength();
        let area = self.shaken(full, now, strength);
        f.render_widget(Block::new().style(Style::new().bg(rgb(th.bg))), full);
        self.particles.render(f.buffer_mut(), full, th.mote);

        let [header, stage, bottom] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(BOTTOM_H),
        ])
        .areas(area);
        self.draw_header(f, header, &th);
        self.draw_stage(f, stage, &th, now);
        let graphics = is_graphics(&ctx.picker);
        if !graphics && let Some(img) = &mut self.image {
            img.draw(f, stage_inner(stage));
        }
        self.draw_bottom(f, bottom, &th, now);

        // line and scene effects, scaled by the Screen Effects setting
        let since = now.saturating_sub(self.fx_start);
        if self.fx_kind == Some(LineFx::Glitch) && since < 450 && strength > 0.0 {
            let k = (1.0 - since as f32 / 450.0) * strength;
            fx::glitch(f.buffer_mut(), full, &mut self.rng, k, th.accent);
        }
        if let Some(kind) = self.fx_pending.take() {
            self.screen_fx =
                effects::screen_effect(kind, &th, strength).map(|e| Running::new(e, self.fx_start));
        }
        if let Some(effect) = &mut self.screen_fx
            && !effect.apply(f.buffer_mut(), full, now)
        {
            self.screen_fx = None;
        }
        if now < self.glitch_until && strength >= 1.0 {
            fx::glitch(f.buffer_mut(), full, &mut self.rng, 0.5, th.accent);
        }
        // scene entrance
        let start = self.runner.scene_start();
        if self.transition_for != Some(start) {
            self.transition_for = Some(start);
            let kind = self.runner.pack.transition_of(self.runner.scene_id());
            // timed from the scene's start, so a late first draw catches up
            self.transition = effects::transition(kind, &th).map(|e| Running::new(e, start));
        }
        if let Some(effect) = &mut self.transition
            && !effect.apply(f.buffer_mut(), full, now)
        {
            self.transition = None;
        }
        // real images go on top of everything, unshaken and outside the effects,
        // and only once the scene has settled: every redraw resends the image
        if graphics
            && self.transition.is_none()
            && matches!(self.overlay, Overlay::None)
            && let Some(img) = &mut self.image
        {
            let [_, still_stage, _] = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(BOTTOM_H),
            ])
            .areas(full);
            img.draw(f, stage_inner(still_stage));
        }
        self.draw_overlay(f, &th, now);
        if let Some(t) = &self.toast {
            t.draw(f, full, &th, now);
        }
    }

    /// The stage shows art or a picture, so the narration goes in the bottom box.
    fn stage_has_picture(&self, now: u64) -> bool {
        self.image.is_some()
            || self
                .runner
                .scene()
                .art_at(now.saturating_sub(self.runner.scene_start()))
                .is_some()
    }

    /// Screen shake: full is 2 cells for 400 ms, reduced 1 cell for 150 ms, off none.
    pub(super) fn shaken(&mut self, area: Rect, now: u64, strength: f32) -> Rect {
        let since = now.saturating_sub(self.fx_start);
        let length = if strength >= 1.0 { 400 } else { 150 };
        if self.fx_kind != Some(LineFx::Shake)
            || strength <= 0.0
            || since > length
            || area.width < 4
        {
            return area;
        }
        let amp = if since < 200 && strength >= 1.0 { 2 } else { 1 };
        let dx = self.rng.range(0, amp * 2 + 1) as i32 - amp as i32;
        let x = (area.x as i32 + dx).max(0) as u16;
        Rect {
            x,
            width: area.width - amp as u16,
            ..area
        }
    }

    pub(super) fn draw_header(&self, f: &mut Frame, area: Rect, th: &Theme) {
        let line = Line::from(vec![
            Span::styled(
                format!(" {} ", self.runner.pack.meta.title),
                Style::new()
                    .fg(rgb(th.bg))
                    .bg(rgb(th.accent))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}", pretty(self.runner.scene_id())),
                Style::new().fg(rgb(th.dim)).add_modifier(Modifier::ITALIC),
            ),
        ]);
        f.render_widget(Paragraph::new(line), area);
        let mut right = Vec::new();
        let badge = match self.mode {
            Mode::Normal => None,
            Mode::Auto => Some(" AUTO ▶ "),
            Mode::Skip => Some(" SKIP ▶▶ "),
        };
        if let Some(b) = badge {
            right.push(Span::styled(
                b,
                Style::new()
                    .fg(rgb(th.bg))
                    .bg(rgb(th.border))
                    .add_modifier(Modifier::BOLD),
            ));
            right.push(Span::raw("  "));
        }
        if !self.runner.state.items.is_empty() {
            right.push(Span::raw(format!(
                "⚔ {}  ",
                self.runner.state.items.join(" · ")
            )));
        }
        right.push(Span::raw(
            "h history · a auto · s skip · F5/F9 quick · esc menu ",
        ));
        f.render_widget(
            Paragraph::new(Line::from(right))
                .style(Style::new().fg(rgb(th.dim)))
                .alignment(Alignment::Right),
            area,
        );
    }

    pub(super) fn draw_stage(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let scene = self.runner.scene();
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(rgb(th.border)))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::horizontal(3));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let phase = self.runner.phase();
        if self.image.is_some() {
            return; // the picture is drawn by `draw`
        }

        let Some(art) = scene.art_at(now.saturating_sub(self.runner.scene_start())) else {
            // no art: the narration lives on the stage and stays (dimmed) during dialogue
            let w = inner.width.min(90);
            let narration = self.runner.narration();
            let h = (wrapped_height(&narration, w as usize) as u16).min(inner.height);
            let r = Rect {
                x: inner.x + (inner.width - w) / 2,
                y: inner.y + (inner.height - h) / 2,
                width: w,
                height: h,
            };
            let (shown, col) = match phase {
                Phase::Narration => (self.runner.shown_chars(now), th.text),
                _ => (usize::MAX, lerp(th.text, th.panel, 0.45)),
            };
            let text = typed(
                &narration,
                shown,
                Style::new().fg(rgb(col)),
                Style::new().fg(rgb(th.panel)),
            );
            f.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), r);
            if phase == Phase::Narration && self.runner.typing_done(now) {
                continue_hint(f, area, now, th.accent);
            }
            return;
        };

        let lines: Vec<&str> = art.trim_matches('\n').lines().collect();
        // crop tall art around its middle
        let skip = lines.len().saturating_sub(inner.height as usize) / 2;
        let lines = &lines[skip..lines.len().min(skip + inner.height as usize)];
        let art_h = lines.len() as u16;
        let since = now.saturating_sub(self.runner.scene_start()) as f32;
        let styled: Vec<Line> = lines
            .iter()
            .enumerate()
            .map(|(i, l)| {
                // vertical gradient, slowly breathing
                let k = i as f32 / art_h.max(1) as f32;
                let col = scale(lerp(th.accent, th.dim, k), (since / 1200.0).min(1.0));
                Line::styled(*l, Style::new().fg(rgb(col)))
            })
            .collect();
        let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as u16;
        let r = Rect {
            x: inner.x + inner.width.saturating_sub(w) / 2,
            y: inner.y + (inner.height - art_h) / 2,
            width: w.min(inner.width),
            height: art_h,
        };
        f.render_widget(Paragraph::new(styled), r);
    }

    pub(super) fn draw_bottom(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let phase = self.runner.phase();
        let has_portrait = self.actor.is_some() && matches!(phase, Phase::Line(_) | Phase::Choose);
        let [portrait, box_col] = Layout::horizontal([
            Constraint::Length(if has_portrait { PORTRAIT_W } else { 0 }),
            Constraint::Fill(1),
        ])
        .areas(area);

        if has_portrait
            && let Some(actor) = &self.actor
            && let Some(s) = self.runner.pack.sprites.get(&actor.sprite)
        {
            let k =
                (now.saturating_sub(self.actor_since) as f32 / PORTRAIT_ENTER_MS as f32).min(1.0);
            let ease = 1.0 - (1.0 - k).powi(3);
            let dx = ((1.0 - ease) * -12.0) as i32;
            sprite::draw(f.buffer_mut(), portrait, s, &actor.pose, now, dx, ease);
        }

        let box_h = 9.min(area.height);
        let dbox = Rect {
            y: box_col.bottom() - box_h,
            height: box_h,
            ..box_col
        };
        match phase {
            Phase::Line(_) => self.draw_line(f, dbox, th, now),
            Phase::Choose => self.draw_choices(f, box_col, th, now),
            Phase::End => self.draw_end(f, box_col, th, now),
            Phase::Narration if self.stage_has_picture(now) => {
                self.draw_narration(f, area, th, now)
            }
            Phase::Narration => {}
        }
    }

    /// Narration box at the bottom, used when the stage is showing art.
    pub(super) fn draw_narration(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(rgb(th.border)))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(3, 3, 1, 1));
        let w = block.inner(area).width as usize;
        let narration = self.runner.narration();
        let h = (wrapped_height(&narration, w) as u16 + 4).min(area.height);
        let area = Rect {
            y: area.bottom() - h,
            height: h,
            ..area
        };
        let inner = block.inner(area);
        f.render_widget(block, area);
        let text = typed(
            &narration,
            self.runner.shown_chars(now),
            Style::new().fg(rgb(th.text)),
            Style::new().fg(rgb(th.panel)),
        );
        f.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), inner);
        if self.runner.typing_done(now) {
            continue_hint(f, area, now, th.accent);
        }
    }

    pub(super) fn draw_line(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let Some(line) = self.runner.line() else {
            return;
        };
        let pack = &self.runner.pack;
        let speaker = pack.speaker(&line.who);
        let thought = speaker.is_some_and(|s| s.thought);
        let own = speaker
            .and_then(|s| s.color.as_deref())
            .and_then(|c| parse_hex(c).ok());
        let accent = match (thought, own) {
            (_, Some(c)) if !thought => c,
            (true, Some(c)) => scale(c, 0.7),
            (true, None) => th.dim,
            _ => th.accent,
        };
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::new().fg(rgb(accent)))
            .title(Span::styled(
                format!(" {} ", pack.speaker_name(&line.who)),
                Style::new()
                    .fg(rgb(th.bg))
                    .bg(rgb(accent))
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(2, 2, 1, 0));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let mut style = Style::new().fg(rgb(th.text));
        if thought {
            style = style.add_modifier(Modifier::ITALIC);
        }
        let said = match self.runner.phase() {
            Phase::Line(i) => self.runner.line_text(i),
            _ => line.line.clone(),
        };
        let quoted = if thought {
            format!("({said})")
        } else {
            format!("“{said}”")
        };
        // +1 for the opening quote/paren
        let shown = self.runner.shown_chars(now).saturating_add(1);
        f.render_widget(
            Paragraph::new(typed(&quoted, shown, style, Style::new().fg(rgb(th.panel))))
                .wrap(Wrap { trim: false }),
            inner,
        );
        if self.runner.typing_done(now) {
            continue_hint(f, area, now, th.accent);
        }
    }

    pub(super) fn draw_choices(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let choices = self.runner.choices();
        let h = (choices.len() as u16 * 2 + 3).min(area.height);
        let area = Rect {
            y: area.bottom() - h,
            height: h,
            ..area
        };
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(rgb(th.accent)))
            .title(Span::styled(
                " WHAT DO YOU DO? ",
                Style::new().fg(rgb(th.accent)).add_modifier(Modifier::BOLD),
            ))
            .style(Style::new().bg(rgb(th.panel)))
            .padding(Padding::new(2, 2, 1, 0));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let since = now.saturating_sub(self.runner.phase_start());
        let mut lines = Vec::new();
        for (i, c) in choices.iter().enumerate() {
            // choices slide in one after another
            let appear = ((since as f32 - i as f32 * 90.0) / 250.0).clamp(0.0, 1.0);
            let selected = i == self.runner.selected();
            let pulse = 0.75 + 0.25 * fx::wave(now, 300.0, 3);
            let col = if selected {
                scale(th.accent, pulse)
            } else {
                th.dim
            };
            let mut style =
                Style::new().fg(rgb(scale(if selected { th.text } else { col }, appear)));
            if selected {
                style = style.add_modifier(Modifier::BOLD);
            }
            lines.push(Line::from(vec![
                Span::raw(" ".repeat(((1.0 - appear) * 6.0) as usize)),
                Span::styled(
                    if selected { "▶ " } else { "  " },
                    Style::new().fg(rgb(th.accent)),
                ),
                Span::styled(
                    format!("{}. ", i + 1),
                    Style::new().fg(rgb(scale(th.dim, appear))),
                ),
                Span::styled(c.text.clone(), style),
            ]));
            lines.push(Line::raw(""));
        }
        f.render_widget(Paragraph::new(lines), inner);
    }

    pub(super) fn draw_end(&self, f: &mut Frame, area: Rect, th: &Theme, now: u64) {
        let k = (now.saturating_sub(self.runner.phase_start()) as f32 / 1200.0).min(1.0);
        let st = &self.runner.state;
        let total = self.runner.pack.scenes.len().max(1);
        let lines = vec![
            Line::styled(
                "·  T H E   E N D  ·",
                Style::new()
                    .fg(rgb(scale(th.accent, k)))
                    .add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
            Line::styled(
                "Thank you for playing!",
                Style::new().fg(rgb(scale(th.text, k))),
            ),
            Line::styled(
                format!("Ending: {}", pretty(self.runner.scene_id())),
                Style::new()
                    .fg(rgb(scale(th.mote, k)))
                    .add_modifier(Modifier::ITALIC),
            ),
            match self.ending {
                Some((new, found, all)) => Line::styled(
                    format!(
                        "{}Endings found: {found} / {all}",
                        if new {
                            "✦ New ending unlocked!  ·  "
                        } else {
                            ""
                        }
                    ),
                    Style::new()
                        .fg(rgb(scale(th.border, k)))
                        .add_modifier(Modifier::BOLD),
                ),
                None => Line::raw(""),
            },
            Line::styled(
                format!(
                    "{} played · {} choices · {:.0}% of the story seen",
                    format_playtime(st.playtime_ms),
                    st.choices.len(),
                    st.visited.len() as f32 / total as f32 * 100.0
                ),
                Style::new().fg(rgb(scale(th.dim, k))),
            ),
            Line::raw(""),
            Line::styled(
                "press ENTER to choose another story",
                Style::new().fg(rgb(scale(th.dim, k))),
            ),
        ];
        let h = lines.len() as u16;
        let r = Rect {
            y: area.y + area.height.saturating_sub(h) / 2,
            height: h.min(area.height),
            ..area
        };
        f.render_widget(Paragraph::new(lines).alignment(Alignment::Center), r);
    }
}

/// Inside of the stage box (border and side padding), where art and pictures go.
fn stage_inner(stage: Rect) -> Rect {
    Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(3))
        .inner(stage)
}
