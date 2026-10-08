# Publishing Terminal Theatre

Every release starts with a version tag. GitHub Actions does the rest: it
builds the game, publishes it on GitHub Releases, and updates any of the other
channels you've switched on.

| Channel | Players type | You set up | Automatic after setup |
|---|---|---|---|
| GitHub Releases | `curl -fsSL …/install.sh \| sh` | nothing | yes |
| Homebrew tap | `brew install mirzabaig313/tap/terminal-theatre` | a tap repo + 1 secret | yes |
| itch.io | download from your itch page, or the itch app | a game page + 1 secret + 1 variable | yes |
| crates.io | `cargo install terminal-theatre` | an account + 1 command per release | no (on purpose) |

---

## 1. GitHub Releases (always on)

1. Put the new version in `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.2.0"
   ```
   Also change `theatre-engine = { path = "engine", version = "0.2.0" }` under
   `[workspace.dependencies]` in the same file.
2. Commit, push, and wait for CI to pass (Actions tab).
3. Tag and push the tag:
   ```sh
   git tag v0.2.0
   git push origin v0.2.0
   ```

The **Release** workflow:
- checks that the tag matches `Cargo.toml`
- builds for 7 targets: macOS arm64/x86_64, Linux x86_64/arm64 (each with sound and `-silent`), Windows x86_64
- publishes the release with `.sha256` checksums, `install.sh` and `install.ps1`

Players install with:
```sh
curl -fsSL https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.sh | sh
```
```powershell
irm https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.ps1 | iex
```

A broken release can be deleted on the Releases page; delete its tag too
(`git push origin :refs/tags/v0.2.0`) before tagging again.

**Shorter install URL (optional):**
1. In repo Settings → Pages, choose "Deploy from a branch", branch `main`, folder `/ (root)`.
   `https://mirzabaig313.github.io/TERMINAL-THEATRE/install.sh` now works.
2. For your own domain, also set Settings → Pages → Custom domain (e.g.
   `theatre.atmez.ai`) and add a DNS `CNAME` record `theatre` → `mirzabaig313.github.io`.

   Players then type `curl -fsSL https://theatre.atmez.ai/install.sh | sh`.

---

## 2. Homebrew tap

A *tap* is a GitHub repo of Homebrew formulas. Its name must start with `homebrew-`.

1. **Create the tap repo.** On GitHub, create a new **public** repository named
   **`homebrew-tap`** under your account, with a README so it isn't empty.
2. **Create a token that can write to it.** Go to GitHub → Settings → Developer settings →
   Personal access tokens → **Fine-grained tokens** → Generate new token.
   - Repository access: *Only select repositories* → `homebrew-tap`
   - Permissions → Repository → **Contents: Read and write**
   - Expiration: up to a year (put a reminder in your calendar to renew it)
3. **Give it to the game's repo.** In `TERMINAL-THEATRE` → Settings → Secrets and
   variables → Actions → **New repository secret**:
   - Name: `HOMEBREW_TAP_TOKEN`
   - Value: the token
