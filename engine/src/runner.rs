//! Playing a story: which scene, which line, which choice. Time is passed in by
//! the front-end (milliseconds), so this runs the same headless or on screen.

use crate::StoryPack;
use crate::logic;
use crate::scene::{Choice, Line, Mood, Scene};
use crate::state::{ChoiceRecord, State};

pub const NARRATION_MS_PER_CHAR: u64 = 16;
pub const DIALOGUE_MS_PER_CHAR: u64 = 30;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    /// scene text typing out
    Narration,
    /// dialogue line `n` typing out
    Line(usize),
    /// waiting for a choice
    Choose,
    /// an ending (or a scene with nowhere to go)
    End,
}

pub struct Runner {
    pub pack: StoryPack,
    pub state: State,
    /// typewriter speed multiplier: 1.0 normal, larger is slower, 0.0 shows text instantly
    pub speed: f32,
    scene: String,
    phase: Phase,
    phase_start: u64,
    scene_start: u64,
    skipped: bool,
    choice: usize,
}

impl Runner {
    pub fn new(pack: StoryPack, now: u64) -> Self {
        let start = pack.meta.start.clone();
        let mut r = Runner {
            pack,
            state: State::default(),
            speed: 1.0,
            scene: String::new(),
            phase: Phase::Narration,
            phase_start: now,
            scene_start: now,
            skipped: false,
            choice: 0,
        };
        r.goto(&start, now);
        r
    }

    /// Continue a saved game from the scene it was saved in.
    pub fn resume(pack: StoryPack, state: State, now: u64) -> Self {
        let scene = state.current_scene.clone();
        let mut r = Runner::new(pack, now);
        if r.pack.scenes.contains_key(&scene) {
            r.state = state;
            r.goto(&scene, now);
        }
        r
    }

    // ------------------------------------------------------------------ queries

    pub fn scene_id(&self) -> &str {
        &self.scene
    }

    pub fn scene(&self) -> &Scene {
        &self.pack.scenes[&self.scene]
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn phase_start(&self) -> u64 {
        self.phase_start
    }

    pub fn scene_start(&self) -> u64 {
        self.scene_start
    }

    pub fn mood(&self) -> Mood {
        self.pack.mood_of(&self.scene)
    }

    /// The scene's opening text as the player sees it: the first variant whose
    /// condition holds (or the plain text), with counters filled in.
    pub fn narration(&self) -> String {
        let scene = self.scene();
        let text = scene
            .variants
            .iter()
            .find(|v| logic::check(Some(&v.cond), &self.state))
            .map(|v| v.text.as_str())
            .unwrap_or(&scene.text);
        logic::fill(text, &self.state)
    }

    /// A dialogue line of the current scene as the player sees it.
    pub fn line_text(&self, i: usize) -> String {
        logic::fill(&self.scene().dialogue[i].line, &self.state)
    }

    /// The next dialogue line whose condition holds, after `after` (or from the start).
    fn next_line(&self, after: Option<usize>) -> Option<usize> {
        let start = after.map_or(0, |i| i + 1);
        let lines = &self.scene().dialogue;
        (start..lines.len()).find(|&i| logic::check(lines[i].cond.as_deref(), &self.state))
    }

    pub fn line(&self) -> Option<&Line> {
        match self.phase {
            Phase::Line(i) => self.scene().dialogue.get(i),
            _ => None,
        }
    }

    /// Choices the player may take now (all of them if conditions hide every one).
    pub fn choices(&self) -> Vec<&Choice> {
        let all = &self.scene().choices;
        let open: Vec<_> = all.iter().filter(|c| self.state.allows(c)).collect();
        if open.is_empty() {
            all.iter().collect()
        } else {
            open
        }
    }

    pub fn selected(&self) -> usize {
        self.choice
    }

    fn current_text(&self) -> Option<(String, u64)> {
        match self.phase {
            Phase::Narration => Some((self.narration(), NARRATION_MS_PER_CHAR)),
            Phase::Line(i) => Some((self.line_text(i), DIALOGUE_MS_PER_CHAR)),
            _ => None,
        }
    }

    /// Characters of the current text revealed at `now` (usize::MAX when finished).
    pub fn shown_chars(&self, now: u64) -> usize {
        match self.current_text() {
            Some(_) if self.skipped => usize::MAX,
            Some(_) if self.speed <= 0.0 => usize::MAX,
            Some((text, ms)) => {
                let ms = ((ms as f32 * self.speed).round() as u64).max(1);
                revealed(&text, now.saturating_sub(self.phase_start), ms)
            }
            None => usize::MAX,
        }
    }

    pub fn typing_done(&self, now: u64) -> bool {
        self.shown_chars(now) == usize::MAX
    }

    // ------------------------------------------------------------------ actions

    pub fn select(&mut self, i: usize) {
        self.choice = i.min(self.choices().len().saturating_sub(1));
    }

    pub fn select_prev(&mut self) {
        self.select(self.choice.saturating_sub(1));
    }

    pub fn select_next(&mut self) {
        self.select(self.choice + 1);
    }

    /// Player pressed "continue". Finishes typing first, otherwise moves on.
    /// Returns the new phase when it changed.
    pub fn advance(&mut self, now: u64) -> Option<Phase> {
        if !self.typing_done(now) {
            self.skipped = true;
            return None;
        }
        let scene = self.scene();
        let after_lines = if scene.ending || scene.choices.is_empty() {
            Phase::End
        } else {
            Phase::Choose
        };
        let next = match self.phase {
            // dialogue lines whose condition fails are skipped
            Phase::Narration => self.next_line(None).map_or(after_lines, Phase::Line),
            Phase::Line(i) => self.next_line(Some(i)).map_or(after_lines, Phase::Line),
            Phase::Choose => {
                let c = *self.choices().get(self.choice)?;
                let (goto, text) = (c.goto.clone(), c.text.clone());
                let (flags, items, add, set) = (
                    c.set_flags.clone(),
                    c.add_items.clone(),
                    c.add.clone(),
                    c.set.clone(),
                );
                self.state.apply(&flags, &items, &add, &set);
                self.state.choices.push(ChoiceRecord {
                    scene: self.scene.clone(),
                    index: self.choice,
                    text,
                });
                self.goto(&goto, now);
                return Some(self.phase);
            }
            Phase::End => return None,
        };
        self.set_phase(next, now);
        Some(next)
    }

    pub fn goto(&mut self, id: &str, now: u64) {
        self.scene = id.to_string();
        self.state.enter(id, &self.pack.scenes[id]);
        self.scene_start = now;
        self.set_phase(Phase::Narration, now);
    }

    pub fn set_phase(&mut self, p: Phase, now: u64) {
        self.phase = p;
        self.phase_start = now;
        self.skipped = false;
        self.choice = 0;
    }
}

/// How many characters of `text` are visible after `elapsed` ms, with pauses on
/// punctuation. usize::MAX once everything is shown.
pub fn revealed(text: &str, elapsed: u64, ms_per_char: u64) -> usize {
    let mut t = 0;
    let mut prev = ' ';
    for (i, c) in text.chars().enumerate() {
        t += ms_per_char
            + match prev {
                '.' | '!' | '?' => ms_per_char * 8,
                ',' | ';' | ':' => ms_per_char * 4,
                '\n' => ms_per_char * 6,
                _ => 0,
            };
        if t > elapsed {
            return i;
        }
        prev = c;
    }
    usize::MAX
}
