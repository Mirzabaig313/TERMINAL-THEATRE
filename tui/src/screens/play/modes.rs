//! Auto and skip modes, read-text tracking, quick save and quick load.

use super::*;

/// Skip mode moves on this often.
const SKIP_STEP_MS: u64 = 70;

impl Play {
    /// (scene, line) of the text on screen; line -1 is the narration.
    pub(super) fn read_key(&self) -> Option<(String, i64)> {
        let scene = self.runner.scene_id().to_string();
        match self.runner.phase() {
            Phase::Narration => Some((scene, NARRATION)),
            Phase::Line(i) => Some((scene, i as i64)),
            _ => None,
        }
    }

    /// The player has read this text: keep it for skip and the history.
    pub(super) fn remember(&mut self, ctx: &Ctx, key: (String, i64)) {
        let scene = &self.runner.pack.scenes[&key.0];
        let entry = if key.1 == NARRATION {
            Entry {
                who: None,
                text: self.runner.narration(),
                thought: false,
                color: None,
            }
        } else {
            let line = &scene.dialogue[key.1 as usize];
            let speaker = self.runner.pack.speaker(&line.who);
            Entry {
                who: Some(self.runner.pack.speaker_name(&line.who).to_string()),
                text: self.runner.line_text(key.1 as usize),
                thought: speaker.is_some_and(|s| s.thought),
                color: speaker
                    .and_then(|s| s.color.as_deref())
                    .and_then(|c| parse_hex(c).ok()),
            }
        };
        self.backlog.push(entry);
        if !self.rehearsing() {
            let _ = ctx.store.mark_seen(&self.runner.pack.id, &key.0, key.1);
        }
        self.seen.insert(key);
    }

    pub(super) fn toggle_mode(&mut self, m: Mode, now: u64) {
        self.mode = if self.mode == m { Mode::Normal } else { m };
        self.done_at = None;
        let label = match self.mode {
            Mode::Normal => "Auto and skip off",
            Mode::Auto => "Auto: lines move on by themselves",
            Mode::Skip => "Skip: racing through text you've read",
        };
        self.toast = Some(Toast {
            life_ms: 1200,
            ..Toast::new(label, (0, 212, 255), now)
        });
    }

    /// Auto and skip modes move the story along from `tick`.
    pub(super) fn run_mode(&mut self, ctx: &Ctx, now: u64) {
        let reading = matches!(self.runner.phase(), Phase::Narration | Phase::Line(_));
        match self.mode {
            Mode::Normal => {}
            Mode::Skip if !reading => self.mode = Mode::Normal,
            Mode::Skip => {
                if now.saturating_sub(self.last_skip) < SKIP_STEP_MS {
                    return;
                }
                let read = self.read_key().is_some_and(|k| self.seen.contains(&k));
                if !read && !ctx.settings.skip_unread {
                    self.mode = Mode::Normal;
                    self.toast = Some(Toast::new("Skip stopped: new text", (255, 176, 0), now));
                    return;
                }
                self.last_skip = now;
                if !self.runner.typing_done(now) {
                    self.runner.advance(now); // finish the line first
                }
                self.advance(ctx, now);
            }
            Mode::Auto if !reading => {}
            Mode::Auto => {
                if !self.runner.typing_done(now) {
                    return;
                }
                let done = *self.done_at.get_or_insert(now);
                let len = self.read_key().map(|k| self.text_len(&k)).unwrap_or(0) as u64;
                let wait = ctx.settings.auto_delay_ms + len * 25;
                if now.saturating_sub(done) >= wait {
                    self.advance(ctx, now);
                }
            }
        }
    }

    pub(super) fn text_len(&self, key: &(String, i64)) -> usize {
        let text = if key.1 == NARRATION {
            self.runner.narration()
        } else {
            self.runner.line_text(key.1 as usize)
        };
        text.chars().count()
    }

    pub(super) fn quick_save(&mut self, ctx: &Ctx, now: u64) {
        let result = ctx
            .store
            .save(&self.runner.pack, QUICKSAVE_SLOT, &self.runner.state, None);
        self.toast = Some(match result {
            Ok(_) => Toast::new("✓ Quicksaved (F9 to load)", (0, 255, 127), now),
            Err(e) => Toast::new(format!("✗ Quicksave failed: {e:#}"), (255, 0, 0), now),
        });
    }

    /// Jump back to this story's quicksave without leaving the screen.
    pub(super) fn quick_load(&mut self, ctx: &Ctx, now: u64) {
        let Ok(file) = ctx.store.load(&self.runner.pack.id, QUICKSAVE_SLOT) else {
            self.toast = Some(Toast::new(
                "No quicksave yet (F5 to make one)",
                (255, 165, 0),
                now,
            ));
            return;
        };
        let scene = file.state.current_scene.clone();
        if !self.runner.pack.scenes.contains_key(&scene) {
            self.toast = Some(Toast::new(
                "The quicksave's scene no longer exists",
                (255, 0, 0),
                now,
            ));
            return;
        }
        self.runner.state = file.state;
        self.runner.goto(&scene, now);
        self.actor = None;
        self.ending = None;
        self.mode = Mode::Normal;
        self.on_phase(Phase::Narration, now);
        self.toast = Some(Toast::new("✓ Quickloaded", (0, 255, 127), now));
    }
}
