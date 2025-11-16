#!/usr/bin/env python3
"""
TERMINAL THEATRE - Interactive ASCII Movie Game
Main entry point for the game with save/load support
"""

import sys
from typing import Optional

from engine.game import Game
from engine.opening import OpeningSequence, SimpleOpening, StoryOption, OpeningResult
from engine.settings import GameSettings
from stories.noir_detective import NoirDetectiveStory
from stories.blood_and_neon import BloodAndNeonStory
from stories.shadow_slave import ShadowSlaveStory


def create_story_options():
    """Define available stories for the menu"""
    return [
        StoryOption(
            key="noir_detective",
            title="The Last Case",
            summary="A Noir Detective Mystery (Demo)",
            blurb=(
                "The rain hasn't stopped for three days. Neither has the blood.\n"
                "You're Jack Malone, a private eye with a dead client and a smoking gun.\n"
                "In this city, everyone's guilty of something. Your job is to find out what.\n\n"
                "Navigate corruption, mob bosses, and conspiracies to clear your name.\n"
                "⏱️ Playtime: 30-45 minutes | 🎭 Demo Story"
            ),
            factory=lambda: NoirDetectiveStory()
        ),
        StoryOption(
            key="blood_and_neon",
            title="Blood and Neon",
            summary="An Epic Noir Detective Saga",
            blurb=(
                "The city never sleeps, but it dreams. Dark dreams.\n"
                "You're Marcus Kane, a detective investigating three ritualistic murders\n"
                "connected by tarot cards and a pharmaceutical conspiracy.\n\n"
                "Seven victims. Seven sacrifices. One chance to stop a brilliant killer\n"
                "before the pattern completes and the city drowns in blood.\n"
                "⏱️ Playtime: 60-90 minutes | 🎭 Full Story | 🔀 Deep Branching | 8+ Endings"
            ),
            factory=lambda: BloodAndNeonStory()
        ),
        StoryOption(
            key="shadow_slave",
            title="Shadow Slave",
            summary="A Dark Fantasy Adventure",
            blurb=(
                "A dark fantasy tale of nightmares, shadows, and survival.\n"
                "Enter the Spell and face the terrors that await in the Dream Realm.\n"
                "Your choices will determine your fate in this nightmare world.\n\n"
                "⏱️ Playtime: TBD | 🎭 Story Template | 🔀 Ready for Content"
            ),
            factory=lambda: ShadowSlaveStory()
        )
    ]


def main():
    """Main game loop with interactive opening"""
    settings = GameSettings()
    stories = create_story_options()
    
    try:
        opening = OpeningSequence(
            settings=settings,
            stories=stories,
            use_colors=settings.color_enabled
        )
        result = opening.run()
    except (ImportError, RuntimeError) as error:
        print("\nLaunching simplified opening sequence (Rich features unavailable):")
        print(f"  Reason: {error}")
        opening = SimpleOpening(settings=settings, stories=stories)
        result = opening.run()
    except Exception as error:
        # Unexpected error – fall back to simple menu but surface the issue
        print("\nAn unexpected issue occurred in the cinematic opening:")
        print(f"  {error}")
        print("Falling back to the simplified opening menu.\n")
        opening = SimpleOpening(settings=settings, stories=stories)
        result = opening.run()
    
    if result.action == "exit":
        try:
            from engine.colors import ColorPalette, ColorRenderer
            renderer = ColorRenderer()
            renderer.console.print(
                "\n✨ Thanks for visiting Terminal Theatre! ✨", 
                style=f"bold {ColorPalette.EMPHASIS}"
            )
            renderer.console.print(
                "See you next time!\n", 
                style=ColorPalette.NOIR_AMBER
            )
        except ImportError:
            print("\n✨ Thanks for visiting Terminal Theatre! ✨")
            print("See you next time!\n")
        sys.exit(0)
    
    elif result.action == "start" and result.story:
        story_instance = result.story.factory()
        active_settings = result.settings or settings
        game = Game(story_instance, settings=active_settings)
        game.start()
    
    else:
        print("Unknown action. Exiting.")
        sys.exit(1)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\nGame interrupted. Thanks for playing!")
        sys.exit(0)

