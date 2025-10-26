"""Terminal rendering utilities with Rich color support"""

import os
import sys
import time
from typing import List, Union
from rich.text import Text
from .colors import ColorRenderer, ColorPalette, MoodColors, CharacterColors


class TerminalRenderer:
    """Handles terminal rendering with color support"""
    
    def __init__(self, use_colors: bool = True, settings=None):
        self.use_colors = use_colors
        self.settings = settings
        if use_colors:
            try:
                self.color_renderer = ColorRenderer()
            except ImportError:
                self.use_colors = False
                self.color_renderer = None
        else:
            self.color_renderer = None
        
        self.current_mood = MoodColors.NOIR_DETECTIVE
    
    def set_mood(self, mood_colors: dict):
        """Set the color mood for the scene"""
        self.current_mood = mood_colors
    
    def clear(self):
        """Clear the terminal screen"""
        if self.use_colors and self.color_renderer:
            self.color_renderer.clear()
        else:
            os.system('cls' if os.name == 'nt' else 'clear')
    
    def display_frame(self, frame: Union[str, Text], delay: float = 0.05, color: str = None):
        """Display a single frame with optional color"""
        self.clear()
        if isinstance(frame, Text):
            if self.use_colors and self.color_renderer:
                self.color_renderer.console.print(frame)
            else:
                print(str(frame))
        else:
            frame_color = color or (self.current_mood.get('ascii_art') if self.use_colors else None)
            if self.use_colors and self.color_renderer:
                self.color_renderer.print_ascii_art(frame, color=frame_color)
            else:
                print(frame)
        time.sleep(delay)
    
    def _typewriter_enabled(self) -> bool:
        if self.settings is None:
            return True
        return getattr(self.settings, "typewriter_enabled", True)
    
    def _effective_delay(self, delay: float) -> float:
        if self.settings is None:
            return delay
        return self.settings.adjust_delay(delay)
    
    def display_text(self, text: str, delay: float = 0.03, clear_first: bool = True, color: str = None):
        """Display text with typewriter effect and color"""
        if clear_first:
            self.clear()
        
        text_color = color or (self.current_mood.get('narration') if self.use_colors else None)
        effective_delay = self._effective_delay(delay)
        use_typewriter = self._typewriter_enabled() and effective_delay > 0
        
        if self.use_colors and self.color_renderer:
            if use_typewriter:
                self.color_renderer.print_narration(text, color=text_color, delay=effective_delay)
            else:
                self.color_renderer.print_colored(text, color=text_color)
        else:
            if use_typewriter:
                for char in text:
                    sys.stdout.write(char)
                    sys.stdout.flush()
                    time.sleep(effective_delay)
                print()
            else:
                print(text)
    
    def display_dialogue(self, speaker: str, text: str, delay: float = 0.03, 
                        speaker_color: str = None, text_color: str = None):
        """Display dialogue with speaker name and color"""
        if not speaker_color and self.use_colors:
            speaker_color = CharacterColors.get_color(speaker)
        
        s_color = speaker_color or (self.current_mood.get('emphasis') if self.use_colors else None)
        t_color = text_color or (self.current_mood.get('dialogue') if self.use_colors else None)
        effective_delay = self._effective_delay(delay)
        use_typewriter = self._typewriter_enabled() and effective_delay > 0
        
        if self.use_colors and self.color_renderer:
            if use_typewriter:
                self.color_renderer.print_dialogue(speaker, text, 
                                                  speaker_color=s_color, 
                                                  text_color=t_color, 
                                                  delay=effective_delay)
            else:
                self.color_renderer.print_colored(f"\n{speaker}:", color=s_color, style="bold")
                self.color_renderer.print_colored(text, color=t_color)
                self.color_renderer.console.print()
        else:
            print(f"\n{speaker}:")
            if use_typewriter:
                for char in text:
                    sys.stdout.write(char)
                    sys.stdout.flush()
                    time.sleep(effective_delay)
                print("\n")
            else:
                print(text)
                print()
    
    def display_choices(self, choices: List[str]) -> int:
        """Display choices and get user input with color"""
        if self.use_colors and self.color_renderer:
            self.color_renderer.print_choices(
                choices,
                number_color=ColorPalette.CHOICE_NUMBER,
                choice_color=ColorPalette.CHOICE,
                border_color=self.current_mood.get('border', ColorPalette.NOIR_NEON_BLUE)
            )
        else:
            print("\n" + "=" * 60)
            print("What do you do?")
            print("=" * 60)
            for i, choice in enumerate(choices, 1):
                print(f"{i}. {choice}")
        
        while True:
            try:
                if self.use_colors and self.color_renderer:
                    self.color_renderer.console.print("\nEnter your choice: ", 
                                                     style=f"{ColorPalette.CHOICE_NUMBER}", end="")
                else:
                    print("\nEnter your choice: ", end="")
                
                selection = int(input().strip())
                if 1 <= selection <= len(choices):
                    return selection - 1
                else:
                    error_msg = f"Please enter a number between 1 and {len(choices)}"
                    if self.use_colors and self.color_renderer:
                        self.color_renderer.console.print(error_msg, style=ColorPalette.ALERT_ORANGE)
                    else:
                        print(error_msg)
            except ValueError:
                error_msg = "Please enter a valid number"
                if self.use_colors and self.color_renderer:
                    self.color_renderer.console.print(error_msg, style=ColorPalette.ALERT_ORANGE)
                else:
                    print(error_msg)
            except KeyboardInterrupt:
                raise
    
    def display_ascii_art(self, art: Union[str, Text], color: str = None, style: str = None):
        """Display ASCII art with color"""
        if isinstance(art, Text):
            if self.use_colors and self.color_renderer:
                self.color_renderer.console.print(art)
            else:
                print(art)
        else:
            art_color = color or (self.current_mood.get('ascii_art') if self.use_colors else None)
            
            if self.use_colors and self.color_renderer:
                self.color_renderer.print_ascii_art(art, color=art_color, style=style or "bold")
            else:
                print(art)
    
    def pause(self, message: str = "\nPress ENTER to continue...", color: str = None):
        """Pause and wait for user input with color"""
        pause_color = color or (ColorPalette.NEUTRAL_GRAY if self.use_colors else None)
        
        if self.use_colors and self.color_renderer:
            self.color_renderer.pause(message, color=pause_color)
        else:
            input(message)
    
    def display_title(self, title: str, subtitle: str = None, color: str = None):
        """Display a title screen with color"""
        title_color = color or (ColorPalette.NOIR_NEON_RED if self.use_colors else None)
        
        if self.use_colors and self.color_renderer:
            self.color_renderer.console.print("\n" + "=" * 60, style=ColorPalette.NOIR_NEON_BLUE)
            self.color_renderer.console.print(f"  {title}".center(60), 
                                            style=f"bold {title_color}")
            self.color_renderer.console.print("=" * 60, style=ColorPalette.NOIR_NEON_BLUE)
            if subtitle:
                self.color_renderer.console.print(f"\n{subtitle}\n", 
                                                 style=ColorPalette.NARRATION)
        else:
            print("\n" + "=" * 60)
            print(f"  {title}".center(60))
            print("=" * 60)
            if subtitle:
                print(f"\n{subtitle}\n")
    
    def display_ending(self, scene_id: str):
        """Display ending screen with special formatting"""
        if self.use_colors and self.color_renderer:
            self.color_renderer.console.print("\n" + "=" * 60, style=ColorPalette.NOIR_NEON_BLUE)
            self.color_renderer.console.print("  THE END".center(60), 
                                            style=f"bold {ColorPalette.NOIR_NEON_RED}")
            self.color_renderer.console.print("=" * 60, style=ColorPalette.NOIR_NEON_BLUE)
            self.color_renderer.console.print("\nThank you for playing!", 
                                            style=ColorPalette.EMPHASIS)
            self.color_renderer.console.print(f"Ending: {scene_id}", 
                                            style=ColorPalette.NOIR_AMBER)
        else:
            print("\n" + "=" * 60)
            print("  THE END".center(60))
            print("=" * 60)
            print("\nThank you for playing!")
            print(f"Ending: {scene_id}")
