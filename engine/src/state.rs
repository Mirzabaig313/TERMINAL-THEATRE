//! The player's progress through one story.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::logic;
use crate::scene::{Choice, Scene};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct State {
    /// scene the player is in (where a loaded game resumes)
    pub current_scene: String,
    pub flags: BTreeSet<String>,
    pub items: Vec<String>,
    pub visited: Vec<String>,
    pub choices: Vec<ChoiceRecord>,
    /// time spent playing, in milliseconds
    pub playtime_ms: u64,
    /// story counters (trust, evidence…); missing ones are 0
    pub vars: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRecord {
    pub scene: String,
    pub index: usize,
    pub text: String,
}

impl State {
    pub fn enter(&mut self, id: &str, scene: &Scene) {
        self.current_scene = id.to_string();
        self.apply(&scene.set_flags, &scene.add_items, &scene.add, &scene.set);
        if !self.visited.iter().any(|v| v == id) {
            self.visited.push(id.to_string());
        }
    }

    /// Flags, items and counter changes from a scene or a choice.
    pub fn apply(
        &mut self,
        flags: &[String],
        items: &[String],
        add: &BTreeMap<String, i64>,
        set: &BTreeMap<String, i64>,
    ) {
        self.flags.extend(flags.iter().cloned());
        for item in items {
            if !self.items.contains(item) {
                self.items.push(item.clone());
            }
        }
        for (k, v) in set {
            self.vars.insert(k.clone(), *v);
        }
        for (k, v) in add {
            let e = self.vars.entry(k.clone()).or_insert(0);
            *e = e.saturating_add(*v);
        }
    }

    /// May the player see this choice now?
    pub fn allows(&self, c: &Choice) -> bool {
        let listed = (c.any_flags.is_empty() && c.any_items.is_empty())
            || c.any_flags.iter().any(|f| self.flags.contains(f))
            || c.any_items.iter().any(|i| self.items.contains(i));
        listed && logic::check(c.cond.as_deref(), self)
    }
}
