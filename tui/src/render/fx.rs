//! Post-processing effects applied to the rendered buffer, plus ambient particles.

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use theatre_engine::Rng;
use theatre_engine::color::{lerp, scale};

pub fn rgb(c: (u8, u8, u8)) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

fn map_colors(buf: &mut Buffer, area: Rect, f: impl Fn((u8, u8, u8)) -> (u8, u8, u8)) {
    let area = area.intersection(buf.area);
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, y)];
            if let Color::Rgb(r, g, b) = cell.fg {
                cell.fg = rgb(f((r, g, b)));
            }
            if let Color::Rgb(r, g, b) = cell.bg {
                cell.bg = rgb(f((r, g, b)));
            }
        }
    }
}

/// Nearest color in the xterm 256-color palette (6×6×6 cube or gray ramp).
pub fn ansi256((r, g, b): (u8, u8, u8)) -> u8 {
    let level = |v: u8| {
        if v < 48 {
            0
        } else if v < 115 {
            1
        } else {
            (v as u16 - 35) as u8 / 40
        }
    };
    let (cr, cg, cb) = (level(r), level(g), level(b));
    let step = |l: u8| if l == 0 { 0 } else { 55 + 40 * l as i32 };
    let cube_err =
        (step(cr) - r as i32).pow(2) + (step(cg) - g as i32).pow(2) + (step(cb) - b as i32).pow(2);
    let avg = (r as i32 + g as i32 + b as i32) / 3;
    let gray = ((avg - 8).max(0) / 10).min(23);
    let gv = 8 + 10 * gray;
    let gray_err = (gv - r as i32).pow(2) + (gv - g as i32).pow(2) + (gv - b as i32).pow(2);
    if gray_err < cube_err {
        232 + gray as u8
    } else {
        16 + 36 * cr + 6 * cg + cb
    }
}

/// For terminals without 24-bit color: turn every RGB color into its nearest 256-color index.
pub fn downgrade_256(buf: &mut Buffer, area: Rect) {
    let area = area.intersection(buf.area);
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, y)];
            if let Color::Rgb(r, g, b) = cell.fg {
                cell.fg = Color::Indexed(ansi256((r, g, b)));
            }
            if let Color::Rgb(r, g, b) = cell.bg {
                cell.bg = Color::Indexed(ansi256((r, g, b)));
            }
        }
    }
}

/// Idle animations advance in steps of this many ms, so most frames are
/// identical and the terminal receives nothing new (editor terminals are slow
/// to draw, and a fully animated screen floods them).
pub const ANIM_STEP_MS: u64 = 125;

/// A slow 0..1 wave for glows and pulses, stepped in time and in `levels`.
pub fn wave(now: u64, period_ms: f32, levels: u32) -> f32 {
    let t = (now / ANIM_STEP_MS * ANIM_STEP_MS) as f32;
    let v = 0.5 + 0.5 * (t / period_ms).sin();
    let n = levels.max(2) as f32 - 1.0;
    (v * n).round() / n
}

/// Monochrome mode: every color becomes its gray.
pub fn grayscale(buf: &mut Buffer, area: Rect) {
    map_colors(buf, area, |(r, g, b)| {
        let l = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        (l, l, l)
    });
}

/// Fade towards black: k = 0 is black, 1 is untouched.
pub fn fade(buf: &mut Buffer, area: Rect, k: f32) {
    if k < 1.0 {
        map_colors(buf, area, |c| scale(c, k));
    }
}

/// Wash towards a color: t = 0 is untouched.
pub fn flash(buf: &mut Buffer, area: Rect, to: (u8, u8, u8), t: f32) {
    if t > 0.0 {
        map_colors(buf, area, |c| lerp(c, to, t));
    }
}

/// Cells appear in a scattered order as k goes from 0 to 1.
pub fn dissolve(buf: &mut Buffer, area: Rect, k: f32, bg: (u8, u8, u8), seed: u64) {
    if k >= 1.0 {
        return;
    }
    let area = area.intersection(buf.area);
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let mut h = Rng::new(seed ^ ((x as u64) << 32) ^ (y as u64 * 2_654_435_761));
            h.next_u64();
            if h.unit() > k {
                let cell = &mut buf[(x, y)];
                cell.set_char(' ').set_bg(rgb(bg));
            }
        }
    }
}

const GLITCH: &[char] = &['▓', '▒', '░', '█', '▚', '▞', '#', '%', '&', '?', '/', '\\'];

/// Tear random rows sideways and sprinkle noise. intensity 0..1.
pub fn glitch(buf: &mut Buffer, area: Rect, rng: &mut Rng, intensity: f32, accent: (u8, u8, u8)) {
    let area = area.intersection(buf.area);
    if intensity <= 0.0 || area.width < 4 {
        return;
    }
    for y in area.top()..area.bottom() {
        if rng.chance(intensity * 0.35) {
            let shift = rng.range(1, 6) as i32 * if rng.chance(0.5) { 1 } else { -1 };
            let row: Vec<_> = (area.left()..area.right())
                .map(|x| buf[(x, y)].clone())
                .collect();
            let w = row.len() as i32;
            for (i, x) in (area.left()..area.right()).enumerate() {
                buf[(x, y)] = row[((i as i32 - shift).rem_euclid(w)) as usize].clone();
            }
        }
    }
    let noise = (area.width as f32 * area.height as f32 * intensity * 0.04) as usize;
    for _ in 0..noise {
        let x = area.x + rng.range(0, area.width as u64) as u16;
        let y = area.y + rng.range(0, area.height as u64) as u16;
        let ch = GLITCH[rng.range(0, GLITCH.len() as u64) as usize];
        buf[(x, y)].set_char(ch).set_fg(rgb(accent));
    }
}

/// Slowly drifting motes of light/ash behind everything.
pub struct Particles {
    motes: Vec<Mote>,
}

struct Mote {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    phase: f32,
}

impl Particles {
    pub fn new(count: usize, rng: &mut Rng) -> Self {
        let motes = (0..count)
            .map(|_| Mote {
                x: rng.unit(),
                y: rng.unit(),
                vx: (rng.unit() - 0.5) * 0.01,
                vy: -0.006 - rng.unit() * 0.012,
                phase: rng.unit() * std::f32::consts::TAU,
            })
            .collect();
        Particles { motes }
    }

    pub fn update(&mut self, dt: f32) {
        for m in &mut self.motes {
            m.x = (m.x + m.vx * dt).rem_euclid(1.0);
            m.y = (m.y + m.vy * dt).rem_euclid(1.0);
            m.phase += dt * 2.0;
        }
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect, color: (u8, u8, u8)) {
        for m in &self.motes {
            let x = area.x + (m.x * area.width as f32) as u16;
            let y = area.y + (m.y * area.height as f32) as u16;
            // three brightness steps only, so a mote's cell changes rarely
            let level = ((0.5 + 0.5 * m.phase.sin()) * 2.99) as u8;
            let (ch, glow) = match level {
                2 => ('•', 1.0),
                1 => ('·', 0.7),
                _ => ('.', 0.4),
            };
            if let Some(cell) = buf.cell_mut(Position::new(x, y))
                && cell.symbol() == " "
            {
                cell.set_char(ch).set_fg(rgb(scale(color, glow)));
            }
        }
    }
}
