# 🎬 TERMINAL THEATRE

An interactive movie game for your terminal. Animated pixel-art characters act
out branching stories with typewriter dialogue, screen effects and multiple
endings, and every choice you make changes where the story goes.

Written in Rust. One binary, nothing else to install.

## 🎮 Features

- **Living Characters**: Pixel-art portraits that blink, breathe and move their lips as they speak, with eight expressions each (happy, smirk, angry, afraid, sad, pained, shocked, neutral)
- **Cinematic Presentation**: Animated neon title sequence, a skippable pre-show cinematic, scene dissolves and typewriter text
- **Screen Effects**: Screen shake for gunshots, glitches for drugs and static, white flashes for shocking reveals, drifting ash and neon in the background
- **Mood Lighting**: Every scene is colored by its mood (noir, danger, alert, calm, mystery), and each story can recolor them
- **Branching Narratives**: Flags, items and conditional choices make earlier decisions matter
- **Multiple Endings**: 50 endings across three stories
- **Save System**: Autosave at every scene, 9 named slots per story, quick save/load, Continue, Load Game, export/import
- **Reading Comfort**: History of everything read, auto-advance, and skip that races through text you've already read
- **Endings Gallery**: See which endings you've found in each story, and how many are left
- **Accessibility**: Screen effects can be reduced or turned off (flash, shake, glitch)
- **Settings That Stick**: Color or monochrome, text speed, typewriter, cinematic intro, screen effects, skip mode, all remembered
- **Story Folders**: Every story is a self-contained folder; drop one in and it appears in the menu

## 📖 Current Stories

### The Last Case (Noir Detective)

*"The rain hasn't stopped for three days. Neither has the blood."*

Play as Jack Malone, a private eye with a dead client and a smoking gun. Navigate corruption, mob bosses and conspiracies to clear your name.

- **Scenes**: 80
- **Endings**: 27
- **Cast**: Jack Malone, Castellano, Eddie, Captain Rodriguez, Morrison, Blackwood, Tony "The Fist", Agent Sarah Chen, Doc Stevens
- **Genre**: Film noir detective mystery

### Blood and Neon (Cyberpunk Noir)

*"The city never sleeps, but it dreams. Dark dreams."*

Play as Detective Marcus Kane, hunting a killer who poses victims as tarot cards. Seven victims. Seven sacrifices. A pharmaceutical conspiracy that reaches all the way up.

- **Scenes**: 85
- **Endings**: 13
- **Decision Points**: 71
- **Cast**: Kane (with a glowing cyber-eye), Sarah Vega, Cassandra Westmore, Elias, Selene, Captain Reeves, Riley, Dr. Helena Marsh, Madame Zora
- **Genre**: Neon-soaked detective thriller

### Shadow Slave (Dark Fantasy)

*"The Nightmare Spell has chosen you."*

Pulled into the Dream Realm, you must survive your First Nightmare: ancient ruins, shadow beasts, trapped spirits, and a Guardian who decides whether you awaken.

- **Scenes**: 53
- **Endings**: 10
- **Cast**: Sunny, The Spell (a watching rune-eye), and the ghosts of a fallen garrison
- **Genre**: Dark fantasy survival

## 🚀 Quick Start

```bash
# Install Rust (once): https://rustup.rs

# Play the game
cargo run --release
```

Best in a truecolor terminal around **120×48** or larger (iTerm2, WezTerm, Kitty, Ghostty, Windows Terminal, the VS Code terminal…). Other terminals get the 256-color palette automatically (`THEATRE_COLOR=256` or `THEATRE_COLOR=truecolor` overrides the guess), and smaller windows still work.

```bash
cargo run --release -- --skip-intro     # straight to the main menu
cargo run --release -- noir_detective   # straight into one story
cargo run --release -- stories          # list installed stories
cargo run --release -- saves            # list saved games
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

1. `cp -r stories/_template stories/my_story`. The template has a commented example of every feature.
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
4. Check and play it:

   ```bash
   python3 tools/check_story.py stories/my_story   # quick check while writing
   cargo test                                      # full validation of every story
   cargo run --release -- my_story
   ```

Give a line an expression and an effect right in the dialogue:

```toml
{ who = "tony", line = "Get in the car!", mood = "angry", fx = "shake" }
```

A broken story never crashes the game: it shows up on the menu with its error.

## 🏗️ Architecture

```
.
├── engine/     theatre-engine: story format, loading, validation, story flow,
│               saves (SQLite), settings, sprites, animation. No terminal code.
├── tui/        terminal-theatre: the `theatre` binary, screens, drawing, effects
├── stories/    one self-contained folder per story, found automatically
├── tools/      portrait.py (character generator), check_story.py (quick checks)
```

| Engine module | What it does |
|---|---|
| `library` | Finds story folders (`stories/*/story.toml`) |
| `pack` | Loads one story and validates scene links, speakers and expressions |
| `scene` | Scene, dialogue line and choice data |
| `runner` | Plays a story: narration → dialogue → choices → next scene |
| `state` | Flags, items, visited scenes, choice history, playtime |
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
| `screens/play/` | Playing a story: `mod.rs` (state, keys, story flow), `stage.rs` (drawing the scene), `overlays.rs` (intro, pause menu, save dialog, history), `modes.rs` (auto, skip, quick save/load) |
| `screens/backlog` | The history of everything read |
| `screens/load`, `settings`, `credits` | The other menu screens |
| `render/*` | Sprite drawing, effects, color themes, text helpers |

The engine never names a story or a character, so adding a story never touches code.

## 🧪 Tests

Tests live in their own folders, separate from the code:

```
engine/tests/   stories, runner, saves, progress, scene_state, sprite, store_settings
tui/tests/      flow (the whole game driven by key presses), comfort, screens
```

```bash
cargo test                                   # everything
cargo test -p theatre-engine --test saves    # one file
SNAP_DIR=/tmp/frames cargo test --test flow  # also save each step of the flow as a picture
```

Every bundled story is checked on each run: it must load, every scene must be reachable, and always picking the first choice must reach an ending.

## 📊 Statistics

- **Stories**: 3
- **Total Scenes**: 218
- **Unique Endings**: 50
- **Animated Characters**: 27, with 8 expressions each (hand-made sprites have their own sets)
- **Lines of Dialogue**: 587
- **Tests**: 55


## 🔮 Future Plans

- [ ] Sound effects and music
- [ ] More stories (sci-fi, horror)
- [ ] Achievements and an endings gallery
- [ ] Timed choices
- [ ] Portraits for the remaining minor characters
- [ ] Real images in terminals that support them (Kitty, iTerm2, WezTerm)

## 📝 License

GNU Affero General Public License v3.0. See [LICENSE](LICENSE).

---

*"In the dark streets of Terminal Theatre, every choice matters. Choose wisely."*

**Ready? Run `cargo run --release`** 🎭
