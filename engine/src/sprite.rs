//! Text-based pixel-art sprites with swappable face parts.
//!
//! A sprite file has a `base` grid of palette letters ('.' is transparent), and
//! `parts` (e.g. brows/eyes/mouth variants) stamped over the base to build an
//! expression. Palette letters listed under `cycles` animate through a list of
//! colors. Front-ends draw the [`Pixels`] that [`Sprite::compose`] returns.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::color::{Rgb, parse_hex};
use crate::rng::Rng;

/// Row-major pixels, `None` = transparent.
pub type Pixels = Vec<Vec<Option<Rgb>>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SpriteFile {
    #[allow(dead_code)]
    name: String,
    size: [usize; 2],
    base: String,
    palette: BTreeMap<String, String>,
    #[serde(default)]
    cycles: BTreeMap<String, CycleFile>,
    anim: Anim,
    #[serde(default)]
    parts: BTreeMap<String, BTreeMap<String, PartFile>>,
    expressions: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Deserialize)]
struct CycleFile {
    ms: u64,
    colors: Vec<String>,
}

#[derive(Deserialize)]
struct PartFile {
    at: [usize; 2],
    grid: String,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Anim {
    /// eyes variant shown while blinking
    pub blink_eyes: String,
    /// random gap between blinks, [min, max] ms
    pub blink_ms: [u64; 2],
    /// breathing period in ms (0 = off); rows above `breathe_from_row` bob by one pixel
    pub breathe_ms: u64,
    pub breathe_from_row: usize,
    /// mouth flap interval while talking (0 = no mouth animation)
    pub talk_ms: u64,
}

struct Part {
    at: (usize, usize),
    rows: Vec<Vec<char>>,
}

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    base: Vec<Vec<char>>,
    palette: BTreeMap<char, Rgb>,
    cycles: BTreeMap<char, (u64, Vec<Rgb>)>,
    pub anim: Anim,
    parts: BTreeMap<String, BTreeMap<String, Part>>,
    expressions: BTreeMap<String, BTreeMap<String, String>>,
}

fn grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

fn key(k: &str) -> Result<char> {
    let mut it = k.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => Ok(c),
        _ => bail!("palette key '{k}' must be a single character"),
    }
}

impl Sprite {
    pub fn load(path: &Path) -> Result<Self> {
        let src =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Self::parse(&src).with_context(|| format!("in {}", path.display()))
    }

    pub fn parse(src: &str) -> Result<Self> {
        let f: SpriteFile = toml::from_str(src)?;
        let [w, h] = f.size;
        let base = grid(&f.base);
        if base.len() != h || base.iter().any(|r| r.len() != w) {
            bail!("base grid must be {w}x{h}");
        }
        let mut palette = BTreeMap::new();
        for (k, v) in &f.palette {
            palette.insert(key(k)?, parse_hex(v)?);
        }
        let mut cycles = BTreeMap::new();
        for (k, c) in &f.cycles {
            let colors = c
                .colors
                .iter()
                .map(|s| parse_hex(s))
                .collect::<Result<Vec<_>>>()?;
            if colors.is_empty() {
                bail!("cycle '{k}' has no colors");
            }
            cycles.insert(key(k)?, (c.ms.max(1), colors));
        }
        let parts: BTreeMap<String, BTreeMap<String, Part>> = f
            .parts
            .into_iter()
            .map(|(group, variants)| {
                let v = variants
                    .into_iter()
                    .map(|(name, p)| {
                        (
                            name,
                            Part {
                                at: (p.at[0], p.at[1]),
                                rows: grid(&p.grid),
                            },
                        )
                    })
                    .collect();
                (group, v)
            })
            .collect();

        // every letter used must have a color
        let used = base.iter().flatten().chain(
            parts
                .values()
                .flat_map(|v| v.values())
                .flat_map(|p| p.rows.iter().flatten()),
        );
        for &c in used {
            if c != '.' && !palette.contains_key(&c) && !cycles.contains_key(&c) {
                bail!("letter '{c}' is not in the palette");
            }
        }
        // every expression must point at existing parts
        if !f.expressions.contains_key("neutral") {
            bail!("an expression named 'neutral' is required");
        }
        for (name, e) in &f.expressions {
            for (group, variant) in e {
                let group = if group == "talk" {
                    "mouth"
                } else {
                    group.as_str()
                };
                if !parts.get(group).is_some_and(|v| v.contains_key(variant)) {
                    bail!("expression '{name}' uses missing part {group}.{variant}");
                }
            }
        }
        if parts.contains_key("eyes") && !parts["eyes"].contains_key(&f.anim.blink_eyes) {
            bail!("blink_eyes '{}' is not an eyes part", f.anim.blink_eyes);
        }
        Ok(Sprite {
            width: w,
            height: h,
            base,
            palette,
            cycles,
            anim: f.anim,
            parts,
            expressions: f.expressions,
        })
    }

