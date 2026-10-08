//! Every built-in sound, made from noise and sine waves: no audio files.
//!
//! Each function returns mono samples at [`RATE`]. One-off sounds are
//! normalized to a common peak; background loops are seamless (their end is
//! blended into their start) and quieter.

use std::f32::consts::TAU;

use theatre_engine::Rng;
use theatre_engine::sound::{Ambient, Sfx};

/// Samples per second. Plenty for these sounds, and cheap to make.
pub const RATE: u32 = 32_000;

/// Menu sounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ui {
    /// moving through a menu
    Move,
    /// choosing something
    Select,
    /// going back
    Back,
}

fn len(secs: f32) -> usize {
    (secs * RATE as f32) as usize
}

fn time(i: usize) -> f32 {
    i as f32 / RATE as f32
}

/// White noise in -1..1.
struct Noise(Rng);

impl Noise {
    fn new(seed: u64) -> Self {
        Noise(Rng::new(seed))
    }
    fn next(&mut self) -> f32 {
        self.0.unit() * 2.0 - 1.0
    }
}

/// One-pole low-pass filter.
struct Lp {
    y: f32,
    a: f32,
}

impl Lp {
    fn new(cutoff: f32) -> Self {
        Lp {
            y: 0.0,
            a: coef(cutoff),
        }
    }
    fn set(&mut self, cutoff: f32) {
        self.a = coef(cutoff);
    }
    fn run(&mut self, x: f32) -> f32 {
        self.y += self.a * (x - self.y);
        self.y
    }
}

fn coef(cutoff: f32) -> f32 {
    1.0 - (-TAU * cutoff / RATE as f32).exp()
}

/// Band of noise between two cutoffs.
struct Band {
    low: Lp,
    high: Lp,
}

impl Band {
    fn new(from: f32, to: f32) -> Self {
        Band {
            low: Lp::new(from),
            high: Lp::new(to),
        }
    }
    fn run(&mut self, x: f32) -> f32 {
        self.high.run(x) - self.low.run(x)
    }
}

/// Exponential decay with a soft 3 ms attack, for t >= 0.
fn hit(t: f32, tau: f32) -> f32 {
    if t < 0.0 {
        0.0
    } else {
        (t / 0.003).min(1.0) * (-t / tau).exp()
    }
}

/// Fade in over `a` seconds and out over the last `r` seconds of `total`.
fn ar(t: f32, a: f32, r: f32, total: f32) -> f32 {
    (t / a).min(1.0) * ((total - t) / r).clamp(0.0, 1.0)
}

/// A sine whose frequency may change every sample.
#[derive(Default)]
struct Osc(f32);

impl Osc {
    fn run(&mut self, freq: f32) -> f32 {
        self.0 = (self.0 + freq / RATE as f32).fract();
        (self.0 * TAU).sin()
    }
}

fn sine(freq: f32, t: f32) -> f32 {
    (TAU * freq * t).sin()
}

/// Scale so the loudest sample is `peak`.
fn normalize(mut s: Vec<f32>, peak: f32) -> Vec<f32> {
    let max = s.iter().fold(0.0f32, |m, x| m.max(x.abs()));
    if max > 0.0 {
        let k = peak / max;
        s.iter_mut().for_each(|x| *x *= k);
    }
    s
}

/// Make a sound by evaluating `f` for every sample index over `secs` seconds.
fn render(secs: f32, mut f: impl FnMut(usize, f32) -> f32) -> Vec<f32> {
    (0..len(secs)).map(|i| f(i, time(i))).collect()
}

