//! Screen effects and scene transitions built with tachyonfx.
//!
//! The story names an effect (`fx = "blood"`, `transition = "sweep"`); this
//! module turns the name into a tachyonfx effect, scaled by the Screen Effects
//! setting. Shake and glitch are drawn by `fx.rs` (shake moves the layout, which a
//! post-render effect can't do).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use tachyonfx::{ColorSpace, Duration, Effect, Interpolation, Motion, fx};
use theatre_engine::color::lerp;
use theatre_engine::scene::{LineFx, Transition};

use super::Theme;
use super::fx::rgb;

/// How long a scene transition takes. Real images wait for it to finish.
pub const TRANSITION_MS: u64 = 700;

/// An effect being played, with the time it was last drawn.
pub struct Running {
    effect: Effect,
    last: u64,
}

impl Running {
    pub fn new(effect: Effect, now: u64) -> Self {
        Running { effect, last: now }
    }

    /// Advance the effect to `now` and draw it over `area`. False once it's over.
    pub fn apply(&mut self, buf: &mut Buffer, area: Rect, now: u64) -> bool {
        let dt = now.saturating_sub(self.last).min(u32::MAX as u64) as u32;
        self.last = now;
        let area = area.intersection(buf.area);
        if self.effect.done() {
            return false;
        }
        let before = buf.clone();
        self.effect.process(Duration::from_millis(dt), buf, area);
        if self.effect.done() {
            // an effect ends on the untouched frame; its own last step can leave
            // cells in the terminal's default colors tinted
            *buf = before;
            return false;
        }
        true
    }

    pub fn running(&self) -> bool {
        !self.effect.done()
    }
}

/// The effect for a line or scene, or None when it isn't drawn here (shake,
/// glitch) or effects are off. `strength` is the Screen Effects setting: 1 full,
/// 0.35 reduced, 0 off.
pub fn screen_effect(kind: LineFx, th: &Theme, strength: f32) -> Option<Effect> {
    if strength <= 0.0 {
        return None;
    }
    let full = strength >= 1.0;
    // reduced effects start from a color closer to the scene itself
    let tint = |c: (u8, u8, u8)| rgb(lerp(th.bg, c, strength.max(0.5)));
    let ms = |full_ms: u32| if full { full_ms } else { full_ms * 3 / 5 };
    let black = rgb((0, 0, 0));
    let effect = match kind {
        LineFx::Shake | LineFx::Glitch => return None,
        LineFx::Flash => {
            let c = tint((255, 255, 255));
            fx::fade_from(c, c, (ms(500), Interpolation::QuadOut))
        }
        LineFx::Blood => {
            // never fully red: the scene shows through the wash
            let c = rgb((170, 0, 14));
            let k = if full { 0.75 } else { 0.4 };
            fx::remap_alpha(
                0.0,
                k,
                fx::fade_from(c, c, (ms(1100), Interpolation::CubicOut)),
            )
        }
        LineFx::Blackout => fx::sequence(&[
            fx::fade_to(black, black, (ms(180), Interpolation::QuadIn)),
            fx::sleep(ms(if full { 450 } else { 150 })),
            fx::fade_from(black, black, (ms(1200), Interpolation::SineOut)),
        ]),
        LineFx::Lightning => {
            let c = tint((235, 240, 255));
            let strike = |t: u32| fx::fade_from(c, c, (t, Interpolation::QuadOut));
            if full {
                fx::sequence(&[strike(120), fx::sleep(70u32), strike(650)])
            } else {
                strike(400)
            }
        }
        LineFx::Heartbeat => {
            let amount = if full { 0.55 } else { 0.25 };
            let beat = |t: u32| {
                fx::ping_pong(fx::darken(
                    Some(amount),
                    Some(amount),
                    (t, Interpolation::QuadOut),
                ))
            };
            fx::sequence(&[beat(140), fx::sleep(60u32), beat(220)])
        }
        LineFx::Dizzy => {
            let hue = if full { 150.0 } else { 50.0 };
            fx::ping_pong(fx::hsl_shift(
                Some([hue, 20.0, 0.0]),
                Some([hue * 0.6, 15.0, 0.0]),
                (ms(900), Interpolation::SineInOut),
            ))
        }
        LineFx::Chill => {
            // one tint for text and background, so half-block pictures keep their shape
            let k = if full { 0.4 } else { 0.22 };
            let cold = (ms(800), Interpolation::SineInOut);
            let blue = rgb((70, 110, 180));
            fx::ping_pong(fx::parallel(&[
                fx::saturate(Some(-0.8), Some(-0.8), cold),
                fx::remap_alpha(0.0, k, fx::fade_to(blue, blue, cold)),
            ]))
        }
        LineFx::Static => {
            if full {
                fx::sequence(&[
                    fx::dissolve((250, Interpolation::QuadIn)),
                    fx::coalesce((450, Interpolation::QuadOut)),
                ])
            } else {
                fx::coalesce((300, Interpolation::QuadOut))
            }
        }
        LineFx::Reveal => fx::sweep_in(
            Motion::LeftToRight,
            20,
            0,
            rgb(th.bg),
            (ms(900), Interpolation::QuadOut),
        ),
    };
    // blend colors in RGB like the rest of the renderer: HSL blends pass through
    // other hues (red to a blue-black background goes by way of purple)
    Some(effect.with_color_space(ColorSpace::Rgb))
}

/// The effect that brings a scene on screen, or None for a cut.
pub fn transition(kind: Transition, th: &Theme) -> Option<Effect> {
    let bg = rgb(th.bg);
    let ms = TRANSITION_MS as u32;
    let effect = match kind {
        Transition::Cut => return None,
        Transition::Dissolve => fx::parallel(&[
            fx::coalesce((ms * 5 / 6, Interpolation::QuadOut)),
            fx::fade_from(bg, bg, (ms, Interpolation::QuadOut)),
        ]),
        Transition::Fade => {
            let black = Color::Rgb(0, 0, 0);
            fx::fade_from(black, black, (ms, Interpolation::SineOut))
        }
        Transition::Sweep => {
            fx::sweep_in(Motion::LeftToRight, 24, 6, bg, (ms, Interpolation::QuadOut))
        }
        Transition::Rise => fx::sweep_in(Motion::DownToUp, 12, 3, bg, (ms, Interpolation::QuadOut)),
        Transition::Coalesce => fx::coalesce((ms, Interpolation::CubicOut)),
    };
    Some(effect.with_color_space(ColorSpace::Rgb))
}
