#!/bin/sh
# Terminal Theatre installer for macOS and Linux.
#
#   curl -fsSL https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.sh | sh
#
# Downloads the right build from GitHub Releases, checks its SHA-256, and
# puts `theatre` in ~/.local/bin. Settings:
#   THEATRE_VERSION=v0.2.0         a specific release (default: the latest)
#   THEATRE_INSTALL_DIR=/some/bin  where to put `theatre`
#   THEATRE_RELEASE_URL=...        download from a mirror (or a local folder, for testing)
# Windows: use install.ps1 instead.

set -eu

REPO="Mirzabaig313/TERMINAL-THEATRE"
VERSION="${THEATRE_VERSION:-latest}"
INSTALL_DIR="${THEATRE_INSTALL_DIR:-$HOME/.local/bin}"

say() { printf '%s\n' "$*"; }
fail() { printf 'error: %s\n' "$*" >&2; exit 1; }

download() { # url file
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$1" -o "$2"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$1" -O "$2"
    else
        fail "need curl or wget"
    fi
}

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | cut -d' ' -f1
    else
        fail "need sha256sum or shasum to check the download"
    fi
}

os=$(uname -s)
arch=$(uname -m)
case "$arch" in
    x86_64 | amd64) arch=x86_64 ;;
    arm64 | aarch64) arch=aarch64 ;;
    *) fail "no build for this processor ($arch)" ;;
esac
case "$os" in
    Darwin)
        # a shell running under Rosetta still gets the Apple Silicon build
        if [ "$arch" = x86_64 ] && [ "$(sysctl -n hw.optional.arm64 2>/dev/null || echo 0)" = 1 ]; then
            arch=aarch64
        fi
        target="$arch-apple-darwin"
        ;;
    Linux)
        target="$arch-unknown-linux-gnu"
        # sound needs the ALSA library; without it, get the silent build
        if ! (ldconfig -p 2>/dev/null | grep -q 'libasound\.so\.2') \
            && ! ls /usr/lib/*/libasound.so.2 /usr/lib/libasound.so.2 /lib/*/libasound.so.2 >/dev/null 2>&1; then
            say "No ALSA sound library found (libasound2): installing the silent build."
            say "For sound, install libasound2 (e.g. sudo apt install libasound2) and run this again."
            target="$target-silent"
        fi
        ;;
    MINGW* | MSYS* | CYGWIN*) fail "on Windows, run in PowerShell: irm https://raw.githubusercontent.com/$REPO/main/install.ps1 | iex" ;;
    *) fail "no build for this system ($os)" ;;
esac

if [ -n "${THEATRE_RELEASE_URL:-}" ]; then
    base="$THEATRE_RELEASE_URL"
elif [ "$VERSION" = latest ]; then
    base="https://github.com/$REPO/releases/latest/download"
else
    base="https://github.com/$REPO/releases/download/$VERSION"
fi
archive="theatre-$target.tar.gz"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

say "Downloading Terminal Theatre ($VERSION, $target)..."
download "$base/$archive" "$tmp/$archive" || fail "download failed: $base/$archive"
download "$base/$archive.sha256" "$tmp/$archive.sha256" || fail "checksum download failed"
want=$(cut -d' ' -f1 <"$tmp/$archive.sha256")
got=$(sha256 "$tmp/$archive")
[ "$want" = "$got" ] || fail "checksum mismatch: the download is damaged or tampered with"

tar -xzf "$tmp/$archive" -C "$tmp"
mkdir -p "$INSTALL_DIR"
cp "$tmp/theatre" "$INSTALL_DIR/theatre"
chmod 755 "$INSTALL_DIR/theatre"

say ""
say "Installed: $INSTALL_DIR/theatre"
case ":$PATH:" in
    *":$INSTALL_DIR:"*) say "Run it with: theatre" ;;
    *)
        say "$INSTALL_DIR is not on your PATH. Add this line to your shell profile (~/.zshrc, ~/.bashrc):"
        say "    export PATH=\"$INSTALL_DIR:\$PATH\""
        say "Then run: theatre"
        ;;
esac
