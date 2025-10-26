"""Interactive opening sequence and main menu for Terminal Theatre."""

from __future__ import annotations

import random
import time
from dataclasses import dataclass
from threading import Event, Thread
from typing import Any, Callable, List, Optional, Sequence

try:  # Rich is optional; fall back to SimpleOpening when unavailable
    from rich.align import Align
    from rich.box import HEAVY, ROUNDED
    from rich.console import Console
    from rich.layout import Layout
    from rich.live import Live
    from rich.panel import Panel
    from rich.table import Table
    from rich.text import Text
    RICH_AVAILABLE = True
except ImportError:  # pragma: no cover - exercised when Rich missing
    Align = None  # type: ignore
    HEAVY = ROUNDED = None  # type: ignore
    Console = None  # type: ignore
    Layout = None  # type: ignore
    Live = None  # type: ignore
    Panel = None  # type: ignore
    Table = None  # type: ignore
    Text = None  # type: ignore
    RICH_AVAILABLE = False

try:
    from .colors import ColorPalette
except ImportError:
    # Define basic color palette constants when Rich is unavailable
    class ColorPalette:  # type: ignore
        NOIR_NEON_RED = "#ff0040"
        NOIR_NEON_BLUE = "#00d4ff"
        NOIR_AMBER = "#ffb000"
        NOIR_FOG = "#4a4a4a"
        CHOICE = "#00ff7f"
        EMPHASIS = "#ffffff"
        NARRATION = "#b0b0b0"
        NEUTRAL_GRAY = "#808080"
        ALERT_ORANGE = "#ffa500"
        CALM_CYAN = "#00ffff"

from .input_handler import InputHandler
from .settings import GameSettings


@dataclass
class MenuOption:
    label: str
    description: str
    action: str


@dataclass
class StoryOption:
    key: str
    title: str
    summary: str
    blurb: str
    factory: Callable[[], Any]


@dataclass
class OpeningResult:
    action: str
    settings: GameSettings
    story: Optional[StoryOption] = None


@dataclass
class CinematicFrame:
    art: str
    caption: str
    duration: float = 2.2
    accent: str = ColorPalette.NOIR_NEON_BLUE


