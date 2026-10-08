# Contributing to Terminal Theatre

Thanks for wanting to help! There are many ways to contribute, and most of them
don't need any Rust:

| You like… | You could… |
|---|---|
| writing | write a new story, or extend one ([Writing Guide](docs/WRITING_GUIDE.md)) |
| art | design characters in `cast.toml`, draw scene pictures (SVG/PNG) |
| playing | report bugs, typos, confusing choices, endings that feel wrong |
| sound | tune the built-in sounds in `tui/src/audio/synth.rs` |
| Rust | fix bugs, add features, improve performance |

---

## Reporting a bug

Open an [issue](https://github.com/Mirzabaig313/TERMINAL-THEATRE/issues) with:

1. **What happened** and **what you expected**.
2. **How to make it happen again:** the story, the scene (the header shows its name),
   and the choices you made.
3. **Your setup:** operating system, terminal app, window size, and `theatre --version`
   (or the release you installed).
4. For display problems, a **screenshot**, plus the output of `theatre graphics`.

Security problems go through private reporting instead: see [SECURITY.md](SECURITY.md).

---

## Setting up

```sh
git clone https://github.com/Mirzabaig313/TERMINAL-THEATRE.git
cd TERMINAL-THEATRE
cargo run --release
```

- **Rust:** install it with [rustup](https://rustup.rs). The project pins its Rust
  version in `rust-toolchain.toml`; rustup downloads that version automatically
  the first time you build.
- **Linux:** sound needs the ALSA headers:
  `sudo apt install libasound2-dev` (Debian/Ubuntu) or `sudo dnf install alsa-lib-devel` (Fedora).
  Or build without sound: `cargo run --release --no-default-features`.
- **Python 3.11+** (optional): for `tools/portrait.py` (the character generator) and
  `tools/check_story.py`.

---

## How the project is organised

```
engine/   theatre-engine: story format, validation, story flow, saves, settings,
          sprites, sound names. No terminal code.
tui/      terminal-theatre: the `theatre` binary, screens, drawing, effects, audio
stories/  one self-contained folder per story
tools/    portrait.py (character generator), check_story.py (quick checks)
docs/     guides
```

Two rules keep it modular:

1. **The engine never draws.** Anything terminal-related (ratatui, colours on screen,
   audio playback) lives in `tui/`. The engine describes *what* happens; the TUI
   decides *how it looks and sounds*.
2. **Code never names a story or a character.** Everything story-specific lives in
   `stories/<id>/`. Adding a story must never need a code change.

The README has a table of every module.

---

## Before you open a pull request

Run the same checks as CI:

```sh
cargo fmt --all                                    # formatting
cargo clippy --all-targets -- -D warnings          # lints, warnings are errors
cargo test                                         # all tests (~10 s)
cargo run --release -- check                       # every story, every route
```

On Linux, also check the build without sound:

```sh
cargo clippy -p terminal-theatre --no-default-features --all-targets -- -D warnings
```

CI runs all of this on macOS, Linux and Windows for every pull request.

### Tests

- Tests live in their own folders, `engine/tests/` and `tui/tests/`, never inside
  the source files.
- **Fixing a bug:** add a test that fails without the fix.
- **Adding a feature:** add a test that uses it the way a player or story writer would.
- **Drawing a screen:** `tui/tests/common/mod.rs` has a `Harness` that drives the whole
  app off-screen with key presses and a fake clock.

  `SNAP_DIR=/tmp/frames cargo test --test flow` saves every frame as a picture.
- **Story content:** `cargo test` plays every route through every bundled story.
  A change that strands the player or makes an ending unreachable fails the tests.

### Code style

- Match the code around you: its naming, comment density and structure.
- Comments explain *why*, not *what*. Doc comments (`///`) on public items.
- Keep the terminal light. Animations step every 125 ms and idle screens should
  barely change between frames (`tui/tests/performance.rs` checks this). Editor
  terminals lag badly when flooded.
- No new dependency without a good reason; mention it in the PR.

### Commit messages

[Conventional Commits](https://www.conventionalcommits.org), as in the history:

```
feat(engine): add timed choices
fix(tui): keep the choice menu visible in small windows
feat(stories): add three endings to Shadow Slave
docs: explain ambience moods in the writing guide
```

Types: `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `ci`, `chore`.
Scopes: `engine`, `tui`, `render`, `audio`, `stories`, `tools`, `release`.

### Pull request checklist

- [ ] `cargo fmt`, `cargo clippy -- -D warnings` and `cargo test` pass
- [ ] new behaviour has a test
- [ ] README or docs updated if players or writers would notice the change
- [ ] stories still pass `theatre check`
- [ ] one topic per pull request (two features → two PRs)

---

## Contributing a story

1. `cargo run --release -- new my_story "My Story"`
2. Write it, trying it out as you go with `cargo run --release -- rehearse my_story`
   ([Writing Guide](docs/WRITING_GUIDE.md))
3. `cargo run --release -- check my_story` must pass with no ✗
4. Open a pull request with the `stories/my_story/` folder

Stories should be your own work (or properly licensed), and suitable for a
general audience of adults: dark themes are welcome, gratuitous content isn't.

---

## License

Terminal Theatre is licensed under the [GNU AGPL v3](LICENSE). By contributing,
you agree that your contribution is released under the same license.