pub fn sfx(s: Sfx) -> Vec<f32> {
    let raw = match s {
        Sfx::Gunshot => gunshot(),
        Sfx::Thunder => thunder(),
        Sfx::Knock => knock(),
        Sfx::Phone => phone(),
        Sfx::Siren => siren(),
        Sfx::Glass => glass(),
        Sfx::Heartbeat => heartbeat(),
        Sfx::Impact => impact(),
        Sfx::Static => crackle(),
        Sfx::Sting => sting(),
        Sfx::Whoosh => whoosh(),
        Sfx::Warble => warble(),
        Sfx::Chill => chill(),
        Sfx::Chime => chime(),
        Sfx::Footsteps => footsteps(),
        Sfx::Silence => return Vec::new(),
    };
    // sounds that hold their level seem louder than ones that die away
    let peak = match s {
        Sfx::Siren => 0.5,
        Sfx::Warble | Sfx::Phone | Sfx::Static => 0.6,
        Sfx::Chill => 0.7,
        _ => 0.9,
    };
    normalize(raw, peak)
}

pub fn ui(u: Ui) -> Vec<f32> {
    let notes: &[(f32, f32)] = match u {
        Ui::Move => &[(2200.0, 0.0)],
        Ui::Select => &[(880.0, 0.0), (1320.0, 0.05)],
        Ui::Back => &[(1320.0, 0.0), (880.0, 0.05)],
    };
    let tau = if u == Ui::Move { 0.006 } else { 0.035 };
    let s = render(0.18, |_, t| {
        notes
            .iter()
            .map(|&(f, at)| sine(f, t - at) * hit(t - at, tau))
            .sum()
    });
    normalize(s, if u == Ui::Move { 0.25 } else { 0.4 })
}

/// A background loop, about `secs` long and seamless.
pub fn ambience(a: Ambient) -> Vec<f32> {
    let (secs, raw) = match a {
        Ambient::Rain => (10.0, rain(10.6, 0x7a1)),
        Ambient::Storm => (20.0, storm(20.6)),
        Ambient::City => (12.0, city(12.6)),
        Ambient::Neon => (10.0, neon(10.6)),
        Ambient::Wind => (14.0, wind(14.6)),
        Ambient::Drone => (16.0, drone(16.6)),
        Ambient::Dream => (16.0, dream(16.6)),
        Ambient::Silence => return Vec::new(),
    };
    normalize(seamless(raw, len(secs)), 0.55)
}

/// Blend the part past `n` samples into the start, so the loop has no seam.
fn seamless(mut s: Vec<f32>, n: usize) -> Vec<f32> {
    let fade = s.len().saturating_sub(n).min(n);
    for i in 0..fade {
        let k = i as f32 / fade as f32;
        s[i] = s[i] * k + s[n + i] * (1.0 - k);
    }
    s.truncate(n);
    s
}

fn gunshot() -> Vec<f32> {
    let mut n = Noise::new(1);
    let (mut body, mut tail) = (Lp::new(1800.0), Lp::new(500.0));
    render(1.0, |_, t| {
        let x = n.next();
        let crack = x * hit(t, 0.012);
        let body = body.run(x) * hit(t, 0.09) * 2.5;
        let thump = sine(70.0 - 30.0 * t, t) * hit(t, 0.12);
        let tail = tail.run(x) * hit(t, 0.35) * 2.0;
        0.8 * crack + body + 0.9 * thump + 0.5 * tail
    })
}

fn thunder() -> Vec<f32> {
    let mut n = Noise::new(2);
    let (mut a, mut b, mut c) = (Lp::new(110.0), Lp::new(110.0), Lp::new(900.0));
    let mut roll = Lp::new(3.0);
    let mut r = Rng::new(22);
    render(4.5, |_, t| {
        let x = n.next();
        let rumble = b.run(a.run(x)) * 12.0;
        let rolling = 0.4 + roll.run(r.unit() * 2.0) * 0.9;
        let env = (t / 0.12).min(1.0) * (-t / 1.3).exp();
        let crackle = c.run(x) * hit(t - 0.03, 0.3) * 1.5;
        rumble * env * rolling + crackle
    })
}

fn knock() -> Vec<f32> {
    let mut n = Noise::new(3);
    let mut lp = Lp::new(1200.0);
    render(1.0, |_, t| {
        let x = lp.run(n.next());
        [0.0, 0.2, 0.4]
            .iter()
            .map(|&at| {
                let k = t - at;
                (sine(150.0, k) * 0.8 + x * 1.5) * hit(k, 0.035)
            })
            .sum()
    })
}

