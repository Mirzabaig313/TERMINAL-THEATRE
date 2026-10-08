//! Terminal Theatre: interactive stories in the terminal.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use theatre_engine::map::Format;
use theatre_engine::save::format_playtime;
use theatre_engine::settings::data_dir;
use theatre_engine::walk::Walk;

use terminal_theatre::app::{self, App, Start};
use terminal_theatre::audio::Audio;
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
    theatre --version               the installed version
    theatre stories                 list installed stories
    theatre saves                   list saved games
    theatre graphics                how this terminal can show scene pictures
    theatre sound                   play every built-in sound (checks your audio)
    theatre check [story-id]        validate stories and play every route through them
                                    (all stories, or one; _template too)
    theatre route <story-id> <scene>
                                    the shortest way to reach a scene
    theatre map <story-id> [--dot]  the scene graph as a Mermaid diagram (or Graphviz)
    theatre new <story-id> [title]  start a new story from the template
    theatre rehearse <story-id> [--scene <id>] [--set <x>]...
                                    try out a story while writing it: start at any
                                    scene, with counters/flags/items set (--set trust=3,
                                    --set flag:met_chen, --set item:Photograph); story
                                    files reload when saved, `d` shows the state, and
                                    nothing is saved
    theatre export <story-id> <slot> <file>
                                    copy a save to a file (slot 0 = autosave)
    theatre import <file> <slot>    put a save file into a slot of its story

ENVIRONMENT:
    THEATRE_STORIES   folder with story folders (default: next to the binary, then the source tree)
    THEATRE_HOME      where the database (saves, settings) lives (default: ~/.terminal_theatre)
    THEATRE_COLOR     256 or truecolor, to override the terminal color detection
    THEATRE_GRAPHICS  halfblocks, to draw scene pictures in character cells only
    THEATRE_SOUND     off, to never use the audio device (sound is off until turned on in Settings)

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
        ["-V" | "--version" | "version"] => {
            println!("theatre {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        ["-h" | "--help" | "help"] => {
            println!("{HELP}");
            return Ok(());
        }
        ["--skip-intro"] => Start::MainMenu,
        ["stories"] => return list_stories(&ctx),
        ["saves"] => return list_saves(&ctx),
        ["graphics"] => return graphics_info(),
        ["sound"] => return sound_demo(&ctx),
        ["check"] => return check_stories(&ctx, None),
        ["check", story] => return check_stories(&ctx, Some(story)),
        ["route", story, scene] => return route(&ctx, story, scene),
        ["map", story] => return map(&ctx, story, Format::Mermaid),
        ["map", story, "--dot"] => return map(&ctx, story, Format::Dot),
        ["new", id, title @ ..] => return new_story(&ctx, id, &title.join(" ")),
        ["rehearse", story, rest @ ..] => rehearsal(&ctx, story, rest)?,
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
        [story] if !story.starts_with('-') => Start::Story(known_story(&ctx, story)?),
        _ => bail!("unknown arguments: {}\n\n{HELP}", args.join(" ")),
    };

    let mut terminal = ratatui::init();
    // ask the terminal how it can show pictures (needs the alternate screen, before events)
    let mut ctx = ctx;
    ctx.picker = terminal_theatre::render::image::detect_picker();
    ctx.audio = Audio::new();
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

/// Ask the terminal how it can show pictures and explain the result.
fn graphics_info() -> Result<()> {
    use ratatui::crossterm::terminal;
    use ratatui_image::picker::ProtocolType;

    terminal::enable_raw_mode()?;
    let picker = terminal_theatre::render::image::detect_picker();
    terminal::disable_raw_mode()?;

    let env = |k: &str| std::env::var(k).unwrap_or_else(|_| "-".into());
    let font = picker.font_size();
    println!(
        "Terminal:       TERM_PROGRAM={}  TERM={}  COLORTERM={}",
        env("TERM_PROGRAM"),
        env("TERM"),
        env("COLORTERM")
    );
    println!("Font cell:      {}x{} px", font.width, font.height);
    let (name, sharp) = match picker.protocol_type() {
        ProtocolType::Kitty => ("Kitty graphics", true),
        ProtocolType::Iterm2 => ("iTerm2 inline images", true),
        ProtocolType::Sixel => ("Sixel", true),
        ProtocolType::Halfblocks => ("half-block characters (fallback)", false),
    };
    println!("Scene pictures: {name}");
    if sharp {
        println!("\nPictures are shown at full resolution.");
    } else {
        println!(
            "\nThis terminal didn't report image support, so pictures are drawn with
character cells (2 pixels per cell) and look soft and blocky.

For full-resolution pictures:
  - VS Code, Cursor, Kiro and other VS Code-based editors: turn on the setting
    \"terminal.integrated.enableImages\": true, then open a new terminal and
    run `theatre graphics` again.
  - Or play in a terminal with image support: Kitty, WezTerm, iTerm2, Ghostty
    or Windows Terminal (Sixel)."
        );
    }
    Ok(())
}

