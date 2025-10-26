# Contributing to Terminal Theatre

Thank you for your interest in contributing to Terminal Theatre! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please treat all community members with respect and kindness.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:
- A clear title and description
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment details (OS, Python version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please:
- Check existing issues to avoid duplicates
- Clearly describe the feature and its benefits
- Provide examples or use cases
- Consider implementation challenges

### Submitting Code Changes

1. **Fork the repository** and clone it locally
2. **Create a new branch** for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the coding style guidelines below
4. **Write or update tests** for your changes
5. **Commit with clear messages**:
   ```bash
   git commit -m "Add feature: description of changes"
   ```
6. **Push to your fork** and create a Pull Request

## Coding Guidelines

### Python Style

- Follow PEP 8 conventions
- Use meaningful variable and function names
- Keep functions focused and modular
- Add docstrings to classes and functions
- Maximum line length: 100 characters

### Testing

- Write unit tests for new features
- Ensure all tests pass before submitting a PR:
  ```bash
  python -m pytest
  ```
- Maintain or improve test coverage

### Documentation

- Update README.md if adding major features
- Add docstrings to new functions and classes
- Comment complex logic
- Update QUICKSTART.md if affecting gameplay

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/Mirzabaig313/TERMINAL-THEATRE.git
   cd TERMINAL-THEATRE
   ```

2. Create a virtual environment:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   ```

3. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

4. Run tests:
   ```bash
   python -m pytest
   ```

## Project Structure

```
TERMINAL-THEATRE/
├── engine/          # Core game engine
├── stories/         # Story content files
├── main.py          # Entry point
├── requirements.txt # Dependencies
├── test_*.py        # Test files
└── docs/            # Documentation (not pushed)
```

## Game Engine Architecture

The engine is built on several core modules:

- **game.py** - Main game state and logic
- **story.py** - Story structure and branching
- **renderer.py** - Terminal output rendering
- **animation.py** - Text animation effects
- **colors.py** - Color management
- **input_handler.py** - User input processing
- **save_manager.py** - Save/load functionality

## Contributing a Story

We welcome new story contributions! Terminal Theatre is a narrative-driven game and community stories are the heart of the project.

### Story Submission Guidelines

**Story Requirements:**
- **Original Content**: Your story must be your own work or properly licensed
- **Tone**: Fit the noir/dark fiction atmosphere (existing stories set the tone)
- **Length**: 10-20 minutes of gameplay (roughly 2000-5000 words of content)
- **Branching**: Multiple meaningful choices that affect outcomes (minimum 3 distinct endings)
- **Testing**: Comprehensive playtesting and unit tests included

### How to Create a Story

#### 1. **Story Structure**

Create a new file in `stories/your_story_name.py`:

```python
from engine.story import Story, Scene, Choice

class YourStoryName(Story):
    def __init__(self):
        super().__init__("Story Title", "Author Name")
        self.setup_scenes()
    
    def setup_scenes(self):
        # Define all your scenes here
        scene_opening = Scene(
            key="opening",
            text="Your opening narrative...",
            choices=[
                Choice("Action 1", target_scene="scene_1a"),
                Choice("Action 2", target_scene="scene_1b"),
            ]
        )
        self.scenes[scene_opening.key] = scene_opening
        
        # Add more scenes following the same pattern
```

#### 2. **Scene Design**

Each scene should include:
- **key**: Unique identifier (used for navigation)
- **text**: Story text (supports Rich markup for formatting)
- **choices**: List of `Choice` objects leading to next scenes
- **is_ending**: Set to `True` for ending scenes

Example:
```python
scene_ending = Scene(
    key="good_ending",
    text="You successfully completed your mission...",
    choices=[],  # No choices for endings
    is_ending=True
)
```

#### 3. **Rich Text Formatting**

Use Rich syntax for enhanced presentation:
```python
"[bold yellow]Important text[/bold yellow]"
"[dim]Whispered words...[/dim]"
"[red]Warning or danger[/red]"
"[green]Success message[/green]"
```

#### 4. **Writing Tips**

- **Atmosphere**: Use vivid, sensory descriptions
- **Pacing**: Mix action, dialogue, and introspection
- **Choices Matter**: Ensure player choices have real consequences
- **Multiple Endings**: Reward different playstyles
- **Easter Eggs**: Hidden paths for replay value

### Adding Your Story to the Game

#### 1. **Create a Test File**

Create `test_your_story.py`:

```python
import pytest
from stories.your_story_name import YourStoryName

def test_story_loads():
    story = YourStoryName()
    assert story is not None
    assert story.title == "Story Title"

def test_all_paths_valid():
    story = YourStoryName()
    visited = set()
    
    def check_scene(scene_key):
        if scene_key in visited:
            return
        visited.add(scene_key)
        scene = story.get_scene(scene_key)
        assert scene is not None
        for choice in scene.choices:
            check_scene(choice.target_scene)
    
    check_scene(story.opening_scene.key)
    assert len(visited) > 0

def test_multiple_endings():
    story = YourStoryName()
    endings = [s for s in story.scenes.values() if s.is_ending]
    assert len(endings) >= 3, "Story should have at least 3 endings"
```

#### 2. **Register in main.py**

Update `main.py` to include your story:

```python
from stories.your_story_name import YourStoryName

AVAILABLE_STORIES = [
    NoirDetective,
    BloodAndNeon,
    YourStoryName,  # Add here
]
```

#### 3. **Document Your Story**

Create `stories/YOUR_STORY_NAME_README.md`:

```markdown
# Your Story Title

**Author**: Your Name  
**Length**: ~15 minutes  
**Theme**: Noir/Dark Fiction  

## Story Summary
Brief description of the plot and setting.

## Key Features
- Multiple meaningful endings
- Complex character relationships
- Atmospheric writing

## Branching Overview
- **Path A**: Description of first major branch
- **Path B**: Description of second major branch
```

### Testing Your Story

Before submitting:

```bash
# Run your specific story tests
python -m pytest test_your_story.py -v

# Play through all paths
python3 main.py  # Select your story and test each ending

# Check code quality
python -m pytest test_your_story.py --cov=stories
```

### Submission Checklist

- [ ] Story file created in `stories/` directory
- [ ] Story class properly inherits from `Story`
- [ ] Minimum 3 distinct endings
- [ ] All scenes tested and accessible
- [ ] Test file with 80%+ coverage
- [ ] Story registered in `main.py`
- [ ] README created explaining the story
- [ ] All tests pass: `python -m pytest`
- [ ] No hardcoded bugs or infinite loops
- [ ] Code follows PEP 8 style guidelines
- [ ] Rich formatting working correctly

### PR Template for Story Submissions

```markdown
## Story Submission: [Story Title]

**Story Details:**
- Author: [Your Name]
- Length: [Estimated playtime]
- Endings: [Number of endings]

**Description:**
[Brief description of plot and themes]

**Testing:**
- [x] All paths tested
- [x] Multiple endings available
- [x] Test coverage 80%+
- [x] No broken links/scenes

**Playtested by:**
[List any beta testers]
```

### Adding New Stories Locally

For rapid iteration during development:

```bash
# Create story
nano stories/my_story.py

# Test it
python -m pytest test_my_story.py -v

# Play it
python3 main.py
```

## Adding New Stories

To add a new story:

1. Create a new Python file in `stories/`
2. Implement the story class inheriting from `Story`
3. Define scenes with branching paths
4. Add tests in a corresponding test file
5. Update the story selector in `main.py`

## Pull Request Process

1. Update documentation as needed
2. Add entry to changelog if applicable
3. Ensure CI/CD passes
4. Request review from maintainers
5. Address feedback promptly

## License

By contributing, you agree that your contributions will be licensed under the GNU Affero General Public License v3.0.

## Questions?

Feel free to open an issue for questions or discussions about the project.

---

Happy contributing!
