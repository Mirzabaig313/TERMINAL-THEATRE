//! Color themes per scene mood.

use std::collections::BTreeMap;

use theatre_engine::pack::ThemeOverride;
use theatre_engine::scene::Mood;

#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: (u8, u8, u8),
    pub panel: (u8, u8, u8),
    pub border: (u8, u8, u8),
    pub text: (u8, u8, u8),
    pub dim: (u8, u8, u8),
    pub accent: (u8, u8, u8),
    pub mote: (u8, u8, u8),
}

impl Theme {
    /// The theatre's own look for menus: neon red and blue on black, amber dust.
    pub fn theatre() -> Theme {
        Theme {
            bg: (8, 8, 12),
            panel: (14, 13, 20),
            border: (0, 212, 255),
            text: (224, 224, 224),
            dim: (128, 128, 136),
            accent: (255, 0, 64),
            mote: (255, 176, 0),
        }
    }

    /// Built-in theme for a mood, with a story's overrides applied.
    pub fn for_mood(m: Mood, overrides: &BTreeMap<Mood, ThemeOverride>) -> Theme {
        let mut t = Self::builtin(m);
        if let Some(o) = overrides.get(&m) {
            let slots = [
                &mut t.bg,
                &mut t.panel,
                &mut t.border,
                &mut t.text,
                &mut t.dim,
                &mut t.accent,
                &mut t.mote,
            ];
            for (slot, c) in slots.into_iter().zip(o.colors()) {
                if let Some(c) = c {
                    *slot = c;
                }
            }
        }
        t
    }

    fn builtin(m: Mood) -> Theme {
        match m {
            Mood::Noir => Theme {
                bg: (10, 10, 14),
                panel: (16, 15, 21),
                border: (70, 64, 52),
                text: (206, 202, 214),
                dim: (112, 108, 124),
                accent: (212, 175, 55),
                mote: (150, 130, 70),
            },
            Mood::Danger => Theme {
                bg: (14, 6, 8),
                panel: (22, 9, 12),
                border: (110, 30, 38),
                text: (232, 214, 214),
                dim: (130, 96, 100),
                accent: (230, 57, 70),
                mote: (190, 60, 50),
            },
            Mood::Alert => Theme {
                bg: (14, 10, 4),
                panel: (22, 16, 8),
                border: (120, 82, 36),
                text: (234, 222, 204),
                dim: (136, 118, 96),
                accent: (244, 162, 97),
                mote: (210, 140, 70),
            },
            Mood::Calm => Theme {
                bg: (6, 10, 16),
                panel: (10, 16, 25),
                border: (44, 78, 108),
                text: (208, 222, 234),
                dim: (98, 118, 138),
                accent: (95, 168, 211),
                mote: (90, 150, 200),
            },
            Mood::Mystery => Theme {
                bg: (10, 6, 18),
                panel: (16, 10, 28),
                border: (78, 44, 120),
                text: (222, 212, 236),
                dim: (118, 104, 140),
                accent: (157, 78, 221),
                mote: (140, 90, 220),
            },
        }
    }
}
