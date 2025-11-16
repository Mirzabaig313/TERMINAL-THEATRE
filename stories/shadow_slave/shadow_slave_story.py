"""Shadow Slave - A Dark Fantasy Story
Based on the Shadow Slave web novel universe
"""

from engine.story import Story, Scene, Choice
from engine.animation import Animation
from engine.colors import get_mood_palette
from .shadow_slave_art import ShadowSlaveArt


class ShadowSlaveStory(Story):
    """Shadow Slave interactive story"""

    def __init__(self):
        super().__init__()
        self.title = "SHADOW SLAVE"
        self.description = (
            "A dark fantasy tale of nightmares, shadows, and survival.\n"
            "Enter the Spell and face the terrors that await in the Dream Realm.\n"
            "Your choices will determine your fate in this nightmare world."
        )
        self.starting_scene = "nightmare_begins"
        self._build_story()
        self._apply_scene_palettes()

    def _apply_scene_palettes(self):
        """Assign color palettes to scenes based on mood"""
        # Mood assignments for different scene types
        mood_keywords = {
            "danger": ["battle", "fight", "nightmare", "attack", "death", "blood"],
            "alert": ["awakening", "spell", "warning", "danger", "threat"],
            "calm": ["rest", "safe", "recovery", "memory", "dream"],
            "mystery": ["shadow", "darkness", "unknown", "secret", "revelation"],
        }

        for scene_id, scene in self.scenes.items():
            mood = "noir"  # default
            lower_id = scene_id.lower()

            for mood_name, keywords in mood_keywords.items():
                if any(keyword in lower_id for keyword in keywords):
                    mood = mood_name
                    break

            scene.palette = get_mood_palette(mood)

    def _build_story(self):
        """Build the complete story graph

        TODO: Add your story content here!
        Follow this structure for each scene:

        self.scenes["scene_id"] = Scene(
            id="scene_id",
            description="Narrative text here",
            ascii_art=ShadowSlaveArt.your_art_function(),  # optional
            dialogue=[
                ("CHARACTER_NAME", "Dialogue text"),
            ],
            choices=[
                Choice(
                    text="Choice text",
                    next_scene="next_scene_id",
                    condition=lambda state: state.has_flag("flag_name")  # optional
                ),
            ],
            on_enter=lambda state: state.set_flag("scene_visited")  # optional
        )
        """

        # ============================================================================
        # EXAMPLE STARTING SCENE - Replace with your content
        # ============================================================================

        self.scenes["nightmare_begins"] = Scene(
            id="nightmare_begins",
            description=(
                "The world dissolves into darkness.\n\n"
                "You feel the Spell taking hold, pulling you into the Dream Realm.\n"
                "Ancient words echo in your mind, weaving themselves into your very soul.\n\n"
                "This is the Nightmare Spell. This is your First Nightmare.\n"
                "And you are about to discover what it truly means to survive."
            ),
            ascii_art=ShadowSlaveArt.nightmare_gate(),
            dialogue=[
                ("THE SPELL", "Aspirant! Welcome to the Nightmare Spell."),
                ("THE SPELL", "Prepare for your First Nightmare..."),
                ("YOUR THOUGHTS", "What... what's happening to me?"),
            ],
            choices=[
                Choice(
                    text="Face the nightmare",
                    next_scene="first_challenge"
                ),
                Choice(
                    text="Try to resist the Spell",
                    next_scene="resistance"
                ),
            ],
            on_enter=lambda state: state.set_flag("nightmare_started")
        )

        # ============================================================================
        # ADD MORE SCENES BELOW - This is where your content goes!
        # ============================================================================

        # Example scene 2
        self.scenes["first_challenge"] = Scene(
            id="first_challenge",
            description=(
                "The darkness parts, revealing a twisted landscape of shadow and stone.\n\n"
                "TODO: Add your scene description here"
            ),
            dialogue=[
                ("YOUR THOUGHTS", "TODO: Add dialogue here"),
            ],
            choices=[
                Choice(
                    text="TODO: Add choice 1",
                    next_scene="ending_survival"
                ),
                Choice(
                    text="TODO: Add choice 2",
                    next_scene="ending_death"
                ),
            ]
        )

        # Example scene 3
        self.scenes["resistance"] = Scene(
            id="resistance",
            description=(
                "You try to fight against the Spell's pull, but it's futile.\n\n"
                "TODO: Add your scene description here"
            ),
            dialogue=[
                ("THE SPELL", "Resistance is futile, Aspirant."),
            ],
            choices=[
                Choice(
                    text="Accept your fate",
                    next_scene="first_challenge"
                ),
            ]
        )

        # ============================================================================
        # ENDINGS - Add your story endings here
        # ============================================================================

        self.scenes["ending_survival"] = Scene(
            id="ending_survival",
            description=(
                "Against all odds, you survived the nightmare.\n\n"
                "TODO: Add your ending description here"
            ),
            ascii_art=ShadowSlaveArt.victory(),
            dialogue=[
                ("THE SPELL", "Congratulations, Aspirant. You have survived your First Nightmare."),
            ],
            choices=[]  # No choices = ending
        )

        self.scenes["ending_death"] = Scene(
            id="ending_death",
            description=(
                "The shadows consume you. The nightmare claims another victim.\n\n"
                "TODO: Add your ending description here"
            ),
            ascii_art=ShadowSlaveArt.death(),
            dialogue=[
                ("THE SPELL", "You have perished in the Dream Realm."),
            ],
            choices=[]  # No choices = ending
        )


# ============================================================================
# INSTRUCTIONS FOR ADDING CONTENT:
# ============================================================================
#
# 1. Replace the TODO sections with your story content
# 2. Add more scenes following the pattern above
# 3. Create branching paths with different choices
# 4. Use conditions for advanced branching:
#    condition=lambda state: state.has_flag("talked_to_cassie")
# 5. Track player choices with flags:
#    on_enter=lambda state: state.set_flag("found_memory")
# 6. Add items to inventory:
#    on_enter=lambda state: state.add_item("Midnight Shard")
# 7. Create ASCII art in shadow_slave_art.py
# 8. Test your story by running: python main.py
#
# For more examples, see: stories/noir_detective.py or stories/blood_and_neon.py
# ============================================================================
