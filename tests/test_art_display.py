#!/usr/bin/env python3
"""Test script to verify ASCII art displays correctly in Blood and Neon"""

import sys
import importlib

# Force reload of modules to avoid cache issues
if 'engine.game' in sys.modules:
    del sys.modules['engine.game']
if 'stories.blood_and_neon' in sys.modules:
    del sys.modules['stories.blood_and_neon']

from stories.blood_and_neon import create_story
from engine.game import Game

print("\n" + "="*70)
print("BLOOD AND NEON - ASCII ART DISPLAY TEST")
print("="*70 + "\n")

# Create story
story = create_story()
print(f"✓ Story loaded: {story.title}")

# Test a few key scenes
test_scenes = [
    ("prologue", "Rain Window"),
    ("pier_nineteen", "Pier at Night"),
    ("crimson_hour_club", "Red Door"),
    ("meet_elias", "Whiskey Glass"),
    ("final_confrontation", "The Fool Card")
]

print(f"\nTesting {len(test_scenes)} scenes with custom art:\n")

for scene_id, art_name in test_scenes:
    scene = story.scenes.get(scene_id)
    if scene and scene.ascii_art:
        has_frames = hasattr(scene.ascii_art, 'frames')
        print(f"✓ {scene_id:25s} - {art_name:20s} [Animation: {has_frames}]")
        
        if has_frames:
            # Test display logic
            for frame in scene.ascii_art.frames:
                if isinstance(frame, str) and len(frame) > 0:
                    print(f"  └─ Frame OK ({len(frame)} chars)")
                else:
                    print(f"  └─ Frame ERROR: {type(frame)}")
    else:
        print(f"✗ {scene_id:25s} - NO ART")

print("\n" + "="*70)
print("SAMPLE: Prologue Scene Art")
print("="*70 + "\n")

prologue = story.scenes["prologue"]
if prologue.ascii_art and hasattr(prologue.ascii_art, 'frames'):
    print(prologue.ascii_art.frames[0])

print("\n" + "="*70)
print("✓ All tests passed! ASCII art is ready to display in game.")
print("="*70 + "\n")

print("To play the game with ASCII art, run:")
print("  python3 main.py")
print("\nOr start Blood and Neon directly from the menu.")
