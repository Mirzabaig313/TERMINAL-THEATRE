# Writing Guide

Everything you need to write a story for Terminal Theatre. No programming:
a story is a folder of text files in [TOML](https://toml.io) format.

**Contents**
1. [Your first story in 10 minutes](#1-your-first-story-in-10-minutes)
2. [The story folder](#2-the-story-folder)
3. [story.toml: the story's settings](#3-storytoml-the-storys-settings)
4. [Scenes](#4-scenes)
5. [Dialogue](#5-dialogue)
6. [Choices](#6-choices)
7. [Making choices matter: flags, items, counters, conditions](#7-making-choices-matter)
8. [Characters and portraits](#8-characters-and-portraits)
9. [Pictures and ASCII art](#9-pictures-and-ascii-art)
10. [Mood and colour](#10-mood-and-colour)
11. [Screen effects and transitions](#11-screen-effects-and-transitions)
12. [Sound](#12-sound)
13. [Testing your story](#13-testing-your-story)
14. [Writing tips](#14-writing-tips)
15. [Sharing your story](#15-sharing-your-story)
16. [Reference: every field](#16-reference-every-field)

---

## 1. Your first story in 10 minutes

```sh
theatre new haunted_lighthouse "The Haunted Lighthouse"
theatre rehearse haunted_lighthouse
```

`new` copies the template, a tiny four-scene story where every feature is shown with a
comment. `rehearse` plays it and **reloads every time you save a file**, so keep it
open in one window and your editor in another.

Where's the folder?

| How you run the game | Your story is in |
|---|---|
| installed (`theatre`) | `~/.terminal_theatre/stories/haunted_lighthouse/` |
| from source (`cargo run`) | `stories/haunted_lighthouse/` in the repository |

Open `scenes/01_beginning.toml`, change the opening text, save, and watch the
game update. That's the whole loop: **write → save → see it**.

When it's done:

```sh
theatre check haunted_lighthouse
```

---

## 2. The story folder

```
haunted_lighthouse/
├── story.toml          the title, start scene, speakers, colours, defaults
├── cast.toml           your characters, described in words (for the portrait generator)
├── scenes/             the story itself
│   ├── 01_beginning.toml
│   ├── 02_the_tower.toml
│   └── 99_endings.toml
├── characters/         pixel-art portraits (generated from cast.toml)
├── images/             scene pictures (optional)
└── sounds/             your own audio files (optional)
```

- **The folder name is the story's id** (`haunted_lighthouse`): lowercase letters,
  digits and `_`.
- **Folders starting with `_`** (like `_template`) are hidden from the menu.
- **Scenes** can be split across as many files as you like. They're loaded in name order
  and merged; one file per chapter works well.

---

## 3. story.toml: the story's settings

```toml
title = "THE HAUNTED LIGHTHOUSE"
description = "One or two lines for the story list."
summary = "A Gothic Ghost Story"             # tagline under the title
blurb = """
The longer pitch shown when the story is selected.
Several paragraphs are fine."""
start = "arrival"                             # the first scene

player = "keeper"                             # whose portrait waits during choices
cover = "keeper"                              # portrait on the story list (a file in characters/)
cover_mood = "mystery"                        # colours of the story list entry
default_mood = "noir"                         # mood for scenes that don't set one

transition = "fade"                           # how scenes come on screen (default: dissolve)
ambience = "wind"                             # background sound for every scene

[ambience_moods]                              # …or by mood
danger = "drone"

[themes.noir]                                 # recolour a mood for this story
accent = "#7fd4ff"

[speakers.keeper]
name = "THE KEEPER"
sprite = "keeper"                             # characters/keeper.toml

[speakers.keeper_thinking]
name = "THE KEEPER (thinking)"
sprite = "keeper"
thought = true                                # drawn in italics, in parentheses

[speakers.voice]
name = "A VOICE"                              # no sprite: a line without a portrait
color = "#9a7fff"                             # name tag colour
```

> TOML rule: plain `key = value` lines must come **before** the first `[table]`.
> Put `ambience`, `transition` and the other settings above `[ambience_moods]`,
> `[themes…]` and `[speakers…]`.

**Every speaker** used in a dialogue line must be listed under `[speakers]`.

---

## 4. Scenes

A scene is one moment: some narration, some dialogue, then choices (or an ending).

```toml
[arrival]
text = """
The ferry leaves you on the rocks. Above, the lighthouse is dark: it hasn't
been dark in ninety years.

Paragraphs are separated by a blank line."""
mood = "mystery"

dialogue = [
  { who = "keeper_thinking", line = "Someone should have lit the lamp by now." },
  { who = "voice", line = "You came back.", fx = "chill" },
]

[[arrival.choices]]
text = "Climb the tower"
goto = "the_stairs"

[[arrival.choices]]
text = "Knock on the keeper's cottage"
goto = "cottage"
```

- **`[arrival]`** starts a scene; `arrival` is its **id**, which choices use in `goto`.
  Ids are lowercase with `_`, and they appear on screen as headings ("The Stairs"),
  so make them readable.
- **`text`** is the narration, typed out first.
- **`dialogue`** lines come next, one at a time.
- **`[[arrival.choices]]`** (double brackets) adds a choice. Repeat it for each choice.

### Endings

```toml
[ending_lamp_lit]
text = "The lamp turns. For the first time in a week, the ships see home."
ending = true
```

A scene with `ending = true`, or with no choices, ends the story. The game counts each
ending for the player's **endings gallery**, so give each a meaningful id:
`ending_lamp_lit` shows as "Ending Lamp Lit".

---

## 5. Dialogue

```toml
dialogue = [
  { who = "keeper", line = "Get away from the lamp!", mood = "angry", fx = "shake" },
  { who = "keeper_thinking", line = "Not again. Please, not again." },
  { who = "voice", line = "You promised.", sound = "chill" },
]
```

| Field | What it does |
|---|---|
| `who` | the speaker id from `story.toml` |
| `line` | what they say |
| `mood` | the portrait's expression: `neutral` `happy` `smirk` `angry` `afraid` `sad` `pained` `shocked` |
| `fx` | a screen effect as the line starts ([§11](#11-screen-effects-and-transitions)) |
| `sound` | a sound as the line starts ([§12](#12-sound)) |
| `if` | only say this line when a condition holds ([§7](#7-making-choices-matter)) |

The portrait **talks** while the line types, and **blinks and breathes** the rest of the time.

---

## 6. Choices

```toml
[[cottage.choices]]
text = "Show him the photograph"
goto = "keeper_remembers"
any_items = ["Old Photograph"]          # only if the player has it
add = { trust = 1 }                     # what taking it changes
set_flags = ["showed_photo"]
```

| Field | What it does |
|---|---|
| `text` | the choice as the player sees it |
| `goto` | the scene it leads to |
| `if` | only offer it when a condition holds |
| `any_flags` | only offer it if **any** of these flags is set |
| `any_items` | only offer it if the player holds **any** of these items |
| `set_flags`, `add_items`, `add`, `set` | changes made when the player takes it ([§7](#7-making-choices-matter)) |

**Every scene that isn't an ending must always have at least one choice the player can
take.** `theatre check` finds places where they can't.

---

## 7. Making choices matter

Four ways to remember what the player did:

| Kind | Example | Set with | Test with |
|---|---|---|---|
| **flag**: something happened | `met_the_keeper` | `set_flags = ["met_the_keeper"]` | `flag('met_the_keeper')`, `any_flags` |
| **item**: something they carry | `Brass Key` | `add_items = ["Brass Key"]` | `item('Brass Key')`, `any_items` |
| **counter**: a number | `trust`, `fear` | `add = { trust = 1 }`, `set = { fear = 0 }` | `trust >= 2` |
| **visited**: a scene they've seen | `the_stairs` | automatic | `visited('the_stairs')` |

Flags, items and counter changes can go on a **scene** (they happen on entering it, every
time) or on a **choice** (when it's taken). Counters you never set are `0`.

### Conditions

`if = "..."` works on choices, dialogue lines and variants:

```toml
if = "trust >= 2 && item('Brass Key')"
if = "fear > 3 || flag('saw_the_ghost')"
if = "!visited('the_stairs')"
if = "trust - fear >= 1"
```

| You can use | Meaning |
|---|---|
| `trust` | a counter's value |
| `flag('x')`, `item('x')`, `visited('scene')` | yes/no tests (single or double quotes) |
| `+ -` | add, subtract |
| `== != < <= > >=` | compare |
| `!` | not |
| `&&` `\|\|` | and, or |
| `( )` | group |

A typo in a condition is reported when the story loads, naming the scene and the choice.

### Different openings (variants)

The first variant whose condition holds replaces the scene's `text`:

```toml
[the_lamp_room]
text = "The lamp room is cold and dark."
variants = [
  { if = "fear >= 3", text = "The lamp room. Your hands won't stop shaking." },
  { if = "item('Brass Key')", text = "The lamp room. The key is warm in your pocket." },
]
```

Use variants when a scene can be reached in different ways: the text should match
how the player got there.

### Counters in text

`{counter}` puts a counter's value into any text:

```toml
{ who = "keeper", line = "{trust} times you've kept your word. That's {trust} more than most." }
```

Write `{{` and `}}` for literal braces.

### A full example

```toml
[[storm.choices]]
text = "Pull the keeper inside"
goto = "shelter"
add = { trust = 1 }

[[storm.choices]]
text = "Leave him to the storm"
goto = "alone"
add = { trust = -2 }
set_flags = ["left_him"]

[final_door]
text = "The keeper stands between you and the stairs."
dialogue = [
  { who = "keeper", line = "I don't forget what you did out there.", if = "flag('left_him')", mood = "angry" },
  { who = "keeper", line = "Go on. I trust you.", if = "trust >= 2", mood = "happy" },
]

[[final_door.choices]]
text = "Ask him to come with you"
goto = "ending_together"
if = "trust >= 2"

[[final_door.choices]]
text = "Push past him"
goto = "ending_alone"
```

---

## 8. Characters and portraits

You don't have to draw. Describe characters in **`cast.toml`** and the generator draws
them, with all 8 expressions, blinking, breathing and talking.

```toml
[keeper]
name = "The Keeper"
hair_style = "short"
hair = "#b8b8b8"
hat = "cap"
outfit = "vest"
cloth = "#2a3a4a"
extras = ["beard", "wrinkles", "scar"]
eyes = "#5a7a9a"
```

| Option | Choices |
|---|---|
| `hair_style` | `messy` `slick` `short` `long` `bob` `undercut` `bald` |
| `hat` | `none` `fedora` `cap` `police` |
| `outfit` | `trench` `suit` `leather` `uniform` `dress` `vest` `rags` |
| `extras` | `stubble` `mustache` `beard` `scar` `cigarette` `glasses` `earrings` `cyber_eye` `neon_streak` `tattoo` `freckles` `lashes` `wrinkles` `pinstripe` `gray_temples` |
| colours (`#rrggbb`) | `skin` `hair` `brows` `eyes` `lips` `cloth` `shirt` `accent` `hat_color` `metal` `neon` `glasses` |

Then:

```sh
python3 tools/portrait.py stories/haunted_lighthouse
python3 tools/portrait.py stories/haunted_lighthouse --preview /tmp/faces   # also save pictures to look at
```

This writes `characters/keeper.toml`. Link it to a speaker with `sprite = "keeper"`
in `story.toml`. Change `cast.toml` and run it again to tweak a character.

Hand-made sprites in `characters/` that aren't in `cast.toml` are left alone. Look at
the bundled stories' character files to see the pixel-art format.

---

## 9. Pictures and ASCII art

### Scene pictures

```toml
[the_lighthouse]
image = "images/lighthouse.svg"     # .svg, .png or .jpg, inside the story folder
art = """
      /\\
     /  \\
    | [] |
    |    |
"""                                 # shown when the player turns pictures off
```

- **SVG** is the best choice: it's drawn sharp at any size, and you can make SVGs with
  Inkscape, Figma, or by asking an AI to write one.
- **Terminals with image support** (Kitty, iTerm2, WezTerm, Ghostty, and Sixel terminals)
  show the real picture; others draw it with coloured character cells.
- **Paths** must stay inside the story folder (`images/x.svg`, not `../x.svg` or
  an absolute path).
- **Dark pictures** with a strong subject and little fine text look best.

### ASCII art

`art` alone (no `image`) shows the art on the stage. In TOML strings a backslash
must be written twice (`\\`), as in the example above.

### Animated art

```toml
art_frames = [
"""
  .  '  .  '
 '  .  '  .""",
"""
 '  .  '  .
  .  '  .  '""",
]
art_ms = 300          # time per frame (default 150)
art_loop = true       # false = stop on the last frame
```

---

## 10. Mood and colour

Each scene has a **mood** that sets its colours:

| Mood | Look | For |
|---|---|---|
| `noir` | gold on black | the default, investigation |
| `danger` | red | fights, threats (with the occasional glitch) |
| `alert` | orange | tension |
| `calm` | blue | safe moments |
| `mystery` | violet | secrets, the supernatural |

Set it with `mood = "danger"`. If a scene doesn't set one, words in its **id** pick one:

| Id contains | Mood |
|---|---|
| battle, fight, nightmare, attack, death, blood | danger |
| awakening, spell, warning, danger, threat | alert |
| rest, safe, recovery, memory, dream | calm |
| shadow, darkness, unknown, secret, revelation | mystery |

Otherwise it gets the story's `default_mood`.

**Recolour** any mood for your story in `story.toml`:

```toml
[themes.noir]
accent = "#ff2a6d"      # the main highlight colour
border = "#00d4ff"
bg = "#0a0a12"          # also: panel, text, dim, mote (the drifting particles)
```

---

## 11. Screen effects and transitions

### Effects

`fx = "..."` on a dialogue line (plays as it starts) or on a scene (plays once the
scene is on screen):

| Effect | Looks like | Use for |
|---|---|---|
| `shake` | the screen jolts | gunshots, blows, slammed doors |
| `glitch` | rows tear, noise | static, broken tech, drugs |
| `flash` | white flash | shocks, revelations |
| `blood` | red wash that drains away | wounds, violence |
| `blackout` | black, then slowly back | knocked out, lights cut |
| `lightning` | two quick white flashes | lightning, camera flashes |
| `heartbeat` | the screen throbs dark twice | fear, adrenaline |
| `dizzy` | colours swim | drugs, poison, vertigo |
| `chill` | colours drain cold | dread, ghosts |
| `static` | breaks apart, pulls back together | shock, memory, signal loss |
| `reveal` | uncovered left to right | a reveal |

Players can make every effect gentler or turn them off in Settings, so the story
must never *depend* on an effect to make sense.

### Transitions

How a scene comes on screen: `transition = "..."` on a scene, or for the whole
story in `story.toml`:

| Transition | Use for |
|---|---|
| `dissolve` (default) | most scenes |
| `fade` | time passing, waking up, somber endings |
| `sweep` | moving to a new place |
| `rise` | climbing, rising, hope |
| `coalesce` | dreams, memories, flashbacks |
| `cut` | sudden violence, shocks |

**Use effects sparingly.** One strong effect at the right moment beats ten
everywhere. Never put effects on several lines in a row.

---

## 12. Sound

Sound is generated by the game, so there are no files to find, though you can add your own.

### One-off sounds

`sound = "..."` on a dialogue line or a scene:

`gunshot` `thunder` `knock` `phone` `siren` `glass` `heartbeat` `impact` `static`
`sting` `whoosh` `warble` `chill` `chime` `footsteps` `silence`

**Effects make a fitting sound on their own:** lightning → thunder, shake → impact,
heartbeat → heartbeat, and so on. Give `sound` only when you want something else:

```toml
{ who = "keeper", line = "Down!", fx = "shake", sound = "gunshot" }   # a shot, not a thud
{ who = "voice", line = "...", fx = "glitch", sound = "silence" }       # a silent glitch
```

### Background loops

```toml
# story.toml
ambience = "rain"            # every scene
[ambience_moods]
danger = "drone"             # scenes of this mood

# a scene
[the_lamp_room]
ambience = "wind"            # this scene only
```

Loops: `rain` `storm` `city` `neon` `wind` `drone` `dream` `silence`.
A scene uses its own, else its mood's, else the story's. Changes cross-fade.

**Tip:** give a whole run of scenes in one place the same loop, so the sound doesn't
switch back and forth.

### Your own audio

```toml
sound = "sounds/foghorn.ogg"
ambience = "sounds/sea.ogg"
```

Supported formats are `.ogg`, `.wav`, `.mp3` and `.flac`, and files must be inside the
story folder. Only use audio you have the rights to (freesound.org has many CC0 sounds).

Sound is off until the player turns it on, so it's an extra layer, never required to
follow the story.

---

## 13. Testing your story

### Rehearse while you write

```sh
theatre rehearse haunted_lighthouse
theatre rehearse haunted_lighthouse --scene final_door --set trust=2 --set flag:left_him --set "item:Brass Key"
```

- **Reloads** every time you save a file. A mistake keeps the last good version
  running and shows what's wrong.
- **`--scene`** starts anywhere; **`--set`** starts with counters (`trust=2`),
  flags (`flag:x`) and items (`item:Name`).
- **`d`** shows the current scene, counters, flags and items.
- **Nothing is saved:** no autosaves, and no endings or read-text records.

### Check everything

```sh
theatre check haunted_lighthouse
```

It first **validates** the story: scene links, speakers, expressions, conditions,
pictures and sound files. Then it **plays every possible route** through it, carrying flags,
items and counters, and reports:

| ✗ problem | meaning |
|---|---|
| ending … can't be reached | no route leads there |
| scene … can't be reached | dead text nobody will read |
| the player can be left with no choice | every choice is locked in some situation (it shows a route there) |
| choice … never opens | its condition can never be true |
| line … is never said / variant … is never used | its condition can never be true |

| ⚠ warning | meaning |
|---|---|
| choice … has a condition but is always open | the condition never matters (maybe intended) |
| line … has a condition but is always said | same, for a line |

### Find your way around

```sh
theatre route haunted_lighthouse ending_together     # the shortest way to reach a scene
theatre map haunted_lighthouse > map.md              # the whole story as a diagram
theatre map haunted_lighthouse --dot > map.dot       # for Graphviz
```

Wrap `map.md` in a ```` ```mermaid ```` block. GitHub, VS Code and many editors
draw it: endings are rounded, locked choices dashed, unreachable scenes greyed out.

---

## 14. Writing tips

- **Open strong.** The first scene should put the player in a situation, not explain the world.
- **Two or three choices** is the sweet spot. Each should be a real decision, with no
  obviously right answer.
- **Make choices echo.** A flag set in chapter 1 that changes a line in chapter 3 makes
  the story feel alive. Counters are great for this: trust, fear, evidence.
- **Let the text match the route.** A scene reached from two directions needs a variant.
  "You run up the stairs" is wrong for the player who came down them.
- **Every ending should feel earned.** Show *why* this ending happened through the
  choices that led to it. Bad endings are fine; random ones aren't.
- **Keep lines short.** Dialogue is read one line at a time with a portrait. One or two
  sentences per line works best; split longer speeches.
- **Use `thought = true`** speakers for the player character's inner voice.
- **Give endings names** that sound good in the gallery: `ending_lamp_lit`, not `end3`.
- **Test like a player.** Rehearse each major branch, then let `theatre check` find the
  ones you missed.

---

## 15. Sharing your story

A story is just its folder.

- **Share the folder** (zip it). Players unzip it into `~/.terminal_theatre/stories/`
  (Windows: `%USERPROFILE%\.terminal_theatre\stories\`), and it shows up in the menu.
- **Contribute it to the game** with a pull request that adds `stories/your_story/`.
  See [CONTRIBUTING.md](../CONTRIBUTING.md). It must pass `theatre check`.

A broken story never crashes the game: it shows up in the story list with its error.

---

## 16. Reference: every field

### story.toml

| Field | Required | Meaning |
|---|---|---|
| `title` | ✅ | shown on the story list and title card |
| `description` | ✅ | one or two lines |
| `start` | ✅ | id of the first scene |
| `summary` | | tagline under the title |
| `blurb` | | longer pitch (defaults to `description`) |
| `player` | | speaker whose portrait waits during choices |
| `cover` | | sprite (file in `characters/`) on the story list |
| `cover_mood` | | mood of the story list entry (default `mystery`) |
| `default_mood` | | mood for scenes without one (default `noir`) |
| `transition` | | default scene transition (default `dissolve`) |
| `ambience` | | default background loop |
| `[ambience_moods]` | | background loop per mood |
| `[themes.<mood>]` | | colours: `bg` `panel` `border` `text` `dim` `accent` `mote` |
| `[speakers.<id>]` | | `name` (required), `sprite`, `thought`, `color` |

### Scene `[id]`

| Field | Meaning |
|---|---|
| `text` | narration (required) |
| `variants` | `[{ if = "…", text = "…" }, …]`: other openings |
| `dialogue` | list of lines (below) |
| `choices` | `[[id.choices]]` tables (below) |
| `ending` | `true` = the story ends here |
| `mood` | `noir` `danger` `alert` `calm` `mystery` |
| `image` | picture file inside the story folder |
| `art` | ASCII art |
| `art_frames`, `art_ms`, `art_loop` | animated ASCII art |
| `transition` | how it comes on screen |
| `fx` | effect once it's on screen |
| `sound` | sound once it's on screen |
| `ambience` | background loop |
| `set_flags`, `add_items`, `add`, `set` | changes on entering (every time) |

### Dialogue line

| Field | Meaning |
|---|---|
| `who` | speaker id (required) |
| `line` | the text (required) |
| `mood` | expression |
| `fx` | effect |
| `sound` | sound |
| `if` | condition |

### Choice `[[id.choices]]`

| Field | Meaning |
|---|---|
| `text` | the choice (required) |
| `goto` | next scene (required) |
| `if` | condition |
| `any_flags`, `any_items` | needs any of these |
| `set_flags`, `add_items`, `add`, `set` | changes when taken |

Unknown fields are errors, so a misspelt field name is caught when the story loads.
