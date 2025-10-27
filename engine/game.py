"""Main game engine"""

import time
from typing import Dict, Any
from .story import Story, Scene
from .renderer import TerminalRenderer
from .colors import ColorPalette
from .save_manager import SaveManager


class Game:
    """Main game controller"""
    
    def __init__(self, story: Story, settings=None):
        self.story = story
        self.settings = settings
        use_colors = True
        if settings is not None:
            use_colors = getattr(settings, "color_enabled", True)
        self.renderer = TerminalRenderer(use_colors=use_colors, settings=settings)
        self.running = False
        self.save_manager = SaveManager()
        self.autosave_interval = 1  # Autosave at each checkpoint
        self.scenes_since_autosave = 0
    
    def start(self, loaded_state: dict = None):
        """Start the game"""
        self.running = True
        self.scenes_since_autosave = 0
        
        self.renderer.clear()
        self.renderer.display_title(self.story.title)
        
        # If loading from save, restore state
        if loaded_state:
            self.load_state(loaded_state)
            current_scene_id = self.story.state.current_scene or self.story.starting_scene
            self.renderer.display_title(self.story.title, subtitle="Resuming your story...")
        else:
            # New game
            adjusted_delay = 0.02
            if self.settings is not None:
                adjusted_delay = self.settings.adjust_delay(adjusted_delay)
            self.renderer.display_text(self.story.description, delay=adjusted_delay, clear_first=False)
            self.renderer.pause()
            current_scene_id = self.story.starting_scene
            # Start tracking playtime
            self.story.state.start_time = time.time()
        
        while self.running and current_scene_id:
            scene = self.story.get_scene(current_scene_id)
            
            if not scene:
                print(f"Error: Scene '{current_scene_id}' not found!")
                break
            
            self.story.state.visit_scene(current_scene_id)
            
            current_scene_id = self.play_scene(scene)
            
            # Autosave check
            if not scene.is_ending:
                self.scenes_since_autosave += 1
                if self.scenes_since_autosave >= self.autosave_interval:
                    self.autosave()
                    self.scenes_since_autosave = 0
            
            if scene.is_ending:
                self.running = False
    
    def play_scene(self, scene: Scene) -> str:
        """Play a scene and return next scene ID"""
        self.renderer.clear()
        
        if scene.palette:
            self.renderer.set_mood(scene.palette)
        
        if scene.on_enter:
            scene.on_enter(self.story.state)
        
        if scene.animation:
            scene.animation.play(self.renderer)
            time.sleep(0.5)
        
        if scene.ascii_art:
            # Handle both Animation objects and plain strings/Text
            if hasattr(scene.ascii_art, 'frames'):
                # It's an Animation object - display all frames
                for frame in scene.ascii_art.frames:
                    self.renderer.display_ascii_art(frame)
            else:
                # It's a plain string or Text object
                self.renderer.display_ascii_art(scene.ascii_art)
            time.sleep(1)
        
        if scene.description:
            self.renderer.display_text(scene.description, delay=0.02, clear_first=False)
            time.sleep(0.5)
        
        if scene.dialogue:
            for speaker, text in scene.dialogue:
                self.renderer.display_dialogue(speaker, text, delay=0.03)
                time.sleep(0.3)
        
        if scene.is_ending:
            self.renderer.display_ending(scene.id)
            self.renderer.pause("\nPress ENTER to exit...")
            return None
        
        available_choices = self.story.get_available_choices(scene)
        
        if not available_choices:
            print("\nNo choices available. Story ends here.")
            return None
        
        selected_index = self._prompt_choice(scene, available_choices)
        
        # Record the choice
        selected_choice = available_choices[selected_index]
        self.story.state.record_choice(scene.id, selected_index, selected_choice.text)
        
        return selected_choice.next_scene
    
    def _prompt_choice(self, scene: Scene, choices):
        """Prompt the player for a choice, allowing saves mid-scene"""
        while True:
            choice_texts = [choice.text for choice in choices]
            choice_texts.append("[Save Game]")
            selected_index = self.renderer.display_choices(choice_texts)
            if selected_index == len(choices):
                self.handle_save_menu()
                continue
            return selected_index
    
    def handle_save_menu(self):
        """Handle the save game menu"""
        self.renderer.clear()
        if self.renderer.use_colors and self.renderer.color_renderer:
            self.renderer.color_renderer.console.print("\n=== SAVE GAME ===\n", 
                                                       style=f"bold {ColorPalette.EMPHASIS}")
        else:
            print("\n=== SAVE GAME ===\n")
        
        # Show available slots
        saves = self.save_manager.list_saves()
        
        if self.renderer.use_colors and self.renderer.color_renderer:
            for i in range(1, min(10, self.save_manager.MAX_SAVE_SLOTS)):
                metadata = saves[i] if i < len(saves) else None
                if metadata:
                    self.renderer.color_renderer.console.print(
                        f"{i}. {metadata.save_name} - {metadata.scene_description}",
                        style=ColorPalette.CHOICE
                    )
                    self.renderer.color_renderer.console.print(
                        f"   {metadata.timestamp[:19]} | Playtime: {self._format_playtime(metadata.playtime)}",
                        style=ColorPalette.NEUTRAL_GRAY
                    )
                else:
                    self.renderer.color_renderer.console.print(
                        f"{i}. [Empty Slot]",
                        style=ColorPalette.NEUTRAL_GRAY
                    )
            
            self.renderer.color_renderer.console.print("\n0. Cancel", 
                                                       style=ColorPalette.ALERT_ORANGE)
            self.renderer.color_renderer.console.print("\nSelect save slot: ", 
                                                       style=ColorPalette.CHOICE_NUMBER, 
                                                       end="")
        else:
            for i in range(1, min(10, self.save_manager.MAX_SAVE_SLOTS)):
                metadata = saves[i] if i < len(saves) else None
                if metadata:
                    print(f"{i}. {metadata.save_name} - {metadata.scene_description}")
                    print(f"   {metadata.timestamp[:19]} | Playtime: {self._format_playtime(metadata.playtime)}")
                else:
                    print(f"{i}. [Empty Slot]")
            
            print("\n0. Cancel")
            print("\nSelect save slot: ", end="")
        
        try:
            slot = int(input().strip())
            if slot == 0:
                return
            if 1 <= slot < self.save_manager.MAX_SAVE_SLOTS:
                # Get save name
                if self.renderer.use_colors and self.renderer.color_renderer:
                    self.renderer.color_renderer.console.print(
                        "\nEnter save name (or press ENTER for default): ",
                        style=ColorPalette.CHOICE_NUMBER,
                        end=""
                    )
                else:
                    print("\nEnter save name (or press ENTER for default): ", end="")
                
                save_name = input().strip()
                if not save_name:
                    save_name = None
                
                # Perform save
                game_state = self.get_save_state()
                if self.save_manager.save_game(slot, game_state, save_name):
                    if self.renderer.use_colors and self.renderer.color_renderer:
                        self.renderer.color_renderer.console.print(
                            f"\n✓ Game saved to slot {slot}!",
                            style=ColorPalette.SUCCESS_GREEN
                        )
                    else:
                        print(f"\nGame saved to slot {slot}!")
                else:
                    if self.renderer.use_colors and self.renderer.color_renderer:
                        self.renderer.color_renderer.console.print(
                            "\n✗ Failed to save game.",
                            style=ColorPalette.DANGER_RED
                        )
                    else:
                        print("\nFailed to save game.")
                
                self.renderer.pause()
            else:
                if self.renderer.use_colors and self.renderer.color_renderer:
                    self.renderer.color_renderer.console.print(
                        "\nInvalid slot number.",
                        style=ColorPalette.ALERT_ORANGE
                    )
                else:
                    print("\nInvalid slot number.")
                self.renderer.pause()
        except ValueError:
            if self.renderer.use_colors and self.renderer.color_renderer:
                self.renderer.color_renderer.console.print(
                    "\nInvalid input.",
                    style=ColorPalette.ALERT_ORANGE
                )
            else:
                print("\nInvalid input.")
            self.renderer.pause()
        except KeyboardInterrupt:
            pass
    
    def autosave(self):
        """Perform an autosave"""
        game_state = self.get_save_state()
        if self.save_manager.save_game(self.save_manager.AUTOSAVE_SLOT, game_state, "Autosave"):
            if self.renderer.use_colors and self.renderer.color_renderer:
                self.renderer.color_renderer.console.print(
                    "\n[Autosaved]",
                    style=ColorPalette.NEUTRAL_GRAY
                )
            else:
                print("\n[Autosaved]")
    
    def get_save_state(self) -> dict:
        """Get current game state for saving"""
        state_dict = self.story.state.to_dict()
        
        # Add story information
        state_dict['story_title'] = self.story.title
        state_dict['story_class'] = self.story.__class__.__name__
        state_dict['total_scenes'] = len(self.story.scenes)
        
        return state_dict
    
    def load_state(self, state_dict: dict):
        """Load game state from dictionary"""
        self.story.state.from_dict(state_dict)
    
    def _format_playtime(self, seconds: float) -> str:
        """Format playtime in a readable format"""
        hours = int(seconds // 3600)
        minutes = int((seconds % 3600) // 60)
        
        if hours > 0:
            return f"{hours}h {minutes}m"
        else:
            return f"{minutes}m"
