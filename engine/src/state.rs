//! The player's progress through one story.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

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
        self.flags.extend(scene.set_flags.iter().cloned());
        for item in &scene.add_items {
            if !self.items.contains(item) {
                self.items.push(item.clone());
            }
        }
        if !self.visited.iter().any(|v| v == id) {
            self.visited.push(id.to_string());
        }
    }

    pub fn allows(&self, c: &Choice) -> bool {
        if c.any_flags.is_empty() && c.any_items.is_empty() {
            return true;
        }
        c.any_flags.iter().any(|f| self.flags.contains(f))
            || c.any_items.iter().any(|i| self.items.contains(i))
    }
}
