"""ASCII animation system with color support"""

import time
from typing import List, Union
from rich.text import Text


class Animation:
    """Represents an ASCII art animation with color support"""
    
    def __init__(self, frames: Union[List[str], List[Text]], frame_delay: float = 0.1, 
                 loop: int = 1, color: str = None):
        """
        Initialize animation
        
        Args:
            frames: List of ASCII art frames (strings or Rich Text objects)
            frame_delay: Delay between frames in seconds
            loop: Number of times to loop the animation
            color: Optional color for all frames
        """
        self.frames = frames
        self.frame_delay = frame_delay
        self.loop = loop
        self.color = color
    
    def play(self, renderer=None):
        """Play the animation"""
        if renderer:
            for _ in range(self.loop):
                for frame in self.frames:
                    renderer.display_frame(frame, self.frame_delay, color=self.color)
        else:
            for _ in range(self.loop):
                for frame in self.frames:
                    print(frame)
                    time.sleep(self.frame_delay)


class AnimationLibrary:
    """Library of pre-built animations with color support"""
    
    @staticmethod
    def rain_effect(colored: bool = True) -> Animation:
        """Create a rain animation effect"""
        if colored:
            from .colors import ColorPalette, VisualEffects
            frames = VisualEffects.rain_colored()
            return Animation(frames, frame_delay=0.2, loop=3)
        else:
            frames = [
                """
    |  '  |     '    |   '     |
  '    |    '   |       |   '
      |   '    |    '       |
   '     |       '  |    '
                """,
                """
  '    |    '   |       |   '
      |   '    |    '       |
   '     |       '  |    '
    |  '  |     '    |   '     |
                """,
                """
      |   '    |    '       |
   '     |       '  |    '
    |  '  |     '    |   '     |
  '    |    '   |       |   '
                """
            ]
            return Animation(frames, frame_delay=0.2, loop=3)
    
    @staticmethod
    def cityscape(colored: bool = True) -> str:
        """Return a noir cityscape"""
        art = """
╔═══════════════════════════════════════════════════════════════════╗
║                    DOWNTOWN SKYLINE - NIGHT                      ║
║                    (Seen from Malone's Office)                   ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                 ║
║              ▌ │ ▌     ▌ │     ▌ │ ▌      ▌ │ ▌             ║
║              █ │ █     █ │ █   █ │ █      █ │ █             ║
║              █ │ █     █ │ █   █ │ █      █ │ █             ║
║              █ │ █  ◈  █ │ █ ◈ █ │ █  ◈  █ │ █             ║
║              █ │ █     █ │ █   █ │ █      █ │ █             ║
║              █ │ █     █ │ █   █ │ █      █ │ █             ║
║              █ │ █  ◈  █ │ █ ◈ █ │ █  ◈  █ │ █             ║
║              █ │ █     █ │ █   █ │ █      █ │ █             ║
║              █ │ █  ◈  █ │ █ ◈ █ │ █  ◈  █ │ █             ║
║         ╔════╬═╬═════╬═╬═╬═╬════╬═╬═════════╗               ║
║         ║ ≈≈ │ │ ≈≈≈ │ │ │ │ ≈≈ │ │ streets ║ (wet)        ║
║         ║≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈║               ║
║         ║  neon signs flicker  • rain falls  ║               ║
║         ║       people hurry past              ║               ║
║         ╚════════════════════════════════════╝               ║
║                                                                 ║
║  "The city breathes tonight. And it stinks of crime."           ║
║                                                                 ║
╚═══════════════════════════════════════════════════════════════════╝
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_AMBER}")
            return text
        return art
    
    @staticmethod
    def detective_office(colored: bool = True) -> str:
        """Return detective office ASCII art"""
        art = """
