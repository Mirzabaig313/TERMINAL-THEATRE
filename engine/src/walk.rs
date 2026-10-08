//! Play every route through a story, carrying flags, items, counters and
//! visited scenes exactly as the runner does, and report what can and can't
//! happen: endings never reached, places where the player is stuck, choices
//! that never open, lines and openings never shown.
//!
//! Counters are capped just past the numbers the story compares them with, so
//! loops end; a story that adds up counters (`a + b >= 5`) may be judged
//! slightly differently than in play. Counters, flags, items and visited scenes
//! that no condition asks about are left out: they change nothing.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::logic::{self, Expr, Op};
use crate::pack::StoryPack;
use crate::scene::Scene;
use crate::state::State;

/// How often something was possible across all the states explored.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tally {
    pub yes: usize,
    pub no: usize,
}

impl Tally {
    fn add(&mut self, yes: bool) {
        if yes {
            self.yes += 1;
        } else {
            self.no += 1;
        }
    }
}

/// One step of a route: the scene and the choice taken there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub scene: String,
    pub choice: String,
}

#[derive(Debug, Default)]
pub struct Walk {
    /// distinct states explored
    pub states: usize,
    /// stopped at the limit before exploring everything
    pub truncated: bool,
    pub reached: BTreeSet<String>,
    /// (scene, choice index) → open / closed, for choices with a gate
    pub choices: BTreeMap<(String, usize), Tally>,
    /// (scene, line index) → shown / skipped, for lines with a condition
    pub lines: BTreeMap<(String, usize), Tally>,
    /// (scene, variant index) → times it was the opening used
    pub variants: BTreeMap<(String, usize), usize>,
    /// scenes where some state leaves no choice to take, with a route there
    pub stuck: BTreeMap<String, Vec<Step>>,
    /// shortest route to every scene reached
    routes: HashMap<String, Vec<Step>>,
}

/// The parts of a state that decide what happens next.
#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    scene: String,
    flags: BTreeSet<String>,
    items: BTreeSet<String>,
    vars: BTreeMap<String, i64>,
    visited: BTreeSet<String>,
}

/// Things every condition in the story reads.
#[derive(Default)]
struct Reads {
    visited: BTreeSet<String>,
    flags: BTreeSet<String>,
    items: BTreeSet<String>,
    /// numbers each counter is compared with
    thresholds: BTreeMap<String, Vec<i64>>,
    /// counters used in sums or on their own: any number in the story may matter
    loose: BTreeSet<String>,
    /// every number in the story's conditions
    numbers: Vec<i64>,
}

/// The value of an expression made only of numbers (`-2` is `0 - 2`).
fn constant(e: &Expr) -> Option<i64> {
    match e {
        Expr::Num(n) => Some(*n),
        Expr::Bin(a, Op::Add, b) => Some(constant(a)? + constant(b)?),
        Expr::Bin(a, Op::Sub, b) => Some(constant(a)? - constant(b)?),
        _ => None,
    }
}

fn collect(e: &Expr, r: &mut Reads) {
    if let Some(n) = constant(e) {
        r.numbers.push(n);
        return;
    }
    match e {
        Expr::Var(v) => {
            r.loose.insert(v.clone());
        }
        Expr::Visited(v) => {
            r.visited.insert(v.clone());
        }
        Expr::Flag(f) => {
            r.flags.insert(f.clone());
        }
        Expr::Item(i) => {
            r.items.insert(i.clone());
        }
        Expr::Not(e) => collect(e, r),
        Expr::Bin(a, op, b) if is_comparison(*op) => {
            // counter compared with a number: only that number matters
            match (&**a, &**b) {
                (Expr::Var(v), other) | (other, Expr::Var(v)) if constant(other).is_some() => {
                    let n = constant(other).unwrap_or(0);
                    r.numbers.push(n);
                    r.thresholds.entry(v.clone()).or_default().push(n);
                }
                _ => {
                    collect(a, r);
                    collect(b, r);
                }
            }
        }
        Expr::Bin(a, _, b) => {
            collect(a, r);
            collect(b, r);
        }
        _ => {}
    }
}

