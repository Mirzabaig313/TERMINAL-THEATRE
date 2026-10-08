# Security Policy

## Supported versions

Security fixes go into the **latest release**. Please update before reporting:
run the installer again, `brew upgrade terminal-theatre`, or download the newest
build from [Releases](https://github.com/Mirzabaig313/TERMINAL-THEATRE/releases).

| Version | Supported |
|---|---|
| latest release | ✅ |
| older releases | ❌ |

## Reporting a vulnerability

**Please don't open a public issue for security problems.**

Report privately through GitHub:

1. Go to the repository's **Security** tab.
2. Click **Report a vulnerability**.
3. Describe the problem, how to reproduce it, and what an attacker could do with it.

You'll get a reply within **7 days**. Once the problem is confirmed, a fix is
released as soon as possible, and you'll be credited in the release notes
(unless you'd rather not be).

## What counts

Terminal Theatre is a single-player game that runs locally. These are in scope:

| Area | Examples |
|---|---|
| **Installers** | `install.sh` or `install.ps1` installing something other than the release, skipping the checksum, or writing outside the install folder |
| **Release integrity** | release files that don't match their published `.sha256` |
| **Story files** | a story folder (TOML, SVG/PNG/JPEG pictures, audio files) that crashes the game in a way that harms the system, reads or writes files outside its folder, or runs code |
| **Saves** | an imported save (`theatre import`) that does more than load a game |
| **Local data** | the game reading or writing outside `~/.terminal_theatre/` (or `THEATRE_HOME`), apart from the story folder you point it at |

Out of scope:
- a story that simply fails to load (that's a bug: open an issue)
- problems in your terminal emulator itself
- denial of service by a story you chose to install (a huge picture that's slow to draw)

## What the game does and doesn't do

So you can judge the risk of playing stories from other people:

- **No network access.** The game never connects to the internet, and it has no
  telemetry or update checks. Only the installers download anything, from GitHub Releases.
- **Stories can't run code.** A story is text, pictures and sounds. Its conditions
  (`if = "trust >= 2"`) are a small expression language that can only read the story's
  own counters, flags, items and visited scenes.
- **Files:** the game writes only to its data folder, `~/.terminal_theatre/`
  (saves database, settings, unpacked built-in stories). Stories are read from
  their own folder; picture and sound paths must stay inside it.
- **Pictures** are decoded and drawn locally (resvg for SVG, the `image` crate
  for PNG/JPEG). **Sounds** are made by the game, or decoded locally from a
  story's own audio files.

## Verifying a download yourself

Every release file has a `.sha256` next to it. The installers check it for you;
to check by hand:

```sh
# macOS / Linux
shasum -a 256 theatre-aarch64-apple-darwin.tar.gz
cat theatre-aarch64-apple-darwin.tar.gz.sha256      # the two must match
```

```powershell
# Windows
Get-FileHash theatre-x86_64-pc-windows-msvc.zip -Algorithm SHA256
Get-Content theatre-x86_64-pc-windows-msvc.zip.sha256
```

Prefer to read an installer before running it? Download it first:

```sh
curl -fsSL https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.sh -o install.sh
less install.sh
sh install.sh
```