4. *(Only if the tap isn't `<you>/homebrew-tap`:)* on the same page, under the
   **Variables** tab, add `HOMEBREW_TAP` = `owner/repo`.
5. Push the next version tag. The **homebrew** job writes
   `Formula/terminal-theatre.rb` into the tap with the new URLs and checksums.

Players then run:
```sh
brew install mirzabaig313/tap/terminal-theatre
brew upgrade terminal-theatre      # later versions
```
To check a formula yourself: `brew install --verbose mirzabaig313/tap/terminal-theatre && brew test terminal-theatre`.

Notes:
- **Builds used:** macOS gets the builds with sound. Linux Homebrew gets the
  `-silent` builds, because Homebrew's own ALSA library isn't found by the game.
- **Release already published:** to add Homebrew to it, re-run the release
  workflow's **homebrew** job from the Actions tab once the secret exists.

---

## 3. itch.io

itch.io is where indie games are found. Releases go to it with **butler**, its upload tool.

1. **Create the game page.** Create an account at https://itch.io, then Dashboard →
   **Create new project**:
   - Kind of project: **Downloadable**
   - Classification: **Games**
   - Pricing: free, or "No payments" (pay-what-you-want also works)
   - Description: point people at the terminal. "Run `theatre` in a terminal
     (Windows Terminal, iTerm2, WezTerm, Kitty…)."

     Add screenshots: `SNAP_DIR=/tmp/frames cargo test --test flow` saves frames
     you can convert to PNG.
   - Save as **Draft** for now.

   The page URL tells you the game's name for butler:
   `https://mirzabaig313.itch.io/terminal-theatre` → `mirzabaig313/terminal-theatre`.
2. **Get an API key.** Go to https://itch.io/user/settings/api-keys → **Generate new API key**.
3. **Add the key and the game name to the repo.** In repo Settings → Secrets and variables → Actions:
   - **Secrets** tab: `BUTLER_API_KEY` = the key
   - **Variables** tab: `ITCH_GAME` = `mirzabaig313/terminal-theatre`
4. **Push the next version tag.** The **itch** job uploads each build to its own
   channel: `mac-arm64`, `mac-intel`, `linux-x64`, `linux-arm64` and `windows`.
5. **Finish the page.** Back on the itch page, under Uploads, mark each channel's
   platform (the Apple, Linux and Windows icons). Then set the page to **Public**.

Notes:
- **Updates:** butler only uploads what changed, and players who use the itch app update automatically.
- **Where the game runs:** the downloaded game is a terminal program. Double-clicking it opens a
  terminal on macOS and Windows; on Linux, players run `./theatre` from a terminal.
- **macOS warning:** it may say the app is from an unidentified developer, because it isn't
  notarized. Players right-click → Open once, or run
  `xattr -d com.apple.quarantine theatre`. Notarizing needs a paid Apple Developer account.

---

## 4. crates.io

For Rust users: `cargo install terminal-theatre` builds the game from source, with
the stories built in. Publishing is **permanent**: a version can be yanked
(hidden from new installs) but never deleted or replaced. That's why this one is
a manual command, not part of the release workflow.

Both names are free as of this writing: `terminal-theatre` (the game) and
`theatre-engine` (the story engine it depends on).

**One-time setup:**
1. Log in at https://crates.io with GitHub.
2. Account Settings → **verify your email** (publishing requires it).
3. Account Settings → **API Tokens** → New token with the scopes
   `publish-new` and `publish-update`. Then on your machine:
   ```sh
   cargo login            # paste the token
   ```

**Each release** (after the tag's GitHub release succeeded):
```sh
git checkout v0.2.0
cargo publish --workspace --dry-run    # packages and builds both crates; uploads nothing
cargo publish --workspace              # publishes theatre-engine, then terminal-theatre
```

Notes:
- **Stories:** `tui/stories` is a link to `../stories`, so the packaged game
  carries the stories; `cargo package --list -p terminal-theatre` shows them.
- **Linux sound:** Linux users need `libasound2-dev` to build with sound, or
  `cargo install terminal-theatre --no-default-features` for a silent game.
- **License:** the crates say `AGPL-3.0-only`. If you meant "version 3 or any
  later version", change `license` in `Cargo.toml` to `AGPL-3.0-or-later` before
  the first publish.
- **Yanking:** `cargo yank --version 0.2.0 terminal-theatre` hides a bad
  version from new installs.

---

## Checklist for a release

- [ ] Version bumped in `Cargo.toml` (both places), committed, CI green
- [ ] `git tag vX.Y.Z && git push origin vX.Y.Z`
- [ ] Release page has 7 downloads, 7 `.sha256` files, `install.sh`, `install.ps1`
- [ ] Homebrew job green (if set up): `brew upgrade terminal-theatre` gets the new version
- [ ] itch job green (if set up)
- [ ] `cargo publish --workspace` (if publishing to crates.io)