╔════════════════════════════════════════════════════════════════╗
║                   MALONE INVESTIGATIONS                       ║
║                      47th Floor                               ║
╠════════════════════════════════════════════════════════════════╣
║                                                              ║
║   ╔══════════════════╗                    ╔══════════════╗   ║
║   ║    FILE          ║                    ║   VENETIAN   ║   ║
║   ║   CABINET        ║       ╭────────╮   ║    BLINDS    ║   ║
║   ║      │           ║      ╱          ╲  ║              ║   ║
║   ║  ┌───┴──┐        ║     │            │ ║   ░░░░░░░   ║   ║
║   ║  │ FILES│        ║     │   DESK    │ ║   ░░░░░░░   ║   ║
║   ║  │ CASE │        ║     │           │ ║   ░░░░░░░   ║   ║
║   ║  │  #47 │        ║     │  ┌─────┐ │ ║   ▓▓▓▓▓▓▓   ║   ║
║   ║  └─────┘        ║     │  │ GUN │ │ ║              ║   ║
║   ║                 ║     │  └─────┘ │ ║   CITY VIEW   ║   ║
║   ║                 ║     │  ┌─────┐ │ ║   (DARK)      ║   ║
║   ║                 ║     │  │ LAMP│ │ ║              ║   ║
║   ╚═════════════════╝     ╰──┬─────┬──╯ ╚══════════════╝   ║
║                             │     │                         ║
║                          ≈ ≈ ≈ ≈ ≈ ≈ ≈                      ║
║                         ( CIGARETTE SMOKE )                  ║
║                                                              ║
║   "Three days of rain. Three days of lies.                   ║
║    She's dead. I'm holding the gun. Cops are coming."        ║
║                                                              ║
╚════════════════════════════════════════════════════════════════╝
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_AMBER}")
            return text
        return art
    
    @staticmethod
    def gun(colored: bool = True) -> str:
        """Return gun ASCII art"""
        art = """
                    .38 SNUBNOSE REVOLVER
                    
          ╔═════════════════════════════╗
        ╭─║░░░░░░░░░░░░░░░░░░░░░░░░░░░║─╮
        │ ║ ◌ ◌ ◌ ◌ ◌                  ║ │ (cold steel)
        │ ║░░░░░░░░░░░░░░░░░░░░░░░░░░░║ │
        │ ╰─────────────────────────────╯ │
        │       ╎                         │
        ╰───────╊─────────────────────────╯
                ◇ (safety)
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NEUTRAL_GRAY}")
            return text
        return art
    
    @staticmethod
    def car(colored: bool = True) -> str:
        """Return car ASCII art"""
        art = """
                    1962 BLACK CADILLAC
                    
                      ╭─────────────────╮
                     ╱                   ╲
                    │  ◇   ◇   ◇   ◇   ◇ │  (chrome trim)
                    │    CADILLAC COUPE │
                    │  ◇   ◇   ◇   ◇   ◇ │
                    │                   │
                    │  [Window]  [Wind] │  (wipers)
                    │                   │
                  ╭─┴─────────────────┬─┴─╮
                  │ ◀━━━━━━━━━━━━━━━━━▶ │
                  │        ENGINE       │
            ◀━━━◀ ║  ◀━━━━━━━━━━━▶   ║ ▶━━━▶
                  ║     STEEL BEAST    ║
            ◀━━━◀ ║  ◀━━━━━━━━━━━▶   ║ ▶━━━▶
                  │  (60 mph, no lights)│
                  ╰──(o)────────────(o)─╯
                      ⚫         ⚫
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_SHADOW}")
            return text
        return art
    
    @staticmethod
    def newspaper(colored: bool = True) -> str:
        """Return newspaper ASCII art"""
        art = """
╔════════════════════════════════════════════════════════╗
║                                                        ║
║   THE DAILY CHRONICLE                                 ║
║   ═══════════════════════════════════════════════     ║
║   Est. 1934 • "All the News That Matters"            ║
║                                                        ║
║   🔴 EXCLUSIVE: BODY DISCOVERED IN THEATRE DISTRICT 🔴║
║                                                        ║
║   "VALENTINA'S DEATH STILL A MYSTERY"                 ║
║   ─────────────────────────────────────              ║
║                                                        ║
║   Authorities Clueless as Third Body Surfaces        ║
║   Connection to Castellano Family Denied             ║
║                                                        ║
║   LATE EDITION:                                       ║
║   → Private Detective Jack Malone Questioned         ║
║   → FBI Involvement Suspected                        ║
║   → "PROJECT NIGHTFALL" - What Does It Mean?        ║
║                                                        ║
║   Read Inside: City Corruption Index (pp. 3-4)       ║
║   Editorial: "Is Our City Safe?" (pp. 5)             ║
║                                                        ║
║   PRICE: 5¢                                          ║
║                                                        ║
╚════════════════════════════════════════════════════════╝
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NARRATION}")
            return text
        return art
    
    @staticmethod
    def blood_splatter(colored: bool = True) -> str:
        """Return blood splatter effect"""
        art = """
                    ·  .  ·
                 ·    *    ·
                   ·  ·  ·
                 ·        ·
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_BLOOD}")
            return text
        return art
    
    @staticmethod
    def neon_sign(text_content: str, colored: bool = True) -> str:
        """Create a neon sign effect"""
        art = f"""
        ╔═══════════════════════════════════════╗
        ║                                       ║
        ║         {text_content.center(25)}        ║
        ║                                       ║
        ╚═══════════════════════════════════════╝
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_NEON_RED}")
            return text
        return art
    
    @staticmethod
    def fire_effect(colored: bool = True) -> Animation:
        """Create a fire animation"""
        if colored:
            from .colors import VisualEffects
            frames = VisualEffects.fire_effect()
            return Animation(frames, frame_delay=0.15, loop=3)
        else:
            frames = [
                "    (  )    ( (   )  )    ",
                "   (    ) (     )   (  )  ",
                "    ( )    (  )    (   )  ",
            ]
            return Animation(frames, frame_delay=0.15, loop=3)
    
    @staticmethod
    def warehouse(colored: bool = True) -> str:
        """Return warehouse ASCII art"""
        art = """