fn is_comparison(op: Op) -> bool {
    matches!(op, Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge)
}

impl Reads {
    fn of(pack: &StoryPack) -> Reads {
        let mut r = Reads::default();
        let mut read = |c: &str| {
            if let Ok(e) = logic::parse(c) {
                collect(&e, &mut r);
            }
        };
        for scene in pack.scenes.values() {
            scene.variants.iter().for_each(|v| read(&v.cond));
            scene
                .dialogue
                .iter()
                .filter_map(|l| l.cond.as_deref())
                .for_each(&mut read);
            scene
                .choices
                .iter()
                .filter_map(|c| c.cond.as_deref())
                .for_each(&mut read);
        }
        for c in pack.scenes.values().flat_map(|s| &s.choices) {
            r.flags.extend(c.any_flags.iter().cloned());
            r.items.extend(c.any_items.iter().cloned());
        }
        r
    }

    /// The range a counter is kept in: one past the numbers it's compared
    /// with, and 0 (where every counter starts). None: no condition reads it.
    fn range(&self, var: &str) -> Option<(i64, i64)> {
        let nums: &[i64] = if self.loose.contains(var) {
            &self.numbers
        } else {
            self.thresholds.get(var)?
        };
        let low = nums.iter().copied().min().unwrap_or(0).min(0) - 1;
        let high = nums.iter().copied().max().unwrap_or(0).max(0) + 1;
        Some((low, high))
    }
}

fn gated(c: &crate::scene::Choice) -> bool {
    c.cond.is_some() || !c.any_flags.is_empty() || !c.any_items.is_empty()
}

impl Walk {
    /// Explore up to `limit` distinct states from the story's start.
    pub fn run(pack: &StoryPack, limit: usize) -> Walk {
        let r = Reads::of(pack);
        let key = |s: &State| Key {
            scene: s.current_scene.clone(),
            flags: s.flags.clone(),
            items: s.items.iter().cloned().collect(),
            vars: s.vars.clone(),
            visited: s.visited.iter().cloned().collect(),
        };
        let cap = |s: &mut State| {
            // counters no condition reads change nothing that can happen
            s.vars.retain(|k, v| match r.range(k) {
                Some((low, high)) => {
                    *v = (*v).clamp(low, high);
                    true
                }
                None => false,
            });
            // only what conditions ask about matters, and keeps states few
            s.visited.retain(|v| r.visited.contains(v));
            s.flags.retain(|f| r.flags.contains(f));
            s.items.retain(|i| r.items.contains(i));
            s.choices.clear();
        };

        let mut walk = Walk::default();
        let mut seen = std::collections::HashSet::new();
        let mut queue = VecDeque::new();
        let mut start = State::default();
        let start_id = pack.meta.start.clone();
        start.enter(&start_id, &pack.scenes[&start_id]);
        cap(&mut start);
        seen.insert(key(&start));
        walk.routes.insert(start_id, Vec::new());
        queue.push_back((start, Vec::<Step>::new()));

        while let Some((state, route)) = queue.pop_front() {
            walk.states += 1;
            let id = state.current_scene.clone();
            let scene: &Scene = &pack.scenes[&id];
            walk.reached.insert(id.clone());
            walk.look(&id, scene, &state);

            let open: Vec<usize> = (0..scene.choices.len())
                .filter(|&i| state.allows(&scene.choices[i]))
                .collect();
            if open.is_empty() && !scene.ending && !scene.choices.is_empty() {
                walk.stuck
                    .entry(id.clone())
                    .or_insert_with(|| route.clone());
            }
            for i in open {
                let c = &scene.choices[i];
                let mut next = state.clone();
                next.apply(&c.set_flags, &c.add_items, &c.add, &c.set);
                next.enter(&c.goto, &pack.scenes[&c.goto]);
                cap(&mut next);
                if walk.states + queue.len() >= limit {
                    walk.truncated = true;
                    continue;
                }
                if seen.insert(key(&next)) {
                    let mut route = route.clone();
                    route.push(Step {
                        scene: id.clone(),
                        choice: c.text.clone(),
                    });
                    walk.routes
                        .entry(c.goto.clone())
                        .or_insert_with(|| route.clone());
                    queue.push_back((next, route));
                }
            }
        }
        walk
    }