/// Play every built-in sound and loop once, naming each.
fn sound_demo(ctx: &Ctx) -> Result<()> {
    use std::io::Write;
    use terminal_theatre::audio::synth::{self, RATE};
    use terminal_theatre::audio::{Cue, Loop, Status};
    use theatre_engine::sound::{Ambient, Sfx};

    let audio = Audio::new();
    let volume = ctx.settings.volume.max(30);
    audio.set_gain(volume as f32 / 100.0);
    if let Status::Unavailable(why) = audio.status() {
        bail!("no sound: {why}");
    }
    let wait = |secs: f32| std::thread::sleep(Duration::from_secs_f32(secs));
    println!("Playing at {volume}% volume (Ctrl+C to stop).\n\nSounds:");
    for s in Sfx::ALL.into_iter().filter(|s| *s != Sfx::Silence) {
        print!("  {:<10}", s.name());
        std::io::stdout().flush().ok();
        audio.play(Cue::Builtin(s));
        wait(synth::sfx(s).len() as f32 / RATE as f32 + 0.4);
        println!("✓");
    }
    println!("\nAmbience (a few seconds of each):");
    for a in Ambient::ALL.into_iter().filter(|a| *a != Ambient::Silence) {
        print!("  {:<10}", a.name());
        std::io::stdout().flush().ok();
        audio.set_ambience(Some(Loop::Builtin(a)));
        wait(5.0);
        println!("✓");
    }
    audio.set_ambience(None);
    wait(1.8);
    if !ctx.settings.sound {
        println!("\nSound is off in the game. Turn it on in Settings → Sound.");
    }
    Ok(())
}

/// Words `theatre` understands as its first argument, besides story ids.
const COMMANDS: [&str; 13] = [
    "version", "stories", "saves", "graphics", "sound", "check", "route", "map", "new", "rehearse",
    "export", "import", "help",
];

/// A story to start straight away, or a helpful error (a typo shouldn't
/// silently open the story list).
fn known_story(ctx: &Ctx, arg: &str) -> Result<String> {
    if ctx.stories.join(arg).join("story.toml").is_file() {
        return Ok(arg.to_string());
    }
    let ids: Vec<String> = theatre_engine::library::discover(&ctx.stories)
        .map(|(entries, _)| entries.into_iter().map(|e| e.id).collect())
        .unwrap_or_default();
    let candidates = COMMANDS
        .iter()
        .map(|c| c.to_string())
        .chain(ids.iter().cloned());
    let close = candidates
        .map(|c| (distance(&arg.to_lowercase(), &c), c))
        .filter(|(d, c)| *d <= 2.max(c.len() / 4))
        .min();
    let mut msg = format!("'{arg}' is not a command or a story.");
    if let Some((_, c)) = close {
        msg += &format!(" Did you mean `theatre {c}`?");
    }
    msg += &format!(
        "\n\nStories: {}\nRun `theatre` for the title screen, or `theatre --help` for every command.",
        ids.join(", ")
    );
    bail!(msg)
}

/// How many single-letter edits turn `a` into `b`.
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev = row[0];
        row[0] = i + 1;
        for j in 0..b.len() {
            let cur = row[j + 1];
            row[j + 1] = (prev + usize::from(ca != b[j]))
                .min(row[j] + 1)
                .min(cur + 1);
            prev = cur;
        }
    }
    row[b.len()]
}

/// Most story states `check` and `route` explore before giving up.
const WALK_LIMIT: usize = 2_000_000;

fn load(ctx: &Ctx, story: &str) -> Result<theatre_engine::StoryPack> {
    theatre_engine::StoryPack::load(&ctx.stories.join(story))
        .with_context(|| format!("story '{story}'"))
}

/// Print the shortest way to reach a scene.
fn route(ctx: &Ctx, story: &str, scene: &str) -> Result<()> {
    let pack = load(ctx, story)?;
    if !pack.scenes.contains_key(scene) {
        bail!("story '{story}' has no scene '{scene}'");
    }
    let walk = Walk::run(&pack, WALK_LIMIT);
    let Some(route) = walk.route(scene) else {
        bail!("'{scene}' can't be reached from the start");
    };
    println!("{} → {scene}  ({} choices)\n", pack.meta.start, route.len());
    for (n, step) in route.iter().enumerate() {
        println!(
            "{:>3}. {:<28} choose \"{}\"",
            n + 1,
            step.scene,
            step.choice
        );
    }
    println!("{:>3}. {scene}", route.len() + 1);
    Ok(())
}

