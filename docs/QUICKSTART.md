# Quickstart

From nothing to playing in two minutes.

## 1. Install

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.sh | sh
```

**Windows** (PowerShell)

```powershell
irm https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.ps1 | iex
```

**Other ways:**

| How | Command |
|---|---|
| Homebrew | `brew install mirzabaig313/tap/terminal-theatre` |
| Cargo (Rust users) | `cargo install terminal-theatre` |
| Download | pick your system on the [Releases page](https://github.com/Mirzabaig313/TERMINAL-THEATRE/releases), unpack, run `theatre` |
| From source | `git clone https://github.com/Mirzabaig313/TERMINAL-THEATRE.git && cd TERMINAL-THEATRE && cargo run --release` |

If the installer says `~/.local/bin is not on your PATH`:

```sh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc    # or ~/.bashrc
source ~/.zshrc
```

## 2. Get the best picture

- **Make the window big:** about **120 columns × 48 rows** or more. Smaller works, but the
  characters and pictures shrink.
- **Use a modern terminal:** iTerm2, WezTerm, Kitty, Ghostty or Windows Terminal. The
  terminals built into VS Code, Cursor and Kiro work too.
- **Check your terminal:** run `theatre graphics`. It says whether scene pictures will
  show as sharp images or as softer character cells, and how to improve that. In VS
  Code-based editors, turn on the setting `"terminal.integrated.enableImages": true`.

## 3. Play

```sh
theatre
```

The title sequence plays; press any key to skip it. Then **New Game** → pick a story → **Enter**.

| Key | Action |
|---|---|
| `Enter` / `Space` | continue (press during typing to show the whole line) |
| `↑` `↓` or `1`–`9` | choose |
| `h` | history of everything read |
| `a` | auto-advance |
| `s` | skip text you've already read |
| `F5` / `F9` | quick save / quick load |
| `Esc` | pause menu: save, history, main menu |
| `q` | quit |

**Sound is off at first:** Main menu → **Settings** → **Sound**. Use `←` / `→` on
**Volume** to set the level. Not sure your audio works? `theatre sound` plays every sound once.

## 4. Good to know

- **Saving is automatic.** The game saves at every scene; **Continue** on the main menu
  picks up where you left off. `Esc` → **Save Game** gives you 9 named slots per story.
- **Endings:** each story has many. On the story list, press `e` to see which you've found.
- **Comfort:** too much flashing or shaking? Settings → **Screen Effects** →
  Reduced or Off. Also in Settings: text speed, monochrome, images on/off.
- **Your data** lives in `~/.terminal_theatre/` (Windows: `%USERPROFILE%\.terminal_theatre\`):
  saves, settings, and the stories.
- **Move saves between computers:** `theatre export noir_detective 0 save.json`, then
  `theatre import save.json 1` on the other computer.

## 5. Update or uninstall

| Installed with | Update | Uninstall |
|---|---|---|
| curl / PowerShell | run the install command again | delete `~/.local/bin/theatre` (Windows: `%LOCALAPPDATA%\Programs\TerminalTheatre`) |
| Homebrew | `brew upgrade terminal-theatre` | `brew uninstall terminal-theatre` |
| Cargo | `cargo install terminal-theatre` | `cargo uninstall terminal-theatre` |

To also remove saves and settings: delete `~/.terminal_theatre/`.

## 6. If something looks wrong

| Problem | Fix |
|---|---|
| `theatre: command not found` | add `~/.local/bin` to your PATH (see step 1) |
| macOS: "developer cannot be verified" | right-click `theatre` → Open, or `xattr -d com.apple.quarantine theatre` |
| pictures look blurry or blocky | `theatre graphics` explains why; use a terminal with image support |
| colours look wrong or washed out | try `THEATRE_COLOR=256 theatre` (or `=truecolor`) |
| no sound | Settings → Sound on; `theatre sound` to test; on Linux install `libasound2` and reinstall |
| everything is cramped | make the window bigger, or zoom out (`Cmd`/`Ctrl` + `-`) |
| the editor terminal feels slow | play in a standalone terminal app |
| something else | [open an issue](https://github.com/Mirzabaig313/TERMINAL-THEATRE/issues) |

## 7. Write your own story

```sh
theatre new my_story "My Story"
theatre rehearse my_story
```

Edit the files in `~/.terminal_theatre/stories/my_story/` (or `stories/my_story/`
in a source checkout). The game reloads them every time you save. Everything else is
in the [Writing Guide](WRITING_GUIDE.md).