import sys
from typing import Optional

from engine.game import Game
from engine.opening import OpeningSequence, SimpleOpening, StoryOption, OpeningResult
from engine.settings import GameSettings
from stories.noir_detective import NoirDetectiveStory


def create_story_options():
    """Define available stories for the menu"""
    return [
        StoryOption(
            key="noir_detective",
            title="The Last Case",
            summary="A Noir Detective Mystery",
            blurb=(
                "The rain hasn't stopped for three days. Neither has the blood.\n"
                "You're Jack Malone, a private eye with a dead client and a smoking gun.\n"
                "In this city, everyone's guilty of something. Your job is to find out what.\n\n"
                "Navigate corruption, mob bosses, and conspiracies to clear your name."
            ),
            factory=lambda: NoirDetectiveStory()
        )
    ]


def main():
    """Main game loop with interactive opening"""
    settings = GameSettings()
    stories = create_story_options()
    
    try:
        opening = OpeningSequence(
            settings=settings,
            stories=stories,
            use_colors=settings.color_enabled
        )
        result = opening.run()
    except (ImportError, RuntimeError) as error:
        print("\nLaunching simplified opening sequence (Rich features unavailable):")
        print(f"  Reason: {error}")
        opening = SimpleOpening(settings=settings, stories=stories)
        result = opening.run()
    except Exception as error:
        # Unexpected error – fall back to simple menu but surface the issue
        print("\nAn unexpected issue occurred in the cinematic opening:")
        print(f"  {error}")
        print("Falling back to the simplified opening menu.\n")
        opening = SimpleOpening(settings=settings, stories=stories)
        result = opening.run()
    
    if result.action == "exit":
        try:
            from engine.colors import ColorPalette, ColorRenderer
            renderer = ColorRenderer()
            renderer.console.print(
                "\n✨ Thanks for visiting Terminal Theatre! ✨", 
                style=f"bold {ColorPalette.EMPHASIS}"
            )
            renderer.console.print(
                "See you next time!\n", 
                style=ColorPalette.NOIR_AMBER
            )
        except ImportError:
            print("\n✨ Thanks for visiting Terminal Theatre! ✨")
            print("See you next time!\n")
        sys.exit(0)
    
    elif result.action == "start" and result.story:
        story_instance = result.story.factory()
        active_settings = result.settings or settings
        game = Game(story_instance, settings=active_settings)
        game.start()
    
    else:
        print("Unknown action. Exiting.")
        sys.exit(1)
from engine.save_manager import SaveManager, SaveMetadata
from stories.noir_detective import NoirDetectiveStory


def cprint(color_renderer, text: str, style: Optional[str] = None, end: str = "\n") -> None:
    """Print text with optional Rich styling if color renderer is available"""
    if color_renderer:
        color_renderer.console.print(text, style=style, end=end)
    else:
        print(text, end=end)


def prompt_input(color_renderer, prompt: str, style: Optional[str] = None) -> str:
    """Prompt the user for input with optional styling"""
    cprint(color_renderer, prompt, style=style, end="")
    return input().strip()


def pause(color_renderer, message: str = "\nPress ENTER to continue...") -> None:
    """Pause execution until the user presses ENTER"""
    prompt_input(color_renderer, message)


