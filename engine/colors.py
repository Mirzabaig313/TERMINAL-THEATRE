"""Rich color support and visual effects system"""

from rich.console import Console
from rich.text import Text
from rich.panel import Panel
from rich.style import Style
import time
from typing import List
from copy import deepcopy
import sys
import os


class ColorPalette:
    """Pre-defined color palettes for different moods and scenes"""
    
    # Noir/Detective colors
    NOIR_DARK = "#1a1a1a"
    NOIR_SHADOW = "#2d2d2d"
    NOIR_FOG = "#4a4a4a"
    NOIR_NEON_RED = "#ff0040"
    NOIR_NEON_BLUE = "#00d4ff"
    NOIR_AMBER = "#ffb000"
    NOIR_BLOOD = "#8b0000"
    NOIR_RAIN = "#6b8e9e"
    
    # Mood colors
    DANGER_RED = "#ff0000"
    DANGER_DARK = "#cc0000"
    ALERT_YELLOW = "#ffff00"
    ALERT_ORANGE = "#ffa500"
    CALM_BLUE = "#4169e1"
    CALM_CYAN = "#00ffff"
    SAFE_GREEN = "#00ff00"
    NEUTRAL_GRAY = "#808080"
    
    # Text types
    DIALOGUE = "#e0e0e0"
    NARRATION = "#b0b0b0"
    EMPHASIS = "#ffffff"
    CHOICE = "#00ff7f"
    CHOICE_NUMBER = "#ffd700"
    
    # Special effects
    FIRE_RED = "#ff4500"
    FIRE_ORANGE = "#ff8c00"
    FIRE_YELLOW = "#ffd700"
    LIGHTNING_WHITE = "#ffffff"
    LIGHTNING_BLUE = "#87ceeb"
    SMOKE_GRAY = "#696969"
    SMOKE_LIGHT = "#a9a9a9"


class VisualEffects:
    """Visual effects for enhanced atmosphere"""
    
    @staticmethod
    def flicker(console: Console, text: str, color: str, duration: float = 1.0, intensity: int = 5):
        """Create a flickering effect like neon signs"""
        end_time = time.time() + duration
        dim_color = ColorPalette.NOIR_SHADOW
        
        while time.time() < end_time:
            # Bright phase
            console.print(text, style=f"bold {color}", end="\r")
            time.sleep(0.1)
            
            # Dim phase (random intensity)
            if time.time() % 0.5 < 0.25:
                console.print(text, style=f"dim {dim_color}", end="\r")
                time.sleep(0.05)
    
    @staticmethod
    def pulse(console: Console, text: str, color: str, cycles: int = 3):
        """Create a pulsing effect for emphasis"""
        for _ in range(cycles):
            # Fade in (bright)
            console.print(text, style=f"bold {color}")
            time.sleep(0.3)
            
            # Fade out (dim)
            console.print(text, style=f"dim {color}", end="\r")
            time.sleep(0.3)
    
    @staticmethod
    def rain_colored() -> List[str]:
        """Create colored rain animation frames"""
        rain_color = ColorPalette.NOIR_RAIN
        frames = []
        
        # Frame 1
        frame1 = Text()
        frame1.append("\n    |  '  |     '    |   '     |\n", style=rain_color)
        frame1.append("  '    |    '   |       |   '\n", style=rain_color)
        frame1.append("      |   '    |    '       |\n", style=rain_color)
        frame1.append("   '     |       '  |    '\n", style=rain_color)
        frames.append(frame1)
        
        # Frame 2
        frame2 = Text()
        frame2.append("\n  '    |    '   |       |   '\n", style=rain_color)
        frame2.append("      |   '    |    '       |\n", style=rain_color)
        frame2.append("   '     |       '  |    '\n", style=rain_color)
        frame2.append("    |  '  |     '    |   '     |\n", style=rain_color)
        frames.append(frame2)
        
        # Frame 3
        frame3 = Text()
        frame3.append("\n      |   '    |    '       |\n", style=rain_color)
        frame3.append("   '     |       '  |    '\n", style=rain_color)
        frame3.append("    |  '  |     '    |   '     |\n", style=rain_color)
        frame3.append("  '    |    '   |       |   '\n", style=rain_color)
        frames.append(frame3)
        
        return frames
    
    @staticmethod
    def fire_effect() -> List[Text]:
        """Create a fire animation with color gradient"""
        frames = []
        
        fire_patterns = [
            "    (  )    ( (   )  )    ",
            "   (    ) (     )   (  )  ",
            "    ( )    (  )    (   )  ",
        ]
        
        colors = [ColorPalette.FIRE_YELLOW, ColorPalette.FIRE_ORANGE, ColorPalette.FIRE_RED]
        
        for i, pattern in enumerate(fire_patterns):
            frame = Text()
            # Layer colors for depth
            frame.append(pattern, style=f"bold {colors[i % len(colors)]}")
            frames.append(frame)
        
        return frames
    
    @staticmethod
    def lightning_flash(console: Console, text: str, flashes: int = 2):
        """Create lightning flash effect"""
        for _ in range(flashes):
            console.print(text, style=f"bold on white {ColorPalette.LIGHTNING_WHITE}")
            time.sleep(0.05)
            console.print(text, style=f"{ColorPalette.NOIR_DARK}")
            time.sleep(0.2)
    
    @staticmethod
    def color_transition(console: Console, text: str, from_color: str, to_color: str, steps: int = 10):
        """Smooth color transition effect"""
        # Note: This is a simplified version - Rich can handle gradients
        for i in range(steps):
            progress = i / steps
            if progress < 0.5:
                style = from_color
            else:
                style = to_color
            console.print(text, style=style, end="\r")
            time.sleep(0.1)


