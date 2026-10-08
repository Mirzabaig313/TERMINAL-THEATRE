//! Scene pictures on screen (tests use the half-block fallback, which works
//! everywhere; real graphics protocols need a real terminal).

mod common;

use common::{Harness, sprite_cells};
use ratatui::crossterm::event::KeyCode;
use terminal_theatre::app::Start;

/// The template's opening scene, past its title card and entrance.
fn opening(name: &str) -> Harness {
    let mut h = Harness::new(name, Start::Story("_template".into()));
    h.wait(20_000);
    h.press(KeyCode::Enter);
    h.wait(1500);
    h
}

#[test]
fn scene_picture_replaces_the_ascii_art() {
    let mut h = opening("img-on");
    h.snap("scene_picture");
    let screen = h.screen();
    assert!(
        !screen.contains("RAIN"),
        "the ASCII art should give way to the picture"
    );
    // the picture fills most of the stage: count cells covered with half-blocks
    assert!(
        sprite_cells(h.draw()) > 1000,
        "the picture should fill the stage"
    );
}

#[test]
fn turning_pictures_off_shows_the_ascii_art() {
    let mut h = opening("img-off");
    h.app.ctx_mut().settings.images = false;
    h.wait(100);
    assert!(h.screen().contains("RAIN"), "ASCII art is the fallback");
}

#[test]
fn monochrome_pictures_are_gray() {
    use ratatui::style::Color;
    let mut h = opening("img-gray");
    h.app.ctx_mut().settings.color = false;
    h.wait(100);
    for cell in h.draw().content.iter() {
        for c in [cell.fg, cell.bg] {
            if let Color::Rgb(r, g, b) = c {
                assert!(r == g && g == b, "color left in monochrome: {c:?}");
            }
        }
    }
}

#[test]
fn a_broken_picture_falls_back_without_crashing() {
    let mut h = Harness::new("img-broken", Start::MainMenu);
    // swap in a copy of the template whose picture is not a real image
    let stories = h.data.join("stories");
    let dir = stories.join("_template");
    let src = common::stories().join("_template");
    for sub in ["scenes", "characters", "images"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
        for f in std::fs::read_dir(src.join(sub)).unwrap() {
            let f = f.unwrap().path();
            std::fs::copy(&f, dir.join(sub).join(f.file_name().unwrap())).unwrap();
        }
    }
    std::fs::copy(src.join("story.toml"), dir.join("story.toml")).unwrap();
    std::fs::write(dir.join("images/rain.png"), b"not a png").unwrap();
    h.app.ctx_mut().stories = stories;

    let mut h2 = h;
    h2.app = terminal_theatre::app::App::new(
        terminal_theatre::screens::Ctx::new(h2.data.join("stories"), h2.data.clone()).unwrap(),
        Start::Story("_template".into()),
    );
    h2.wait(20_000);
    h2.press(KeyCode::Enter);
    h2.wait(500);
    let screen = h2.screen();
    assert!(
        screen.contains("rain.png not shown"),
        "a short notice explains it:\n{screen}"
    );
    h2.wait(3000);
    assert!(
        h2.screen().contains("RAIN"),
        "and the ASCII art is shown instead"
    );
}

/// Finish the text on screen and move to the next.
fn next(h: &mut Harness) {
    h.press(KeyCode::Enter);
    h.wait(200);
    h.press(KeyCode::Enter);
}

#[test]
fn svg_pictures_are_drawn() {
    let mut h = opening("img-svg");
    for _ in 0..3 {
        next(&mut h);
    }
    h.char('1'); // open the door: this scene's picture is an SVG
    h.wait(1500);
    h.snap("svg_scene");
    assert!(
        sprite_cells(h.draw()) > 1000,
        "the SVG should fill the stage"
    );
    assert!(!h.screen().contains("not shown"));
}

#[test]
fn a_broken_svg_falls_back_without_crashing() {
    let data = common::temp_dir("img-bad-svg");
    let dir = data.join("stories/_template");
    let src = common::stories().join("_template");
    for sub in ["scenes", "characters", "images"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
        for f in std::fs::read_dir(src.join(sub)).unwrap() {
            let f = f.unwrap().path();
            std::fs::copy(&f, dir.join(sub).join(f.file_name().unwrap())).unwrap();
        }
    }
    std::fs::copy(src.join("story.toml"), dir.join("story.toml")).unwrap();
    std::fs::write(dir.join("images/doorway.svg"), "<svg this is not xml").unwrap();
    let ctx = terminal_theatre::screens::Ctx::new(data.join("stories"), data.clone()).unwrap();
    let mut h = Harness::new("img-bad-svg-run", Start::MainMenu);
    h.app = terminal_theatre::app::App::new(ctx, Start::Story("_template".into()));
    h.wait(20_000);
    h.press(KeyCode::Enter);
    for _ in 0..3 {
        next(&mut h);
    }
    h.char('1');
    h.wait(300);
    assert!(
        h.screen().contains("doorway.svg not shown"),
        "a notice explains it"
    );
    std::fs::remove_dir_all(data).ok();
}
