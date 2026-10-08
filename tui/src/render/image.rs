//! Scene pictures. Terminals with a graphics protocol (Kitty, iTerm2, WezTerm,
//! Sixel) show the real image; others get it drawn in half-block cells.

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use image::{DynamicImage, RgbaImage};
use resvg::tiny_skia::{self, Pixmap, Transform};
use resvg::usvg::fontdb::Database;
use resvg::usvg::{Options, Tree};

use anyhow::{Context, Result};
use ratatui::Frame;
use ratatui::layout::{Rect, Size};
use ratatui_image::picker::cap_parser::QueryStdioOptions;
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::protocol::StatefulProtocol;
use ratatui_image::{FilterType, Resize, StatefulImage};

/// Fill the stage (enlarging small pictures), keeping proportions, with smooth scaling.
fn fit() -> Resize {
    Resize::Scale(Some(FilterType::Triangle))
}

/// How the terminal can show pictures, detected once at startup.
/// `THEATRE_GRAPHICS=halfblocks` forces the plain-cell fallback.
pub fn detect_picker() -> Picker {
    if std::env::var("THEATRE_GRAPHICS").is_ok_and(|v| v.eq_ignore_ascii_case("halfblocks")) {
        return Picker::halfblocks();
    }
    // real terminals answer in milliseconds; one that never answers costs at
    // most this much, once, at startup (the library default is 2 s)
    let options = QueryStdioOptions {
        timeout: Duration::from_millis(500),
        ..QueryStdioOptions::default()
    };
    Picker::from_query_stdio_with_options(options).unwrap_or_else(|_| Picker::halfblocks())
}

/// True when pictures are real images, not character cells. Those must not be
/// redrawn every frame or touched by cell effects: each redraw resends the image.
pub fn is_graphics(picker: &Picker) -> bool {
    picker.protocol_type() != ProtocolType::Halfblocks
}

/// The current scene's picture, decoded and ready to draw.
pub struct SceneImage {
    pub path: PathBuf,
    /// decoded in grayscale (monochrome mode)
    pub gray: bool,
    protocol: StatefulProtocol,
}

impl SceneImage {
    pub fn load(picker: &Picker, path: &Path, gray: bool) -> Result<Self> {
        let is_svg = path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("svg"));
        let mut img = if is_svg {
            rasterize_svg(path)?
        } else {
            image::open(path).with_context(|| format!("reading {}", path.display()))?
        };
        if gray {
            img = img.grayscale();
        }
        Ok(SceneImage {
            path: path.to_path_buf(),
            gray,
            protocol: picker.new_resize_protocol(img),
        })
    }

    /// Draw fitted and centered inside `area`.
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        let size = self
            .protocol
            .size_for(fit(), Size::new(area.width, area.height));
        let (w, h) = (size.width.min(area.width), size.height.min(area.height));
        let r = Rect {
            x: area.x + (area.width - w) / 2,
            y: area.y + (area.height - h) / 2,
            width: w,
            height: h,
        };
        f.render_stateful_widget(
            StatefulImage::default().resize(fit()),
            r,
            &mut self.protocol,
        );
    }
}

/// SVG pictures are drawn this wide (in pixels): sharp at any stage size.
const SVG_WIDTH: f32 = 1600.0;

/// Background behind transparent parts of an SVG: the dark of the stage.
const SVG_BACKGROUND: (u8, u8, u8) = (10, 10, 14);

/// System fonts for SVG `<text>`, loaded once on first use.
fn fonts() -> Arc<Database> {
    static FONTS: OnceLock<Arc<Database>> = OnceLock::new();
    FONTS
        .get_or_init(|| {
            let mut db = Database::new();
            db.load_system_fonts();
            Arc::new(db)
        })
        .clone()
}

/// Draw an SVG file into pixels.
fn rasterize_svg(path: &Path) -> Result<DynamicImage> {
    let data = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    // scanning system fonts is slow; only SVGs with text need them
    let has_text = data.windows(5).any(|w| w == b"<text");
    let fontdb = if has_text { fonts() } else { Arc::new(Database::new()) };
    let options = Options {
        resources_dir: path.parent().map(Path::to_path_buf),
        fontdb,
        ..Options::default()
    };
    let tree = Tree::from_data(&data, &options)
        .with_context(|| format!("{} is not a valid SVG", path.display()))?;
    let size = tree.size();
    let scale = SVG_WIDTH / size.width();
    let (w, h) = (
        (size.width() * scale).ceil() as u32,
        (size.height() * scale).ceil() as u32,
    );
    let mut pixmap = Pixmap::new(w.max(1), h.max(1)).context("SVG has no size")?;
    let (r, g, b) = SVG_BACKGROUND;
    pixmap.fill(tiny_skia::Color::from_rgba8(r, g, b, 255));
    resvg::render(
        &tree,
        Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    // fully opaque after the background fill, so premultiplied pixels are plain RGBA
    let img = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
        .context("SVG pixel buffer")?;
    Ok(DynamicImage::ImageRgba8(img))
}