def format_playtime(seconds: float) -> str:
    """Format playtime in a readable format"""
    hours = int(seconds // 3600)
    minutes = int((seconds % 3600) // 60)
    
    if hours > 0:
        return f"{hours}h {minutes}m"
    return f"{minutes}m"


def get_continue_slot(save_manager: SaveManager) -> Optional[int]:
    """Determine the most recent save slot to continue from"""
    latest_manual = save_manager.get_last_save_slot()
    if latest_manual is not None:
        return latest_manual
    if save_manager.has_autosave():
        return save_manager.AUTOSAVE_SLOT
    return None


def display_main_menu(color_renderer, save_manager: SaveManager) -> tuple[str, Optional[int]]:
    """Display the main menu and return the user's choice and continue slot"""
    cprint(color_renderer, "\n" + "=" * 60, style="bold cyan")
    cprint(color_renderer, "  TERMINAL THEATRE".center(60), style="bold red")
    cprint(color_renderer, "  An Interactive ASCII Movie Experience".center(60), style="bright_white")
    cprint(color_renderer, "=" * 60 + "\n", style="bold cyan")
    
    continue_slot = get_continue_slot(save_manager)
    continue_meta: Optional[SaveMetadata] = (
        save_manager.get_save_metadata(continue_slot) if continue_slot is not None else None
    )
    
    cprint(color_renderer, "Main Menu:", style="bold yellow")
    cprint(color_renderer, "1. New Game", style="bright_cyan")
    
    if continue_meta:
        summary = f"{continue_meta.save_name} — {continue_meta.scene_description}"
        summary += f" ({continue_meta.timestamp[:19]})"
        cprint(color_renderer, f"2. Continue ({summary})", style="bright_cyan")
    else:
        cprint(color_renderer, "2. Continue (no saves found)", style="dim")
    
    cprint(color_renderer, "3. Load Game", style="bright_cyan")
    cprint(color_renderer, "4. Delete Save", style="bright_cyan")
    cprint(color_renderer, "5. Quit", style="bright_cyan")
    
    choice = prompt_input(color_renderer, "\nEnter your choice: ", style="bold yellow")
    return choice, continue_slot


def load_game_menu(color_renderer, save_manager: SaveManager) -> Optional[dict]:
    """Display the load game menu and return the loaded game state"""
    cprint(color_renderer, "\n=== LOAD GAME ===\n", style="bold yellow")
    slot_map: dict[str, int] = {}
    
    autosave_meta = save_manager.get_save_metadata(save_manager.AUTOSAVE_SLOT)
    if autosave_meta:
        slot_map["0"] = save_manager.AUTOSAVE_SLOT
        cprint(
            color_renderer,
            f"0. {autosave_meta.save_name} — {autosave_meta.scene_description}",
            style="bright_cyan"
        )
        cprint(
            color_renderer,
            f"   {autosave_meta.timestamp[:19]} | Playtime: {format_playtime(autosave_meta.playtime)}",
            style="dim"
        )
    
    for slot in range(1, save_manager.MAX_SAVE_SLOTS):
        metadata = save_manager.get_save_metadata(slot)
        if metadata:
            slot_map[str(slot)] = slot
            cprint(
                color_renderer,
                f"{slot}. {metadata.save_name} — {metadata.scene_description}",
                style="bright_cyan"
            )
            cprint(
                color_renderer,
                f"   {metadata.timestamp[:19]} | Playtime: {format_playtime(metadata.playtime)}",
                style="dim"
            )
    
    if not slot_map:
        cprint(color_renderer, "No save files found.", style="yellow")
        pause(color_renderer)
        return None
    
    cprint(color_renderer, "\n[Q] Cancel", style="red")
    selection = prompt_input(color_renderer, "Select a slot to load: ", style="bold yellow")
    selection_key = selection.strip().lower()
    
    if selection_key in {"q", "-1"}:
        return None
    
    slot = slot_map.get(selection.strip())
    if slot is None:
        cprint(color_renderer, "\nInvalid slot selection.", style="red")
        pause(color_renderer)
        return None
    
    game_state = save_manager.load_game(slot)
    if not game_state:
        cprint(color_renderer, "\nFailed to load the selected save.", style="red")
        pause(color_renderer)
        return None
    
    return game_state


def delete_save_menu(color_renderer, save_manager: SaveManager) -> None:
    """Display the delete save menu"""
    cprint(color_renderer, "\n=== DELETE SAVE ===\n", style="bold yellow")
    slot_map: dict[str, int] = {}
    
    autosave_meta = save_manager.get_save_metadata(save_manager.AUTOSAVE_SLOT)
    if autosave_meta:
        slot_map["0"] = save_manager.AUTOSAVE_SLOT
        cprint(color_renderer, f"0. {autosave_meta.save_name}", style="bright_cyan")
    
    for slot in range(1, save_manager.MAX_SAVE_SLOTS):
        metadata = save_manager.get_save_metadata(slot)
        if metadata:
            slot_map[str(slot)] = slot
            cprint(color_renderer, f"{slot}. {metadata.save_name} — {metadata.scene_description}", style="bright_cyan")
    
    if not slot_map:
        cprint(color_renderer, "No save files available to delete.", style="yellow")
        pause(color_renderer)
        return
    
    cprint(color_renderer, "\n[Q] Cancel", style="red")
    selection = prompt_input(color_renderer, "Select a slot to delete: ", style="bold yellow")
    selection_key = selection.strip().lower()
    
    if selection_key in {"q", "-1"}:
        return
    
    slot = slot_map.get(selection.strip())
    if slot is None:
        cprint(color_renderer, "\nInvalid slot selection.", style="red")
        pause(color_renderer)
        return
    
    metadata = save_manager.get_save_metadata(slot)
    if not metadata:
        cprint(color_renderer, "\nNo save exists in that slot.", style="yellow")
        pause(color_renderer)
        return
    
    confirm = prompt_input(
        color_renderer,
        f"Are you sure you want to delete '{metadata.save_name}'? (y/n): ",
        style="bold red"
    ).lower()
    if confirm != "y":
        return
    
    if save_manager.delete_save(slot):
        cprint(color_renderer, "\n✓ Save deleted.", style="green")
    else:
        cprint(color_renderer, "\n✗ Failed to delete save.", style="red")
    pause(color_renderer)


def ensure_story_compatibility(color_renderer, game_state: dict) -> bool:
    """Verify that the saved story is compatible with the available stories"""
    story_class = game_state.get("story_class")
    if story_class and story_class != "NoirDetectiveStory":
        cprint(
            color_renderer,
            f"\nThis save was created for an unavailable story ({story_class}).",
            style="red"
        )
        pause(color_renderer)
        return False
    return True


def start_story(color_renderer, game_state: Optional[dict] = None) -> None:
    """Instantiate the Noir Detective story and start the game"""
    story = NoirDetectiveStory()
    game = Game(story)
    game.start(loaded_state=game_state)


def main():
    """Main game loop"""
    try:
        from engine.colors import ColorRenderer
        color_renderer = ColorRenderer()
    except ImportError:
        color_renderer = None
    
    save_manager = SaveManager()
    
    while True:
        choice, continue_slot = display_main_menu(color_renderer, save_manager)
        normalized_choice = choice.lower()
        
        if normalized_choice == "1":
            start_story(color_renderer)
        elif normalized_choice == "2":
            if continue_slot is None:
                cprint(color_renderer, "\nNo saves available to continue.", style="yellow")
                pause(color_renderer)
                continue
            game_state = save_manager.load_game(continue_slot)
            if not game_state:
                cprint(color_renderer, "\nFailed to load the most recent save.", style="red")
                pause(color_renderer)
                continue
            if ensure_story_compatibility(color_renderer, game_state):
                start_story(color_renderer, game_state)
        elif normalized_choice == "3":
            game_state = load_game_menu(color_renderer, save_manager)
            if game_state and ensure_story_compatibility(color_renderer, game_state):
                start_story(color_renderer, game_state)
        elif normalized_choice == "4":
            delete_save_menu(color_renderer, save_manager)
        elif normalized_choice in {"5", "q"}:
            cprint(color_renderer, "\nGoodbye!", style="bold yellow")
            sys.exit(0)
        else:
            cprint(color_renderer, "\nInvalid choice. Please try again.", style="red")
            pause(color_renderer)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\nGame interrupted. Thanks for playing!")
        sys.exit(0)
