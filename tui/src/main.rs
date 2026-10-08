//! Terminal Theatre: interactive stories in the terminal.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use theatre_engine::save::format_playtime;
use theatre_engine::settings::data_dir;

use terminal_theatre::app::{self, App, Start};
use terminal_theatre::screens::Ctx;

/// Frame time while something moves quickly (typing, transitions, effects).
const FRAME: Duration = Duration::from_millis(33);
/// Frame time when the screen is idle: animations step every 125 ms anyway.
const IDLE_FRAME: Duration = Duration::from_millis(125);

const HELP: &str = "\
Terminal Theatre - interactive stories in the terminal

USAGE:
    theatre                         title sequence and main menu
    theatre <story-id>              start a story straight away
    theatre --skip-intro            go straight to the main menu
    theatre stories                 list installed stories
    theatre saves                   list saved games
    theatre export <story-id> <slot> <file>
                                    copy a save to a file (slot 0 = autosave)
    theatre import <file> <slot>    put a save file into a slot of its story

ENVIRONMENT:
    THEATRE_STORIES   folder with story folders (default: next to the binary, then the source tree)
    THEATRE_HOME      where the database (saves, settings) lives (default: ~/.terminal_theatre)
    THEATRE_COLOR     256 or truecolor, to override the terminal color detection
    THEATRE_GRAPHICS  halfblocks, to draw scene pictures in character cells only

KEYS:
    Enter/Space continue · ↑/↓ or 1-9 choose · h history · a auto · s skip
    F5/F9 quick save/load · Esc pause menu (save, main menu) · q quit";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ctx = Ctx::new(app::default_stories_dir(), data_dir())?;

    let start = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => Start::Opening,
        ["-h" | "--help" | "help"] => {
            println!("{HELP}");
            return Ok(());
        }
        ["--skip-intro"] => Start::MainMenu,
        ["stories"] => return list_stories(&ctx),
        ["saves"] => return list_saves(&ctx),
        ["export", story, slot, file] => {
            ctx.store.export(
                story,
                slot.parse().context("slot must be a number")?,
                &PathBuf::from(file),
            )?;
            println!("Exported {story} slot {slot} to {file}");
            return Ok(());
        }
        ["import", file, slot] => {
            let m = ctx.store.import(
                &PathBuf::from(file),
                slot.parse().context("slot must be a number")?,
            )?;
            println!(
                "Imported \"{}\" ({}) into slot {}",
                m.name, m.story_title, m.slot
            );
            return Ok(());
        }
        [story] if !story.starts_with('-') => Start::Story(story.to_string()),
        _ => bail!("unknown arguments: {}\n\n{HELP}", args.join(" ")),
    };

    let mut terminal = ratatui::init();
    // ask the terminal how it can show pictures (needs the alternate screen, before events)
    let mut ctx = ctx;
    ctx.picker = terminal_theatre::render::image::detect_picker();
    let mut app = App::new(ctx, start);
    let begin = Instant::now();
    let result = (|| -> Result<()> {
        while !app.quit {
            app.tick(begin.elapsed().as_millis() as u64);
            terminal.draw(|f| app.draw(f))?;
            let frame = if app.busy() { FRAME } else { IDLE_FRAME };
            let deadline = Instant::now() + frame;
            while let Some(left) = deadline.checked_duration_since(Instant::now()) {
                if !event::poll(left)? {
                    break;
                }
                match event::read()? {
                    Event::Key(k) if k.kind == KeyEventKind::Press => {
                        app.key(k);
                        // show the result right away instead of at the frame deadline
                        break;
                    }
                    // a resized window needs a full redraw now
                    Event::Resize(..) => break,
                    _ => {}
                }
            }
        }
        Ok(())
    })();
    ratatui::restore();
    result?;
    println!("\n\x1b[1;97m✨ Thanks for visiting Terminal Theatre! ✨\x1b[0m");
    println!("\x1b[38;2;255;176;0mSee you next time!\x1b[0m\n");
    Ok(())
}

fn list_stories(ctx: &Ctx) -> Result<()> {
    let (entries, errors) = theatre_engine::library::discover(&ctx.stories)?;
    for e in entries {
        println!(
            "{:<20} {}  {}",
            e.id,
            e.meta.title,
            e.meta.summary.unwrap_or_default()
        );
    }
    for e in errors {
        println!("broken: {e:#}");
    }
    Ok(())
}

fn list_saves(ctx: &Ctx) -> Result<()> {
    let saves = ctx.store.all();
    if saves.is_empty() {
        println!("No saves yet.");
    }
    for m in saves {
        println!(
            "{:<18} slot {}  {:<24} {}  {}  {}",
            m.story_id,
            m.slot,
            m.name,
            m.scene_description,
            m.time_label(),
            format_playtime(m.playtime_ms)
        );
    }
    Ok(())
}
