use anyhow::{Context, Result, bail};

pub type Rgb = (u8, u8, u8);

/// Parse "#rrggbb".
pub fn parse_hex(s: &str) -> Result<Rgb> {
    let h = s.trim_start_matches('#');
    if h.len() != 6 {
        bail!("bad color '{s}', expected #rrggbb");
    }
    let v = u32::from_str_radix(h, 16).with_context(|| format!("bad color '{s}'"))?;
    Ok(((v >> 16) as u8, (v >> 8) as u8, v as u8))
}

pub fn scale((r, g, b): Rgb, k: f32) -> Rgb {
    let k = k.clamp(0.0, 1.0);
    (
        (r as f32 * k) as u8,
        (g as f32 * k) as u8,
        (b as f32 * k) as u8,
    )
}

pub fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    (m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}
