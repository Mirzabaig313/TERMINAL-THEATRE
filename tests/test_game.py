#!/usr/bin/env python3
"""
Quick test script to verify game functionality
"""

import sys
import tempfile
from engine.story import Story, Scene, Choice, GameState
from engine.renderer import TerminalRenderer
from engine.animation import AnimationLibrary
from engine.save_manager import SaveManager
from stories.noir_detective import NoirDetectiveStory


def test_story_structure():
    """Test that the story is properly structured"""
    print("Testing story structure...")
    story = NoirDetectiveStory()
    
    assert story.title == "THE LAST CASE", "Story title not set"
    assert story.starting_scene == "opening", "Starting scene not set"
    assert "opening" in story.scenes, "Opening scene not found"
    
    opening = story.get_scene("opening")
    assert opening is not None, "Cannot get opening scene"
    assert len(opening.choices) > 0, "Opening scene has no choices"
    
    print("✓ Story structure is valid")


def test_game_state():
    """Test game state management"""
    print("Testing game state...")
    state = GameState()
    
    state.set_flag("test_flag")
    assert state.has_flag("test_flag"), "Flag not set"
    
    state.add_item("key")
    assert state.has_item("key"), "Item not added"
    
    state.visit_scene("test_scene")
    assert "test_scene" in state.visited_scenes, "Scene not marked as visited"
    
    print("✓ Game state management works")


def test_scene_branching():
    """Test that scene branching works"""
    print("Testing scene branching...")
    story = NoirDetectiveStory()
    
    opening = story.get_scene("opening")
    choices = story.get_available_choices(opening)
    
    assert len(choices) > 0, "No choices available"
    
    first_choice = choices[0]
    next_scene = story.get_scene(first_choice.next_scene)
    assert next_scene is not None, f"Next scene '{first_choice.next_scene}' not found"
    
    print("✓ Scene branching works")


def test_endings():
    """Test that endings are properly defined"""
    print("Testing endings...")
    story = NoirDetectiveStory()
    
    endings = [scene for scene in story.scenes.values() if scene.is_ending]
    assert len(endings) >= 3, f"Found only {len(endings)} endings, need at least 3"
    
    print(f"✓ Found {len(endings)} unique endings")


def test_renderer():
    """Test renderer functionality"""
    print("Testing renderer...")
    renderer = TerminalRenderer()
    
    print("  Testing clear...")
    renderer.clear()
    
    print("  Testing text display...")
    test_text = "Test message"
    print(test_text)
    
    print("✓ Renderer works")


def test_animations():
    """Test animation library"""
    print("Testing animations...")
    
    cityscape = AnimationLibrary.cityscape()
    assert cityscape is not None, "Cityscape not generated"
    assert len(cityscape) > 0, "Cityscape is empty"
    
    office = AnimationLibrary.detective_office()
    assert office is not None, "Detective office not generated"
    
    print("✓ Animation library works")


def test_save_load_system():
    """Test save/load functionality"""
    print("Testing save/load system...")
    
    # Create a temporary save directory for testing
    with tempfile.TemporaryDirectory() as tmpdir:
        save_manager = SaveManager(save_dir=tmpdir)
        
        # Create a test game state
        test_state = {
            'current_scene': 'opening',
            'visited_scenes': ['opening', 'search_body'],
            'flags': {'test_flag': True},
            'inventory': ['key', 'map'],
            'variables': {'test_var': 42},
            'choice_history': [('opening', 0, 'Test choice')],
            'playtime': 120.5,
            'story_title': 'Test Story',
            'story_class': 'NoirDetectiveStory',
            'total_scenes': 10
        }
        
        # Test save
        assert save_manager.save_game(1, test_state, "Test Save"), "Failed to save game"
        
        # Test load
        loaded_state = save_manager.load_game(1)
        assert loaded_state is not None, "Failed to load game"
        assert loaded_state['current_scene'] == 'opening', "Scene not loaded correctly"
        assert 'test_flag' in loaded_state['flags'], "Flags not loaded correctly"
        
        # Test autosave
        assert save_manager.save_game(0, test_state), "Failed to create autosave"
        assert save_manager.has_autosave(), "Autosave not detected"
        
        # Test save metadata
        metadata = save_manager.get_save_metadata(1)
        assert metadata is not None, "Failed to get save metadata"
        assert metadata.save_name == "Test Save", "Save name incorrect"
        assert metadata.current_scene == 'opening', "Scene in metadata incorrect"
        
        # Test delete
        assert save_manager.delete_save(1), "Failed to delete save"
        assert save_manager.get_save_metadata(1) is None, "Save not deleted"
        
        print("✓ Save/load system works")


def count_scenes_and_choices():
    """Count total scenes and choice points"""
    print("\nStory Statistics:")
    story = NoirDetectiveStory()
    
    total_scenes = len(story.scenes)
    endings = len([s for s in story.scenes.values() if s.is_ending])
    choice_points = len([s for s in story.scenes.values() if len(s.choices) > 0])
    
    print(f"  Total scenes: {total_scenes}")
    print(f"  Choice points: {choice_points}")
    print(f"  Unique endings: {endings}")


def main():
    print("=" * 60)
    print("TERMINAL THEATRE - Test Suite")
    print("=" * 60)
    print()
    
    try:
        test_story_structure()
        test_game_state()
        test_scene_branching()
        test_endings()
        test_renderer()
        test_animations()
        test_save_load_system()
        count_scenes_and_choices()
        
        print("\n" + "=" * 60)
        print("✓ ALL TESTS PASSED")
        print("=" * 60)
        print("\nThe game is ready to play!")
        
    except AssertionError as e:
        print(f"\n✗ TEST FAILED: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"\n✗ UNEXPECTED ERROR: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
