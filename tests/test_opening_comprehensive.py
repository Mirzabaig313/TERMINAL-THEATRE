#!/usr/bin/env python3
"""Comprehensive test of the opening sequence features"""

import sys
from engine.opening import OpeningSequence, SimpleOpening, StoryOption, OpeningResult, MenuOption, CinematicFrame
from engine.settings import GameSettings
from stories.noir_detective import NoirDetectiveStory


def test_story_option():
    """Test StoryOption dataclass"""
    print("Testing StoryOption...")
    story = StoryOption(
        key="test",
        title="Test Story",
        summary="A test",
        blurb="Test blurb",
        factory=lambda: NoirDetectiveStory()
    )
    assert story.key == "test"
    assert story.title == "Test Story"
    assert story.factory() is not None
    print("✓ StoryOption works correctly")


def test_menu_option():
    """Test MenuOption dataclass"""
    print("Testing MenuOption...")
    option = MenuOption(
        label="Test",
        description="Test description",
        action="test_action"
    )
    assert option.label == "Test"
    assert option.action == "test_action"
    print("✓ MenuOption works correctly")


def test_cinematic_frame():
    """Test CinematicFrame dataclass"""
    print("Testing CinematicFrame...")
    frame = CinematicFrame(
        art="Test art",
        caption="Test caption",
        duration=1.0
    )
    assert frame.art == "Test art"
    assert frame.caption == "Test caption"
    assert frame.duration == 1.0
    print("✓ CinematicFrame works correctly")


def test_game_settings():
    """Test GameSettings"""
    print("Testing GameSettings...")
    settings = GameSettings()
    
    assert settings.color_enabled == True
    assert settings.cinematics_enabled == True
    assert settings.text_speed == "Normal"
    
    # Test toggle
    settings.toggle_colors()
    assert settings.color_enabled == False
    settings.toggle_colors()
    assert settings.color_enabled == True
    
    # Test cycle
    speed = settings.cycle_text_speed()
    assert speed in ["Relaxed", "Normal", "Fast", "Instant"]
    
    # Test delay adjustment
    delay = settings.adjust_delay(0.03)
    assert isinstance(delay, float)
    
    print("✓ GameSettings works correctly")


def test_simple_opening():
    """Test SimpleOpening"""
    print("Testing SimpleOpening...")
    settings = GameSettings()
    stories = [
        StoryOption(
            key="test",
            title="Test Story",
            summary="Test",
            blurb="Test blurb",
            factory=lambda: NoirDetectiveStory()
        )
    ]
    opening = SimpleOpening(settings=settings, stories=stories)
    assert opening.settings is not None
    assert len(opening.stories) == 1
    print("✓ SimpleOpening initializes correctly")


def test_opening_result():
    """Test OpeningResult"""
    print("Testing OpeningResult...")
    settings = GameSettings()
    result = OpeningResult(action="exit", settings=settings)
    assert result.action == "exit"
    assert result.settings is not None
    assert result.story is None
    print("✓ OpeningResult works correctly")


def test_opening_sequence_constants():
    """Test OpeningSequence class constants"""
    print("Testing OpeningSequence constants...")
    assert OpeningSequence.VERSION is not None
    assert len(OpeningSequence.TAGLINES) > 0
    assert len(OpeningSequence.ATMOSPHERIC_QUOTES) > 0
    assert len(OpeningSequence.KONAMI_SEQUENCE) == 10
    assert len(OpeningSequence.TITLE_ART) > 0
    assert len(OpeningSequence.STAGE_BACKDROP) > 0
    assert len(OpeningSequence.CINEMATIC_FRAMES) > 0
    print("✓ OpeningSequence constants are defined")


def test_opening_sequence_init():
    """Test OpeningSequence initialization"""
    print("Testing OpeningSequence initialization...")
    settings = GameSettings()
    stories = [
        StoryOption(
            key="test",
            title="Test Story",
            summary="Test",
            blurb="Test blurb",
            factory=lambda: NoirDetectiveStory()
        )
    ]
    
    try:
        opening = OpeningSequence(
            settings=settings,
            stories=stories,
            use_colors=True
        )
        assert opening.settings is not None
        assert len(opening.stories) == 1
        assert opening.console is not None
        print("✓ OpeningSequence initializes correctly")
    except Exception as e:
        print(f"✗ OpeningSequence initialization failed: {e}")


def main():
    """Run all tests"""
    print("=" * 60)
    print("TERMINAL THEATRE - Opening Sequence Tests")
    print("=" * 60 + "\n")
    
    tests = [
        test_story_option,
        test_menu_option,
        test_cinematic_frame,
        test_game_settings,
        test_simple_opening,
        test_opening_result,
        test_opening_sequence_constants,
        test_opening_sequence_init,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except Exception as e:
            print(f"✗ {test.__name__} failed: {e}")
            failed += 1
        print()
    
    print("=" * 60)
    print(f"Tests passed: {passed}/{len(tests)}")
    if failed > 0:
        print(f"Tests failed: {failed}")
        sys.exit(1)
    else:
        print("All tests passed! ✨")
        sys.exit(0)


if __name__ == "__main__":
    main()
