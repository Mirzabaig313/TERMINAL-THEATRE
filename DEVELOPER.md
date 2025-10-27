# 🛠️ Developer Guide

## 🏗️ Architecture

The game is built with a modular architecture that makes it easy to add new stories:

```
terminal-theatre/
├── main.py                 # Entry point
├── engine/                 # Core game engine
│   ├── __init__.py
│   ├── game.py            # Main game controller
│   ├── story.py           # Story/scene management
│   ├── renderer.py        # Terminal rendering utilities
│   ├── animation.py       # ASCII animation system
│   ├── colors.py          # Color and mood system
│   ├── input_handler.py   # Input processing
│   ├── settings.py        # Configuration
│   └── save_manager.py    # Save/load functionality
├── stories/               # Story content
│   ├── __init__.py
│   ├── noir_detective.py  # The Last Case story
│   └── blood_and_neon.py  # Additional story
├── tests/                 # Test suite
│   ├── __init__.py
│   ├── test_game.py
│   ├── test_opening.py
│   ├── test_blood_and_neon.py
│   └── test_opening_comprehensive.py
```

## 🎮 Game Features

### Engine Capabilities
1. **Scene Management**: Graph-based navigation system
2. **State Tracking**: Persistent game state with flags and inventory
3. **Conditional Choices**: Choices can appear/disappear based on state
4. **Callbacks**: Scene entry callbacks for dynamic state changes
5. **Animation System**: Frame-based ASCII art animations
6. **Typewriter Effect**: Cinematic text display
7. **Color System**: Mood-based palettes and character-specific colors
8. **Save/Load**: Multiple save slots with detailed metadata

### Story Features
1. **Branching Narrative**: Multiple paths through the story
2. **Meaningful Choices**: Decisions have real consequences
3. **Multiple Endings**: Diverse ending types and outcomes
4. **Replayability**: Different choices reveal new content
5. **Atmospheric Presentation**: Rich dialogue and ASCII art
6. **Complex Characters**: Personality-driven dialogue

## 📖 Creating New Stories

### Step 1: Create a Story File

Create a new Python file in `stories/` directory:

```python
from engine.story import Story, Scene, Choice
from engine.animation import AnimationLibrary

class YourStory(Story):
    def __init__(self):
        super().__init__()
        self.title = "YOUR STORY TITLE"
        self.description = "A compelling story description..."
        self.starting_scene = "opening"
        self._build_story()
    
    def _build_story(self):
        # Define your scenes here
        self.scenes["opening"] = Scene(
            id="opening",
            description="Scene description...",
            dialogue=[("CHARACTER", "Dialogue text...")],
            choices=[
                Choice("Choice text", "next_scene_id"),
            ]
        )
```

### Step 2: Define Your Scenes

Each scene can include:
- `id`: Unique identifier
- `description`: Narrative text (displayed in typewriter effect)
- `ascii_art`: ASCII art to display
- `animation`: Animation object to play
- `dialogue`: List of (speaker, text) tuples
- `choices`: List of Choice objects
- `is_ending`: Boolean indicating if this is an ending
- `on_enter`: Callback function when entering the scene

### Step 3: Example Scene with All Features

```python
def my_callback(state):
    state.set_flag("visited_warehouse")
    state.add_item("evidence")

self.scenes["warehouse"] = Scene(
    id="warehouse",
    description="You approach the abandoned warehouse...",
    ascii_art="""
    ╔═══════════════╗
    ║   WAREHOUSE   ║
    ╚═══════════════╝
    """,
    animation=self.animations.get("rain_effect"),
    dialogue=[
        ("Jack", "This is it. The warehouse."),
        ("Guard", "Hold it right there!"),
    ],
    choices=[
        Choice(
            "Sneak around",
            "warehouse_sneak",
            condition=lambda state: state.has_item("lockpick")
        ),
        Choice("Talk your way in", "warehouse_talk"),
        Choice("Fight your way in", "warehouse_fight"),
    ],
    on_enter=my_callback
)
```

### Step 4: Use Game State

Track player choices and progress:

```python
# Set a flag
def on_enter_callback(state):
    state.set_flag("found_evidence")
    state.add_item("key")
    state.set_variable("suspect_count", 3)

# Check state in choices
Choice(
    "Use the key",
    "locked_room",
    condition=lambda state: state.has_item("key")
)

# Check flags
Choice(
    "Mention what you found",
    "confront_scene",
    condition=lambda state: state.has_flag("found_evidence")
)
```

### Step 5: Create Animations

```python
from engine.animation import Animation, Frame

rain_animation = Animation([
    Frame(""". . . . . """, 0.1),
    Frame("""  . . . .  """, 0.1),
    Frame(""". . . . . """, 0.1),
])

self.animations["rain"] = rain_animation
```