class ColorRenderer:
    """Enhanced renderer with Rich color support"""
    
    def __init__(self):
        self.console = Console()
        self.supports_color = self._check_color_support()
    
    def _check_color_support(self) -> bool:
        """Check if terminal supports colors"""
        # Rich handles this automatically, but we track it for graceful fallback
        return self.console.is_terminal and not os.environ.get('NO_COLOR')
    
    def clear(self):
        """Clear the terminal screen"""
        self.console.clear()
    
    def print_colored(self, text: str, color: str = None, style: str = None, end: str = "\n"):
        """Print colored text"""
        full_style = ""
        if color:
            full_style = color
        if style:
            full_style = f"{style} {full_style}" if full_style else style
        
        if self.supports_color and full_style:
            self.console.print(text, style=full_style, end=end)
        else:
            # Fallback for no color support
            print(text, end=end)
    
    def print_dialogue(self, speaker: str, text: str, speaker_color: str = None, 
                       text_color: str = None, delay: float = 0.03):
        """Print dialogue with color and typewriter effect"""
        # Speaker name
        speaker_style = speaker_color or ColorPalette.EMPHASIS
        self.console.print(f"\n{speaker}:", style=f"bold {speaker_style}")
        
        # Dialogue text with typewriter effect
        text_style = text_color or ColorPalette.DIALOGUE
        if self.supports_color:
            for char in text:
                self.console.print(char, style=text_style, end="")
                time.sleep(delay)
            self.console.print("\n")
        else:
            # Fast fallback
            print(text)
    
    def print_narration(self, text: str, color: str = None, delay: float = 0.02):
        """Print narration text with color"""
        narration_color = color or ColorPalette.NARRATION
        
        if self.supports_color:
            for char in text:
                self.console.print(char, style=narration_color, end="")
                sys.stdout.flush()
                time.sleep(delay)
            self.console.print()
        else:
            print(text)
    
    def print_choices(self, choices: List[str], 
                     number_color: str = None, 
                     choice_color: str = None,
                     border_color: str = None):
        """Print choices with color"""
        num_color = number_color or ColorPalette.CHOICE_NUMBER
        text_color = choice_color or ColorPalette.CHOICE
        border = border_color or ColorPalette.NOIR_NEON_BLUE
        
        if self.supports_color:
            self.console.print("\n" + "=" * 60, style=border)
            self.console.print("What do you do?", style=f"bold {ColorPalette.EMPHASIS}")
            self.console.print("=" * 60, style=border)
            
            for i, choice in enumerate(choices, 1):
                choice_text = Text()
                choice_text.append(f"{i}. ", style=f"bold {num_color}")
                choice_text.append(choice, style=text_color)
                self.console.print(choice_text)
        else:
            print("\n" + "=" * 60)
            print("What do you do?")
            print("=" * 60)
            for i, choice in enumerate(choices, 1):
                print(f"{i}. {choice}")
    
    def print_ascii_art(self, art: str, color: str = None, style: str = None):
        """Print ASCII art with color"""
        art_color = color or ColorPalette.NOIR_AMBER
        art_style = style or "bold"
        
        if self.supports_color:
            self.console.print(art, style=f"{art_style} {art_color}")
        else:
            print(art)
    
    def print_panel(self, content: str, title: str = None, 
                   border_color: str = None, title_color: str = None):
        """Print content in a colored panel"""
        if self.supports_color:
            border_style = Style(color=border_color or ColorPalette.NOIR_NEON_BLUE)
            panel = Panel(
                content,
                title=title,
                border_style=border_style,
                style=title_color or ColorPalette.EMPHASIS
            )
            self.console.print(panel)
        else:
            if title:
                print(f"\n{'=' * 60}")
                print(f"  {title}")
                print('=' * 60)
            print(content)
    
    def print_gradient_text(self, text: str, color1: str, color2: str):
        """Print text with color gradient"""
        if self.supports_color:
            # Use Rich's gradient capability
            styled_text = Text(text)
            styled_text.stylize(f"bold {color1} on {color2}")
            self.console.print(styled_text)
        else:
            print(text)
    
    def pause(self, message: str = "\nPress ENTER to continue...", 
             color: str = None):
        """Pause with colored message"""
        pause_color = color or ColorPalette.NEUTRAL_GRAY
        if self.supports_color:
            self.console.print(message, style=f"dim {pause_color}", end="")
        else:
            print(message, end="")
        input()


