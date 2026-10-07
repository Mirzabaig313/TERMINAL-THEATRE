//! Helpers for driving the app off-screen in tests.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Color;
use terminal_theatre::app::{App, Start};
use terminal_theatre::screens::Ctx;

pub fn stories() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../stories")
}

/// An empty, unique data folder (database lives here) for one test.
pub fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("theatre-ui-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The app with its own clock and a 120×48 off-screen terminal.
pub struct Harness {
    pub app: App,
    pub term: Terminal<TestBackend>,
    pub t: u64,
    pub data: PathBuf,
}

impl Harness {
    pub fn new(name: &str, start: Start) -> Self {
        Self::with_size(name, start, 120, 48)
    }

    pub fn with_size(name: &str, start: Start, w: u16, h: u16) -> Self {
        let data = temp_dir(name);
        let mut ctx = Ctx::new(stories(), data.clone()).unwrap();
        ctx.truecolor = true;
        Harness {
            app: App::new(ctx, start),
            term: Terminal::new(TestBackend::new(w, h)).unwrap(),
            t: 0,
            data,
        }
    }

    /// Let `ms` pass in 33 ms frames.
    pub fn wait(&mut self, ms: u64) {
        let end = self.t + ms;
        while self.t < end {
            self.t += 33;
            self.app.tick(self.t);
        }
    }

    pub fn press(&mut self, code: KeyCode) {
        self.app.key(KeyEvent::new(code, KeyModifiers::NONE));
        self.wait(66);
    }

    pub fn char(&mut self, c: char) {
        self.press(KeyCode::Char(c));
    }

    pub fn draw(&mut self) -> &Buffer {
        self.term.draw(|f| self.app.draw(f)).unwrap();
        self.term.backend().buffer()
    }

    /// Everything on screen as text, one line per row.
    pub fn screen(&mut self) -> String {
        let buf = self.draw().clone();
        (0..buf.area.height)
            .map(|y| {
                (0..buf.area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Draw and, when `SNAP_DIR` is set, also save the frame as `<name>.ppm`.
    pub fn snap(&mut self, name: &str) {
        let buf = self.draw().clone();
        if let Ok(dir) = std::env::var("SNAP_DIR") {
            save_ppm(&buf, &Path::new(&dir).join(format!("{name}.ppm")));
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.data).ok();
    }
}

/// Number of half-block (sprite) cells on screen.
pub fn sprite_cells(buf: &Buffer) -> usize {
    buf.content
        .iter()
        .filter(|c| c.symbol() == "▀" || c.symbol() == "▄")
        .count()
}

/// Rough picture of a frame: half-blocks as two pixels, other glyphs as a block.
pub fn save_ppm(buf: &Buffer, path: &Path) {
    let (cw, ch) = (6usize, 12usize);
    let (w, h) = (buf.area.width as usize, buf.area.height as usize);
    let mut img = vec![0u8; w * cw * h * ch * 3];
    let c = |col: Color, d: (u8, u8, u8)| match col {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => d,
    };
    for y in 0..h {
        for x in 0..w {
            let cell = &buf[(x as u16, y as u16)];
            let (bg, fg) = (c(cell.bg, (0, 0, 0)), c(cell.fg, (200, 200, 200)));
            for py in 0..ch {
                for px in 0..cw {
                    let col = match cell.symbol() {
                        "▀" if py < ch / 2 => fg,
                        "▄" if py >= ch / 2 => fg,
                        "▀" | "▄" | " " => bg,
                        _ if px > 0 && px < cw - 1 && py > 3 && py < ch - 2 => fg,
                        _ => bg,
                    };
                    let i = ((y * ch + py) * w * cw + x * cw + px) * 3;
                    img[i..i + 3].copy_from_slice(&[col.0, col.1, col.2]);
                }
            }
        }
    }
    let mut out = format!("P6 {} {} 255\n", w * cw, h * ch).into_bytes();
    out.extend(img);
    std::fs::write(path, out).unwrap();
}
