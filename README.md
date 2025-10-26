# 🎬 TERMINAL THEATRE

An interactive ASCII movie game where you experience cinematic stories through your terminal with real-time animations, branching narratives, and multiple endings based on your choices.

## 🎮 Features

- **Real-time ASCII Art Animations**: Frame-by-frame animations that bring scenes to life
- **Rich Color Cinematics**: 256-color gradients, mood palettes, and neon effects powered by Rich
- **Interactive Storytelling**: Make choices that meaningfully impact the narrative
- **Branching Narratives**: Multiple story paths based on your decisions
- **Multiple Endings**: Each playthrough can end differently (20+ unique endings)
- **Atmospheric Presentation**: Cinematic typewriter effects and dramatic pacing
- **Replayable Stories**: Discover different outcomes by making different choices
- **Save/Load System**: Save your progress and resume later with multiple save slots

## 📖 Current Stories

### The Last Case (Noir Detective)
*"The rain hasn't stopped for three days. Neither has the blood."*

You play as Jack Malone, a private investigator framed for murder in a city drowning in corruption. Navigate a complex web of mob bosses, corrupt cops, and military conspiracies as you fight to clear your name and uncover the truth behind Project Nightfall.

- **Duration**: 10-15 minutes per playthrough
- **Choice Points**: 7+ major decision moments
- **Endings**: 20+ unique endings
- **Genre**: Film noir detective mystery
- **Themes**: Corruption, justice, moral ambiguity

## 🚀 How to Play

### Requirements
- Python 3.7 or higher
- A terminal that supports ANSI escape codes and 256 colors (most modern terminals)
- Rich library (for color support)

### Installation & Running

```bash
# Clone or download the repository
cd terminal-theatre

# Install dependencies
pip install -r requirements.txt

# Run the game
python3 main.py
```

Or make it executable:

```bash
chmod +x main.py
./main.py
```

**Note**: The game will still work without Rich installed, but with basic monochrome output. For the full cinematic color experience, install Rich.

### Controls

- **Number Keys (1-9)**: Select choices during decision points
- **Enter**: Continue through dialogue and scenes
- **Ctrl+C**: Exit the game at any time

## 💾 Save & Load System

Terminal Theatre now includes a robust save/load system so you can take a break and return to the story without losing progress:

- **Autosave**: The game automatically saves after each major scene to the autosave slot.
- **Manual Saves**: Access the save menu during any choice by selecting the `[Save Game]` option. Choose one of the numbered slots (1-9) and optionally provide a custom name.
- **Continue**: The main menu's *Continue* option resumes from the most recent save (manual or autosave).
- **Load Game**: Open the load menu from the title screen to view detailed information (timestamp, scene, playtime) for each save slot before loading.
- **Delete Save**: Manage your storage by deleting old saves from the main menu.
- **Save Location**: Save files are stored as human-readable JSON in `~/.terminal_theatre/saves/`.

Each save file tracks your current scene, visited locations, inventory, flags, cumulative playtime, and completion percentage to ensure the story resumes exactly where you left off.

## 🎯 Gameplay Tips

1. **Read Carefully**: Dialogue and descriptions contain clues about the best choices
2. **Consider Context**: Your earlier choices may affect which options are available later
3. **Experiment**: The game is designed for multiple playthroughs - try different paths!
4. **Moral Choices**: Some endings are "good," some are "bad," and some are morally ambiguous
5. **Speed vs. Caution**: Sometimes rushing in works; sometimes patience pays off

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
│   └── animation.py       # ASCII animation system
├── stories/               # Story content
│   ├── __init__.py
│   └── noir_detective.py  # The Last Case story
└── assets/                # Additional assets (if needed)
```

## 🛠️ For Developers

- Detailed save system documentation: [`docs/SAVE_SYSTEM.md`](docs/SAVE_SYSTEM.md)

### Creating New Stories

To add a new story to Terminal Theatre:

1. **Create a new story file** in the `stories/` directory
2. **Inherit from the Story class**:

```python
from engine.story import Story, Scene, Choice
from engine.animation import AnimationLibrary

