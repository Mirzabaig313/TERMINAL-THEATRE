//! Individual screens and drawing helpers.

mod common;

use common::{sprite_cells, stories};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Style;
use terminal_theatre::render::fx::ansi256;
use terminal_theatre::render::text::{pretty, typed, wrapped_height};
use terminal_theatre::screens::menu::Menu;
use theatre_engine::Store;

fn draw_menu(menu: &mut Menu) -> ratatui::buffer::Buffer {
    let mut term = Terminal::new(TestBackend::new(120, 48)).unwrap();
    term.draw(|f| menu.draw(f, 3000)).unwrap();
    term.backend().buffer().clone()
}

fn text(buf: &ratatui::buffer::Buffer) -> String {
    buf.content.iter().map(|c| c.symbol()).collect()
}

#[test]
fn story_select_lists_every_story_without_errors() {
    let mut menu = Menu::new(&stories(), &Store::in_memory().unwrap(), 0, None);
    let screen = text(&draw_menu(&mut menu));
    for title in ["SHADOW SLAVE", "THE LAST CASE", "BLOOD AND NEON"] {
        assert!(screen.contains(title), "{title} missing");
    }
    assert!(!screen.contains('⚠'), "a story reported a problem");
    assert!(
        !screen.contains("MY NEW STORY"),
        "the template must stay hidden"
    );
}

#[test]
fn every_story_shows_an_animated_cover() {
    let mut menu = Menu::new(&stories(), &Store::in_memory().unwrap(), 0, None);
    for _ in 0..3 {
        assert!(
            sprite_cells(&draw_menu(&mut menu)) > 200,
            "cover character missing"
        );
        menu.key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), 0);
    }
}

#[test]
fn story_select_reports_broken_story() {
    let mut menu = Menu::new(
        &stories(),
        &Store::in_memory().unwrap(),
        0,
        Some("story 'x': start scene missing".into()),
    );
    assert!(text(&draw_menu(&mut menu)).contains("start scene missing"));
}

#[test]
fn typed_keeps_layout() {
    let t = typed("hello\nworld", 7, Style::new(), Style::new());
    assert_eq!(t.lines[0].spans[0].content, "hello");
    assert_eq!(t.lines[1].spans[0].content, "w");
    assert_eq!(t.lines[1].spans[1].content, "orld");
}

#[test]
fn text_helpers() {
    assert_eq!(pretty("dark_archway"), "Dark Archway");
    assert_eq!(wrapped_height("abcdef\n\nab", 3), 4);
}

#[test]
fn nearest_256_colors() {
    assert_eq!(ansi256((0, 0, 0)), 16);
    assert_eq!(ansi256((255, 255, 255)), 231);
    assert_eq!(ansi256((255, 0, 0)), 196);
    assert_eq!(ansi256((0, 212, 255)), 45);
    // grays use the gray ramp
    assert!((232..=255).contains(&ansi256((128, 128, 130))));
}
