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
        """Build the complete story graph - Shadow Slave Chapters 1-2"""

        # ============================================================================
        # CHAPTER 1: THE AWAKENING
        # ============================================================================

        self.scenes["nightmare_begins"] = Scene(
            id="nightmare_begins",
            description=(
                "Your eyes snap open. But this isn't your bed. This isn't your room.\n\n"
                "The ceiling above you writhes with impossible shadows. Ancient words burn "
                "themselves into your consciousness, searing through your mind like molten iron.\n\n"
                "The Nightmare Spell has chosen you.\n\n"
                "You try to move, but your body feels foreign. Wrong. The world spins as "
                "reality itself seems to crack and fold around you."
            ),
            ascii_art=ShadowSlaveArt.nightmare_gate(),
            dialogue=[
                ("THE SPELL", "Aspirant! Welcome to the Nightmare Spell."),
                ("THE SPELL", "You have been infected with the Nightmare Seed."),
                ("YOUR THOUGHTS", "No... this can't be real. Wake up. WAKE UP!"),
                ("THE SPELL", "Prepare to enter your First Nightmare..."),
            ],
            choices=[
                Choice(
                    text="Try to stay calm and listen",
                    next_scene="spell_explanation"
                ),
                Choice(
                    text="Panic and resist the Spell",
                    next_scene="resistance"
                ),
            ],
            on_enter=lambda state: state.set_flag("nightmare_started")
        )

        self.scenes["resistance"] = Scene(
            id="resistance",
            description=(
                "You thrash against the invisible bonds holding your consciousness.\n"
                "Pain lances through your skull as the Spell tightens its grip.\n\n"
                "Fighting the Nightmare Spell is like fighting the tide. Futile. Exhausting."
            ),
            dialogue=[
                ("THE SPELL", "Resistance will only make the transition more... painful."),
                ("YOUR THOUGHTS", "I won't... I won't give in!"),
                ("THE SPELL", "You have no choice, Aspirant. The Seed has already taken root."),
            ],
            choices=[
                Choice(
                    text="Stop resisting and accept it",
                    next_scene="spell_explanation"
                ),
                Choice(
                    text="Keep fighting despite the pain",
                    next_scene="forced_entry"
                ),
            ],
            on_enter=lambda state: state.set_flag("resisted_spell")
        )

        self.scenes["forced_entry"] = Scene(
            id="forced_entry",
            description=(
                "The pain becomes unbearable. Your consciousness fractures under the strain.\n\n"
                "The Spell doesn't care about your resistance. It never did.\n\n"
                "With a violent wrench, you're torn from reality and hurled into the abyss.\n"
                "The last thing you feel is your own scream echoing in the void."
            ),
            dialogue=[
                ("THE SPELL", "So be it."),
            ],
            choices=[
                Choice(
                    text="Fall into darkness",
                    next_scene="dream_realm_entry_rough"
                ),
            ]
        )

        self.scenes["spell_explanation"] = Scene(
            id="spell_explanation",
            description=(
                "You force yourself to breathe slowly. To think. To listen.\n\n"
                "The Spell's words continue to flow, each syllable carrying weight beyond comprehension.\n"
                "Information floods your mind: the Dream Realm, Nightmares, Attributes, Memories..."
            ),
            ascii_art=ShadowSlaveArt.spell_runes(),
            dialogue=[
                ("THE SPELL", "Wise choice, Aspirant."),
                ("THE SPELL", "You will now be transported to the Dream Realm to face your First Nightmare."),
                ("THE SPELL", "Survive, and you will awaken with power beyond mortal limits."),
                ("THE SPELL", "Fail, and you will be lost forever in the darkness."),
                ("YOUR THOUGHTS", "How do I survive? What am I supposed to do?"),
                ("THE SPELL", "That, Aspirant, is for you to discover."),
            ],
            choices=[
                Choice(
                    text="Ask about the Dream Realm",
                    next_scene="dream_realm_info"
                ),
                Choice(
                    text="Ask about how to survive",
                    next_scene="survival_info"
                ),
                Choice(
                    text="Accept your fate in silence",
                    next_scene="dream_realm_entry_calm"
                ),
            ],
            on_enter=lambda state: state.set_flag("learned_spell_basics")
        )

        self.scenes["dream_realm_info"] = Scene(
            id="dream_realm_info",
            description=(
                "The Spell pauses, as if considering your question."
            ),
            dialogue=[
                ("THE SPELL", "The Dream Realm is a place between sleep and death."),
                ("THE SPELL", "A realm of nightmares made manifest. Of ancient horrors and forgotten gods."),
                ("THE SPELL", "Reality there follows different rules. Logic is... flexible."),
                ("YOUR THOUGHTS", "That's not exactly reassuring..."),
            ],
            choices=[
                Choice(
                    text="Prepare yourself mentally",
                    next_scene="dream_realm_entry_calm"
                ),
            ],
            on_enter=lambda state: state.set_flag("learned_dream_realm")
        )

        self.scenes["survival_info"] = Scene(
            id="survival_info",
            description=(
                "You need to know. Need to understand what you're walking into."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "How do I survive this nightmare?"),
                ("THE SPELL", "Trust your instincts. Question everything."),
                ("THE SPELL", "The Dream Realm will test more than your strength, Aspirant."),
                ("THE SPELL", "It will test your will. Your mind. Your very soul."),
            ],
            choices=[
                Choice(
                    text="Steel yourself for what's coming",
                    next_scene="dream_realm_entry_calm"
                ),
            ],
            on_enter=lambda state: state.set_flag("learned_survival_tips")
        )

        self.scenes["dream_realm_entry_calm"] = Scene(
            id="dream_realm_entry_calm",
            description=(
                "You take a deep breath and center yourself.\n\n"
                "The world begins to fade. Not violently, but like a gentle dissolution.\n"
                "Colors bleed into grays. Sounds become distant echoes.\n\n"
                "And then... you fall."
            ),
            dialogue=[
                ("THE SPELL", "Good luck, Aspirant."),
                ("YOUR THOUGHTS", "Here goes nothing..."),
            ],
            choices=[
                Choice(
                    text="Open your eyes in the Dream Realm",
                    next_scene="first_awakening"
                ),
            ]
        )

        self.scenes["dream_realm_entry_rough"] = Scene(
            id="dream_realm_entry_rough",
            description=(
                "You crash into consciousness like a drowning man breaching water.\n\n"
                "Every nerve screams. Your head throbs with phantom pain.\n"
                "But you're alive. For now."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "Ugh... that was... awful."),
            ],
            choices=[
                Choice(
                    text="Struggle to your feet",
                    next_scene="first_awakening"
                ),
            ]
        )

        # ============================================================================
        # CHAPTER 2: FIRST TRIAL
        # ============================================================================

        self.scenes["first_awakening"] = Scene(
            id="first_awakening",
            description=(
                "⚠ REALITY SHIFTS ⚠\n\n"
                "You open your eyes to madness incarnate.\n\n"
                "Above you stretches an alien sky - a roiling mass of crimson and obsidian that "
                "pulses like an infected wound. Clouds of coagulated darkness drift across it, "
                "bleeding shadow-rain that evaporates before touching the ground.\n\n"
                "You're lying on cold basalt stone, your body stiff and unfamiliar. As you struggle "
                "to sit up, you realize you're in the ruins of what must have been a magnificent "
                "city - thousands of years ago. Now it's nothing but shattered dreams made stone.\n\n"
                "Broken pillars of obsidian and bone reach toward the bleeding sky like accusing "
                "fingers. Strange symbols are carved into every surface, glowing faintly with an "
                "inner light that hurts to look at directly.\n\n"
                "The air tastes of copper and ozone. Every breath feels heavy, laden with the "
                "weight of countless nightmares.\n\n"
                "In the distance... something massive moves through the ruins. Shadow-shapes, "
                "dozens of them, like a herd of impossible beasts."
            ),
            ascii_art=ShadowSlaveArt.crimson_sky(),
            dialogue=[
                ("YOUR THOUGHTS", "This is... the Dream Realm. The place between sleep and death."),
                ("YOUR THOUGHTS", "It's real. It's all REAL. And I'm trapped here."),
                ("YOUR THOUGHTS", "Focus. I need to survive. Find shelter. Find weapons. Find a way OUT."),
            ],
            choices=[
                Choice(
                    text="Look around for shelter",
                    next_scene="seek_shelter"
                ),
                Choice(
                    text="Investigate the moving shadow in the distance",
                    next_scene="investigate_shadow"
                ),
                Choice(
                    text="Search the ruins for supplies",
                    next_scene="search_ruins"
                ),
            ],
            on_enter=lambda state: state.set_flag("entered_dream_realm")
        )

        self.scenes["seek_shelter"] = Scene(
            id="seek_shelter",
            description=(
                "Survival instinct kicks in. Find shelter. Find safety.\n\n"
                "You spot a partially intact structure among the ruins - an archway leading "
                "into darkness. It could provide cover, or it could be a trap.\n\n"
                "Your eyes catch something else: a faint glow from within the shadows."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "That building might be safe... or it might be where something lives."),
            ],
            choices=[
                Choice(
                    text="Enter the dark archway carefully",
                    next_scene="dark_archway"
                ),
                Choice(
                    text="Investigate the glowing light",
                    next_scene="mysterious_glow"
                ),
                Choice(
                    text="Change your mind and explore elsewhere",
                    next_scene="first_awakening"
                ),
            ],
            on_enter=lambda state: state.set_flag("sought_shelter")
        )

        self.scenes["investigate_shadow"] = Scene(
            id="investigate_shadow",
            description=(
                "Curiosity wars with fear. You need to understand this place.\n\n"
                "You move carefully through the ruins toward the massive shadow. "
                "As you get closer, details emerge. It's not one creature - it's dozens.\n\n"
                "A herd of shadowy beasts, each the size of a horse, grazing on something "
                "that might have once been grass. They haven't noticed you yet."
            ),
            ascii_art=ShadowSlaveArt.shadow_creature(),
            dialogue=[
                ("YOUR THOUGHTS", "Nightmare Creatures... they're real. They're all real."),
            ],
            choices=[
                Choice(
                    text="Observe them from a safe distance",
                    next_scene="observe_creatures"
                ),
                Choice(
                    text="Try to sneak past them",
                    next_scene="sneak_past_creatures"
                ),
                Choice(
                    text="Retreat quietly",
                    next_scene="first_awakening"
                ),
            ],
            on_enter=lambda state: state.set_flag("saw_shadow_beasts")
        )

        self.scenes["search_ruins"] = Scene(
            id="search_ruins",
            description=(
                "You search through the crumbling ruins methodically.\n\n"
                "Most of it is worthless debris, but then your hand closes around something "
                "solid. A shard of dark metal, sharp enough to cut. It's not much, but "
                "it's better than nothing.\n\n"
                "As you pocket the shard, you hear something. A whisper in a language you "
                "shouldn't understand... but somehow do."
            ),
            dialogue=[
                ("WHISPER", "Lost... so lost... help us..."),
                ("YOUR THOUGHTS", "What was that?!"),
            ],
            choices=[
                Choice(
                    text="Follow the whispers",
                    next_scene="mysterious_whispers"
                ),
                Choice(
                    text="Ignore it and keep searching",
                    next_scene="find_weapon"
                ),
            ],
            on_enter=lambda state: state.add_item("Metal Shard")
        )

        self.scenes["dark_archway"] = Scene(
            id="dark_archway",
            description=(
                "⛧ TEMPLE OF THE FORGOTTEN ⛧\n\n"
                "You step through the archway into blessed darkness - a reprieve from the "
                "bleeding sky. The temperature drops immediately. Your breath mists in the air.\n\n"
                "Your eyes slowly adjust to the gloom. You're standing in what must have been "
                "a temple to some forgotten god. The architecture is impossible - walls that "
                "curve in ways that shouldn't exist in three-dimensional space. Broken statues "
                "line the chamber, their faces worn smooth by eons of wind and shadow.\n\n"
                "But what catches your attention is the center of the room.\n\n"
                "There, upon an obsidian pedestal carved with spiraling runes, rests something "
                "that glows with soft, silver light. It's beautiful - a crystallized memory, "
                "frozen in time like a precious gem. Even from here, you can feel the power "
                "radiating from it.\n\n"
                "The statues seem to be watching you. Guarding it. Or perhaps... warning you."
            ),
            ascii_art=ShadowSlaveArt.ancient_temple(),
            dialogue=[
                ("YOUR THOUGHTS", "A Memory... the Spell mentioned these. Fragments of power left behind "
                "by those who came before."),
                ("YOUR THOUGHTS", "But why is it just... sitting here? What's the catch?"),
            ],
            choices=[
                Choice(
                    text="Touch the glowing object",
                    next_scene="first_memory"
                ),
                Choice(
                    text="Examine it without touching",
                    next_scene="examine_memory"
                ),
                Choice(
                    text="Leave it alone and exit",
                    next_scene="first_awakening"
                ),
            ]
        )

        self.scenes["mysterious_glow"] = Scene(
            id="mysterious_glow",
            description=(
                "≋ SHADOW ESSENCE ≋\n\n"
                "You approach the glow with a mixture of fear and fascination.\n\n"
                "It's coming from a crack in the basalt floor - a jagged fracture that looks "
                "like a wound in reality itself. As you peer into the darkness, you see it: "
                "liquid shadow, flowing like mercury but glowing with an inner light that "
                "cycles through impossible colors.\n\n"
                "Shadow Essence. The raw stuff of the Dream Realm, condensed into physical form.\n\n"
                "It pulses hypnotically, each throb synchronized with your own heartbeat. "
                "You can FEEL it calling to you, promising power beyond imagination. Strength. "
                "Speed. The ability to command the very shadows themselves.\n\n"
                "But there's something else too - a hunger lurking beneath the promise. A "
                "darkness that goes deeper than any shadow. This power has a price.\n\n"
                "A voice echoes in your mind - definitely not the Spell. Something older. "
                "Something that's been waiting here for a very, very long time."
            ),
            ascii_art=ShadowSlaveArt.shadow_essence(),
            dialogue=[
                ("UNKNOWN VOICE", "Drink, young Aspirant. Become more than human."),
                ("UNKNOWN VOICE", "The shadows will answer your call... if you have the will to command them."),
                ("YOUR THOUGHTS", "This is dangerous. I can feel it. But the power... it could save my life."),
                ("YOUR THOUGHTS", "Do I trust this voice? Can I afford NOT to?"),
            ],
            choices=[
                Choice(
                    text="Drink the glowing liquid",
                    next_scene="drink_essence"
                ),
                Choice(
                    text="Refuse and back away",
                    next_scene="refuse_essence"
                ),
            ]
        )

        self.scenes["observe_creatures"] = Scene(
            id="observe_creatures",
            description=(
                "You watch the shadow beasts carefully, learning their patterns.\n\n"
                "They're not aggressive, you realize. Just... present. Existing in this "
                "twisted reality like everything else.\n\n"
                "One of them lifts its head, as if sensing something. Its eyeless face "
                "turns in your direction."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "Don't move. Don't even breathe."),
            ],
            choices=[
                Choice(
                    text="Stay perfectly still",
                    next_scene="stealth_success"
                ),
                Choice(
                    text="Run before it fully notices you",
                    next_scene="alert_creatures"
                ),
            ],
            on_enter=lambda state: state.set_flag("observed_beasts")
        )

        self.scenes["sneak_past_creatures"] = Scene(
            id="sneak_past_creatures",
            description=(
                "You move with careful precision, using the ruins as cover.\n\n"
                "Your heart hammers in your chest, but you force yourself to move slowly. "
                "One step. Then another.\n\n"
                "You're halfway past when your foot dislodges a loose stone."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "No!"),
            ],
            choices=[
                Choice(
                    text="Freeze and hope they ignore it",
                    next_scene="stealth_check"
                ),
                Choice(
                    text="Run immediately",
                    next_scene="alert_creatures"
                ),
            ]
        )

        self.scenes["mysterious_whispers"] = Scene(
            id="mysterious_whispers",
            description=(
                "You follow the whispers deeper into the ruins.\n\n"
                "They lead you to a chamber filled with fading light. Ghostly figures "
                "float in the air - echoes of people who once lived here.\n\n"
                "They're trapped. Bound to this place by something ancient and cruel."
            ),
            dialogue=[
                ("GHOST", "Free us... please... the chains..."),
                ("YOUR THOUGHTS", "How? How can I help you?"),
                ("GHOST", "Break... the seal..."),
            ],
            choices=[
                Choice(
                    text="Search for the seal they mentioned",
                    next_scene="find_seal"
                ),
                Choice(
                    text="Leave them - it's too dangerous",
                    next_scene="abandon_ghosts"
                ),
            ],
            on_enter=lambda state: state.set_flag("heard_whispers")
        )

        self.scenes["find_weapon"] = Scene(
            id="find_weapon",
            description=(
                "⚔ WEAPON DISCOVERED ⚔\n\n"
                "You ignore the whispers and continue your methodical search through the ruins. "
                "Every stone turned, every shadow investigated.\n\n"
                "Your persistence pays off.\n\n"
                "Beneath a collapsed pillar, half-buried in debris and centuries of dust, your "
                "fingers close around something solid. Metal. You pull it free, sending a shower "
                "of pebbles cascading down.\n\n"
                "It's a blade - ancient, rusty, the edge chipped and notched from countless "
                "battles. The grip is worn smooth by hands long dead. But when you heft it, "
                "the balance is perfect. This was a warrior's weapon once. Carried through "
                "nightmares and back.\n\n"
                "It's not much. But in a realm of nightmares, even a rusty blade is better "
                "than bare hands.\n\n"
                "You give it a practice swing. The weapon feels right. Deadly."
            ),
            ascii_art=ShadowSlaveArt.rusty_blade(),
            dialogue=[
                ("YOUR THOUGHTS", "This will have to do. At least I'm not defenseless anymore."),
                ("YOUR THOUGHTS", "Whatever comes for me next... I'll be ready."),
            ],
            choices=[
                Choice(
                    text="Continue exploring with your new weapon",
                    next_scene="trial_encounter"
                ),
            ],
            on_enter=lambda state: state.add_item("Rusty Blade")
        )

        self.scenes["first_memory"] = Scene(
            id="first_memory",
            description=(
                "≋ MEMORY ABSORBED ≋\n\n"
                "Your fingers brush the crystallized memory.\n\n"
                "Reality SHATTERS.\n\n"
                "You're no longer in the temple. You're no longer yourself. You ARE someone else - "
                "seeing through their eyes, feeling through their soul, living their final moments.\n\n"
                "A warrior. Clad in battered armor, standing atop a mountain of corpses. Surrounded "
                "by an endless horde of nightmare creatures that blot out the horizon. Alone. "
                "Abandoned. But not defeated.\n\n"
                "You feel their determination. Their refusal to yield. Their perfect form as they "
                "raise their sword one final time...\n\n"
                "'Not one step back,' they whisper. 'Not one inch of ground.'\n\n"
                "And then they charge.\n\n"
                "The memory fractures like glass. You're gasping, back in the temple, but changed. "
                "The knowledge burns in your muscles, your bones, your very soul. You know how to "
                "stand now. How to strike. How to survive against impossible odds.\n\n"
                "The crystallized memory has dissolved into nothing. But it lives on... in you."
            ),
            ascii_art=ShadowSlaveArt.warrior_memory_vision(),
            dialogue=[
                ("THE SPELL", "You have acquired a Memory: [Warrior's Echo - Rank: Dormant]"),
                ("THE SPELL", "Type: Tool. Description: The final stand of an unnamed warrior."),
                ("THE SPELL", "Attribute granted: [Combat Proficiency] (Dormant)"),
                ("YOUR THOUGHTS", "I can... I can FIGHT now. I know how. It's like I've trained for years..."),
                ("YOUR THOUGHTS", "Thank you, warrior. I'll make your sacrifice count."),
            ],
            choices=[
                Choice(
                    text="Leave the temple with your newfound power",
                    next_scene="trial_encounter"
                ),
            ],
            on_enter=lambda state: state.add_item("Warrior's Echo")
        )

        self.scenes["examine_memory"] = Scene(
            id="examine_memory",
            description=(
                "You study the glowing object carefully without touching it.\n\n"
                "It's beautiful - a crystallized memory, frozen in time. But something "
                "about it feels wrong. Dangerous.\n\n"
                "You decide caution is wiser than greed."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "Maybe I should be more careful in this place."),
            ],
            choices=[
                Choice(
                    text="Leave it and explore elsewhere",
                    next_scene="trial_encounter"
                ),
                Choice(
                    text="Change your mind and take it",
                    next_scene="first_memory"
                ),
            ],
            on_enter=lambda state: state.set_flag("examined_memory")
        )

        self.scenes["drink_essence"] = Scene(
            id="drink_essence",
            description=(
                "You cup your hands and drink the glowing liquid.\n\n"
                "It tastes like starlight and copper. Power floods through you, "
                "raw and overwhelming. Your body changes, adapts, becomes something more.\n\n"
                "But there's a price. You can feel it - a hunger, deep in your soul."
            ),
            dialogue=[
                ("THE SPELL", "You have consumed Shadow Essence."),
                ("THE SPELL", "Attribute granted: Shadow Affinity (Dormant)"),
                ("YOUR THOUGHTS", "What have I done to myself?"),
            ],
            choices=[
                Choice(
                    text="Embrace the power",
                    next_scene="trial_encounter_empowered"
                ),
            ],
            on_enter=lambda state: state.set_flag("consumed_essence")
        )

        self.scenes["refuse_essence"] = Scene(
            id="refuse_essence",
            description=(
                "You back away from the crack in the ground.\n\n"
                "The voice in your mind seems disappointed, but you trust your instincts. "
                "Some gifts come with prices too high to pay."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "I'll find power my own way."),
            ],
            choices=[
                Choice(
                    text="Continue exploring",
                    next_scene="trial_encounter"
                ),
            ],
            on_enter=lambda state: state.set_flag("refused_essence")
        )

        self.scenes["stealth_success"] = Scene(
            id="stealth_success",
            description=(
                "The shadow beast's attention drifts away. It returns to grazing.\n\n"
                "You let out a breath you didn't know you were holding. You made it past. "
                "But you've learned something valuable - in this realm, patience is survival."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "I can do this. I can survive here."),
            ],
            choices=[
                Choice(
                    text="Move onward",
                    next_scene="trial_encounter"
                ),
            ],
            on_enter=lambda state: state.set_flag("stealth_mastery")
        )

        self.scenes["stealth_check"] = Scene(
            id="stealth_check",
            description=(
                "The stone clatters down the rubble. Time seems to slow.\n\n"
                "The nearest shadow beast's head snaps toward the sound. It takes a step "
                "in your direction... then stops. Another beast makes a sound, drawing "
                "its attention away.\n\n"
                "You've survived. Barely."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "That was too close."),
            ],
            choices=[
                Choice(
                    text="Continue past them carefully",
                    next_scene="trial_encounter"
                ),
            ]
        )

        self.scenes["alert_creatures"] = Scene(
            id="alert_creatures",
            description=(
                "The shadow beasts' heads snap up as one. They've seen you.\n\n"
                "For a moment, everything is still. Then they move.\n\n"
                "You run."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "Run! Just run!"),
            ],
            choices=[
                Choice(
                    text="Sprint for cover",
                    next_scene="desperate_escape"
                ),
                Choice(
                    text="Stand and fight",
                    next_scene="hopeless_battle",
                    condition=lambda state: state.has_item("Rusty Blade") or state.has_item("Warrior's Echo")
                ),
            ]
        )

        self.scenes["find_seal"] = Scene(
            id="find_seal",
            description=(
                "◈ THE BINDING SEAL ◈\n\n"
                "You search the chamber methodically, following the pulse of dark energy that "
                "permeates the air. The ghosts hover anxiously, their translucent forms flickering.\n\n"
                "Then you find it.\n\n"
                "Carved into the floor at the chamber's center is an intricate seal - a "
                "mandala of death and binding. Ancient runes spiral outward from a central "
                "skull symbol, each line pulsing with malevolent energy. The craftsmanship is "
                "exquisite and terrible.\n\n"
                "Dark power radiates from it in waves, like heat from a furnace. This is what "
                "binds the spirits to this place. Chains forged from cruelty and written in "
                "the language of nightmares.\n\n"
                "You could break it. The seal is powerful, but you can sense its age. Centuries "
                "of erosion have weakened it. A few well-placed strikes would shatter it.\n\n"
                "But should you? These spirits have been here for eons. What if they're bound "
                "for a reason? What if releasing them unleashes something worse?\n\n"
                "The ghosts watch you, pleading silently. The choice is yours."
            ),
            ascii_art=ShadowSlaveArt.ghost_seal(),
            dialogue=[
                ("GHOST", "Please... we beg you... free us from this torment..."),
                ("GHOST", "We were innocent... trapped here by a tyrant long dead..."),
                ("YOUR THOUGHTS", "This feels like a test. The Dream Realm testing my character."),
                ("YOUR THOUGHTS", "Do I have the courage to show mercy... even when it might cost me?"),
            ],
            choices=[
                Choice(
                    text="Break the seal and free the spirits",
                    next_scene="break_seal"
                ),
                Choice(
                    text="Leave the seal intact and walk away",
                    next_scene="abandon_ghosts"
                ),
            ]
        )

        self.scenes["abandon_ghosts"] = Scene(
            id="abandon_ghosts",
            description=(
                "You turn away from the ghosts. Their whispers follow you, pleading, "
                "but you keep walking.\n\n"
                "Survival is what matters. You can't save everyone. Not here."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "I'm sorry. But I have to survive."),
            ],
            choices=[
                Choice(
                    text="Leave the chamber",
                    next_scene="trial_encounter"
                ),
            ],
            on_enter=lambda state: state.set_flag("abandoned_ghosts")
        )

        self.scenes["break_seal"] = Scene(
            id="break_seal",
            description=(
                "You raise your foot and bring it down on the seal.\n\n"
                "The stone cracks. Light erupts from the fracture. The ghosts scream - "
                "not in pain, but in joy - as they're finally released.\n\n"
                "One of them touches your shoulder before fading. You feel... blessed."
            ),
            dialogue=[
                ("GHOST", "Thank you, brave one. We will not forget this kindness."),
                ("THE SPELL", "You have been granted: [Ghost's Blessing]"),
                ("THE SPELL", "Attribute granted: Spirit Sight (Dormant)"),
            ],
            choices=[
                Choice(
                    text="Continue your journey",
                    next_scene="trial_encounter"
                ),
            ],
            on_enter=lambda state: state.set_flag("freed_ghosts")
        )

        self.scenes["trial_encounter"] = Scene(
            id="trial_encounter",
            description=(
                "⚠ FINAL TRIAL BEGINS ⚠\n\n"
                "You've explored the ruins. You've gained power. You've survived this far.\n\n"
                "But the Dream Realm has one final test.\n\n"
                "The ground trembles beneath your feet. Cracks spiderweb across the basalt stone. "
                "The crimson sky darkens, clouds roiling like a storm-tossed sea. The air grows "
                "thick with pressure, as if reality itself is holding its breath.\n\n"
                "Then... it emerges.\n\n"
                "From the deepest shadows of the ruined city comes a being that defies description. "
                "A towering colossus of nightmare and bone, shadow and malice given physical form. "
                "It stands three times your height, its body a patchwork of obsidian plates and "
                "exposed bone that gleams like polished ivory.\n\n"
                "Its eyes - burning with ancient hunger - lock onto you.\n\n"
                "The Guardian. The final obstacle between you and freedom. Between you and death.\n\n"
                "This is the moment everything has been building toward. Every choice you've made, "
                "every power you've gained, every lesson you've learned - it all comes down to this.\n\n"
                "The Guardian raises one massive arm. Reality screams."
            ),
            ascii_art=ShadowSlaveArt.guardian_boss(),
            dialogue=[
                ("THE SPELL", "Your First Nightmare reaches its climax."),
                ("THE SPELL", "Before you stands the Guardian - Rank: Fallen Terror."),
                ("THE SPELL", "Defeat it, and you will awaken as one of the Awakened."),
                ("THE SPELL", "Fail... and you will never leave the Dream Realm."),
                ("YOUR THOUGHTS", "This is it. Everything comes down to this single moment."),
                ("YOUR THOUGHTS", "I will NOT die here. I will NOT fail. I WILL survive!"),
            ],
            choices=[
                Choice(
                    text="Fight with everything you have",
                    next_scene="final_battle",
                    condition=lambda state: state.has_item("Rusty Blade") or state.has_item("Warrior's Echo")
                ),
                Choice(
                    text="Use your cunning instead of strength",
                    next_scene="clever_victory",
                    condition=lambda state: state.has_flag("observed_beasts") or state.has_flag("stealth_mastery")
                ),
                Choice(
                    text="Call upon the spirits for aid",
                    next_scene="spirit_aid",
                    condition=lambda state: state.has_flag("freed_ghosts")
                ),
                Choice(
                    text="Run - you're not ready for this",
                    next_scene="coward_ending"
                ),
            ]
        )

        self.scenes["trial_encounter_empowered"] = Scene(
            id="trial_encounter_empowered",
            description=(
                "The Shadow Essence burns in your veins as the Guardian appears.\n\n"
                "But you're different now. The shadows themselves seem to recognize you. "
                "The Guardian hesitates, sensing the power within you."
            ),
            ascii_art=ShadowSlaveArt.nightmare_creature_animation(),
            dialogue=[
                ("THE SPELL", "The Guardian recognizes your Shadow Affinity."),
                ("YOUR THOUGHTS", "I can feel it... the shadows. They're mine to command."),
            ],
            choices=[
                Choice(
                    text="Command the shadows to fight for you",
                    next_scene="shadow_victory"
                ),
                Choice(
                    text="Fight alongside the shadows",
                    next_scene="final_battle"
                ),
            ]
        )

        self.scenes["desperate_escape"] = Scene(
            id="desperate_escape",
            description=(
                "You run faster than you've ever run. Behind you, the shadow beasts give chase.\n\n"
                "You spot a narrow crevice in the ruins. You might fit. They won't."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "There! I can make it!"),
            ],
            choices=[
                Choice(
                    text="Dive into the crevice",
                    next_scene="narrow_escape"
                ),
            ]
        )

        self.scenes["hopeless_battle"] = Scene(
            id="hopeless_battle",
            description=(
                "You stand your ground, weapon raised.\n\n"
                "It's a brave choice. A foolish choice.\n\n"
                "You manage to wound one beast before the others overwhelm you. "
                "As darkness closes in, you wonder if anyone will remember your defiance."
            ),
            ascii_art=ShadowSlaveArt.death(),
            dialogue=[
                ("THE SPELL", "You have perished in the Dream Realm."),
            ],
            choices=[]
        )

        self.scenes["narrow_escape"] = Scene(
            id="narrow_escape",
            description=(
                "You squeeze through the crevice. The shadow beasts claw at the entrance, "
                "but they can't reach you.\n\n"
                "Eventually, they lose interest and wander away.\n\n"
                "You survived. This time."
            ),
            dialogue=[
                ("YOUR THOUGHTS", "I need to be smarter. More careful."),
            ],
            choices=[
                Choice(
                    text="Exit the crevice cautiously",
                    next_scene="trial_encounter"
                ),
            ]
        )

        # ============================================================================
        # ENDINGS
        # ============================================================================

        self.scenes["final_battle"] = Scene(
            id="final_battle",
            description=(
                "You charge the Guardian with a battle cry.\n\n"
                "The fight is brutal. Desperate. You take wounds that should be fatal, "
                "but somehow you keep moving. Keep fighting.\n\n"
                "Finally, you find an opening. Your blade - or your fists, or your will - "
                "strikes true. The Guardian falls.\n\n"
                "As it dissolves into shadow, the Dream Realm begins to fade."
            ),
            ascii_art=ShadowSlaveArt.victory(),
            dialogue=[
                ("THE SPELL", "Your First Nightmare is complete."),
                ("THE SPELL", "You have survived. You are now Awakened."),
                ("YOUR THOUGHTS", "I... I did it. I actually did it."),
            ],
            choices=[],
            on_enter=lambda state: state.set_flag("completed_nightmare")
        )

        self.scenes["clever_victory"] = Scene(
            id="clever_victory",
            description=(
                "You don't fight the Guardian head-on. That would be suicide.\n\n"
                "Instead, you use what you've learned. You lure it into unstable ruins. "
                "You use its size against it. You survive through cunning.\n\n"
                "When the Guardian finally falls - crushed under a collapsing pillar you "
                "weakened - you realize something important: strength isn't everything."
            ),
            ascii_art=ShadowSlaveArt.victory(),
            dialogue=[
                ("THE SPELL", "Your First Nightmare is complete."),
                ("THE SPELL", "You have survived through wisdom. Impressive."),
                ("THE SPELL", "Additional Attribute granted: Tactical Mind (Dormant)"),
            ],
            choices=[],
            on_enter=lambda state: state.set_flag("clever_victory")
        )

        self.scenes["spirit_aid"] = Scene(
            id="spirit_aid",
            description=(
                "☆ THE SPIRITS ANSWER YOUR CALL ☆\n\n"
                "The Guardian lunges toward you, massive fists ready to crush you into paste.\n\n"
                "You cry out - not in fear, but in hope. Calling to the spirits you freed. "
                "Praying that kindness means something in this nightmare realm.\n\n"
                "For a heartbeat, nothing happens. The Guardian's fist descends.\n\n"
                "Then...\n\n"
                "Light erupts around you. Not the harsh light of the crimson sky, but soft, "
                "silvery radiance. The ghosts appear - no longer formless wisps, but warriors "
                "in spectral armor, weapons gleaming with ethereal fire.\n\n"
                "'We owe you a debt, Aspirant of Mercy,' they speak as one. 'And we ALWAYS "
                "repay our debts.'\n\n"
                "The ghost warriors surge forward, three translucent figures moving with "
                "the grace of master fighters. They intercept the Guardian's blow, their "
                "spirit blades meeting nightmare flesh.\n\n"
                "Together, you fight. You - flesh and blood and desperate courage. Them - "
                "echoes of the dead, given purpose by your compassion.\n\n"
                "The Guardian falls.\n\n"
                "As it dissolves, the ghosts turn to you one final time. They smile - truly "
                "smile - before fading into peace at last.\n\n"
                "Kindness, it turns out, has terrible power in the Dream Realm."
            ),
            ascii_art=ShadowSlaveArt.ghost_warriors(),
            dialogue=[
                ("GHOST CAPTAIN", "Thank you, child of mercy. We can finally rest."),
                ("THE SPELL", "Your First Nightmare is complete."),
                ("THE SPELL", "You have proven that compassion can exist even in the darkest places."),
                ("THE SPELL", "Additional Attribute granted: [Spirit Companion] (Dormant)"),
                ("THE SPELL", "The ghosts' blessing will remain with you... always."),
            ],
            choices=[],
            on_enter=lambda state: state.set_flag("spirit_victory")
        )

        self.scenes["shadow_victory"] = Scene(
            id="shadow_victory",
            description=(
                "◈◈◈ THE SHADOWS OBEY ◈◈◈\n\n"
                "The Shadow Essence burns through your veins like liquid midnight. You can "
                "FEEL them now - the shadows all around you, alive and hungry and waiting.\n\n"
                "The Guardian charges. You raise your hand.\n\n"
                "And the shadows... RISE.\n\n"
                "They erupt from every corner of the ruins, from every crack in the stone, "
                "from your own elongated silhouette. Dozens of shadow-forms, humanoid but "
                "wrong, each one a fragment of darkness given terrible purpose.\n\n"
                "'ATTACK,' you command, and your voice echoes with power that isn't entirely human.\n\n"
                "The shadow army swarms the Guardian like locusts. It roars - the first sound "
                "you've heard it make - as darkness consumes it from all sides. Its massive "
                "fists crush shadow-soldiers by the handful, but for each one destroyed, two "
                "more rise to take its place.\n\n"
                "In minutes, it's over. The Guardian collapses, overwhelmed by sheer numbers.\n\n"
                "As it dissolves, your shadow army turns to you, awaiting further orders. "
                "Their eyeless faces seem almost... reverent.\n\n"
                "You've become something the Dream Realm respects. Something powerful.\n"
                "Something changed.\n\n"
                "But as you stand there, master of shadows, you can't help but wonder:\n"
                "What price will this power ultimately exact from your soul?"
            ),
            ascii_art=ShadowSlaveArt.shadow_army(),
            dialogue=[
                ("THE SPELL", "Your First Nightmare is complete."),
                ("THE SPELL", "You have consumed the Shadow Essence and claimed dominion over darkness itself."),
                ("THE SPELL", "Title granted: [Shadow Aspirant]"),
                ("THE SPELL", "Additional Attribute granted: [Shadow Legion] (Dormant)"),
                ("YOUR THOUGHTS", "I have power now. Incredible power. But I can feel it... changing me."),
                ("YOUR THOUGHTS", "Was it worth it? ...I hope so."),
            ],
            choices=[],
            on_enter=lambda state: state.set_flag("shadow_master")
        )

        self.scenes["coward_ending"] = Scene(
            id="coward_ending",
            description=(
                "You turn and run from the Guardian.\n\n"
                "But there's nowhere to run. The Dream Realm has no exits for those who "
                "fail their trials.\n\n"
                "The Guardian catches you. The nightmare is eternal."
            ),
            ascii_art=ShadowSlaveArt.death(),
            dialogue=[
                ("THE SPELL", "You have failed your First Nightmare."),
                ("THE SPELL", "You will remain in the Dream Realm forever."),
            ],
            choices=[]
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