class OpeningSequence:
    """Full-screen interactive title sequence and menu."""

    VERSION = "v1.0.0"

    TAGLINES = [
        "Where every choice steals the spotlight",
        "An interactive ASCII experience",
        "Stories written in neon and noir",
        "Lights up. Curtains rise. Your turn.",
        "Drama unfolds in every line",
        "Choose wisely. The city is watching.",
        "From terminal to theatre – let the show begin",
        "The stage is yours, detective"
    ]

    ATMOSPHERIC_QUOTES = [
        "\u201cIn the dark, every whisper is a confession.\u201d",
        "\u201cTonight the rain writes the prologue.\u201d",
        "\u201cEvery flicker of neon hides a secret.\u201d",
        "\u201cCurtains up. Fate takes a bow.\u201d",
        "\u201cSome stories you watch. This one, you live.\u201d",
        "\u201cA single choice can rewrite the whole act.\u201d",
        "\u201cSpotlights reveal more than they hide.\u201d",
        "\u201cLet the theatre of shadows begin.\u201d",
    ]

    KONAMI_SEQUENCE = [
        "UP", "UP", "DOWN", "DOWN", "LEFT", "RIGHT", "LEFT", "RIGHT", "b", "a"
    ]

    TITLE_ART = tuple(
        line for line in """
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║  ████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗ █████╗   ║
║  ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██╔══██╗  ║
║     ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║███████║  ║
║     ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██╔══██║  ║
║     ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║██║  ██║  ║
║     ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝  ║
║                                                              ║
║  ████████╗██╗  ██╗███████╗ █████╗ ████████╗██████╗ ███████╗ ║
║  ╚══██╔══╝██║  ██║██╔════╝██╔══██╗╚══██╔══╝██╔══██╗██╔════╝ ║
║     ██║   ███████║█████╗  ███████║   ██║   ██████╔╝█████╗   ║
║     ██║   ██╔══██║██╔══╝  ██╔══██║   ██║   ██╔══██╗██╔══╝   ║
║     ██║   ██║  ██║███████╗██║  ██║   ██║   ██║  ██║███████╗ ║
║     ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝ ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
""".strip("\n").split("\n")
    )

    STAGE_BACKDROP = tuple(
        line for line in """
╔══════════════════════════════════════════════════════════════╗
║  ☆                     ☆        ☆        ☆                  ║
║        ╔════════════════════════════════════════════╗        ║
║        ║ ███ ███ ███ ███ ███ ███ ███ ███ ███ ███ █ ║        ║
║        ║ ███ ███ ███ ███ ███ ███ ███ ███ ███ ███ █ ║        ║
║        ╚════════════════════════════════════════════╝        ║
║                                                              ║
║                ╔════════════════════════════╗                 ║
║                ║  ██████████████████████  ║                 ║
║                ║  ██████████████████████  ║    ☆            ║
║                ║  ██████████████████████  ║                 ║
║                ╚════════════════════════════╝                 ║
║                                                              ║
║  ☆            The orchestra tunes to a noir overture          ║
╚══════════════════════════════════════════════════════════════╝
""".strip("\n").split("\n")
    )

    CINEMATIC_FRAMES: Sequence[CinematicFrame] = (
        CinematicFrame(
            art="""
╔══════════════════════════════════════════════════════════════╗
║ ╔══════════════════════════════════════════════════════════╗ ║
║ ║ ██╗    ██╗██████╗  ██████╗ ██╗███████╗     ╔════════════╗ ║ ║
║ ║ ██║    ██║██╔══██╗██╔════╝ ██║██╔════╝     ║  CURTAIN  ║ ║ ║
║ ║ ██║ █╗ ██║██████╔╝██║  ███╗██║█████╗       ║   CALL    ║ ║ ║
║ ║ ██║███╗██║██╔═══╝ ██║   ██║██║██╔══╝       ╚════════════╝ ║ ║
║ ║ ╚███╔███╔╝██║     ╚██████╔╝██║███████╗                    ║ ║
║ ╚══════════════════════════════════════════════════════════╝ ║
║            Velvet curtains draw back in rich crimson          ║
╚══════════════════════════════════════════════════════════════╝
""",
            caption="The house lights fade. A hush settles over the theatre...",
            duration=2.4,
            accent=ColorPalette.NOIR_NEON_RED,
        ),
        CinematicFrame(
            art="""
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║            ⋱⋰        ☆      ⋱⋰        ☆       ⋱⋰            ║
║                                                              ║
║             ╔══════════════════════════════════╗             ║
║             ║      A CITY OF SHADOWS          ║             ║
║             ║    Rain-slick neon boulevards   ║             ║
║             ╚══════════════════════════════════╝             ║
║                                                              ║
║        Streetlights flicker. Footsteps echo in alleys.       ║
╚══════════════════════════════════════════════════════════════╝
""",
            caption="Somewhere downtown, a saxophone cries against the rain.",
            duration=2.6,
            accent=ColorPalette.NOIR_NEON_BLUE,
        ),
        CinematicFrame(
            art="""
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║    ╭────────────────────────────────────────────────────╮    ║
║    │  MALONE INVESTIGATIONS                            │    ║
║    │  Venetian blinds cut the moonlight into ribbons   │    ║
║    ╰────────────────────────────────────────────────────╯    ║
║                                                              ║
║       A silhouette leans over a desk, fedora brim low.       ║
║            Cigarette smoke sketches anxious spirals.         ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
""",
            caption="Jack Malone watches the city breathe in secrets and exhale lies.",
            duration=2.5,
            accent=ColorPalette.NOIR_AMBER,
        ),
        CinematicFrame(
            art="""
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║        ╔════════════════════════════════════════════╗        ║
║        ║  PROJECT NIGHTFALL - TOP SECRET           ║        ║
║        ║  Names. Accounts. Deadline: Midnight.     ║        ║
║        ╚════════════════════════════════════════════╝        ║
║                                                              ║
║   Pages rustle like thunder. A storm is about to break.       ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
""",
            caption="Someone is tying off loose ends. You're next on the list.",
            duration=2.2,
            accent=ColorPalette.ALERT_ORANGE,
        ),
        CinematicFrame(
            art="""
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║              ╔══════════════════════════════╗                ║
║              ║  THE RITZ THEATRE - STAGE 47 ║                ║
║              ║  Velvet seats. Empty stage.  ║                ║
║              ╚══════════════════════════════╝                ║
║                                                              ║
║      A single spotlight. Dust dances like falling snow.      ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
""",
            caption="Someone waits in the dark with answers... or a bullet.",
            duration=2.4,
            accent=ColorPalette.NOIR_NEON_RED,
        ),
        CinematicFrame(
            art="""
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║         ╔══════════════════════════════════════════╗         ║
║         ║  \u2726  TERMINAL THEATRE PRESENTS  \u2726  ║         ║
║         ║      THE LAST CASE - AN INTERACTIVE      ║         ║
║         ║          NOIR DETECTIVE STORY            ║         ║
║         ╚══════════════════════════════════════════╝         ║
║                                                              ║
║        The curtain trembles. Somewhere, a revolver clicks.   ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
""",
            caption="Press any key to skip the cinematic.",
            duration=2.6,
            accent=ColorPalette.NOIR_NEON_BLUE,
        ),
    )

    def __init__(
        self,
        settings: GameSettings,
        stories: Sequence[StoryOption],
        use_colors: bool = True,
        version: str = VERSION,
    ):
        if not RICH_AVAILABLE:
            raise RuntimeError(
                "Rich library is required for the cinematic OpeningSequence."
            )

        console_kwargs = {}
        if not use_colors:
            console_kwargs["color_system"] = None
        self.console = Console(**console_kwargs)
        self.settings = settings
        self.stories = list(stories)
        self.version = version
        self.tagline = random.choice(self.TAGLINES)
        self.quote = random.choice(self.ATMOSPHERIC_QUOTES)
        self.konami_progress = 0
        self.frame_width = min(max(self.console.size.width - 6, 72), 110)
        self.frame_height = self.console.size.height

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------
    def run(self) -> OpeningResult:
        """Run the complete opening flow and return the chosen action."""
        self.console.clear()
        self._animate_title()
        self._wait_for_any_key()

        while True:
            selection = self._show_main_menu()

            if selection == "new_game":
                if self.settings.cinematics_enabled:
                    self._play_cinematic()
                story = self._select_story()
                if story:
                    return OpeningResult(
                        action="start",
                        settings=self.settings,
                        story=story,
                    )
            elif selection == "continue":
                self._show_feature_unavailable("Continue")
            elif selection == "load":
                self._show_feature_unavailable("Load Game")
            elif selection == "credits":
                self._show_credits()
            elif selection == "settings":
                self._show_settings_menu()
            elif selection == "exit":
                return OpeningResult(action="exit", settings=self.settings)

    # ------------------------------------------------------------------
    # Title & intro
    # ------------------------------------------------------------------
    def _animate_title(self) -> None:
        """Animate the Terminal Theatre title with neon reveal."""
        max_chars = max(len(line) for line in self.TITLE_ART)
        accent_cycle = [ColorPalette.NOIR_NEON_RED, ColorPalette.NOIR_NEON_BLUE]

        with Live(console=self.console, refresh_per_second=30, screen=True) as live:
            for step in range(max_chars + 1):
                live.update(self._render_title_frame(step, accent_cycle[step % 2]))
                time.sleep(0.03)

        # Neon flicker for dramatic flair
        for _ in range(3):
            frame = self._render_title_frame(max_chars, ColorPalette.NOIR_NEON_RED)
            self.console.print(frame)
            time.sleep(0.12)
            frame = self._render_title_frame(max_chars, ColorPalette.NOIR_NEON_BLUE)
            self.console.print(frame)
            time.sleep(0.12)
        time.sleep(0.3)

    def _render_title_frame(self, progress: int, accent_color: str) -> Layout:
        """Construct a title frame for the given progress point."""
        revealed_lines: List[str] = []
        for line in self.TITLE_ART:
            line_chars = [
                ch if idx <= progress or ch.strip() == "" else " "
                for idx, ch in enumerate(line)
            ]
            revealed_lines.append("".join(line_chars))

        title_text = Text("\n".join(revealed_lines), style=f"bold {accent_color}")
        title_panel = Panel(
            Align.center(title_text, vertical="middle"),
            border_style=ColorPalette.NOIR_NEON_BLUE,
            padding=(1, 4),
            title="TERMINAL THEATRE",  # Rich adds flare
            title_align="center",
        )

        backdrop_text = Text("\n".join(self.STAGE_BACKDROP), style=f"dim {ColorPalette.NOIR_FOG}")
        backdrop_panel = Panel(
            Align.center(backdrop_text, vertical="middle"),
            border_style=ColorPalette.NOIR_NEON_BLUE,
            padding=(0, 2),
        )

        layout = Layout()
        layout.split(
            Layout(name="top", size=3),
            Layout(name="backdrop", ratio=3),
            Layout(name="title", ratio=5),
            Layout(name="bottom", size=5),
        )
        layout["top"].update(
            Align.center(
                Text(
                    f"{self.version}   |   {self.quote}",
                    style=f"dim {ColorPalette.NARRATION}",
                )
            )
        )
        layout["backdrop"].update(Align.center(backdrop_panel))
        layout["title"].update(Align.center(title_panel))
        layout["bottom"].update(
            Align.center(
                Text(
                    self.tagline,
                    style=f"italic {ColorPalette.NOIR_AMBER}",
                )
            )
        )
        return layout

    def _wait_for_any_key(self) -> None:
        """Show a pulsing prompt until the player presses any key."""
        message_variants = [
            (f"bold {ColorPalette.CHOICE}", "Press any key to start"),
            (f"bold {ColorPalette.NOIR_NEON_RED}", "Press any key to start"),
            (f"dim {ColorPalette.NEUTRAL_GRAY}", "Press any key to start"),
        ]

        stop_event = Event()

        def listener() -> None:
            try:
                with InputHandler() as handler:
                    handler.read_key()
                    stop_event.set()
            except (KeyboardInterrupt, EOFError, Exception):
                stop_event.set()

        Thread(target=listener, daemon=True).start()

        variant_index = 0
        with Live(console=self.console, refresh_per_second=10) as live:
            while not stop_event.is_set():
                style, text = message_variants[variant_index % len(message_variants)]
                prompt = Text(f"\n{text}...", style=style)
                live.update(Align.center(prompt))
                time.sleep(0.5)
                variant_index += 1

    # ------------------------------------------------------------------
    # Main menu
    # ------------------------------------------------------------------
    def _show_main_menu(self) -> str:
        options = [
            MenuOption("New Game", "Begin a fresh performance", "new_game"),
            MenuOption("Continue", "Resume your last investigation", "continue"),
            MenuOption("Load Game", "Choose a saved spotlight", "load"),
            MenuOption("Credits", "Meet the cast & crew", "credits"),
            MenuOption("Settings", "Tailor your experience", "settings"),
            MenuOption("Exit", "Leave the theatre", "exit"),
        ]

        selected = 0
        konami_progress = 0

        with InputHandler() as handler:
            with Live(console=self.console, refresh_per_second=30) as live:
                while True:
                    live.update(self._render_menu(options, selected))
                    key = handler.read_key()

                    if key in ("UP", "k", "K"):
                        selected = (selected - 1) % len(options)
                    elif key in ("DOWN", "j", "J"):
                        selected = (selected + 1) % len(options)
                    elif key in ("ENTER", "\r", "\n"):
                        return options[selected].action
                    elif key.lower() == "q" or key == "ESC":
                        return "exit"
                    elif key.isdigit():
                        idx = int(key) - 1
                        if 0 <= idx < len(options):
                            selected = idx
                            return options[selected].action

                    konami_progress = self._update_konami_progress(konami_progress, key)
                    if konami_progress == len(self.KONAMI_SEQUENCE):
                        konami_progress = 0
                        self._show_easter_egg()

    def _render_menu(self, options: Sequence[MenuOption], selected: int) -> Layout:
        table = Table.grid(padding=(0, 2))
        table.expand = False
        for index, option in enumerate(options, start=1):
            is_selected = index - 1 == selected
            pointer = "▶" if is_selected else "  "
            label_style = f"bold {ColorPalette.CHOICE}" if is_selected else ColorPalette.NARRATION
            desc_style = (f"italic {ColorPalette.NOIR_FOG}" if is_selected
                          else f"dim {ColorPalette.NEUTRAL_GRAY}")

            table.add_row(
                Text(pointer, style=ColorPalette.NOIR_NEON_RED),
                Text(f"{index}. {option.label}", style=label_style),
                Text(option.description, style=desc_style),
            )

        panel = Panel(
            Align.center(table),
            title="MAIN MENU",
            border_style=ColorPalette.NOIR_NEON_BLUE,
            padding=(1, 4),
            subtitle="Use ↑/↓ or 1-6 • Enter to confirm • Q to exit",
            subtitle_align="left",
            box=ROUNDED,
        )

        footer = Text(
            "Tip: Discover hidden surprises with classic codes...",
            style=f"dim {ColorPalette.NARRATION}",
        )

        layout = Layout()
        layout.split(
            Layout(name="spacer", size=2),
            Layout(name="panel", ratio=5),
            Layout(name="footer", size=3),
        )
        layout["panel"].update(Align.center(panel))
        layout["footer"].update(Align.center(footer))
        return layout

    def _update_konami_progress(self, progress: int, key: str) -> int:
        expected = self.KONAMI_SEQUENCE[progress]
        normalized = key.lower() if len(expected) == 1 else key
        if normalized == expected or key == expected:
            return progress + 1
        return 1 if key in ("UP", "DOWN", "LEFT", "RIGHT") and expected == "UP" else 0

    def _show_easter_egg(self) -> None:
        panel = Panel(
            Align.center(
                Text(
                    "Secret Unlocked!\n" +
                    "You found the Phantom Stage Entrance.\n" +
                    "In future updates, hidden stories await...",
                    justify="center",
                    style=f"bold {ColorPalette.NOIR_NEON_BLUE}",
                )
            ),
            border_style=ColorPalette.NOIR_NEON_RED,
            padding=(2, 4),
            box=HEAVY,
        )
        self.console.print(panel)
        time.sleep(2.2)

    # ------------------------------------------------------------------
    # Cinematic
    # ------------------------------------------------------------------
    def _play_cinematic(self) -> None:
        skip_event = Event()

        def listener() -> None:
            try:
                with InputHandler() as handler:
                    handler.read_key()
                    skip_event.set()
            except (KeyboardInterrupt, EOFError, Exception):
                skip_event.set()

        Thread(target=listener, daemon=True).start()

        with Live(console=self.console, refresh_per_second=24) as live:
            for frame in self.CINEMATIC_FRAMES:
                start_time = time.time()
                while time.time() - start_time < frame.duration:
                    live.update(self._render_cinematic_frame(frame))
                    if skip_event.is_set():
                        break
                    time.sleep(0.2)
                if skip_event.is_set():
                    break

        if skip_event.is_set():
            self.console.print(
                Align.center(
                    Text(
                        "Cinematic skipped. The show must go on!",
                        style=f"dim {ColorPalette.NOIR_NEON_BLUE}",
                    )
                )
            )
            time.sleep(1.2)

    def _render_cinematic_frame(self, frame: CinematicFrame) -> Layout:
        art_panel = Panel(
            Text(frame.art, style=f"bold {frame.accent}"),
            border_style=frame.accent,
            padding=(1, 4),
            box=ROUNDED,
        )
        caption = Text(
            frame.caption,
            style=f"italic {ColorPalette.NARRATION}",
            justify="center",
        )
        prompt = Text(
            "Press any key to skip",
            style=f"dim {ColorPalette.NEUTRAL_GRAY}",
        )

        layout = Layout()
        layout.split(
            Layout(name="spacer", size=2),
            Layout(name="art", ratio=5),
            Layout(name="caption", size=3),
            Layout(name="prompt", size=2),
        )
        layout["art"].update(Align.center(art_panel))
        layout["caption"].update(Align.center(caption))
        layout["prompt"].update(Align.center(prompt))
        return layout

    # ------------------------------------------------------------------
    # Story selection
    # ------------------------------------------------------------------
    def _select_story(self) -> Optional[StoryOption]:
        if not self.stories:
            return None
        if len(self.stories) == 1:
            return self.stories[0]

        selected = 0
        with InputHandler() as handler:
            with Live(console=self.console, refresh_per_second=30) as live:
                while True:
                    live.update(self._render_story_menu(selected))
                    key = handler.read_key()

                    if key in ("UP", "k", "K"):
                        selected = (selected - 1) % len(self.stories)
                    elif key in ("DOWN", "j", "J"):
                        selected = (selected + 1) % len(self.stories)
                    elif key in ("ENTER", "\r", "\n"):
                        return self.stories[selected]
                    elif key.lower() in ("q", "esc"):
                        return None
                    elif key.isdigit():
                        idx = int(key) - 1
                        if 0 <= idx < len(self.stories):
                            selected = idx
                            return self.stories[selected]

    def _render_story_menu(self, selected: int) -> Layout:
        options_table = Table.grid(padding=(0, 2))
        options_table.expand = False
        for index, story in enumerate(self.stories, start=1):
            is_selected = selected == index - 1
            style = f"bold {ColorPalette.NOIR_NEON_RED}" if is_selected else ColorPalette.NARRATION
            pointer = "▶" if is_selected else "  "
            options_table.add_row(
                Text(pointer, style=ColorPalette.NOIR_NEON_RED),
                Text(f"{index}. {story.title}", style=style),
            )

        selected_story = self.stories[selected]
        detail_panel = Panel(
            Text(
                f"{selected_story.summary}\n\n{selected_story.blurb}",
                justify="left",
                style=ColorPalette.NARRATION,
            ),
            title=selected_story.title,
            border_style=ColorPalette.NOIR_NEON_BLUE,
            padding=(1, 3),
            subtitle="Enter to begin • Q to cancel",
            subtitle_align="left",
        )

        layout = Layout()
        layout.split_row(
            Layout(name="list", ratio=1),
            Layout(name="detail", ratio=2),
        )
        layout["list"].update(Align.center(Panel(options_table, border_style=ColorPalette.NOIR_NEON_BLUE)))
        layout["detail"].update(detail_panel)
        return layout

    # ------------------------------------------------------------------
    # Supporting screens
    # ------------------------------------------------------------------
    def _show_feature_unavailable(self, feature: str) -> None:
        panel = Panel(
            Text(
                f"The {feature} feature is coming soon!\n"
                "Story autosave, branching recaps, and timeline rewinds\n"
                "are currently in development. Replay the mystery to\n"
                "discover new endings in the meantime!",
                justify="center",
                style=ColorPalette.NARRATION,
            ),
            border_style=ColorPalette.ALERT_ORANGE,
            padding=(2, 4),
            title=f"{feature.upper()} - UNDER CONSTRUCTION",
            subtitle="Press ENTER to return",
        )
        self.console.print(panel)
        input()

    def _show_credits(self) -> None:
        credits_text = Text(justify="center")
        credits_text.append("TERMINAL THEATRE\n", style=f"bold {ColorPalette.NOIR_NEON_BLUE}")
        credits_text.append("An Interactive ASCII Experience\n\n", style=ColorPalette.NOIR_AMBER)
        credits_text.append("Directed by: Terminal Theatre Development Team\n")
        credits_text.append("Lead Story Architect: Noir Narrative Collective\n")
        credits_text.append("Engine & Renderer: Python + Rich\n")
        credits_text.append("ASCII Art Direction: Retro Terminal Artists\n\n")
        credits_text.append("Special Thanks:\n", style=f"bold {ColorPalette.CHOICE}")
        credits_text.append("— Film noir classics for eternal inspiration\n")
        credits_text.append("— The players who keep the spotlight alive\n")
        credits_text.append("— You, for stepping onto this stage\n")

        panel = Panel(
            credits_text,
            border_style=ColorPalette.NOIR_NEON_BLUE,
            padding=(2, 6),
            title="CREDITS",
            subtitle="Press ENTER to return",
        )
        self.console.print(panel)
        input()

    def _show_settings_menu(self) -> None:
        settings_options = [
            (
                "Color Mode",
                lambda: "Vivid" if self.settings.color_enabled else "Monochrome",
                self.settings.toggle_colors,
            ),
            (
                "Cinematic Intro",
                lambda: "Enabled" if self.settings.cinematics_enabled else "Disabled",
                self.settings.toggle_cinematics,
            ),
            (
                "Text Speed",
                lambda: self.settings.text_speed,
                self.settings.cycle_text_speed,
            ),
            (
                "Typewriter Effect",
                lambda: "Enabled" if self.settings.typewriter_enabled else "Disabled",
                self.settings.toggle_typewriter,
            ),
            (
                "Sound Effects",
                lambda: "Coming Soon" if not self.settings.sound_enabled else "Coming Soon",
                self.settings.toggle_sound,
            ),
            ("Back", lambda: "Return", lambda: None),
        ]

        selected = 0
        with InputHandler() as handler:
            with Live(console=self.console, refresh_per_second=30) as live:
                while True:
                    live.update(self._render_settings_menu(settings_options, selected))
                    key = handler.read_key()

                    if key in ("UP", "k", "K"):
                        selected = (selected - 1) % len(settings_options)
                    elif key in ("DOWN", "j", "J"):
                        selected = (selected + 1) % len(settings_options)
                    elif key in ("ENTER", "\r", "\n"):
                        label, _, action = settings_options[selected]
                        if label == "Back":
                            return
                        result = action()
                        if label == "Sound Effects":
                            self._sound_placeholder_message()
                    elif key.lower() in ("q", "esc"):
                        return

    def _render_settings_menu(self, options, selected: int) -> Layout:
        table = Table.grid(padding=(0, 3))
        table.expand = False
        for index, (label, value_getter, _) in enumerate(options, start=1):
            is_selected = index - 1 == selected
            pointer = "▶" if is_selected else "  "
            style = f"bold {ColorPalette.CHOICE}" if is_selected else ColorPalette.NARRATION
            value = value_getter()
            value_style = (
                f"bold {ColorPalette.NOIR_NEON_BLUE}" if is_selected else f"dim {ColorPalette.NARRATION}"
            )
            table.add_row(
                Text(pointer, style=ColorPalette.NOIR_NEON_RED),
                Text(f"{label}", style=style),
                Text(str(value), style=value_style),
            )

        description = Text(
            "Adjust presentation preferences. Coming updates will add audio, \n"
            "dynamic lighting, and accessibility presets.",
            style=f"dim {ColorPalette.NARRATION}",
            justify="center",
        )

        panel = Panel(
            Align.center(table),
            title="SETTINGS",
            border_style=ColorPalette.NOIR_NEON_BLUE,
            padding=(1, 4),
            subtitle="Enter to toggle • Q to exit",
            subtitle_align="left",
        )

        layout = Layout()
        layout.split(
            Layout(name="panel", ratio=5),
            Layout(name="description", size=4),
        )
        layout["panel"].update(Align.center(panel))
        layout["description"].update(Align.center(description))
        return layout

    def _sound_placeholder_message(self) -> None:
        panel = Panel(
            Text(
                "Soundscapes are in composition! Soon you'll hear thunder, jazz,\n"
                "and the crackle of neon. For now, imagine the soundtrack...",
                justify="center",
                style=ColorPalette.NARRATION,
            ),
            border_style=ColorPalette.CALM_CYAN,
            padding=(2, 4),
            subtitle="Press ENTER to continue",
        )
        self.console.print(panel)
        input()