class MoodColors:
    """Scene mood color configurations"""
    
    NOIR_DETECTIVE = {
        'narration': ColorPalette.NARRATION,
        'dialogue': ColorPalette.DIALOGUE,
        'emphasis': ColorPalette.NOIR_NEON_RED,
        'border': ColorPalette.NOIR_NEON_BLUE,
        'ascii_art': ColorPalette.NOIR_AMBER
    }
    
    DANGER = {
        'narration': ColorPalette.DANGER_DARK,
        'dialogue': ColorPalette.DIALOGUE,
        'emphasis': ColorPalette.DANGER_RED,
        'border': ColorPalette.DANGER_RED,
        'ascii_art': ColorPalette.DANGER_RED
    }
    
    CALM = {
        'narration': ColorPalette.CALM_BLUE,
        'dialogue': ColorPalette.DIALOGUE,
        'emphasis': ColorPalette.CALM_CYAN,
        'border': ColorPalette.CALM_BLUE,
        'ascii_art': ColorPalette.CALM_CYAN
    }
    
    ALERT = {
        'narration': ColorPalette.ALERT_ORANGE,
        'dialogue': ColorPalette.DIALOGUE,
        'emphasis': ColorPalette.ALERT_YELLOW,
        'border': ColorPalette.ALERT_YELLOW,
        'ascii_art': ColorPalette.ALERT_YELLOW
    }
    
    MYSTERY = {
        'narration': ColorPalette.NOIR_FOG,
        'dialogue': ColorPalette.DIALOGUE,
        'emphasis': ColorPalette.NOIR_AMBER,
        'border': ColorPalette.NOIR_NEON_BLUE,
        'ascii_art': ColorPalette.NOIR_FOG
    }


def get_mood_palette(mood_name: str) -> dict:
    """Get a mood color palette by name"""
    moods = {
        'noir': MoodColors.NOIR_DETECTIVE,
        'danger': MoodColors.DANGER,
        'calm': MoodColors.CALM,
        'alert': MoodColors.ALERT,
        'mystery': MoodColors.MYSTERY
    }
    return deepcopy(moods.get(mood_name.lower(), MoodColors.NOIR_DETECTIVE))


class CharacterColors:
    """Color mapping for key characters"""
    
    DEFAULT = ColorPalette.DIALOGUE
    SPEAKER_STYLES = {
        'JACK MALONE': ColorPalette.NOIR_NEON_BLUE,
        'EDDIE': ColorPalette.CALM_CYAN,
        'CASTELLANO': ColorPalette.NOIR_NEON_RED,
        'TONY': ColorPalette.DANGER_RED,
        "TONY 'THE FIST'": ColorPalette.DANGER_RED,
        'MORRISON': ColorPalette.ALERT_ORANGE,
        'BLACKWOOD': ColorPalette.NOIR_BLOOD,
        'RODRIGUEZ': ColorPalette.SAFE_GREEN,
        'CHEN': ColorPalette.CALM_CYAN,
        'NARRATOR': ColorPalette.NARRATION,
        'VOICE FROM DARKNESS': MoodColors.MYSTERY['emphasis'],
    }
    
    @classmethod
    def get_color(cls, speaker: str) -> str:
        key = speaker.upper()
        return cls.SPEAKER_STYLES.get(key, cls.DEFAULT)