fn map(ctx: &Ctx, story: &str, format: Format) -> Result<()> {
    print!(
        "{}",
        theatre_engine::map::render(&load(ctx, story)?, format)
    );
    Ok(())
}

/// Copy the template into a new story folder and give it a title.
fn new_story(ctx: &Ctx, id: &str, title: &str) -> Result<()> {
    if id.is_empty()
        || !id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        bail!("story ids use lowercase letters, digits and _ (like my_story)");
    }
    let from = ctx.stories.join("_template");
    let to = ctx.stories.join(id);
    if to.exists() {
        bail!("{} already exists", to.display());
    }
    copy_dir(&from, &to)?;
    let title = if title.is_empty() {
        id.replace('_', " ").to_uppercase()
    } else {
        title.to_string()
    };
    let meta = to.join("story.toml");
    let text = std::fs::read_to_string(&meta)?
        .replace("title = \"MY NEW STORY\"", &format!("title = {title:?}"));
    std::fs::write(&meta, text)?;
    println!("Created {} (\"{title}\")\n", to.display());
    println!("Next:");
    println!("  edit  {}/scenes/*.toml and story.toml", to.display());
    println!("  try   theatre rehearse {id}        (reloads as you save)");
    println!("  check theatre check {id}");
    Ok(())
}

fn copy_dir(from: &std::path::Path, to: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

/// Parse `rehearse` options into a starting point.
fn rehearsal(ctx: &Ctx, story: &str, rest: &[&str]) -> Result<Start> {
    let pack = load(ctx, story)?;
    let mut state = theatre_engine::state::State::default();
    let mut scene = pack.meta.start.clone();
    let mut args = rest.iter();
    while let Some(a) = args.next() {
        match *a {
            "--scene" => {
                let id = args.next().context("--scene needs a scene id")?;
                if !pack.scenes.contains_key(*id) {
                    bail!("story '{story}' has no scene '{id}'");
                }
                scene = id.to_string();
            }
            "--set" => {
                let spec = args.next().context("--set needs a value, like trust=3")?;
                state.set_from(spec).map_err(anyhow::Error::msg)?;
            }
            other => bail!("unknown option for rehearse: {other}"),
        }
    }
    state.current_scene = scene;
    Ok(Start::Rehearse(story.to_string(), Box::new(state)))
}

/// Load and validate stories, reporting problems the way `cargo test` would.
fn check_stories(ctx: &Ctx, only: Option<&str>) -> Result<()> {
    use theatre_engine::StoryPack;
    let dirs: Vec<PathBuf> = match only {
        Some(id) => vec![ctx.stories.join(id)],
        None => {
            let (entries, errors) = theatre_engine::library::discover(&ctx.stories)?;
            for e in &errors {
                println!("✗ {e:#}");
            }
            entries.into_iter().map(|e| e.dir).collect()
        }
    };
    let mut failed = 0;
    for dir in dirs {
        let id = dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        match StoryPack::load(&dir) {
            Ok(pack) => {
                let started = Instant::now();
                let walk = Walk::run(&pack, WALK_LIMIT);
                let problems = walk.problems(&pack);
                let unreachable = pack.unreachable();
                let pictures = pack.scenes.values().filter(|s| s.image.is_some()).count();
                println!(
                    "✓ {id}: {} scenes, {} endings, {} speakers, {} pictures",
                    pack.scenes.len(),
                    pack.ending_ids().len(),
                    pack.meta.speakers.len(),
                    pictures
                );
                if !unreachable.is_empty() {
                    println!("  ⚠ unreachable scenes: {}", unreachable.join(", "));
                }
                println!(
                    "  {} every route: {} states in {:.1}s, {} of {} endings reachable",
                    if problems.is_empty() { "✓" } else { "✗" },
                    walk.states,
                    started.elapsed().as_secs_f32(),
                    pack.ending_ids()
                        .iter()
                        .filter(|e| walk.reached.contains(**e))
                        .count(),
                    pack.ending_ids().len()
                );
                for p in &problems {
                    println!("    ✗ {p}");
                }
                for w in walk.warnings(&pack) {
                    println!("    ⚠ {w}");
                }
                if !problems.is_empty() {
                    failed += 1;
                }
            }
            Err(e) => {
                failed += 1;
                println!("✗ {id}: {e:#}");
            }
        }
    }
    if failed > 0 {
        bail!("{failed} story folder(s) have problems");
    }
    Ok(())
}