class SimpleOpening:
    """Minimal fallback when advanced rendering isn't available."""

    def __init__(self, settings: Optional[GameSettings] = None, stories: Optional[Sequence[StoryOption]] = None):
        self.settings = settings or GameSettings()
        self.stories = list(stories) if stories else []

    def run(self) -> OpeningResult:
        print("\n" + "=" * 60)
        print("TERMINAL THEATRE".center(60))
        print("An Interactive ASCII Experience".center(60))
        print("=" * 60)
        input("\nPress ENTER to continue...")

        while True:
            print("\n" + "=" * 60)
            print("MAIN MENU".center(60))
            print("=" * 60)
            print("1. New Game")
            print("2. Continue (Coming Soon)")
            print("3. Load Game (Coming Soon)")
            print("4. Credits")
            print("5. Settings")
            print("6. Exit")

            choice = input("\nSelect an option: ").strip()
            if choice == "1":
                story = self.stories[0] if self.stories else None
                return OpeningResult("start", self.settings, story)
            if choice == "2" or choice == "3":
                print("\nFeature coming soon!")
                input("Press ENTER to return...")
            elif choice == "4":
                print("\nTERMINAL THEATRE - Credits")
                print("Developed by the Terminal Theatre Team")
                input("\nPress ENTER to return...")
            elif choice == "5":
                print("\nSettings will return in the full experience.")
                input("Press ENTER to return...")
            elif choice == "6":
                return OpeningResult("exit", self.settings, None)
            else:
                print("Invalid choice. Try again.")