fn phone() -> Vec<f32> {
    // an old bell telephone: a striker hits two bells twenty times a second
    render(2.6, |_, t| {
        let on = t < 0.9 || (1.4..2.3).contains(&t);
        if !on {
            return 0.0;
        }
        let since_strike = t % 0.05;
        let f = 1000.0;
        let bell = sine(f, t) + 0.5 * sine(2.76 * f, t) + 0.25 * sine(5.4 * f, t);
        bell * hit(since_strike, 0.04) * ar(t % 1.4, 0.01, 0.05, 0.9)
    })
}

fn siren() -> Vec<f32> {
    let mut osc = Osc::default();
    let mut lp = Lp::new(1800.0);
    let total = 3.6;
    render(total, |_, t| {
        let f = 750.0 + 350.0 * (TAU * 0.35 * t - TAU / 4.0).sin();
        let s = osc.run(f);
        let wave = s + 0.3 * (s * 2.0).tanh();
        lp.run(wave) * ar(t, 0.8, 0.9, total)
    })
}

fn glass() -> Vec<f32> {
    let mut n = Noise::new(4);
    let mut lp = Lp::new(2500.0);
    let mut r = Rng::new(44);
    let shards: Vec<(f32, f32, f32, f32)> = (0..36)
        .map(|_| {
            let at = r.unit().powi(2) * 0.55;
            let f = 2500.0 + r.unit() * 4500.0;
            let tau = 0.04 + r.unit() * 0.2;
            let amp = 0.1 + r.unit() * 0.25;
            (at, f, tau, amp)
        })
        .collect();
    render(1.4, |_, t| {
        let x = n.next();
        let crash = (x - lp.run(x)) * hit(t, 0.06) * 1.2;
        let ring: f32 = shards
            .iter()
            .map(|&(at, f, tau, amp)| sine(f, t - at) * hit(t - at, tau) * amp)
            .sum();
        crash + ring
    })
}

fn heartbeat() -> Vec<f32> {
    // lub-dub, lub-dub: timed like the heartbeat screen effect
    let beats = [(0.0, 1.0), (0.18, 0.7), (0.62, 1.0), (0.8, 0.7)];
    let mut lp = Lp::new(300.0);
    render(1.4, |_, t| {
        let s: f32 = beats
            .iter()
            .map(|&(at, amp)| {
                let k = t - at;
                sine(62.0 - 25.0 * k.max(0.0), k) * hit(k, 0.07) * amp
            })
            .sum();
        lp.run(s) * 3.0
    })
}

fn impact() -> Vec<f32> {
    let mut n = Noise::new(5);
    let mut lp = Lp::new(500.0);
    render(0.8, |_, t| {
        let thump = sine(55.0 - 20.0 * t, t) * hit(t, 0.15);
        let smack = lp.run(n.next()) * hit(t, 0.05) * 2.5;
        thump + smack
    })
}

fn crackle() -> Vec<f32> {
    let mut n = Noise::new(6);
    let mut r = Rng::new(66);
    let mut band = Band::new(500.0, 5000.0);
    let mut gate = 1.0;
    let mut level = 0.0;
    let total = 0.9;
    render(total, |i, t| {
        if i % (RATE as usize / 60) == 0 {
            gate = if r.chance(0.7) { 1.0 } else { 0.0 };
            level = 0.4 + r.unit() * 0.6;
        }
        // a few bits only: harsh, radio-like
        let x = (band.run(n.next()) * 8.0).round() / 8.0;
        x * gate * level * ar(t, 0.01, 0.25, total)
    })
}

fn sting() -> Vec<f32> {
    let mut n = Noise::new(7);
    let mut lp = Lp::new(700.0);
    let chord = [146.8, 155.6, 220.0, 311.1];
    render(2.0, |_, t| {
        let env = (t / 0.015).min(1.0) * (-t / 0.8).exp();
        let tones: f32 = chord
            .iter()
            .map(|&f| sine(f, t) + 0.3 * sine(2.0 * f, t))
            .sum();
        let hit_noise = lp.run(n.next()) * hit(t, 0.03) * 3.0;
        let boom = sine(45.0, t) * hit(t, 0.4) * 1.2;
        tones * env * 0.35 + hit_noise + boom
    })
}

