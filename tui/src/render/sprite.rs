//! Drawing engine sprites as half-block cells: each cell shows two stacked pixels.

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use theatre_engine::color::scale;
use theatre_engine::sprite::{Pose, Sprite};

use super::fx::rgb;

/// Size of a sprite in terminal cells.
pub fn cells(sprite: &Sprite) -> (u16, u16) {
    (sprite.width as u16, sprite.height.div_ceil(2) as u16)
}

/// Draw into `area` (bottom-aligned, horizontally centred, shifted by `dx` cells),
/// with brightness 0.0..=1.0 for fades. Transparent pixels keep the buffer's background.
pub fn draw(
    buf: &mut Buffer,
    area: Rect,
    sprite: &Sprite,
    pose: &Pose,
    t_ms: u64,
    dx: i32,
    brightness: f32,
) {
    let px = sprite.compose(pose, t_ms);
    let (cw, ch) = cells(sprite);
    let ox = area.x as i32 + (area.width as i32 - cw as i32) / 2 + dx;
    let oy = area.y as i32 + area.height as i32 - ch as i32;
    let tint = |c| rgb(scale(c, brightness));
    for cy in 0..ch as i32 {
        for cx in 0..cw as i32 {
            let (x, y) = (ox + cx, oy + cy);
            if x < area.x as i32
                || x >= area.right() as i32
                || y < area.y as i32
                || y >= area.bottom() as i32
            {
                continue;
            }
            let top = px[(cy * 2) as usize][cx as usize];
            let bot = px.get((cy * 2 + 1) as usize).and_then(|r| r[cx as usize]);
            let Some(cell) = buf.cell_mut(Position::new(x as u16, y as u16)) else {
                continue;
            };
            match (top, bot) {
                (None, None) => {}
                (Some(t), Some(b)) => {
                    cell.set_char('▀').set_fg(tint(t)).set_bg(tint(b));
                }
                (Some(t), None) => {
                    cell.set_char('▀').set_fg(tint(t));
                }
                (None, Some(b)) => {
                    cell.set_char('▄').set_fg(tint(b));
                }
            }
        }
    }
}