class YourStory(Story):
    def __init__(self):
        super().__init__()
        self.title = "YOUR STORY TITLE"
        self.description = "Story description..."
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

3. **Add to main.py**:

```python
from stories.your_story import YourStory

# Add to the story selection menu
```

### Scene Structure

Each scene can include:
- `id`: Unique identifier
- `description`: Narrative text
- `ascii_art`: ASCII art to display
- `animation`: Animation object to play
- `dialogue`: List of (speaker, text) tuples
- `choices`: List of Choice objects
- `is_ending`: Boolean indicating if this is an ending
- `on_enter`: Callback function when entering the scene

### Using Game State

Track player choices using flags:

```python
def some_callback(state):
    state.set_flag("found_evidence")
    state.add_item("key")

# Later, in a choice condition:
Choice(
    "Use the key",
    "locked_room",
    condition=lambda state: state.has_item("key")
)
```

## 🎨 ASCII Art Tips

- Keep ASCII art width under 60 characters for compatibility
- Use box-drawing characters (╔═╗║└┘) for clean borders
- Test your art in multiple terminal emulators
- Simple is often better than complex for readability

## 📊 Story Statistics (The Last Case)

- **Total Scenes**: 60+
- **Major Choice Points**: 7+
- **Unique Endings**: 20+
- **Story Branches**: Multiple independent paths
- **Playtime**: 10-15 minutes per playthrough
- **Replayability**: High (discover new paths each time)

## 🎭 Ending Categories

The Last Case features multiple ending types:

- **Victorious Endings**: Solve the case, clear your name
- **Tragic Endings**: Death or failure
- **Moral Endings**: Success at a cost
- **Escape Endings**: Survive by fleeing
- **Alliance Endings**: Team up with unexpected partners

## 🐛 Troubleshooting

**Screen doesn't clear properly:**
- Make sure your terminal supports ANSI escape codes
- Try running in a different terminal emulator

**Typewriter effect is too slow/fast:**
- Edit the `delay` parameter in `renderer.py`'s display functions

**Game crashes on choice selection:**
- Make sure you're entering valid numbers (1-based indexing)

## 🤝 Contributing

Want to add a new story? Found a bug? Contributions are welcome!

1. Fork the repository
2. Create a new branch for your story/fix
3. Test thoroughly
4. Submit a pull request

### Story Submission Guidelines

- Complete stories should be 10-20 minutes of gameplay
- Include at least 3 different endings
- Test all story branches for dead ends
- Include atmospheric ASCII art for key scenes
- Follow the existing code style

## 📝 License

This project is open source and available for modification and distribution.

## 🎬 Credits

**Engine & Story**: Terminal Theatre Development Team
**Genre**: Interactive Fiction / Visual Novel
**Inspiration**: Classic film noir, choose-your-own-adventure books, text adventures

## 🎨 Color System

TERMINAL THEATRE now features a rich color experience powered by Rich:

### Color Features
- **Mood Palettes**: Each scene has a color scheme matching its mood (noir, danger, calm, alert, mystery)
- **Character-Specific Colors**: Important characters have distinct color signatures
- **ASCII Art Enhancements**: All ASCII art is now rendered in atmospheric colors
- **Visual Effects**: Rain, fire, and other effects with color animations
- **Graceful Fallback**: Works without color in terminals that don't support it

### Color Moods
- **Noir** (Blue/Amber): Default detective atmosphere
- **Danger** (Red): Combat and life-threatening situations
- **Alert** (Orange/Yellow): Tense investigative moments
- **Calm** (Blue/Cyan): Safe zones and allies
- **Mystery** (Gray/Amber): Shadowy conspiracies and secrets

## 🌟 Future Plans

- [x] Save/Load game system
- [ ] Sound effects (using terminal bell)
- [ ] More animated transitions
- [ ] Additional stories (sci-fi, horror, fantasy)
- [ ] Achievements system
- [ ] Story statistics tracking

---

*"In the dark streets of Terminal Theatre, every choice matters. Choose wisely."*

Enjoy your stay at TERMINAL THEATRE! 🎭