fn whoosh() -> Vec<f32> {
    let mut n = Noise::new(8);
    let (mut a, mut b) = (Lp::new(3000.0), Lp::new(3000.0));
    let mut osc = Osc::default();
    let total = 1.6;
    render(total, |_, t| {
        let cutoff = 3000.0 * (0.03f32).powf(t / total);
        a.set(cutoff);
        b.set(cutoff);
        let air = b.run(a.run(n.next())) * 3.0;
        let drop = osc.run(120.0 * (0.3f32).powf(t / total)) * hit(t, 0.6);
        (air + drop) * ar(t, 0.15, 0.4, total)
    })
}

fn warble() -> Vec<f32> {
    let (mut a, mut b) = (Osc::default(), Osc::default());
    let total = 2.2;
    render(total, |_, t| {
        let depth = 0.02 + 0.05 * t / total;
        let wobble = (TAU * 3.0 * t).sin();
        let x = a.run(330.0 * (1.0 + depth * wobble)) + b.run(497.0 * (1.0 - depth * wobble));
        x * (std::f32::consts::PI * t / total).sin()
    })
}

fn chill() -> Vec<f32> {
    let mut n = Noise::new(9);
    let mut band = Band::new(400.0, 1200.0);
    let total = 2.6;
    render(total, |_, t| {
        let glass = sine(1318.5, t) + sine(1324.0, t);
        let wind = band.run(n.next()) * 1.5;
        let low = sine(82.0, t) * 0.4;
        (glass * 0.25 + wind + low) * ar(t, 0.6, 1.0, total)
    })
}

fn chime() -> Vec<f32> {
    let bell = |f: f32, t: f32| -> f32 {
        [(1.0, 2.0), (2.0, 1.2), (2.76, 0.8), (4.07, 0.5), (5.4, 0.3)]
            .iter()
            .map(|&(p, tau)| sine(f * p, t) * hit(t, tau) / p)
            .sum()
    };
    render(3.2, |_, t| bell(523.25, t) + 0.8 * bell(659.25, t - 0.25))
}

fn footsteps() -> Vec<f32> {
    let mut n = Noise::new(10);
    let mut lp = Lp::new(900.0);
    render(1.7, |_, t| {
        let x = lp.run(n.next());
        [(0.0, 1.0), (0.4, 0.85), (0.8, 1.0), (1.2, 0.85)]
            .iter()
            .map(|&(at, amp)| {
                let k = t - at;
                (x * 2.0 * hit(k, 0.03) + sine(90.0, k) * hit(k, 0.04) * 0.6) * amp
            })
            .sum()
    })
}

fn rain(secs: f32, seed: u64) -> Vec<f32> {
    let mut n = Noise::new(seed);
    let mut hiss = Band::new(250.0, 2500.0);
    let mut patter = Lp::new(800.0);
    let mut r = Rng::new(seed ^ 0xd209);
    // droplets: (start sample, frequency, loudness)
    let drops: Vec<(usize, f32, f32)> = (0..(secs * 40.0) as usize)
        .map(|_| {
            (
                (r.unit() * len(secs) as f32) as usize,
                2000.0 + r.unit() * 3000.0,
                0.05 + r.unit() * 0.15,
            )
        })
        .collect();
    let mut s = render(secs, |_, _| {
        let x = n.next();
        hiss.run(x) * 0.5 + patter.run(x) * 0.6
    });
    for (at, f, amp) in drops {
        for i in 0..len(0.015) {
            if let Some(v) = s.get_mut(at + i) {
                let t = time(i);
                *v += sine(f, t) * hit(t, 0.003) * amp;
            }
        }
    }
    s
}

