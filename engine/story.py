"""Story and scene management system"""

from typing import List, Dict, Callable, Optional, Any
from dataclasses import dataclass
from .animation import Animation
from .colors import get_mood_palette


@dataclass
class Choice:
    """Represents a player choice"""
    text: str
    next_scene: str
    condition: Optional[Callable] = None


@dataclass
class Scene:
    """Represents a story scene"""
    id: str
    description: str
    ascii_art: Optional[Any] = None  # Can be str or Rich Text
    animation: Optional[Animation] = None
    dialogue: List[tuple] = None  # List of (speaker, text) tuples
    choices: List[Choice] = None
    is_ending: bool = False
    on_enter: Optional[Callable] = None  # Callback when entering scene
    palette: Optional[Dict[str, str]] = None  # Color palette for the scene
    
    def __post_init__(self):
        if self.dialogue is None:
            self.dialogue = []
        if self.choices is None:
            self.choices = []
        if self.palette is None:
            self.palette = get_mood_palette('noir')


class GameState:
    """Tracks player state throughout the game"""
    
    def __init__(self):
        self.current_scene: str = ""
        self.visited_scenes: List[str] = []
        self.flags: Dict[str, bool] = {}
        self.inventory: List[str] = []
        self.variables: Dict[str, any] = {}
        self.choice_history: List[tuple] = []  # (scene_id, choice_index, choice_text)
        self.playtime: float = 0.0  # Total playtime in seconds
        self.start_time: float = 0.0  # Used for tracking session time
    
    def set_flag(self, flag: str, value: bool = True):
        """Set a story flag"""
        self.flags[flag] = value
    
    def has_flag(self, flag: str) -> bool:
        """Check if a flag is set"""
        return self.flags.get(flag, False)
    
    def add_item(self, item: str):
        """Add item to inventory"""
        if item not in self.inventory:
            self.inventory.append(item)
    
    def has_item(self, item: str) -> bool:
        """Check if player has an item"""
        return item in self.inventory
    
    def visit_scene(self, scene_id: str):
        """Mark a scene as visited"""
        if scene_id not in self.visited_scenes:
            self.visited_scenes.append(scene_id)
        self.current_scene = scene_id
    
    def to_dict(self) -> Dict[str, Any]:
        """Serialize the game state"""
        return {
            "current_scene": self.current_scene,
            "visited_scenes": list(self.visited_scenes),
            "flags": dict(self.flags),
            "inventory": list(self.inventory),
            "variables": dict(self.variables),
        }
    
    def from_dict(self, data: Dict[str, Any]):
        """Load the game state from serialized data"""
        self.current_scene = data.get("current_scene", "")
        self.visited_scenes = data.get("visited_scenes", []).copy()
        self.flags = data.get("flags", {}).copy()
        self.inventory = data.get("inventory", []).copy()
        self.variables = data.get("variables", {}).copy()
    def record_choice(self, scene_id: str, choice_index: int, choice_text: str):
        """Record a choice made by the player"""
        self.choice_history.append((scene_id, choice_index, choice_text))
    
    def update_playtime(self):
        """Update playtime from start_time"""
        import time
        if self.start_time > 0:
            self.playtime += time.time() - self.start_time
            self.start_time = time.time()
    
    def to_dict(self) -> Dict:
        """Convert game state to dictionary for saving"""
        self.update_playtime()
        return {
            'current_scene': self.current_scene,
            'visited_scenes': self.visited_scenes,
            'flags': self.flags,
            'inventory': self.inventory,
            'variables': self.variables,
            'choice_history': self.choice_history,
            'playtime': self.playtime
        }
    
    def from_dict(self, data: Dict):
        """Load game state from dictionary"""
        import time
        self.current_scene = data.get('current_scene', '')
        self.visited_scenes = data.get('visited_scenes', [])
        self.flags = data.get('flags', {})
        self.inventory = data.get('inventory', [])
        self.variables = data.get('variables', {})
        self.choice_history = data.get('choice_history', [])
        self.playtime = data.get('playtime', 0.0)
        self.start_time = time.time()


class Story:
    """Base class for stories"""
    
    def __init__(self):
        self.title: str = ""
        self.description: str = ""
        self.scenes: Dict[str, Scene] = {}
        self.starting_scene: str = ""
        self.state: GameState = GameState()
    
    def get_scene(self, scene_id: str) -> Scene:
        """Get a scene by ID"""
        return self.scenes.get(scene_id)
    
    def get_available_choices(self, scene: Scene) -> List[Choice]:
        """Get available choices for a scene based on conditions"""
        available = []
        for choice in scene.choices:
            if choice.condition is None or choice.condition(self.state):
                available.append(choice)
        return available