    /// Record which choices, lines and openings this state allows.
    fn look(&mut self, id: &str, scene: &Scene, state: &State) {
        for (i, c) in scene.choices.iter().enumerate() {
            if gated(c) {
                self.choices
                    .entry((id.to_string(), i))
                    .or_default()
                    .add(state.allows(c));
            }
        }
        for (i, line) in scene.dialogue.iter().enumerate() {
            if line.cond.is_some() {
                self.lines
                    .entry((id.to_string(), i))
                    .or_default()
                    .add(logic::check(line.cond.as_deref(), state));
            }
        }
        if let Some(i) = scene
            .variants
            .iter()
            .position(|v| logic::check(Some(&v.cond), state))
        {
            *self.variants.entry((id.to_string(), i)).or_default() += 1;
        }
    }

    /// The shortest route from the start to a scene, if one was found.
    pub fn route(&self, scene: &str) -> Option<&[Step]> {
        self.routes.get(scene).map(Vec::as_slice)
    }

    /// Things that are certainly wrong: endings and scenes never reached,
    /// players left with no choice, gates that never open, text never shown.
    pub fn problems(&self, pack: &StoryPack) -> Vec<String> {
        let mut out = Vec::new();
        for id in pack.ending_ids() {
            if !self.reached.contains(id) {
                out.push(format!("ending '{id}' can't be reached"));
            }
        }
        for id in pack.scenes.keys() {
            let ending = pack.scenes[id].ending;
            if !ending && !self.reached.contains(id) {
                out.push(format!("scene '{id}' can't be reached"));
            }
        }
        for (id, route) in &self.stuck {
            out.push(format!(
                "scene '{id}': the player can be left with no choice to take (for example after {})",
                describe(route)
            ));
        }
        for ((id, i), t) in &self.choices {
            if t.yes == 0 {
                out.push(format!(
                    "scene '{id}': choice \"{}\" never opens",
                    pack.scenes[id].choices[*i].text
                ));
            }
        }
        for ((id, i), t) in &self.lines {
            if t.yes == 0 {
                out.push(format!("scene '{id}': line {} is never said", i + 1));
            }
        }
        for (id, scene) in &pack.scenes {
            if !self.reached.contains(id) {
                continue;
            }
            for i in 0..scene.variants.len() {
                if !self.variants.contains_key(&(id.clone(), i)) {
                    out.push(format!("scene '{id}': variant {} is never used", i + 1));
                }
            }
        }
        out
    }

    /// Things that may be intended but are worth a look: gates that are
    /// always open, conditional lines that are always said.
    pub fn warnings(&self, pack: &StoryPack) -> Vec<String> {
        let mut out = Vec::new();
        if self.truncated {
            out.push(format!(
                "stopped after {} states: the results may be incomplete",
                self.states
            ));
        }
        for ((id, i), t) in &self.choices {
            if t.no == 0 && t.yes > 0 {
                out.push(format!(
                    "scene '{id}': choice \"{}\" has a condition but is always open",
                    pack.scenes[id].choices[*i].text
                ));
            }
        }
        for ((id, i), t) in &self.lines {
            if t.no == 0 && t.yes > 0 {
                out.push(format!(
                    "scene '{id}': line {} has a condition but is always said",
                    i + 1
                ));
            }
        }
        out
    }
}

/// A route in words: `scene → "choice" → …`.
pub fn describe(route: &[Step]) -> String {
    if route.is_empty() {
        return "the start".into();
    }
    route
        .iter()
        .map(|s| format!("{} → \"{}\"", s.scene, s.choice))
        .collect::<Vec<_>>()
        .join(" → ")
}