    pub fn has_expression(&self, name: &str) -> bool {
        self.expressions.contains_key(name)
    }

    pub fn expressions(&self) -> impl Iterator<Item = &str> {
        self.expressions.keys().map(String::as_str)
    }

    fn color(&self, c: char, t_ms: u64) -> Option<Rgb> {
        if let Some((ms, colors)) = self.cycles.get(&c) {
            return Some(colors[(t_ms / ms) as usize % colors.len()]);
        }
        self.palette.get(&c).copied()
    }

    /// Build the pixels for a pose at time `t_ms` (time drives color cycles).
    pub fn compose(&self, pose: &Pose, t_ms: u64) -> Pixels {
        let mut g = self.base.clone();
        let expr = self
            .expressions
            .get(&pose.expression)
            .or_else(|| self.expressions.get("neutral"));
        let mut chosen: BTreeMap<&str, &str> = BTreeMap::new();
        if let Some(e) = expr {
            for (group, variant) in e {
                if group != "talk" {
                    chosen.insert(group, variant);
                }
            }
            if pose.mouth_open
                && let Some(t) = e.get("talk")
            {
                chosen.insert("mouth", t);
            }
        }
        if pose.blinking {
            chosen.insert("eyes", &self.anim.blink_eyes);
        }
        for (group, variant) in chosen {
            if let Some(part) = self.parts.get(group).and_then(|v| v.get(variant)) {
                for (dy, row) in part.rows.iter().enumerate() {
                    for (dx, &ch) in row.iter().enumerate() {
                        let (x, y) = (part.at.0 + dx, part.at.1 + dy);
                        if ch != '.' && y < self.height && x < self.width {
                            g[y][x] = ch;
                        }
                    }
                }
            }
        }
        if pose.exhale && self.anim.breathe_from_row > 1 {
            for y in (1..self.anim.breathe_from_row.min(self.height)).rev() {
                g[y] = g[y - 1].clone();
            }
            g[0] = vec!['.'; self.width];
        }
        g.iter()
            .map(|row| {
                row.iter()
                    .map(|&c| if c == '.' { None } else { self.color(c, t_ms) })
                    .collect()
            })
            .collect()
    }
}

/// What a character is doing right now.
#[derive(Default, Clone)]
pub struct Pose {
    pub expression: String,
    pub blinking: bool,
    pub mouth_open: bool,
    pub exhale: bool,
}

/// Keeps a character alive on screen: blinking, breathing and mouth flaps.
pub struct Actor {
    pub sprite: String,
    pub pose: Pose,
    pub talking: bool,
    next_blink: u64,
    blink_until: u64,
}

impl Actor {
    pub fn new(sprite: &str, expression: &str) -> Self {
        Actor {
            sprite: sprite.to_string(),
            pose: Pose {
                expression: expression.to_string(),
                ..Pose::default()
            },
            talking: false,
            next_blink: 1500,
            blink_until: 0,
        }
    }

    pub fn update(&mut self, sprite: &Sprite, t_ms: u64, rng: &mut Rng) {
        let a = &sprite.anim;
        if t_ms >= self.next_blink {
            self.blink_until = t_ms + 140;
            // occasional double blink
            let gap = if rng.chance(0.2) {
                260
            } else {
                rng.range(a.blink_ms[0], a.blink_ms[1])
            };
            self.next_blink = t_ms + gap;
        }
        self.pose.blinking = t_ms < self.blink_until;
        self.pose.exhale = a.breathe_ms > 0 && (t_ms / a.breathe_ms) % 2 == 1;
        self.pose.mouth_open =
            self.talking && a.talk_ms > 0 && (t_ms / a.talk_ms).is_multiple_of(2);
    }
}
