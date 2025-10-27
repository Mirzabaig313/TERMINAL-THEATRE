#!/usr/bin/env python3
"""Quick test of opening sequence"""

import sys
import os

# Suppress fancy input
os.environ['TERM'] = 'dumb'

from engine.opening import SimpleOpening, StoryOption, OpeningResult
from engine.settings import GameSettings
from stories.noir_detective import NoirDetectiveStory


def main():
    """Test the opening system"""
    settings = GameSettings()
    stories = [
        StoryOption(
            key="noir_detective",
            title="The Last Case",
            summary="A Noir Detective Mystery",
            blurb="Test story",
            factory=lambda: NoirDetectiveStory()
        )
    ]
    
    print("Testing SimpleOpening fallback...")
    opening = SimpleOpening(settings=settings, stories=stories)
    print("SimpleOpening initialized successfully!")
    print("Menu would be displayed here. Exiting test.")


if __name__ == "__main__":
    main()
