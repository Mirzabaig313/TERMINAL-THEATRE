# 🎬 TERMINAL THEATRE

An interactive movie game for your terminal. Animated pixel-art characters act
out branching stories with typewriter dialogue, screen effects and multiple
endings, and every choice you make changes where the story goes.

Written in Rust. One binary, nothing else to install.

## 🎮 Features

- **Living Characters**: Pixel-art portraits that blink, breathe and move their lips as they speak, with eight expressions each (happy, smirk, angry, afraid, sad, pained, shocked, neutral)
- **Cinematic Presentation**: Animated neon title sequence, a skippable pre-show cinematic, scene dissolves and typewriter text
- **Screen Effects**: Eleven effects built on [tachyonfx](https://github.com/ratatui/tachyonfx): shake, glitch, flash, blood, blackout, lightning, heartbeat, dizzy, chill, static and reveal, plus drifting ash and neon in the background
- **Sound**: Gunshots, thunder, ringing phones, heartbeats and sirens, with rain, neon hum, wind or a low drone under each scene. Every sound is made by the game itself, so there are no audio files to install. Sound is off until you turn it on in Settings
- **Scene Transitions**: Scenes dissolve, fade from black, sweep or rise into view, gather out of scattered cells, or cut straight in
- **Scene Pictures**: Real images in Kitty, iTerm2, WezTerm and Sixel terminals, drawn in character cells everywhere else, with ASCII art as the fallback
- **Mood Lighting**: Every scene is colored by its mood (noir, danger, alert, calm, mystery), and each story can recolor them
- **Branching Narratives**: Flags, items and counters (evidence, heat, trust, corruption…) change which choices, lines and endings you see
- **Multiple Endings**: 54 endings across three stories
- **Save System**: Autosave at every scene, 9 named slots per story, quick save/load, Continue, Load Game, export/import
- **Reading Comfort**: History of everything read, auto-advance, and skip that races through text you've already read
- **Endings Gallery**: See which endings you've found in each story, and how many are left
- **Accessibility**: Screen effects can be made gentler or turned off
- **Settings That Stick**: Color or monochrome, text speed, typewriter, cinematic intro, screen effects, skip mode, sound and volume, all remembered
- **Story Folders**: Every story is a self-contained folder; drop one in and it appears in the menu

## 📖 Current Stories

### The Last Case (Noir Detective)

*"The rain hasn't stopped for three days. Neither has the blood."*

Play as Jack Malone, a private eye with a dead client and a smoking gun. Navigate corruption, mob bosses and conspiracies to clear your name.

- **Scenes**: 81
- **Endings**: 28
- **Cast**: Jack Malone, Castellano, Eddie, Captain Rodriguez, Morrison, Blackwood, Tony "The Fist", Agent Sarah Chen, Doc Stevens
- **Genre**: Film noir detective mystery

### Blood and Neon (Cyberpunk Noir)

*"The city never sleeps, but it dreams. Dark dreams."*

Play as Detective Marcus Kane, hunting a killer who poses victims as tarot cards. Seven victims. Seven sacrifices. A pharmaceutical conspiracy that reaches all the way up.

- **Scenes**: 87
- **Endings**: 15
- **Decision Points**: 72
- **Cast**: Kane (with a glowing cyber-eye), Sarah Vega, Cassandra Westmore, Elias, Selene, Captain Reeves, Riley, Dr. Helena Marsh, Madame Zora
- **Genre**: Neon-soaked detective thriller

### Shadow Slave (Dark Fantasy)

*"The Nightmare Spell has chosen you."*

Pulled into the Dream Realm, you must survive your First Nightmare: ancient ruins, shadow beasts, trapped spirits, and a Guardian who decides whether you awaken.

- **Scenes**: 55
- **Endings**: 11
- **Cast**: Sunny, The Spell (a watching rune-eye), and the ghosts of a fallen garrison
- **Genre**: Dark fantasy survival

## 📦 Install

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.sh | sh
```

**Windows** (PowerShell)

```powershell
irm https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.ps1 | iex
```

Then run `theatre`. The installer picks the build for your system from [GitHub Releases](https://github.com/Mirzabaig313/TERMINAL-THEATRE/releases), checks its SHA-256 checksum, and puts `theatre` in `~/.local/bin` (on Windows, `%LOCALAPPDATA%\Programs\TerminalTheatre`). It's one file with the stories built in: on first run they're unpacked into `~/.terminal_theatre/stories/`, and you can add your own story folders there. On Linux without the ALSA sound library, the installer picks the silent build. You can also download a build from the releases page and unpack it yourself.

## 🚀 Play from Source

```bash
# Install Rust (once): https://rustup.rs
# Linux only: sound needs ALSA headers (Debian/Ubuntu: sudo apt install libasound2-dev),
# or build a silent game with: cargo run --release --no-default-features

# Play the game
cargo run --release
```

Best in a truecolor terminal around **120×48** or larger (iTerm2, WezTerm, Kitty, Ghostty, Windows Terminal, the VS Code terminal…). Scene pictures appear as real images in Kitty, iTerm2, WezTerm, Ghostty and Sixel terminals and as character cells elsewhere, which look soft and blocky. Run `theatre graphics` to see what your terminal supports; in VS Code-based editors (VS Code, Cursor, Kiro) turn on `"terminal.integrated.enableImages": true` for full-resolution pictures. `THEATRE_GRAPHICS=halfblocks` forces character cells. Other terminals get the 256-color palette automatically (`THEATRE_COLOR=256` or `THEATRE_COLOR=truecolor` overrides the guess), and smaller windows still work.

```bash
cargo run --release -- --skip-intro     # straight to the main menu
cargo run --release -- noir_detective   # straight into one story
cargo run --release -- stories          # list installed stories
cargo run --release -- saves            # list saved games
cargo run --release -- graphics         # can this terminal show real pictures?
cargo run --release -- sound            # play every built-in sound (checks your audio)
cargo run --release -- check            # validate every story and play every route through it
cargo run --release -- --help           # every command
```

## 🕹️ Controls

| Key | Action |
|---|---|
| `Enter` / `Space` | Continue (press during typing to show the whole line) |
| `↑` `↓` or `1`–`9` | Choose |
| `h` or `PgUp` | History: scroll back through everything read |
| `a` | Auto: lines move on by themselves (waits at choices) |
| `s` | Skip: race through text you've already read (stops at new text) |
| `F5` / `F9` | Quick save / quick load |
| `Esc` | Pause menu: Resume, Save Game, History, Main Menu, Quit |
| `q` | Quit |
| `e` (story select) | Endings gallery for the selected story |

## 💾 Saving

- The game **autosaves** every time you enter a scene.
- **Save Game** (Esc in a story) offers 9 named slots per story. Stories never overwrite each other's saves.
- **Quick save** (`F5`) and **quick load** (`F9`) use a separate slot, so they never overwrite your named saves.
- **Continue** picks up your most recent save of any story. **Load Game** lists every save with its scene, time, playtime, completion and choices made. Press `D` to delete one.
- Everything lives in one SQLite file, `~/.terminal_theatre/theatre.db`. SQLite is built into the game. Set `THEATRE_HOME` to keep it somewhere else.
- Move a save between machines with `theatre export <story> <slot> <file>` and `theatre import <file> <slot>`.
- Saves from the old Python version are imported automatically on first run.

## 🎨 Color System

Each scene takes its colors from its mood:

- **Noir** (Gold on black): Detective atmosphere
- **Danger** (Red): Combat and threats, with the occasional glitch
- **Alert** (Orange): Tension and investigation
- **Calm** (Blue): Safe moments
- **Mystery** (Violet): Conspiracies and secrets

Stories can recolor any mood. *Blood and Neon* turns noir into hot pink and cyan.

## ✍️ Create Your Own Story

No programming needed. A story is a folder of text files:

```
stories/my_story/
├── story.toml        title, description, start scene, speakers, colors
├── cast.toml         who your characters look like (in words)
├── scenes/*.toml     the scenes: narration, dialogue, choices
└── characters/*.toml the pixel-art portraits
```

1. `cargo run --release -- new my_story "My Story"` copies the template into `stories/my_story`. The template has a commented example of every feature.
2. Write your scenes in `scenes/*.toml`. Split them however you like, for example one file per chapter.
3. Describe your characters in `cast.toml` and let the generator draw them:

   ```toml
   [detective]
   name = "Jack Malone"
   hat = "fedora"
   outfit = "trench"
   extras = ["stubble", "cigarette"]
   ```

   ```bash
   python3 tools/portrait.py stories/my_story
   ```

   Hair styles, hats, outfits, scars, glasses, earrings, neon streaks, tattoos and cyber-eyes are all available, in any colors.
4. Try it while you write. Rehearsal reloads the story every time you save a file, starts at any scene with any state, and saves nothing:

   ```bash
   cargo run --release -- rehearse my_story
   cargo run --release -- rehearse my_story --scene warehouse --set evidence=4 --set flag:met_chen --set item:Photograph
   ```

   Press `d` to see the scene, counters, flags and items. A broken edit keeps the last good version running and shows what's wrong.
5. Check it:

   ```bash
   cargo run --release -- check my_story           # validate, then play every route through it
   cargo run --release -- route my_story warehouse # the shortest way to reach a scene
   cargo run --release -- map my_story > map.md    # the scene graph as a Mermaid diagram (--dot for Graphviz)
   ```

   `check` plays every possible route through the story, carrying flags, items and counters. It reports endings or scenes that can't be reached, places where the player is left with no choice, choices that never open, and lines or openings that are never shown. It also warns about conditions that are always true. Wrap the map in a ```` ```mermaid ```` block and GitHub draws it.

Give a scene a picture (PNG, JPEG or SVG inside the story folder; SVGs are drawn sharp at any size). Terminals with image support show it as a real image; others draw it in character cells; players who turn Scene Images off see the scene's ASCII art instead:

```toml
[opening]
image = "images/rain.png"
art = """ ...ASCII fallback... """
```

Make choices matter with counters and conditions:

```toml
[[alley.choices]]
text = "Spare Tony"
goto = "tony_message"
add = { trust = 1 }                      # counters start at 0; `set = {...}` replaces
set_flags = ["spared_tony"]

[[warehouse.choices]]
text = "Call in the favour"
goto = "tony_vouches"
if = "trust >= 2 && flag('spared_tony') && !item('Badge')"

[warehouse]
text = "Rain on the corrugated roof."
variants = [                             # the first matching opening replaces `text`
  { if = "visited('rodriguez')", text = "Rodriguez's flare is heavy in your pocket." },
]
dialogue = [
  { who = "tony", line = "I owe you one, Malone.", if = "flag('spared_tony')" },
  { who = "jack_malone", line = "Evidence so far: {evidence} pieces." },
]
```

Conditions understand counters, `flag("x")`, `item("x")`, `visited("scene")`, `+ -`, comparisons, `!`, `&&`, `||` and parentheses. A line whose condition fails is skipped; `{counter}` puts a value into any text. Mistakes in conditions are reported when the story loads.

Give a line an expression and an effect right in the dialogue:

```toml
{ who = "tony", line = "Get in the car!", mood = "angry", fx = "shake" }
```

| Effect | For |
|---|---|
| `shake` | gunshots, blows, slammed doors |
| `glitch` | static, drugs, broken implants |
| `flash` | shocks and revelations |
| `blood` | wounds and violence: a red wash that drains away |
| `blackout` | knocked out, lights cut: black, then the scene slowly returns |
| `lightning` | two quick white flashes |
| `heartbeat` | fear: the screen throbs dark twice |
| `dizzy` | drugs, poison, vertigo: the colors swim |
| `chill` | dread, the supernatural: the colors drain cold |
| `static` | the screen breaks apart and pulls itself back together |
| `reveal` | the screen is uncovered left to right |

Scenes can play an effect too, once they're on screen, and choose how they arrive:

```toml
[warehouse]
transition = "sweep"     # dissolve (default) | fade | sweep | rise | coalesce | cut
fx = "lightning"
```

Set `transition = "fade"` in `story.toml` to change the default for a whole story. The Screen Effects setting makes every effect gentler (Reduced) or turns them off.

Give scenes sound. Every effect already makes a fitting sound (lightning thunders, a heartbeat thumps); name one to choose your own:

```toml
# story.toml: a background loop for every scene, and per mood
ambience = "rain"            # rain | storm | city | neon | wind | drone | dream | silence
[ambience_moods]
danger = "drone"

# scenes/*.toml
[warehouse]
ambience = "wind"            # this scene's own loop
sound = "siren"              # once the scene is on screen
dialogue = [
  { who = "tony", line = "Down!", fx = "shake", sound = "gunshot" },
]
```

Sounds: `gunshot` `thunder` `knock` `phone` `siren` `glass` `heartbeat` `impact` `static` `sting` `whoosh` `warble` `chill` `chime` `footsteps` `silence`. You can also use your own audio file (`sound = "sounds/door.ogg"`; .ogg, .wav, .mp3 or .flac inside the story folder).

A broken story never crashes the game: it shows up on the menu with its error.

## 🏗️ Architecture

```
.
├── engine/     theatre-engine: story format, loading, validation, story flow,
│               saves (SQLite), settings, sprites, animation. No terminal code.
├── tui/        terminal-theatre: the `theatre` binary, screens, drawing, effects
├── stories/    one self-contained folder per story, found automatically
├── tools/      portrait.py (character generator), check_story.py (quick checks)
├── install.sh, install.ps1   one-line installers (download from GitHub Releases)
└── .github/workflows         CI (tests on macOS, Linux, Windows) and releases
```

## 🚢 Releasing

1. Set the version in `Cargo.toml` (`[workspace.package] version = "0.2.0"`) and commit.
2. `git tag v0.2.0 && git push origin v0.2.0`

GitHub Actions builds for macOS (Apple Silicon and Intel), Linux (x86_64 and ARM, each with and without sound) and Windows, then publishes the release with checksums and the install scripts. The tag has to match the version in `Cargo.toml`. The stories are built into the binary (`tui/build.rs`).

| Engine module | What it does |
|---|---|
| `library` | Finds story folders (`stories/*/story.toml`) |
| `pack` | Loads one story and validates scene links, speakers and expressions |
| `scene` | Scene, dialogue line and choice data |
| `runner` | Plays a story: narration → dialogue → choices → next scene |
| `state` | Flags, items, counters, visited scenes, choice history, playtime |
| `sound` | Sound and ambience names, audio files, and the sound each effect makes |
| `walk` | Plays every route through a story for `check` and `route` |
| `map` | The scene graph as Mermaid or Graphviz |
| `logic` | Conditions (`if = "trust >= 2 && flag('x')"`) and `{counter}` text |
| `store` | The SQLite database and its schema migrations |
| `save` | Save slots, autosave, quick save, export/import, Python save import |
| `progress` | Lines already read (for skip) and endings found (for the gallery) |
| `settings` | Player preferences |
| `sprite` | Pixel-art sprites, expressions, blinking, breathing, talking |

| TUI module | What it does |
|---|---|
| `app` | Switches between screens; monochrome mode |
| `screens/opening`, `main_menu`, `cinematic` | Title sequence, main menu, pre-show |
| `screens/menu` | Story select with animated covers |
| `screens/play/` | Playing a story: `mod.rs` (state, keys, story flow), `stage.rs` (drawing the scene), `overlays.rs` (intro, pause menu, save dialog, history), `modes.rs` (auto, skip, quick save/load), `rehearsal.rs` (live reload, state panel) |
| `screens/backlog` | The history of everything read |
| `screens/load`, `settings`, `credits` | The other menu screens |
| `audio` | Sound on its own thread (rodio); `audio/synth` makes every built-in sound and loop from noise and sine waves |
| `render/effects` | Screen effects and scene transitions (tachyonfx), scaled by the Screen Effects setting |
| `render/*` | Sprite drawing, scene pictures, hand-drawn effects (shake, glitch), color themes, text helpers |

The engine never names a story or a character, so adding a story never touches code.

## 🧪 Tests

Tests live in their own folders, separate from the code:

```
engine/tests/   stories, runner, saves, progress, logic, effects, sound, authoring, images, scene_state, sprite, store_settings
tui/tests/      flow (the whole game driven by key presses), comfort, effects, sound, rehearsal, bundled, images, performance, screens
```

```bash
cargo test                                   # everything
cargo test -p theatre-engine --test saves    # one file
SNAP_DIR=/tmp/frames cargo test --test flow  # also save each step of the flow as a picture
```

Every bundled story is checked on each run: it must load, every scene must be reachable, and every route through it is played: all endings reachable, no player left without a choice, every gated choice able to open, every conditional line and opening shown somewhere.

## 📊 Statistics

- **Stories**: 3
- **Total Scenes**: 223
- **Unique Endings**: 54
- **Animated Characters**: 27, with 8 expressions each (hand-made sprites have their own sets)
- **Scene Pictures**: 31 hand-drawn SVG illustrations (10 The Last Case, 10 Blood and Neon, 11 Shadow Slave)
- **Lines of Dialogue**: 680
- **Tests**: 110


## 🔮 Future Plans

- [ ] More stories (sci-fi, horror)
- [ ] Achievements
- [ ] Timed choices
- [ ] Portraits for the remaining minor characters

## 📝 License

GNU Affero General Public License v3.0. See [LICENSE](LICENSE).

---

*"In the dark streets of Terminal Theatre, every choice matters. Choose wisely."*

**Ready? Run `cargo run --release`** 🎭
