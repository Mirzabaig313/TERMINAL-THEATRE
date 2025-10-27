#!/usr/bin/env python3
"""
Test script to verify Blood and Neon story loads correctly
"""

import sys
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from stories.blood_and_neon import BloodAndNeonStory


def test_story_creation():
    """Test that the story can be instantiated"""
    print("Testing story creation...")
    story = BloodAndNeonStory()
    print(f"✓ Story created: {story.title}")
    print(f"✓ Description: {story.description[:100]}...")
    print(f"✓ Starting scene: {story.starting_scene}")
    return story


def test_scenes_exist(story):
    """Test that key scenes are defined"""
    print("\nTesting scene structure...")
    required_scenes = [
        "prologue",
        "pier_nineteen",
        "call_sarah",
        "review_files",
        "crimson_hour_club",
        "final_confrontation",
    ]
    
    missing = []
    for scene_id in required_scenes:
        if scene_id in story.scenes:
            print(f"✓ Scene '{scene_id}' exists")
        else:
            print(f"✗ Scene '{scene_id}' missing")
            missing.append(scene_id)
    
    print(f"\nTotal scenes defined: {len(story.scenes)}")
    return len(missing) == 0


def test_endings(story):
    """Test that endings are properly marked"""
    print("\nTesting endings...")
    endings = [scene for scene in story.scenes.values() if scene.is_ending]
    print(f"Found {len(endings)} ending scenes:")
    for ending in endings:
        print(f"  - {ending.id}")
    return len(endings) > 0


def test_scene_connections(story):
    """Test that scenes have valid choice connections"""
    print("\nTesting scene connections...")
    broken_links = []
    
    for scene_id, scene in story.scenes.items():
        for choice in scene.choices:
            if choice.next_scene not in story.scenes:
                broken_links.append(f"{scene_id} -> {choice.next_scene}")
    
    if broken_links:
        print("✗ Found broken connections:")
        for link in broken_links[:10]:  # Show first 10
            print(f"  - {link}")
        if len(broken_links) > 10:
            print(f"  ... and {len(broken_links) - 10} more")
        return False
    else:
        print("✓ All scene connections are valid")
        return True


def test_starting_scene(story):
    """Test that starting scene exists and has choices"""
    print("\nTesting starting scene...")
    start_scene = story.scenes.get(story.starting_scene)
    
    if not start_scene:
        print(f"✗ Starting scene '{story.starting_scene}' not found")
        return False
    
    print(f"✓ Starting scene '{story.starting_scene}' exists")
    print(f"✓ Description length: {len(start_scene.description)} chars")
    print(f"✓ Number of choices: {len(start_scene.choices)}")
    print(f"✓ Dialogue lines: {len(start_scene.dialogue)}")
    
    return True


def main():
    """Run all tests"""
    print("=" * 60)
    print("BLOOD AND NEON - Story Validation Test")
    print("=" * 60)
    
    try:
        story = test_story_creation()
        
        tests = [
            ("Scene existence", lambda: test_scenes_exist(story)),
            ("Endings", lambda: test_endings(story)),
            ("Scene connections", lambda: test_scene_connections(story)),
            ("Starting scene", lambda: test_starting_scene(story)),
        ]
        
        results = []
        for test_name, test_func in tests:
            try:
                result = test_func()
                results.append((test_name, result))
            except Exception as e:
                print(f"\n✗ Test '{test_name}' failed with error: {e}")
                results.append((test_name, False))
        
        print("\n" + "=" * 60)
        print("TEST SUMMARY")
        print("=" * 60)
        
        passed = sum(1 for _, result in results if result)
        total = len(results)
        
        for test_name, result in results:
            status = "✓ PASS" if result else "✗ FAIL"
            print(f"{status}: {test_name}")
        
        print(f"\nResults: {passed}/{total} tests passed")
        
        if passed == total:
            print("\n🎉 All tests passed! Story is ready to play.")
            return 0
        else:
            print(f"\n⚠️  {total - passed} test(s) failed. Review issues above.")
            return 1
            
    except Exception as e:
        print(f"\n✗ Fatal error during testing: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
