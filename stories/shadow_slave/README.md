# Shadow Slave Story - Setup Guide

This folder contains the **Shadow Slave** interactive story for TERMINAL-THEATRE.

## Quick Start

The story structure is ready! Here's where to add your content:

### 1. Add Your Story Content

Edit `shadow_slave_story.py` and add your scenes in the `_build_story()` method:

```python
def _build_story(self):
    # Add your scenes here following this pattern:

    self.scenes["scene_id"] = Scene(
        id="scene_id",
        description="Your narrative text here...",
        ascii_art=ShadowSlaveArt.your_art(),  # Optional
        dialogue=[
            ("CHARACTER", "Dialogue text"),
        ],
        choices=[
            Choice(
                text="Choice text",
                next_scene="next_scene_id"
            ),
        ]
    )
```

### 2. Add ASCII Art (Optional)

Edit `shadow_slave_art.py` to add custom ASCII art:

```python
@staticmethod
def your_art_name():
    """Description"""
    return """
    Your ASCII art here
    """
```

### 3. Test Your Story

Run the game:
```bash
python main.py
```

Select "Shadow Slave" from the menu.

## Story Structure Guide

### Scene Components

A Scene can have:
- **id**: Unique identifier (string)
- **description**: Narrative text shown to player
- **ascii_art**: Visual element (string or Animation)
- **dialogue**: List of (speaker, text) tuples
- **choices**: List of Choice objects
- **on_enter**: Callback function to modify game state (optional)

### Creating Choices

```python
Choice(
    text="What the player sees",
    next_scene="where_to_go",
    condition=lambda state: state.has_flag("flag_name")  # Optional
)
```

### Using Game State

Track player progress with:

**Flags** (boolean):
```python
on_enter=lambda state: state.set_flag("met_cassie")
condition=lambda state: state.has_flag("met_cassie")
```

**Inventory**:
```python
on_enter=lambda state: state.add_item("Midnight Shard")
condition=lambda state: state.has_item("Midnight Shard")
```

**Variables** (any data):
```python
on_enter=lambda state: state.set_variable("corruption", 50)
condition=lambda state: state.get_variable("corruption", 0) > 30
```

### Creating Endings

Endings are scenes with no choices:

```python
self.scenes["ending_victory"] = Scene(
    id="ending_victory",
    description="You survived the nightmare!",
    ascii_art=ShadowSlaveArt.victory(),
    choices=[]  # Empty = ending
)
```

## Example Story Flow

```
nightmare_begins (start)
    ├─> face_nightmare
    │   ├─> explore_left
    │   │   └─> ending_death
    │   └─> explore_right
    │       └─> ending_survival
    └─> resist_spell
        └─> face_nightmare (loops back)
```

## Tips for Writing Interactive Stories

1. **Start Simple**: Create a linear path first, then add branches
2. **Test Often**: Run the game after adding each scene
3. **Track State**: Use flags to remember player choices
4. **Multiple Endings**: Create at least 3-5 different endings
5. **Meaningful Choices**: Make sure choices actually matter
6. **Atmospheric Art**: Add ASCII art to key moments
7. **Character Colors**: Assign colors in dialogue for atmosphere

## Color Palettes

Scenes automatically get colored based on keywords:
- **danger**: fight, battle, attack, blood
- **alert**: awakening, warning, threat
- **calm**: rest, safe, recovery
- **mystery**: shadow, darkness, secret

Override by setting: `scene.palette = get_mood_palette("danger")`

## File Structure

```
shadow_slave/
├── __init__.py              # Package initialization
├── shadow_slave_story.py    # Main story file (ADD CONTENT HERE)
├── shadow_slave_art.py      # ASCII art library (ADD ART HERE)
└── README.md                # This file
```

## Examples to Study

Check out these files for complete examples:
- `stories/noir_detective.py` - Simple branching story
- `stories/blood_and_neon.py` - Complex multi-ending story
- `stories/blood_and_neon_art.py` - Extensive ASCII art library

## Story Template Checklist

- [ ] Add starting scene content
- [ ] Create at least 5-10 scenes
- [ ] Add branching choices
- [ ] Create at least 2 different endings
- [ ] Add ASCII art to key scenes
- [ ] Test full playthrough
- [ ] Update story description in main.py (optional)

## Need Help?

1. Read `engine/story.py` for the Story/Scene/Choice API
2. Look at existing stories in `stories/` folder
3. Test frequently by running `python main.py`

---

**Ready to create your story!** Start by editing `shadow_slave_story.py` and replacing the TODO sections with your content.