╔═══════════════════════════════════════════════════════════════════╗
║                    ABANDONED WAREHOUSE                           ║
║                        PIER DISTRICT                             ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                 ║
║      ╔═══════════════╗              ╔═══════════════╗            ║
║      ║               ║              ║               ║            ║
║      ║   STORAGE     ║              ║   CRATES      ║            ║
║      ║    UNIT 1     ║              ║    & BOXES    ║            ║
║      ║               ║              ║               ║            ║
║      ╚═══════════════╝              ╚═══════════════╝            ║
║                                                                 ║
║  ╔══════════════════════════════════════════════════════════╗    ║
║  ║                                                          ║    ║
║  ║            [STAGE 47]                                   ║    ║
║  ║     ┌──────────────────────┐                            ║    ║
║  ║     │  LOADING DOCK DOOR   │                            ║    ║
║  ║     │                      │  ▲ SKYLIGHT                ║    ║
║  ║     │   ╔╗╔╗╔╗╔╗╔╗╔╗     │  │ (2 stories up)           ║    ║
║  ║     │   ║║║║║║║║║║║║║║    │  │                         ║    ║
║  ║     └──────────────────────┘                            ║    ║
║  ║                                                          ║    ║
║  ║  Moisture. Rust. The smell of old money and older       ║    ║
║  ║  crimes. Shadows dance under emergency lighting.         ║    ║
║  ║                                                          ║    ║
║  ╚══════════════════════════════════════════════════════════╝    ║
║                                                                 ║
╚═══════════════════════════════════════════════════════════════════╝
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_FOG}")
            return text
        return art
    
    @staticmethod
    def velvet_room(colored: bool = True) -> str:
        """Return Velvet Room ASCII art"""
        art = """
╔═══════════════════════════════════════════════════════════════════╗
║              THE VELVET ROOM - PRIVATE SANCTUM                   ║
║                    CASTELLANO RESIDENCE                          ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                 ║
║  ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆  ║
║                                                                 ║
║       ╔════════════════════════════════════════╗                ║
║       ║    Rich mahogany. Cut crystal.        ║                ║
║       ║           Soft leather chairs.        ║                ║
║       ║                                        ║                ║
║       ║    ╔──────────────────────────╗       ║                ║
║       ║    │   CASTELLANO'S TABLE     │       ║                ║
║       ║    │                          │       ║                ║
║       ║    │  ◇ Chess set (white)    │       ║                ║
║       ║    │                          │       ║                ║
║       ║    │  Smoking Cigar (Macanudo)       ║                ║
║       ║    │  Glass of 30-year Scotch        ║                ║
║       ║    │                          │       ║                ║
║       ║    └──────────────────────────┘       ║                ║
║       ║                                        ║                ║
║       ║       "Sit, Mr. Malone. We have     ║                ║
║       ║        much to discuss about        ║                ║
║       ║        Miss Valentina."              ║                ║
║       ║                                        ║                ║
║       ╚════════════════════════════════════════╝                ║
║                                                                 ║
║  ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆ ◆  ║
║                                                                 ║
╚═══════════════════════════════════════════════════════════════════╝
        """
        if colored:
            from .colors import ColorPalette
            text = Text(art)
            text.stylize(f"bold {ColorPalette.NOIR_NEON_RED}")
            return text
        return art
