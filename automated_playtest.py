#!/usr/bin/env python3
"""
Automated playtest - walks through a complete game path
"""

from stories.noir_detective import NoirDetectiveStory
from engine.game import Game
import sys


def simulate_playthrough():
    """Simulate a complete playthrough by checking scene connectivity"""
    print("=" * 60)
    print("AUTOMATED PLAYTHROUGH TEST")
    print("=" * 60)
    print()
    
    story = NoirDetectiveStory()
    
    print(f"Story: {story.title}")
    print(f"Starting scene: {story.starting_scene}")
    print()
    
    visited = set()
    to_visit = [story.starting_scene]
    endings_found = []
    dead_ends = []
    
    print("Exploring all story paths...")
    
    while to_visit:
        scene_id = to_visit.pop(0)
        
        if scene_id in visited:
            continue
            
        visited.add(scene_id)
        scene = story.get_scene(scene_id)
        
        if scene is None:
            dead_ends.append(scene_id)
            print(f"  ✗ Dead end: {scene_id} (scene not found)")
            continue
        
        if scene.is_ending:
            endings_found.append(scene_id)
            print(f"  ✓ Found ending: {scene_id}")
            continue
        
        choices = story.get_available_choices(scene)
        
        if len(choices) == 0 and not scene.is_ending:
            dead_ends.append(scene_id)
            print(f"  ✗ Dead end: {scene_id} (no choices, not marked as ending)")
            continue
        
        for choice in choices:
            if choice.next_scene not in visited:
                to_visit.append(choice.next_scene)
    
    print()
    print("=" * 60)
    print("RESULTS")
    print("=" * 60)
    print(f"Scenes explored: {len(visited)}")
    print(f"Endings found: {len(endings_found)}")
    print(f"Dead ends: {len(dead_ends)}")
    print()
    
    if dead_ends:
        print("Dead end scenes:")
        for dead_end in dead_ends:
            print(f"  - {dead_end}")
        print()
    
    print("All endings:")
    for i, ending in enumerate(endings_found, 1):
        print(f"  {i}. {ending}")
    
    print()
    print("=" * 60)
    
    if dead_ends:
        print("⚠ WARNING: Dead ends found!")
        return False
    else:
        print("✓ ALL PATHS LEAD TO ENDINGS")
        return True


def check_scene_quality():
    """Check that scenes have proper content"""
    print("\n" + "=" * 60)
    print("SCENE QUALITY CHECK")
    print("=" * 60)
    print()
    
    story = NoirDetectiveStory()
    issues = []
    
    for scene_id, scene in story.scenes.items():
        if not scene.description and not scene.dialogue and not scene.ascii_art:
            issues.append(f"Scene '{scene_id}' has no content")
        
        if not scene.is_ending and len(scene.choices) == 0:
            issues.append(f"Scene '{scene_id}' has no choices (not marked as ending)")
    
    if issues:
        print("⚠ Issues found:")
        for issue in issues:
            print(f"  - {issue}")
        return False
    else:
        print("✓ All scenes have proper content")
        return True


def main():
    try:
        connectivity_ok = simulate_playthrough()
        quality_ok = check_scene_quality()
        
        print("\n" + "=" * 60)
        if connectivity_ok and quality_ok:
            print("✓ PLAYTEST PASSED - GAME IS READY")
        else:
            print("✗ PLAYTEST FAILED - ISSUES FOUND")
            sys.exit(1)
        print("=" * 60)
        
    except Exception as e:
        print(f"\n✗ ERROR DURING PLAYTEST: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
