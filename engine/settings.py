"""Configuration and runtime settings for Terminal Theatre."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, Iterable


@dataclass
class GameSettings:
    """Persistent settings shared between menus and gameplay."""

    color_enabled: bool = True
    text_speed: str = "Normal"
    cinematics_enabled: bool = True
    sound_enabled: bool = False
    typewriter_enabled: bool = True

    _speed_presets: Dict[str, float] = field(default_factory=lambda: {
        "Relaxed": 1.35,
        "Normal": 1.0,
        "Fast": 0.65,
        "Instant": 0.0,
    })

    def available_speeds(self) -> Iterable[str]:
        return self._speed_presets.keys()

    def adjust_delay(self, base_delay: float) -> float:
        """Apply the current speed preset to a base delay value."""
        if not self.typewriter_enabled:
            return 0.0
        multiplier = self._speed_presets.get(self.text_speed, 1.0)
        return 0.0 if multiplier == 0.0 else max(base_delay * multiplier, 0.0)

    def cycle_text_speed(self) -> str:
        """Cycle through predefined speed presets."""
        presets = list(self._speed_presets.keys())
        try:
            idx = presets.index(self.text_speed)
        except ValueError:
            idx = 0
        self.text_speed = presets[(idx + 1) % len(presets)]
        if self.text_speed == "Instant":
            self.typewriter_enabled = False
        else:
            self.typewriter_enabled = True
        return self.text_speed

    def toggle_colors(self) -> bool:
        """Toggle color rendering preference."""
        self.color_enabled = not self.color_enabled
        return self.color_enabled

    def toggle_cinematics(self) -> bool:
        """Toggle whether opening cinematics play automatically."""
        self.cinematics_enabled = not self.cinematics_enabled
        return self.cinematics_enabled

    def toggle_sound(self) -> bool:
        """Toggle sound preference (placeholder for future expansion)."""
        self.sound_enabled = not self.sound_enabled
        return self.sound_enabled

    def toggle_typewriter(self) -> bool:
        """Enable or disable the typewriter effect entirely."""
        self.typewriter_enabled = not self.typewriter_enabled
        if not self.typewriter_enabled:
            self.text_speed = "Instant"
        elif self.text_speed == "Instant":
            self.text_speed = "Normal"
        return self.typewriter_enabled
