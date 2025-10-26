"""Terminal input handler for capturing key presses and navigation events."""

from __future__ import annotations

import os
import sys
import select

if os.name == "nt":  # pragma: no cover - platform specific
    import msvcrt  # type: ignore
else:  # pragma: no cover - platform specific
    import termios
    import tty


class InputHandler:
    """Read single key presses (including arrow keys) without requiring ENTER."""

    SPECIAL_KEYS_POSIX = {
        "A": "UP",
        "B": "DOWN",
        "C": "RIGHT",
        "D": "LEFT",
        "H": "HOME",
        "F": "END",
    }

    SPECIAL_KEYS_WINDOWS = {
        b"H": "UP",
        b"P": "DOWN",
        b"M": "RIGHT",
        b"K": "LEFT",
        b"G": "HOME",
        b"O": "END",
    }

    def __init__(self):
        self.is_windows = os.name == "nt"
        self.is_tty = sys.stdin.isatty()
        self.disabled = False

        if not self.is_windows and self.is_tty:
            self.fd = sys.stdin.fileno()
            try:
                self.old_settings = termios.tcgetattr(self.fd)
            except termios.error:
                self.is_tty = False
                self.old_settings = None
                self.disabled = True
        elif self.is_windows:
            self.fd = None
            self.old_settings = None
        else:
            self.fd = None
            self.old_settings = None
            self.disabled = True

    def __enter__(self) -> "InputHandler":
        if not self.is_windows and not self.disabled and self.fd is not None:
            tty.setraw(self.fd)
        return self

    def __exit__(self, exc_type, exc, tb) -> None:
        if not self.is_windows and not self.disabled and self.old_settings is not None:
            termios.tcsetattr(self.fd, termios.TCSADRAIN, self.old_settings)

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------
    def read_key(self) -> str:
        """Read a single key press and normalize the result."""
        if self.disabled:
            return "ENTER"
        if self.is_windows:
            return self._read_windows()  # pragma: no cover - platform specific
        return self._read_posix()  # pragma: no cover - platform specific

    # ------------------------------------------------------------------
    # Platform specific helpers
    # ------------------------------------------------------------------
    def _read_windows(self) -> str:
        """Read key press on Windows terminals."""
        while True:
            ch = msvcrt.getch()

            if ch in (b"\x00", b"\xe0"):
                # Special key (arrow, function keys...)
                next_ch = msvcrt.getch()
                mapped = self.SPECIAL_KEYS_WINDOWS.get(next_ch)
                if mapped:
                    return mapped
                # Ignore unhandled specials, keep listening
                continue

            if ch == b"\r":
                return "ENTER"
            if ch == b"\x1b":
                return "ESC"
            if ch == b"\x08":
                return "BACKSPACE"
            if ch == b"\x03":
                raise KeyboardInterrupt()

            try:
                decoded = ch.decode("utf-8")
            except UnicodeDecodeError:
                continue
            return decoded

    def _read_posix(self) -> str:
        """Read key press on POSIX terminals."""
        try:
            if not sys.stdin or not sys.stdin.isatty():
                return "ENTER"
            
            ch = sys.stdin.read(1)
            
            if not ch:
                return "ENTER"

            if ch == "\x03":
                raise KeyboardInterrupt()

            if ch in ("\r", "\n"):
                return "ENTER"

            if ch == "\x1b":  # Escape or special sequence
                if self._input_available():
                    next_char = sys.stdin.read(1)
                    if next_char == "[":  # CSI sequence for arrows
                        code = sys.stdin.read(1)
                        mapped = self.SPECIAL_KEYS_POSIX.get(code)
                        if mapped:
                            return mapped
                        return "ESC"
                    return "ESC"
                return "ESC"

            if ch == "\x7f":
                return "BACKSPACE"

            return ch
        except (EOFError, OSError):
            return "ENTER"

    def _input_available(self, timeout: float = 0.0001) -> bool:
        """Check if more characters are waiting in the input buffer."""
        readable, _, _ = select.select([sys.stdin], [], [], timeout)
        return bool(readable)