fn storm(secs: f32) -> Vec<f32> {
    let mut s = rain(secs, 0x570);
    let roll = thunder();
    let at = len(secs * 0.4);
    for (i, x) in roll.iter().enumerate() {
        if let Some(v) = s.get_mut(at + i) {
            *v += x * 0.12;
        }
    }
    s
}

fn city(secs: f32) -> Vec<f32> {
    let mut n = Noise::new(11);
    let (mut rumble, mut tires) = (Lp::new(150.0), Band::new(500.0, 1500.0));
    let mut r = Rng::new(111);
    let cars: Vec<f32> = (0..(secs / 2.5) as usize)
        .map(|_| r.unit() * secs)
        .collect();
    render(secs, |_, t| {
        let x = n.next();
        let passing: f32 = cars
            .iter()
            .map(|&at| (-((t - at) / 0.8).powi(2)).exp())
            .sum();
        rumble.run(x) * 4.0 * (0.7 + 0.3 * sine(0.1, t)) + tires.run(x) * passing * 0.8
    })
}

fn neon(secs: f32) -> Vec<f32> {
    let mut n = Noise::new(12);
    let mut r = Rng::new(121);
    let mut crackle = Band::new(1500.0, 6000.0);
    let mut buzz_left = 0usize;
    let mut room = city(secs).into_iter();
    render(secs, |_, t| {
        let hum: f32 = [(60.0, 0.5), (120.0, 0.35), (180.0, 0.2), (240.0, 0.1)]
            .iter()
            .map(|&(f, a)| sine(f, t) * a)
            .sum();
        let hum = (hum * 2.0).tanh() * 0.35;
        if buzz_left == 0 && r.chance(0.5 / RATE as f32) {
            buzz_left = len(0.03 + r.unit() * 0.05);
        }
        let x = crackle.run(n.next());
        let buzz = if buzz_left > 0 {
            buzz_left -= 1;
            x * 0.8
        } else {
            0.0
        };
        hum + buzz + room.next().unwrap_or(0.0) * 0.35
    })
}

fn wind(secs: f32) -> Vec<f32> {
    let mut n = Noise::new(13);
    let (mut a, mut b) = (Lp::new(500.0), Lp::new(500.0));
    let mut high = Lp::new(200.0);
    let mut whistle = Osc::default();
    render(secs, |_, t| {
        let gust = 0.6 + 0.25 * sine(0.13, t) + 0.15 * sine(0.31, t + 1.7);
        let cutoff = 300.0 + 600.0 * gust;
        a.set(cutoff);
        b.set(cutoff);
        let x = b.run(a.run(n.next()));
        let air = (x - high.run(x)) * 6.0;
        let tone = whistle.run(700.0 + 200.0 * gust) * 0.04 * gust;
        air * gust + tone
    })
}

fn drone(secs: f32) -> Vec<f32> {
    let mut n = Noise::new(14);
    let mut lp = Lp::new(200.0);
    render(secs, |_, t| {
        let tones = [(55.0, 0.07), (55.3, 0.05), (82.6, 0.11), (110.2, 0.03)];
        let s: f32 = tones
            .iter()
            .map(|&(f, lfo)| sine(f, t) * (0.6 + 0.4 * sine(lfo, t)))
            .sum();
        (s * 0.6 + lp.run(n.next()) * 3.0).tanh()
    })
}

fn dream(secs: f32) -> Vec<f32> {
    let mut n = Noise::new(15);
    let mut air = Band::new(2000.0, 6000.0);
    render(secs, |_, t| {
        let pad: f32 = [(220.0, 0.11), (277.2, 0.07), (329.6, 0.13), (440.4, 0.05)]
            .iter()
            .map(|&(f, lfo)| sine(f, t) * (0.5 + 0.5 * sine(lfo, t)))
            .sum();
        let shimmer = (sine(1760.0, t) * sine(0.2, t).max(0.0)
            + sine(2637.0, t) * sine(0.17, t + 2.0).max(0.0))
            * 0.08;
        pad * 0.3 + shimmer + air.run(n.next()) * 0.15
    })
}
