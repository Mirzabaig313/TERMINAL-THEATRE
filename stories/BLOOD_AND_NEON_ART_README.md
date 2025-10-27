# Blood and Neon - ASCII Art Library

Noir-themed CLI art specifically designed for the **Blood and Neon** detective story.

## Overview

This library contains 21 unique ASCII art pieces that enhance the atmospheric noir detective experience of Blood and Neon. Each piece is carefully crafted to match the dark, rain-soaked aesthetic of the story.

## Art Pieces

### 1. **rain_city** - Noir Cityscape with Rain
Rain-soaked city with neon lights and dark buildings. Perfect for establishing the noir atmosphere.

### 2. **crime_scene** - Crime Scene Tape & Body Outline
Classic crime scene with police tape and body outline. Used for murder investigation scenes.

### 3. **tarot_death** - The Death Card (XIII)
Tarot card representing death and transformation. Key element in the story's symbolism.

### 4. **tarot_fool** - The Fool Card (0)
The protagonist's card - representing the beginning and end of the journey.

### 5. **neon_sign** - The Crimson Hour Sign
Glowing neon sign for the mysterious nightclub central to the plot.

### 6. **detective_badge** - Detective Kane's Shield
Badge #4517 - Marcus Kane's identity as a homicide detective.

### 7. **gun_and_badge** - Detective's Tools
The essential tools of the trade - gun and badge side by side.

### 8. **pier_at_night** - Pier 19 at 3:00 AM
Dark pier stretching into water - major crime scene location.

### 9. **ouroboros_symbol** - Ouroboros Pharmaceuticals Logo
The snake eating its tail - symbol of the conspiracy.

### 10. **nightshade_vial** - Nightshade Compound
Toxic vial containing the weaponized drug central to the plot.

### 11. **rain_window** - Rain on Apartment Window
Rain streaming down Kane's apartment window at 3 AM. Opens the story.

### 12. **police_lights** - Flashing Police Lights
Animated red and blue police lights (3-frame animation).

### 13. **blood_splatter** - Arterial Spray Pattern
Forensic blood splatter evidence marker.

### 14. **morgue** - City Morgue
Basement morgue with body drawers - investigation location.

### 15. **whiskey_glass** - Detective's Liquid Courage
Glass of whiskey - Kane's companion through dark nights.

### 16. **case_files** - Sealed Case Files
Confidential Westmore case files - marked as classified.

### 17. **phone_ringing** - Ringing Telephone
Animated phone ringing (3-frame animation) - the call that starts it all.

### 18. **red_door** - The Crimson Hour Entrance
Blood-red door with no sign - entrance to the mysterious club.

### 19. **syringe** - Nightshade Weapon
Syringe filled with amber liquid - the killer's weapon of choice.

### 20. **chess_pieces** - Chess Board
Full chess setup - metaphor for the game being played.

### 21. **conspiracy_board** - Detective's Investigation Wall
Web of connections with red string linking suspects and evidence.

## Usage

### Basic Import

```python
from stories.blood_and_neon_art import BloodAndNeonArt
```

### Display Single Art Piece

```python
# Get a specific art piece
badge = BloodAndNeonArt.detective_badge()

# Display it
for frame in badge.frames:
    print(frame)
```

### Get All Art at Once

```python
from stories.blood_and_neon_art import get_all_blood_and_neon_art

all_art = get_all_blood_and_neon_art()

# Access by name
crime_scene = all_art['crime_scene']
tarot_fool = all_art['tarot_fool']
```

### In Story Integration

```python
from stories.blood_and_neon_art import BloodAndNeonArt
from engine.story import Scene, Choice

scene = Scene(
    id="my_scene",
    description="Your scene description...",
    ascii_art=BloodAndNeonArt.rain_city(),
    choices=[...]
)
```

## Animation Support

Some art pieces are animated:

- **police_lights** - 3 frames, 0.3s delay
- **phone_ringing** - 3 frames, 0.5s delay

These will automatically cycle through frames when displayed by the game engine.

## Art Style

All pieces follow these design principles:

1. **Noir Aesthetic** - Dark, atmospheric, film noir inspired
2. **Box Drawing Characters** - Unicode characters for clean lines
3. **Block Characters** - Shading using ░▒▓█ characters
4. **Symbolic Elements** - Tarot cards, evidence markers, neon signs
5. **Atmospheric Text** - Captions and flavor text integrated into art

## Technical Details

- **Format**: Unicode text art wrapped in Animation objects
- **Compatibility**: Works with the Terminal Theatre engine's renderer
- **Size**: Each piece optimized for 80-column terminal display
- **Dependencies**: Uses `engine.animation.Animation` class

## Integration Status

Currently integrated into these key scenes:

- ✓ **prologue** - Uses `rain_window()`
- ✓ **pier_nineteen** - Uses `pier_at_night()`
- ✓ **crimson_hour_club** - Uses `red_door()`
- ✓ **meet_elias** - Uses `whiskey_glass()`
- ✓ **final_confrontation** - Uses `tarot_card_fool()`

## Future Additions

Potential art pieces to add:

- Additional tarot cards (Queen of Swords, King of Cups, Knight of Wands)
- Yacht explosion scene
- Prometheus Industries building
- Laboratory equipment
- More weapon variations
- Character portraits (Kane, Cassandra, Sarah)

## Credits

Created for Terminal Theatre's Blood and Neon story.
ASCII art designed to enhance the noir detective narrative experience.

---

**Note**: All art is monochrome and designed for terminal display. Color support is handled by the game engine's color palette system.