### Step 6: Add to Main Menu

In `main.py`, import and add your story:

```python
from stories.your_story import YourStory

# In the story selection menu
stories = [
    YourStory(),
    # ... other stories
]
```

## 🎨 ASCII Art Tips

- **Width**: Keep ASCII art under 60 characters for compatibility
- **Characters**: Use box-drawing characters (╔═╗║└┘├┤) for clean borders
- **Simplicity**: Simple is often better than complex for readability
- **Testing**: Test art in multiple terminal emulators
- **Alignment**: Use monospace fonts for proper alignment

### Example ASCII Art

```python
OFFICE = """
╔════════════════════════════╗
║  JACK'S OFFICE             ║
║  Downtown Detective Agency ║
╚════════════════════════════╝

The rain hammers the window.
Your desk is messy, your whiskey
glass empty. Just another night.
"""
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
python -m pytest tests/

# Run specific test
python -m pytest tests/test_game.py

# With coverage
python -m pytest tests/ --cov=engine --cov=stories
```

### Writing Tests

```python
import unittest
from engine.game import Game
from stories.your_story import YourStory

class TestYourStory(unittest.TestCase):
    def setUp(self):
        self.story = YourStory()
    
    def test_opening_exists(self):
        self.assertIn("opening", self.story.scenes)
    
    def test_story_starts_at_opening(self):
        self.assertEqual(self.story.starting_scene, "opening")
    
    def test_has_endings(self):
        endings = [s for s in self.story.scenes.values() if s.is_ending]
        self.assertGreater(len(endings), 0)
```

## 🎯 Design Decisions

### Why Python?
- Excellent terminal handling with standard library
- Cross-platform compatibility
- Easy to read and modify
- Great for rapid prototyping

### Modular Architecture
- Easy to add new stories without modifying core engine
- Each story is self-contained
- Engine is reusable for different genres

### State-Driven Choices
- Choices can be conditional on player state
- Enables complex branching narratives
- Players feel agency in their decisions

## 🔮 Future Enhancement Possibilities

The architecture supports easy additions:
- More stories (sci-fi, horror, fantasy)
- Achievement system
- Statistics tracking
- More complex animations
- Sound effects
- Timed choices
- Inventory puzzles
- Skill-based challenges

## 🤝 Contributing Guidelines

### Before Submitting a Story

1. **Complete**: Full story with beginning, middle, and end
2. **Diverse Endings**: At least 3 different endings
3. **Testing**: Test all story branches for dead ends
4. **Quality**: Atmospheric ASCII art for key scenes
5. **Style**: Follow existing code conventions

### Code Style

- Use descriptive variable names
- Add docstrings to functions
- Follow PEP 8 guidelines
- Include type hints where appropriate
- Comment complex logic

### Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b story/my-story`
3. Test thoroughly: `python -m pytest tests/`
4. Commit with clear messages
5. Push and create pull request
6. Include story description in PR

## 🎬 Example: Complete Mini-Story

```python
class MinistoryExample(Story):
    def __init__(self):
        super().__init__()
        self.title = "The Choice"
        self.description = "A simple story about making choices"
        self.starting_scene = "start"
        self._build_story()
    
    def _build_story(self):
        self.scenes["start"] = Scene(
            id="start",
            description="You stand at a crossroads.",
            choices=[
                Choice("Go left", "left_end"),
                Choice("Go right", "right_end"),
            ]
        )
        
        self.scenes["left_end"] = Scene(
            id="left_end",
            description="You find treasure!",
            is_ending=True,
            choices=[]
        )
        
        self.scenes["right_end"] = Scene(
            id="right_end",
            description="You find nothing but regret.",
            is_ending=True,
            choices=[]
        )
```

---

**Happy developing! Create amazing stories for Terminal Theatre.** 🎭
        super().__init__()
        self.title = "The Choice"
        self.description = "A simple story about making choices"
        self.starting_scene = "start"
        self._build_story()
    
    def _build_story(self):
        self.scenes["start"] = Scene(
            id="start",
            description="You stand at a crossroads.",
            choices=[
                Choice("Go left", "left_end"),
                Choice("Go right", "right_end"),
            ]
        )
        
        self.scenes["left_end"] = Scene(
            id="left_end",
            description="You find treasure!",
            is_ending=True,
            choices=[]
        )
        
        self.scenes["right_end"] = Scene(
            id="right_end",
            description="You find nothing but regret.",
            is_ending=True,
            choices=[]
        )
```

---

**Happy developing! Create amazing stories for Terminal Theatre.** 🎭
