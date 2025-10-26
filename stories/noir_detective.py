"""The Last Case - A Noir Detective Story"""

from engine.story import Story, Scene, Choice
from engine.animation import Animation, AnimationLibrary
from engine.colors import get_mood_palette


class NoirDetectiveStory(Story):
    """A noir detective mystery story"""
    
    def __init__(self):
        super().__init__()
        self.title = "THE LAST CASE"
        self.description = (
            "The rain hasn't stopped for three days. Neither has the blood.\n"
            "You're Jack Malone, a private eye with a dead client and a smoking gun.\n"
            "In this city, everyone's guilty of something. Your job is to find out what."
        )
        self.starting_scene = "opening"
        self._build_story()
        self._apply_scene_palettes()
    
    def _apply_scene_palettes(self):
        """Assign color palettes to scenes based on mood"""
        overrides = {
            "opening": "mystery",
            "search_body": "alert",
            "check_gun": "alert",
            "run_away": "danger",
            "castellano_meeting": "mystery",
            "alley_fight": "danger",
            "warehouse": "mystery",
            "warehouse_with_eddie": "danger",
            "warehouse_sneak": "mystery",
            "velvet_room": "mystery",
            "theater": "mystery",
            "rodriguez": "calm",
            "call_eddie": "calm",
            "rodriguez_plan": "alert",
            "solo_mission": "danger",
            "warehouse_shootout_eddie": "danger",
            "warehouse_evidence": "alert",
            "velvet_room_with_eddie": "mystery",
            "theater_reveal": "mystery",
            "theater_fight": "danger",
            "doctor_visit": "calm",
            "escape_with_evidence": "calm",
            "rodriguez_raid": "danger",
            "chen_partnership": "alert",
            "fbi_cooperation": "calm",
            "warehouse_negotiation": "alert",
            "warehouse_rush_guards": "danger",
            "warehouse_wait": "alert",
            "warehouse_front": "alert",
            "warehouse_retreat": "alert",
            "warehouse_skylight_attack": "danger",
            "warehouse_shootout": "danger",
            "warehouse_injured": "danger",
            "escape_run": "danger",
            "escape_with_rodriguez": "calm",
            "morrison_confrontation": "danger",
            "project_nightfall": "mystery",
        }
        keyword_map = [
            ("danger", ["fight", "shoot", "shootout", "attack", "ambush", "raid", "gun", "fire", "blood", "kill", "execution", "burn", "bomb"]),
            ("alert", ["search", "check", "investigate", "follow", "trail", "watch", "stake", "intercept", "trace", "surveil", "shadow", "pursuit", "chase"]),
            ("calm", ["doctor", "safe", "ally", "friend", "eddie", "rodriguez", "chen", "partner", "home", "office", "c3", "safehouse"]),
            ("mystery", ["warehouse", "velvet", "night", "project", "nightfall", "secret", "meeting", "intel", "evidence", "shadow", "conspiracy", "ritual", "mystery", "dark"]),
        ]
        
        for scene_id, scene in self.scenes.items():
            mood = overrides.get(scene_id)
            if not mood:
                lower_id = scene_id.lower()
                for mood_name, keywords in keyword_map:
                    if any(keyword in lower_id for keyword in keywords):
                        mood = mood_name
                        break
            if not mood:
                mood = "noir"
            scene.palette = get_mood_palette(mood)
    
    def _build_story(self):
        """Build the complete story graph"""
        
        self.scenes["opening"] = Scene(
            id="opening",
            description=(
                "The neon sign outside your office window flickers - 'MALONE INVESTIGATIONS'.\n"
                "It's 2 AM. The dame who walked in an hour ago is now lying face-down on your carpet,\n"
                "a red rose blooming across her white dress. Your gun is still warm in your hand.\n\n"
                "You don't remember pulling the trigger. But in this business, memory is just\n"
                "another luxury you can't afford."
            ),
            ascii_art=AnimationLibrary.detective_office(),
            dialogue=[
                ("JACK MALONE", "Think, Jack. Think... What happened?"),
                ("JACK MALONE", "The dame said she was being followed. Said she had proof that would\n                bring down the Castellano family. Then... nothing."),
                ("JACK MALONE", "I need to figure this out fast. Cops will be here any minute.")
            ],
            choices=[
                Choice("Search the woman's body for clues", "search_body"),
                Choice("Check your own gun - how many bullets fired?", "check_gun"),
                Choice("Run. Get out while you still can.", "run_away")
            ]
        )
        
        self.scenes["search_body"] = Scene(
            id="search_body",
            description=(
                "You kneel beside the body, your hands steady despite everything.\n"
                "Professional habits die hard. In her clutch, you find a photograph:\n"
                "Vincent Castellano shaking hands with Police Chief Morrison.\n\n"
                "Behind them, a warehouse with '47' painted on the door.\n"
                "There's something else - a ticket stub. Tonight's performance at the Ritz Theater.\n"
                "Section 47. Seat 13."
            ),
            dialogue=[
                ("JACK MALONE", "Warehouse 47... Seat 47... That's not coincidence."),
                ("JACK MALONE", "Either she wanted me to find this, or someone wanted me to find her.")
            ],
            choices=[
                Choice("Go to Warehouse 47 - confront this head-on", "warehouse"),
                Choice("Visit the Ritz Theater - check out the connection", "theater"),
                Choice("Take the evidence to Captain Rodriguez - he's clean", "rodriguez")
            ]
        )
        
        def check_gun_enter(state):
            state.set_flag("examined_gun")
        
        self.scenes["check_gun"] = Scene(
            id="check_gun",
            description=(
                "You pop the cylinder. Six chambers. Six bullets.\n"
                "Not a single one fired.\n\n"
                "Your blood runs cold. Someone else pulled the trigger.\n"
                "Someone who wants you to take the fall."
            ),
            dialogue=[
                ("JACK MALONE", "I've been played. Set up like a patsy."),
                ("JACK MALONE", "But who? And why?")
            ],
            on_enter=check_gun_enter,
            choices=[
                Choice("Search the dame for clues", "search_body_after_gun"),
                Choice("Check the office - maybe they're still here", "check_office"),
                Choice("Call your old partner Eddie for help", "call_eddie")
            ]
        )
        
        self.scenes["run_away"] = Scene(
            id="run_away",
            description=(
                "You grab your coat and hat, heading for the fire escape.\n"
                "The rain hits you like a fist as you descend into the alley.\n\n"
                "But as you reach the street, three figures emerge from the shadows.\n"
                "Castellano's men. They've been waiting."
            ),
            ascii_art=AnimationLibrary.cityscape(),
            dialogue=[
                ("TONY 'THE FIST'", "Going somewhere, Malone?"),
                ("JACK MALONE", "Just out for a smoke, Tony."),
                ("TONY", "Mr. Castellano wants to see you. Says you got something belongs to him."),
            ],
            choices=[
                Choice("Go with them peacefully - maybe you can talk to Castellano", "castellano_meeting"),
                Choice("Fight your way out - three against one, you've had worse odds", "alley_fight"),
                Choice("Tell them about the dame - claim you were set up", "blame_setup")
            ]
        )
        
        self.scenes["search_body_after_gun"] = Scene(
            id="search_body_after_gun",
            description=(
                "The photograph tells you everything: Vincent Castellano, Police Chief Morrison,\n"
                "and Warehouse 47. There's also a ticket stub from the Ritz Theater.\n\n"
                "But now you know you're being framed. Someone wanted this woman dead\n"
                "and wanted you holding the bag."
            ),
            dialogue=[
                ("JACK MALONE", "This is bigger than I thought. The cops, the mob... all connected."),
            ],
            choices=[
                Choice("Head to Warehouse 47 - end this tonight", "warehouse"),
                Choice("Find Eddie first - you need backup", "call_eddie"),
            ]
        )
        
        self.scenes["check_office"] = Scene(
            id="check_office",
            description=(
                "You move through your office like a ghost, checking corners, shadows.\n"
                "Behind the filing cabinet, you find it: a service panel left ajar.\n\n"
                "Someone was in the walls. They could have shot her from there,\n"
                "tossed your gun in your hand while you were... what? Drugged?\n\n"
                "On the floor, a business card: 'The Velvet Room - Where Secrets Sleep.'"
            ),
            dialogue=[
                ("JACK MALONE", "The Velvet Room. High-class nightclub. Castellano territory."),
                ("JACK MALONE", "Why leave a calling card unless they want me to follow?")
            ],
            choices=[
                Choice("Go to the Velvet Room - walk into the trap with eyes open", "velvet_room"),
                Choice("Search the dame for more clues first", "search_body_after_gun"),
                Choice("Get out of town - this is above your pay grade", "leave_town")
            ]
        )
        
        def call_eddie_enter(state):
            state.set_flag("called_eddie")
        
        self.scenes["call_eddie"] = Scene(
            id="call_eddie",
            description=(
                "You dial Eddie's number. Three rings.\n"
                "'Yeah?' His voice is rough with sleep.\n"
                "'Eddie, it's Jack. I need help. There's a body in my office and—'\n"
                "'Say no more. I'll be there in ten.'\n\n"
                "Eddie shows up in eight. He surveys the scene with cop's eyes,\n"
                "even though he turned in his badge six months ago."
            ),
            dialogue=[
                ("EDDIE", "Jesus, Jack. You really stepped in it this time."),
                ("JACK MALONE", "I didn't do it, Eddie."),
                ("EDDIE", "I know. You're a lot of things, but you ain't a killer.\n          Not like this, anyway. We need to find who did."),
                ("EDDIE", "I've been hearing things. Warehouse 47. Big meeting tonight.\n          Castellano, Morrison... maybe more. This dame was probably\n          insurance - proof for whoever wanted out.")
            ],
            on_enter=call_eddie_enter,
            choices=[
                Choice("Hit Warehouse 47 with Eddie - time to crash the party", "warehouse_with_eddie"),
                Choice("Have Eddie watch your back while you visit the Velvet Room", "velvet_room_with_eddie"),
            ]
        )
        
        self.scenes["warehouse"] = Scene(
            id="warehouse",
            description=(
                "Warehouse 47 sits on the docks like a rotting tooth.\n"
                "Two guards out front, cigarettes glowing in the rain.\n"
                "Through a grimy window, you see them: Castellano, Morrison,\n"
                "and a third man you don't recognize. Military bearing. Cold eyes."
            ),
            ascii_art=AnimationLibrary.warehouse(),
            dialogue=[
                ("JACK MALONE", "Whatever's happening in there, it's big enough to kill for."),
            ],
            choices=[
                Choice("Sneak in through the roof - get close and listen", "warehouse_sneak"),
                Choice("Walk in the front door - surprise is a weapon too", "warehouse_front"),
                Choice("Wait and follow whoever leaves first", "warehouse_wait"),
            ]
        )
        
        self.scenes["theater"] = Scene(
            id="theater",
            description=(
                "The Ritz Theater is dark except for the exit signs.\n"
                "You find Section 47, Seat 13. Under the cushion: an envelope.\n"
                "Inside, documents. Bank transfers. Payoffs. Names.\n\n"
                "The whole department is dirty. Not just Morrison - dozens of cops,\n"
                "judges, city councilmen. And at the center: Project Nightfall.\n"
                "Military weapons being sold to the highest bidder."
            ),
            dialogue=[
                ("JACK MALONE", "The dame wasn't running from Castellano. She was running\n                from everyone. This is enough to burn the whole city down."),
                ("VOICE FROM DARKNESS", "That's exactly right, Mr. Malone.")
            ],
            choices=[
                Choice("Spin and draw your gun", "theater_fight"),
                Choice("Stay calm - 'Who's there?'", "theater_reveal"),
            ]
        )
        
        def rodriguez_enter(state):
            state.set_flag("went_to_rodriguez")
        
        self.scenes["rodriguez"] = Scene(
            id="rodriguez",
            description=(
                "Captain Rodriguez looks at the photograph, his face unreadable.\n"
                "For a long moment, he says nothing. Then he opens his desk drawer\n"
                "and pulls out a bottle of whiskey. Two glasses.\n\n"
                "'I've been waiting for this day, Jack. Hoping it wouldn't come,\n"
                "but knowing it would.'"
            ),
            dialogue=[
                ("RODRIGUEZ", "I've known about Morrison for two years. Couldn't prove it.\n              Couldn't trust anyone. The rot goes deep."),
                ("JACK MALONE", "And now?"),
                ("RODRIGUEZ", "Now we have proof. And now you're a murder suspect.\n              They played this smart - take you off the board, silence\n              their leak, all in one move."),
                ("RODRIGUEZ", "I can protect you, Jack. But we need to move fast.\n              There's a meeting tonight. Warehouse 47. All the players\n              will be there.")
            ],
            on_enter=rodriguez_enter,
            choices=[
                Choice("Trust Rodriguez - let him handle this officially", "rodriguez_plan"),
                Choice("Take the evidence and do this your way", "solo_mission"),
            ]
        )
        
        self.scenes["castellano_meeting"] = Scene(
            id="castellano_meeting",
            description=(
                "They drive you to the Velvet Room. Vincent Castellano sits in a\n"
                "private booth, flanked by men who look like they eat bullets for breakfast.\n\n"
                "'Jack Malone. I'd say it's good to see you, but we both know\n"
                "I'd be lying.' He gestures to the seat across from him."
            ),
            ascii_art=AnimationLibrary.velvet_room(),
            dialogue=[
                ("CASTELLANO", "You got a dead woman in your office and your prints on the gun.\n                Bad night for you, Jack."),
                ("JACK MALONE", "I didn't kill her."),
                ("CASTELLANO", "I know. I did."),
            ],
            choices=[
                Choice("'Why tell me?' - keep him talking", "castellano_reveal"),
                Choice("Go for your gun - end this now", "castellano_shootout"),
            ]
        )
        
        self.scenes["alley_fight"] = Scene(
            id="alley_fight",
            description=(
                "You throw the first punch. Tony's nose breaks with a satisfying crunch.\n"
                "The second goon pulls a knife, but you're already moving.\n\n"
                "The fight is brutal, quick, messy. When it's over, you're bleeding\n"
                "from three places and Tony's friends aren't getting up.\n"
                "Tony himself is on one knee, glaring at you."
            ),
            dialogue=[
                ("TONY", "You're... dead, Malone. Mr. C will... burn this city... to find you."),
                ("JACK MALONE", "Tell your boss I'm coming for him. And I'm bringing answers."),
            ],
            choices=[
                Choice("Head to Warehouse 47 - finish this tonight", "warehouse_injured"),
                Choice("Find a doctor you can trust - you won't survive much more bleeding", "doctor_visit"),
            ]
        )
        
        self.scenes["blame_setup"] = Scene(
            id="blame_setup",
            description=(
                "'Your boss wanted me to take the fall for a murder I didn't commit.\n"
                "The dame in my office? Someone else pulled that trigger.'\n\n"
                "Tony exchanges a glance with his boys. You see the doubt creep in."
            ),
            dialogue=[
                ("TONY", "Mr. C don't play games like that. He wants someone dead,\n          they die. No setups, no frames."),
                ("JACK MALONE", "Then maybe someone's playing him too. Someone bigger."),
                ("TONY", "Get in the car. We're all gonna have a talk with the boss."),
            ],
            choices=[
                Choice("Get in the car - meet Castellano", "castellano_meeting"),
                Choice("Make a break for it - run while they're confused", "escape_run"),
            ]
        )
        
        self.scenes["warehouse_with_eddie"] = Scene(
            id="warehouse_with_eddie",
            description=(
                "You and Eddie approach Warehouse 47 like the old days.\n"
                "No words needed - you take the roof, he takes the back door.\n\n"
                "From above, you see the meeting: Castellano, Morrison,\n"
                "and a man in a military uniform. General Marcus Blackwood.\n"
                "You know that name. Army weapons contractor. Officially dead two years ago."
            ),
            dialogue=[
                ("BLACKWOOD", "Gentlemen, Project Nightfall is proceeding as planned.\n              The weapons shipment arrives in three days. Half goes to\n              Castellano's buyers. Half goes to Morrison's distribution network."),
                ("MORRISON", "What about the Malone situation?"),
                ("BLACKWOOD", "By morning, he'll be wanted for murder. By noon, he'll be dead.\n              One way or another."),
            ],
            choices=[
                Choice("Drop in guns blazing - citizen's arrest", "warehouse_shootout_eddie"),
                Choice("Record everything and get out - build a real case", "warehouse_evidence"),
            ]
        )
        
        self.scenes["velvet_room_with_eddie"] = Scene(
            id="velvet_room_with_eddie",
            description=(
                "The Velvet Room is packed. Jazz plays soft and low.\n"
                "Eddie takes a position at the bar while you head to the back rooms.\n\n"
                "You find what you're looking for: an office. Inside, filing cabinets\n"
                "full of records. Blackmail material. Insurance policies.\n"
                "And a ledger with every dirty cop in the city."
            ),
            dialogue=[
                ("EDDIE (radio)", "Jack, we got company. Morrison just walked in.\n                  He's headed your way."),
                ("JACK MALONE", "Copy that. I've got what we need. Meet at the back exit."),
            ],
            choices=[
                Choice("Confront Morrison - end this face to face", "morrison_confrontation"),
                Choice("Slip out with the evidence - let the law handle it", "escape_with_evidence"),
            ]
        )
        
        self.scenes["warehouse_sneak"] = Scene(
            id="warehouse_sneak",
            description=(
                "The skylight gives you a perfect view. You listen as they discuss\n"
                "Project Nightfall - a massive weapons smuggling operation.\n"
                "Military hardware being sold to criminals. The dead dame was\n"
                "an accountant who discovered the truth."
            ),
            dialogue=[
                ("BLACKWOOD", "Malone will be eliminated before dawn. The evidence is in place."),
                ("CASTELLANO", "What about the other copies?"),
                ("BLACKWOOD", "There are no other copies. The woman was working alone."),
                ("MALONE (thinking)", "They're wrong. If she came to me, she had insurance."),
            ],
            choices=[
                Choice("Crash through the skylight - surprise attack", "warehouse_skylight_attack"),
                Choice("Back out slowly and regroup - you're outgunned", "warehouse_retreat"),
            ]
        )
        
        self.scenes["warehouse_front"] = Scene(
            id="warehouse_front",
            description=(
                "You walk up to the front door and knock. The guards look at you\n"
                "like you've lost your mind.\n\n"
                "'Tell Castellano that Jack Malone is here. Tell him I know\n"
                "about Nightfall. And tell him I have copies.'"
            ),
            dialogue=[
                ("GUARD", "You got a death wish, Malone?"),
                ("JACK MALONE", "Just good business sense. Your boss wants what I have.\n                I want what he knows. Let's deal."),
            ],
            choices=[
                Choice("Wait to be let in - play this cool", "warehouse_negotiation"),
                Choice("The guards are distracted - rush them", "warehouse_rush_guards"),
            ]
        )
        
        self.scenes["warehouse_wait"] = Scene(
            id="warehouse_wait",
            description=(
                "Patience is a virtue in your line of work. You wait in the rain,\n"
                "watching. An hour passes. Then Morrison emerges alone, carrying a briefcase.\n\n"
                "He looks nervous. Keeps checking over his shoulder.\n"
                "You follow him through the rain-slicked streets."
            ),
            dialogue=[
                ("JACK MALONE", "Where are you going, Chief? What's so important you're\n                cutting out early?"),
            ],
            choices=[
                Choice("Confront Morrison in the street", "street_confrontation"),
                Choice("Follow him to his destination", "follow_morrison"),
            ]
        )
        
        self.scenes["theater_fight"] = Scene(
            id="theater_fight",
            description=(
                "You spin, gun drawn. The shot echoes through the empty theater.\n"
                "The figure drops. You approach carefully.\n\n"
                "It's not a soldier. Not a mobster. It's a woman. Late fifties.\n"
                "She's wearing a federal badge."
            ),
            ascii_art=AnimationLibrary.gun(),
            dialogue=[
                ("DYING WOMAN", "Idiot... I was here... to help you... FBI... investigating...\n                Project Nightfall..."),
                ("JACK MALONE", "Jesus... I didn't know..."),
                ("DYING WOMAN", "They'll come... for you... run..."),
            ],
            choices=[
                Choice("Turn yourself in - it was self defense", "turn_self_in"),
                Choice("Run - you're in too deep now", "run_from_fbi"),
            ]
        )
        
        self.scenes["theater_reveal"] = Scene(
            id="theater_reveal",
            description=(
                "The figure steps into the dim light. FBI Special Agent Sarah Chen.\n"
                "She lowers her weapon.\n\n"
                "'Mr. Malone. I've been tracking Project Nightfall for two years.\n"
                "The woman in your office was one of my informants.'"
            ),
            dialogue=[
                ("CHEN", "They killed her to frame you. Two birds, one stone.\n          You've been making people nervous, asking the wrong questions."),
                ("JACK MALONE", "So what happens now?"),
                ("CHEN", "Now we have a choice. You can come in, become a witness,\n          and hope my protection holds. Or we can end this tonight.\n          Warehouse 47. All the players. One final move."),
            ],
            choices=[
                Choice("Trust the FBI - let them handle it", "fbi_cooperation"),
                Choice("Partner with Chen - end it tonight at the warehouse", "chen_partnership"),
            ]
        )
        
        self.scenes["rodriguez_plan"] = Scene(
            id="rodriguez_plan",
            description=(
                "Rodriguez works the phones. Within an hour, he's assembled a team\n"
                "of cops he trusts - not many, but enough. You suit up: tactical gear,\n"
                "body armor, the works.\n\n"
                "'This is it, Jack. We go in hard, we go in clean. Everyone comes out\n"
                "in cuffs or body bags. And when it's done, you're clear.'"
            ),
            dialogue=[
                ("JACK MALONE", "Let's finish this."),
            ],
            choices=[
                Choice("Raid Warehouse 47 - the right way", "rodriguez_raid"),
            ]
        )
        
        self.scenes["solo_mission"] = Scene(
            id="solo_mission",
            description=(
                "You take the evidence and walk out of Rodriguez's office.\n"
                "He doesn't stop you. You both know this is personal now.\n\n"
                "Warehouse 47 looms ahead. You check your gun one more time.\n"
                "Six bullets. Hopefully enough."
            ),
            dialogue=[
                ("JACK MALONE", "One man against the world. Just how I like it."),
            ],
            choices=[
                Choice("Attack the warehouse - no backup, no plan, just justice", "solo_warehouse_attack"),
            ]
        )
        
        self.scenes["castellano_reveal"] = Scene(
            id="castellano_reveal",
            description=(
                "Castellano leans back, lighting a cigar.\n\n"
                "'I tell you because you're already dead, Jack. Question is whether\n"
                "you die knowing the truth or die confused. Me? I'm generous that way.'"
            ),
            dialogue=[
                ("CASTELLANO", "That dame worked for a man named Blackwood. General Marcus Blackwood.\n                He's running guns, weapons, military hardware. I'm just the distributor."),
                ("CASTELLANO", "She got cold feet. Wanted out. Wanted to talk. So Blackwood\n                ordered her dead. And you framed. Two problems, one solution."),
                ("JACK MALONE", "Why tell me this?"),
                ("CASTELLANO", "Because I'm tired of being Blackwood's errand boy. You want\n                revenge? Help me take him down. Then we both walk away clean."),
            ],
            choices=[
                Choice("Accept the deal - enemy of my enemy", "castellano_alliance"),
                Choice("Refuse - 'I don't work with killers'", "refuse_castellano"),
            ]
        )
        
        self.scenes["castellano_shootout"] = Scene(
            id="castellano_shootout",
            description=(
                "Your hand moves for your gun. Castellano's guards are faster.\n"
                "Three guns bark. You feel the impacts like hammers.\n\n"
                "The world tilts. You hit the floor. Castellano stands over you."
            ),
            dialogue=[
                ("CASTELLANO", "Shame, Malone. We could've helped each other.\n                Instead, you die stupid."),
            ],
            choices=[
                Choice("...", "ending_killed_by_castellano"),
            ]
        )
        
        self.scenes["warehouse_injured"] = Scene(
            id="warehouse_injured",
            description=(
                "You stagger to Warehouse 47, leaving a trail of blood.\n"
                "The guards see you coming. They raise their weapons.\n\n"
                "You raise your hands. 'I need to see Castellano. Tell him\n"
                "I know who really killed the dame. Tell him it wasn't me.'"
            ),
            dialogue=[
                ("GUARD", "Boss! Malone's here. Says he's got information."),
                ("CASTELLANO (distant)", "Bring him in. Let's hear what the dead man walking has to say."),
            ],
            choices=[
                Choice("Enter the warehouse - last gambit", "warehouse_wounded_entry"),
            ]
        )
        
        self.scenes["doctor_visit"] = Scene(
            id="doctor_visit",
            description=(
                "Doc Stevens doesn't ask questions anymore. He patches you up,\n"
                "gives you painkillers, and points you toward the door.\n\n"
                "'Whatever you're into, Jack, finish it soon. Next time I might\n"
                "not be able to put you back together.'"
            ),
            dialogue=[
                ("JACK MALONE", "Thanks, Doc. Put it on my tab."),
                ("DOC STEVENS", "Your tab could buy a new clinic. Just... be careful."),
            ],
            choices=[
                Choice("Head to Warehouse 47 - time to end this", "warehouse"),
                Choice("Investigate the Velvet Room instead", "velvet_room"),
            ]
        )
        
        self.scenes["escape_run"] = Scene(
            id="escape_run",
            description=(
                "You bolt. Tony shouts, but you're already around the corner.\n"
                "The rain works in your favor - sound dampened, visibility low.\n\n"
                "You know these streets. You disappear into the night like smoke.\n"
                "But now Castellano knows you're onto something. And he'll be looking."
            ),
            dialogue=[
                ("JACK MALONE", "Bought myself some time. Now I need to use it."),
            ],
            choices=[
                Choice("Go to ground at Eddie's place - need backup", "call_eddie"),
                Choice("Hit Warehouse 47 before they expect it", "warehouse"),
            ]
        )
        
        self.scenes["warehouse_shootout_eddie"] = Scene(
            id="warehouse_shootout_eddie",
            description=(
                "You drop through the skylight, gun blazing. Eddie kicks in the back door.\n"
                "The warehouse erupts in chaos.\n\n"
                "Morrison goes down first. Castellano's men return fire.\n"
                "Blackwood pulls a weapon you've never seen before - military prototype.\n"
                "The beam cuts through metal like paper."
            ),
            dialogue=[
                ("EDDIE", "Jack! Get down!"),
            ],
            choices=[
                Choice("Take cover and return fire", "shootout_tactical"),
                Choice("Charge Blackwood - end this fast", "charge_blackwood"),
            ]
        )
        
        self.scenes["warehouse_evidence"] = Scene(
            id="warehouse_evidence",
            description=(
                "Eddie has it all on tape. Video, audio, everything.\n"
                "You slip out the way you came in.\n\n"
                "Two days later, the evidence hits every news station simultaneously.\n"
                "The arrests start within hours. Morrison, Castellano, Blackwood.\n"
                "The whole rotten system comes crashing down."
            ),
            dialogue=[
                ("NEWS ANCHOR", "In the biggest corruption scandal in city history..."),
                ("RODRIGUEZ", "Good work, Jack. You're cleared of all charges."),
                ("JACK MALONE", "Just doing my job, Captain."),
            ],
            choices=[
                Choice("The End", "ending_evidence_victory"),
            ]
        )
        
        self.scenes["morrison_confrontation"] = Scene(
            id="morrison_confrontation",
            description=(
                "You step out of the office as Morrison rounds the corner.\n"
                "His hand goes for his gun. Yours is already out."
            ),
            dialogue=[
                ("JACK MALONE", "Don't. I've got enough evidence here to bury you ten times over."),
                ("MORRISON", "You're a dead man, Malone. You just don't know it yet."),
                ("JACK MALONE", "Maybe. But I'm taking you with me."),
            ],
            choices=[
                Choice("Arrest Morrison at gunpoint", "arrest_morrison"),
                Choice("Shoot first, ask questions never", "shoot_morrison"),
            ]
        )
        
        self.scenes["escape_with_evidence"] = Scene(
            id="escape_with_evidence",
            description=(
                "You and Eddie vanish into the night with the ledger.\n"
                "The evidence is damning. Every dirty cop, every payoff, every crime.\n\n"
                "You deliver it anonymously to three journalists you trust.\n"
                "Within a week, the city is in chaos. In a month, it's clean.\n"
                "Well, cleaner."
            ),
            dialogue=[
                ("EDDIE", "We did good, Jack."),
                ("JACK MALONE", "We did what we had to. That's all."),
            ],
            choices=[
                Choice("The End", "ending_silent_victory"),
            ]
        )
        
        self.scenes["warehouse_skylight_attack"] = Scene(
            id="warehouse_skylight_attack",
            description=(
                "Glass shatters. You drop into hell.\n"
                "Bullets fly. You take down two guards before you hit the ground.\n\n"
                "Castellano draws. Morrison runs. Blackwood stands his ground,\n"
                "a strange weapon in his hands."
            ),
            dialogue=[
                ("BLACKWOOD", "Impressive, Mr. Malone. But foolish."),
            ],
            choices=[
                Choice("Dive for cover", "dive_cover"),
                Choice("Take the shot at Blackwood", "shoot_blackwood"),
            ]
        )
        
        self.scenes["warehouse_retreat"] = Scene(
            id="warehouse_retreat",
            description=(
                "You back away slowly. Sometimes living to fight another day is the only win.\n\n"
                "You take the information to the FBI. Special Agent Chen takes your statement.\n"
                "Three weeks later, coordinated raids bring down the entire operation.\n"
                "You testify. They go to prison. You go free."
            ),
            dialogue=[
                ("CHEN", "You made the smart choice, Malone. Not many do."),
                ("JACK MALONE", "I've been called a lot of things. Smart isn't usually one of them."),
            ],
            choices=[
                Choice("The End", "ending_smart_play"),
            ]
        )
        
        self.scenes["warehouse_negotiation"] = Scene(
            id="warehouse_negotiation",
            description=(
                "They let you in. Castellano, Morrison, and Blackwood sit at a table\n"
                "like business partners. Which, you suppose, they are."
            ),
            dialogue=[
                ("CASTELLANO", "Jack Malone. You've got balls, I'll give you that."),
                ("JACK MALONE", "I've got evidence. Documentation of Project Nightfall.\n                Every transaction, every weapons sale, every dirty cop.\n                I want a deal."),
                ("BLACKWOOD", "What kind of deal?"),
                ("JACK MALONE", "I walk away clean. You let me disappear. The evidence\n                disappears with me."),
            ],
            choices=[
                Choice("Wait for their response", "negotiation_response"),
            ]
        )
        
        self.scenes["warehouse_rush_guards"] = Scene(
            id="warehouse_rush_guards",
            description=(
                "You're fast. The first guard goes down with a broken jaw.\n"
                "The second pulls a gun. You're faster.\n\n"
                "Inside the warehouse, alarms blare. You've lost the element\n"
                "of surprise, but you're inside. Now it's just survival."
            ),
            dialogue=[
                ("JACK MALONE", "No turning back now."),
            ],
            choices=[
                Choice("Fight your way to the meeting room", "warehouse_combat"),
            ]
        )
        
        self.scenes["street_confrontation"] = Scene(
            id="street_confrontation",
            description=(
                "You step out of the shadows, gun drawn.\n"
                "'Going somewhere, Chief?'\n\n"
                "Morrison freezes. His hand hovers near his weapon."
            ),
            dialogue=[
                ("MORRISON", "Malone. You should be in custody by now."),
                ("JACK MALONE", "Should be a lot of things. But here I am. What's in the briefcase?"),
                ("MORRISON", "Insurance. Same as you, I imagine. We're not so different."),
            ],
            choices=[
                Choice("Take the briefcase", "take_briefcase"),
                Choice("Let Morrison go - follow the bigger fish", "let_morrison_go"),
            ]
        )
        
        self.scenes["follow_morrison"] = Scene(
            id="follow_morrison",
            description=(
                "Morrison leads you to a train station. He's leaving town.\n"
                "Smart man. But before he boards, he stops at a locker.\n"
                "Deposits the briefcase. Takes out another.\n\n"
                "After he leaves, you pick the lock. Inside: more evidence.\n"
                "And a note: 'For whoever comes looking - finish what I couldn't.'"
            ),
            dialogue=[
                ("JACK MALONE", "Morrison was trying to get out. They probably killed\n                him before the train reached the next station."),
            ],
            choices=[
                Choice("Use Morrison's evidence to end this", "morrison_evidence_ending"),
            ]
        )
        
        self.scenes["turn_self_in"] = Scene(
            id="turn_self_in",
            description=(
                "You call Rodriguez. Tell him everything.\n"
                "He brings you in, but he believes you. The FBI confirms the dead woman's identity.\n\n"
                "You spend six months in protective custody while they build the case.\n"
                "When it's over, you're cleared. Project Nightfall is exposed.\n"
                "But the woman in your office is still dead. And so is the FBI agent."
            ),
            dialogue=[
                ("JACK MALONE", "Two good people dead because I pulled a trigger.\n                Self defense doesn't make it hurt less."),
            ],
            choices=[
                Choice("The End", "ending_guilty_conscience"),
            ]
        )
        
        self.scenes["run_from_fbi"] = Scene(
            id="run_from_fbi",
            description=(
                "You run. Mexico first, then further south.\n"
                "The evidence goes to a journalist you trust.\n\n"
                "Project Nightfall gets exposed, but you can never go home.\n"
                "Some nights, you wonder if it was worth it.\n"
                "Most nights, you try not to think at all."
            ),
            dialogue=[
                ("JACK MALONE", "They say you can't run from your past.\n                But you can sure as hell try."),
            ],
            choices=[
                Choice("The End", "ending_exile"),
            ]
        )
        
        self.scenes["fbi_cooperation"] = Scene(
            id="fbi_cooperation",
            description=(
                "You trust Agent Chen. She brings in her team.\n"
                "The FBI raids Warehouse 47 with tactical precision.\n\n"
                "Blackwood resists. He's killed in the firefight.\n"
                "Castellano and Morrison are arrested. You testify.\n"
                "They go away for life. You get your badge back.\n"
                "Metaphorically speaking."
            ),
            dialogue=[
                ("CHEN", "You're a good man, Malone. In a bad business."),
                ("JACK MALONE", "Only business I know."),
            ],
            choices=[
                Choice("The End", "ending_fbi_victory"),
            ]
        )
        
        self.scenes["chen_partnership"] = Scene(
            id="chen_partnership",
            description=(
                "You and Chen approach Warehouse 47 together.\n"
                "Federal agent and private eye. Strange bedfellows.\n\n"
                "The raid is surgical. Chen's team takes the perimeter.\n"
                "You and Chen go inside. Blackwood surrenders when he sees\n"
                "the FBI badges. Castellano tries to run. Morrison eats his gun."
            ),
            dialogue=[
                ("CHEN", "It's over. You're cleared of all charges."),
                ("JACK MALONE", "The dame is still dead."),
                ("CHEN", "Yes. But her death wasn't for nothing. We got them all."),
            ],
            choices=[
                Choice("The End", "ending_partnership_success"),
            ]
        )
        
        self.scenes["rodriguez_raid"] = Scene(
            id="rodriguez_raid",
            description=(
                "The raid is by the book. Rodriguez's team hits the warehouse hard.\n"
                "Castellano surrenders immediately - he's a businessman, not a fighter.\n"
                "Morrison tries to shoot his way out. He doesn't make it.\n"
                "Blackwood triggers a dead man's switch. The warehouse explodes.\n\n"
                "But you all got out first. And the evidence survived."
            ),
            dialogue=[
                ("RODRIGUEZ", "Good work, Jack. The city owes you one."),
                ("JACK MALONE", "The city owes me about a hundred. But who's counting?"),
            ],
            choices=[
                Choice("The End", "ending_by_the_book"),
            ]
        )
        
        self.scenes["solo_warehouse_attack"] = Scene(
            id="solo_warehouse_attack",
            description=(
                "You walk into Warehouse 47 like it's your office.\n"
                "The guards raise their weapons. You raise yours.\n\n"
                "The firefight is short and brutal. When the smoke clears,\n"
                "you're still standing. Barely. Castellano's dead. Morrison's dead.\n"
                "Blackwood is gone - he slipped out during the chaos.\n\n"
                "You collapse. When you wake up, you're in a hospital.\n"
                "Rodriguez is sitting by your bed."
            ),
            dialogue=[
                ("RODRIGUEZ", "You crazy son of a bitch. You did it."),
                ("JACK MALONE", "Blackwood got away."),
                ("RODRIGUEZ", "FBI picked him up at the airport. It's over, Jack. You're clear."),
            ],
            choices=[
                Choice("The End", "ending_lone_wolf_victory"),
            ]
        )
        
        self.scenes["castellano_alliance"] = Scene(
            id="castellano_alliance",
            description=(
                "You shake hands with the devil.\n"
                "Castellano provides the location of Blackwood's headquarters.\n"
                "You provide the distraction.\n\n"
                "Together, you bring down Project Nightfall.\n"
                "Blackwood dies in the raid. Morrison goes to prison.\n"
                "Castellano walks free - there's no evidence linking him anymore.\n\n"
                "You wonder if you made the right choice.\n"
                "Then again, in this city, there are no right choices.\n"
                "Only choices you can live with."
            ),
            dialogue=[
                ("CASTELLANO", "Pleasure doing business with you, Malone."),
                ("JACK MALONE", "If we ever meet again, the deal's off."),
                ("CASTELLANO", "Understood. Let's hope we don't."),
            ],
            choices=[
                Choice("The End", "ending_deal_with_devil"),
            ]
        )
        
        self.scenes["refuse_castellano"] = Scene(
            id="refuse_castellano",
            description=(
                "'I don't work with killers.'\n"
                "'Pity,' Castellano says. He nods to his men.\n\n"
                "The last thing you see is the muzzle flash."
            ),
            dialogue=[
                ("CASTELLANO", "Shame. We could've done great things together."),
            ],
            choices=[
                Choice("...", "ending_died_with_honor"),
            ]
        )
        
        self.scenes["warehouse_wounded_entry"] = Scene(
            id="warehouse_wounded_entry",
            description=(
                "They take you inside. You're leaving a blood trail.\n"
                "Castellano looks at you with something approaching respect."
            ),
            dialogue=[
                ("CASTELLANO", "You got guts, Malone. I'll give you that much.\n                Talk. You got thirty seconds before I finish what Tony started."),
                ("JACK MALONE", "General Marcus Blackwood. He's using you. He killed the dame,\n                framed me, and when this goes south, you're next."),
                ("BLACKWOOD (emerging)", "That's enough, Mr. Malone."),
            ],
            choices=[
                Choice("'There he is. The man behind the curtain.'", "final_confrontation_wounded"),
            ]
        )
        
        self.scenes["velvet_room"] = Scene(
            id="velvet_room",
            description=(
                "The Velvet Room is all red velvet and dim lights.\n"
                "Jazz plays. Smoke curls. You flash your PI license at the hostess.\n"
                "'I need to see the owner. Tell him it's about the dame.'\n\n"
                "Five minutes later, you're in a back office.\n"
                "Castellano pours two drinks."
            ),
            dialogue=[
                ("CASTELLANO", "I heard about your problem, Jack. Bad business."),
                ("JACK MALONE", "Your business?"),
                ("CASTELLANO", "Not mine. Bigger fish. I'm just the middleman.\n                You want answers? Help me take down the real monster."),
            ],
            choices=[
                Choice("Hear him out", "castellano_reveal"),
                Choice("This is a trap - get out", "velvet_room_escape"),
            ]
        )
        
        self.scenes["shootout_tactical"] = Scene(
            id="shootout_tactical",
            description=(
                "You take cover behind steel crates. Eddie flanks right.\n"
                "The energy weapon tears through the warehouse.\n\n"
                "But you and Eddie are professionals. You wait for the reload.\n"
                "Then you move. Two shots. Blackwood goes down.\n"
                "The weapon skitters across the floor."
            ),
            dialogue=[
                ("EDDIE", "Clear!"),
                ("JACK MALONE", "Call it in. We're done here."),
            ],
            choices=[
                Choice("The End", "ending_tactical_victory"),
            ]
        )
        
        self.scenes["charge_blackwood"] = Scene(
            id="charge_blackwood",
            description=(
                "You charge. The energy beam catches you in the shoulder.\n"
                "Pain like you've never felt. But you keep moving.\n\n"
                "Three steps. Two. One.\n"
                "You crash into Blackwood. Your gun against his chest.\n"
                "One shot. It's over."
            ),
            dialogue=[
                ("EDDIE", "Jack! You hit?"),
                ("JACK MALONE", "I'll live. Probably. Is he dead?"),
                ("EDDIE", "Very."),
            ],
            choices=[
                Choice("The End", "ending_heroic_victory"),
            ]
        )
        
        self.scenes["arrest_morrison"] = Scene(
            id="arrest_morrison",
            description=(
                "You hold Morrison at gunpoint while Eddie calls Rodriguez.\n"
                "The good cops come. Morrison goes quietly in the end.\n\n"
                "With his testimony, the whole operation falls apart.\n"
                "Project Nightfall ends not with a bang, but with handcuffs."
            ),
            dialogue=[
                ("RODRIGUEZ", "Good work, Jack. Both of you."),
            ],
            choices=[
                Choice("The End", "ending_justice_served"),
            ]
        )
        
        self.scenes["shoot_morrison"] = Scene(
            id="shoot_morrison",
            description=(
                "You pull the trigger. Morrison drops.\n"
                "Eddie stares at you."
            ),
            dialogue=[
                ("EDDIE", "Jack... he was surrendering..."),
                ("JACK MALONE", "He killed that dame. He killed dozens. He was never\n                going to see the inside of a prison. You know it.\n                I know it."),
                ("EDDIE", "Maybe. But that was murder."),
            ],
            choices=[
                Choice("The End", "ending_dark_justice"),
            ]
        )
        
        self.scenes["dive_cover"] = Scene(
            id="dive_cover",
            description=(
                "The energy beam passes over your head. You roll, fire twice.\n"
                "Both shots hit. Blackwood staggers, falls.\n\n"
                "Castellano tries to run. You shoot him in the leg.\n"
                "When the police arrive, you're sitting on a crate,\n"
                "smoking a cigarette, surrounded by bodies."
            ),
            dialogue=[
                ("JACK MALONE", "Sometimes you get lucky. Sometimes you get even.\n                Tonight, I got both."),
            ],
            choices=[
                Choice("The End", "ending_survivor"),
            ]
        )
        
        self.scenes["shoot_blackwood"] = Scene(
            id="shoot_blackwood",
            description=(
                "You take the shot. The bullet catches Blackwood in the throat.\n"
                "He goes down. The energy weapon powers down.\n\n"
                "Castellano raises his hands. Morrison is already running.\n"
                "You let him. He'll be dead by morning anyway.\n"
                "Men like that always are."
            ),
            dialogue=[
                ("JACK MALONE", "It's over."),
            ],
            choices=[
                Choice("The End", "ending_quick_draw"),
            ]
        )
        
        self.scenes["negotiation_response"] = Scene(
            id="negotiation_response",
            description=(
                "Blackwood laughs. It's not a pleasant sound.\n"
                "'Mr. Malone, we both know you don't have any copies.\n"
                "If you did, you would have used them already.\n"
                "You're a good detective, but a terrible liar.'\n\n"
                "He pulls out a weapon. Not a gun. Something worse.\n"
                "'Goodbye, Mr. Malone.'"
            ),
            dialogue=[
                ("JACK MALONE", "Worth a shot..."),
            ],
            choices=[
                Choice("...", "ending_bad_bluff"),
            ]
        )
        
        self.scenes["warehouse_combat"] = Scene(
            id="warehouse_combat",
            description=(
                "You fight through the warehouse like a one-man army.\n"
                "Guards fall. You take bullets but keep moving.\n\n"
                "By the time you reach the meeting room, you're running on\n"
                "adrenaline and fury. Blackwood, Castellano, and Morrison\n"
                "turn to face you. Too late."
            ),
            dialogue=[
                ("JACK MALONE", "It ends tonight."),
            ],
            choices=[
                Choice("Finish it", "warehouse_final_showdown"),
            ]
        )
        
        self.scenes["take_briefcase"] = Scene(
            id="take_briefcase",
            description=(
                "You take the briefcase. Morrison doesn't resist.\n"
                "Inside: enough evidence to bury everyone involved.\n\n"
                "'Why?' you ask.\n"
                "'Because I'm tired,' Morrison says. 'Tired of the lies.\n"
                "Tired of the blood. Use it. End this.'\n\n"
                "He walks away into the rain. You never see him again."
            ),
            dialogue=[
                ("JACK MALONE", "Even bad men get tired of being bad. Sometimes."),
            ],
            choices=[
                Choice("Use the evidence", "morrison_evidence_ending"),
            ]
        )
        
        self.scenes["let_morrison_go"] = Scene(
            id="let_morrison_go",
            description=(
                "You let Morrison go. He's not the real villain here.\n"
                "Just another piece on the board.\n\n"
                "You tail him back to Warehouse 47. Now you know where everyone is.\n"
                "Now you can end it."
            ),
            choices=[
                Choice("Attack the warehouse", "solo_warehouse_attack"),
            ]
        )
        
        self.scenes["morrison_evidence_ending"] = Scene(
            id="morrison_evidence_ending",
            description=(
                "Morrison's evidence is devastating. You leak it to every news outlet.\n"
                "Within 48 hours, the arrests begin. Blackwood. Castellano. Dozens of dirty cops.\n\n"
                "You're cleared of all charges. The dame's death is solved.\n"
                "Morrison is found dead a week later. Suicide, they say.\n"
                "You don't believe it. But you let it go."
            ),
            dialogue=[
                ("JACK MALONE", "Justice isn't clean. But it's better than nothing."),
            ],
            choices=[
                Choice("The End", "ending_evidence_endgame"),
            ]
        )
        
        self.scenes["final_confrontation_wounded"] = Scene(
            id="final_confrontation_wounded",
            description=(
                "Blackwood draws his weapon. You draw yours.\n"
                "For a moment, everything is still.\n\n"
                "Then Castellano draws too. Not at you. At Blackwood.\n"
                "'Nobody frames me,' Castellano says. 'Nobody.'\n\n"
                "The warehouse erupts in gunfire. When it's over,\n"
                "Blackwood is dead. Castellano is dead. You're barely alive.\n"
                "But you're alive."
            ),
            dialogue=[
                ("JACK MALONE", "Sometimes the bad guys kill each other.\n                Sometimes you get lucky."),
            ],
            choices=[
                Choice("The End", "ending_pyrrhic_victory"),
            ]
        )
        
        self.scenes["velvet_room_escape"] = Scene(
            id="velvet_room_escape",
            description=(
                "You back toward the door. Castellano doesn't stop you.\n"
                "'You'll be back,' he says. 'They always come back.'\n\n"
                "He's wrong. You go to the FBI. Give them everything.\n"
                "Three months later, it's over. Castellano, Blackwood, Morrison.\n"
                "All behind bars. You're free."
            ),
            dialogue=[
                ("JACK MALONE", "Sometimes the smart play is walking away.\n                Then hitting them from behind."),
            ],
            choices=[
                Choice("The End", "ending_smart_escape"),
            ]
        )
        
        self.scenes["warehouse_final_showdown"] = Scene(
            id="warehouse_final_showdown",
            description=(
                "The fight is quick and brutal. You're wounded, exhausted, running on empty.\n"
                "But you're also right. And that counts for something.\n\n"
                "When the smoke clears, Blackwood and Morrison are dead.\n"
                "Castellano lives, barely. He'll talk. He'll deal.\n"
                "And you'll finally get the truth."
            ),
            dialogue=[
                ("JACK MALONE", "It's over. Finally."),
            ],
            choices=[
                Choice("The End", "ending_final_showdown_victory"),
            ]
        )
        
        self.scenes["ending_killed_by_castellano"] = Scene(
            id="ending_killed_by_castellano",
            description="You died trying to fight the mob. Brave, but stupid.",
            is_ending=True
        )
        
        self.scenes["ending_evidence_victory"] = Scene(
            id="ending_evidence_victory",
            description=(
                "You brought down the conspiracy with evidence and patience.\n"
                "The city is cleaner. Your name is cleared. The dame's death is avenged.\n\n"
                "ENDING: THE EVIDENCE SPEAKS"
            ),
            is_ending=True
        )
        
        self.scenes["ending_silent_victory"] = Scene(
            id="ending_silent_victory",
            description=(
                "You operated from the shadows and brought down the corrupt system.\n"
                "No glory, no recognition. Just justice.\n\n"
                "ENDING: THE SILENT GUARDIAN"
            ),
            is_ending=True
        )
        
        self.scenes["ending_smart_play"] = Scene(
            id="ending_smart_play",
            description=(
                "You made the smart choice and lived to tell about it.\n"
                "Sometimes retreat is victory.\n\n"
                "ENDING: WISDOM OVER VALOR"
            ),
            is_ending=True
        )
        
        self.scenes["ending_guilty_conscience"] = Scene(
            id="ending_guilty_conscience",
            description=(
                "You're cleared of all charges, but the weight of the deaths stays with you.\n"
                "Justice was served, but at what cost?\n\n"
                "ENDING: THE BURDEN OF TRUTH"
            ),
            is_ending=True
        )
        
        self.scenes["ending_exile"] = Scene(
            id="ending_exile",
            description=(
                "You exposed the conspiracy but lost everything in the process.\n"
                "Freedom has a price.\n\n"
                "ENDING: THE PRICE OF JUSTICE"
            ),
            is_ending=True
        )
        
        self.scenes["ending_fbi_victory"] = Scene(
            id="ending_fbi_victory",
            description=(
                "By working with the FBI, you brought down Project Nightfall officially.\n"
                "Justice served, badge and all.\n\n"
                "ENDING: BY THE BOOK"
            ),
            is_ending=True
        )
        
        self.scenes["ending_partnership_success"] = Scene(
            id="ending_partnership_success",
            description=(
                "You and Agent Chen made a great team. The conspiracy is destroyed,\n"
                "and you both walk away clean.\n\n"
                "ENDING: UNLIKELY ALLIES"
            ),
            is_ending=True
        )
        
        self.scenes["ending_by_the_book"] = Scene(
            id="ending_by_the_book",
            description=(
                "Rodriguez led a clean operation. Everything by the book.\n"
                "The evidence is solid. Justice is served.\n\n"
                "ENDING: PROPER PROCEDURE"
            ),
            is_ending=True
        )
        
        self.scenes["ending_lone_wolf_victory"] = Scene(
            id="ending_lone_wolf_victory",
            description=(
                "You took on the entire conspiracy alone and won.\n"
                "Battered, bleeding, but victorious.\n\n"
                "ENDING: THE LONE WOLF"
            ),
            is_ending=True
        )
        
        self.scenes["ending_deal_with_devil"] = Scene(
            id="ending_deal_with_devil",
            description=(
                "You made a deal with a monster to catch a bigger monster.\n"
                "The conspiracy is destroyed, but at what moral cost?\n\n"
                "ENDING: DEAL WITH THE DEVIL"
            ),
            is_ending=True
        )
        
        self.scenes["ending_died_with_honor"] = Scene(
            id="ending_died_with_honor",
            description=(
                "You refused to compromise your principles, even at the cost of your life.\n"
                "Some would call it stupid. Others would call it heroic.\n\n"
                "ENDING: DIED WITH HONOR"
            ),
            is_ending=True
        )
        
        self.scenes["ending_tactical_victory"] = Scene(
            id="ending_tactical_victory",
            description=(
                "You and Eddie executed a perfect tactical operation.\n"
                "Precision, skill, and teamwork brought down the conspiracy.\n\n"
                "ENDING: TACTICAL PERFECTION"
            ),
            is_ending=True
        )
        
        self.scenes["ending_heroic_victory"] = Scene(
            id="ending_heroic_victory",
            description=(
                "You charged into danger and emerged victorious despite your wounds.\n"
                "A hero's ending to a dark story.\n\n"
                "ENDING: HEROIC SACRIFICE"
            ),
            is_ending=True
        )
        
        self.scenes["ending_justice_served"] = Scene(
            id="ending_justice_served",
            description=(
                "By arresting Morrison and securing his testimony, you brought down\n"
                "the entire operation legally and properly.\n\n"
                "ENDING: JUSTICE SERVED"
            ),
            is_ending=True
        )
        
        self.scenes["ending_dark_justice"] = Scene(
            id="ending_dark_justice",
            description=(
                "You executed Morrison rather than let him face trial.\n"
                "The conspiracy ends, but you've become what you fought against.\n\n"
                "ENDING: DARK JUSTICE"
            ),
            is_ending=True
        )
        
        self.scenes["ending_survivor"] = Scene(
            id="ending_survivor",
            description=(
                "Through skill and luck, you survived impossible odds.\n"
                "The conspiracy is destroyed. You live to fight another day.\n\n"
                "ENDING: THE SURVIVOR"
            ),
            is_ending=True
        )
        
        self.scenes["ending_quick_draw"] = Scene(
            id="ending_quick_draw",
            description=(
                "One perfect shot ended it all. Clean, quick, effective.\n"
                "Just like you.\n\n"
                "ENDING: THE QUICK DRAW"
            ),
            is_ending=True
        )
        
        self.scenes["ending_bad_bluff"] = Scene(
            id="ending_bad_bluff",
            description=(
                "Your bluff failed. Blackwood called it.\n"
                "You died trying to negotiate with monsters.\n\n"
                "ENDING: BAD BLUFF"
            ),
            is_ending=True
        )
        
        self.scenes["ending_evidence_endgame"] = Scene(
            id="ending_evidence_endgame",
            description=(
                "Morrison's evidence proved devastating to the conspiracy.\n"
                "Even bad men can do good things in the end.\n\n"
                "ENDING: THE EVIDENCE ENDGAME"
            ),
            is_ending=True
        )
        
        self.scenes["ending_pyrrhic_victory"] = Scene(
            id="ending_pyrrhic_victory",
            description=(
                "You survived by letting the villains destroy each other.\n"
                "Victory, but at great cost.\n\n"
                "ENDING: PYRRHIC VICTORY"
            ),
            is_ending=True
        )
        
        self.scenes["ending_smart_escape"] = Scene(
            id="ending_smart_escape",
            description=(
                "You walked away when it mattered, then struck from a position of strength.\n"
                "The conspiracy is destroyed through cleverness, not violence.\n\n"
                "ENDING: THE SMART ESCAPE"
            ),
            is_ending=True
        )
        
        self.scenes["ending_final_showdown_victory"] = Scene(
            id="ending_final_showdown_victory",
            description=(
                "Despite overwhelming odds, you stormed the warehouse and won.\n"
                "Wounded but victorious. The perfect ending to a noir tale.\n\n"
                "ENDING: THE FINAL SHOWDOWN"
            ),
            is_ending=True
        )
        
        self.scenes["leave_town"] = Scene(
            id="leave_town",
            description=(
                "You pack a bag. One suitcase, one gun, and all the cash you can carry.\n"
                "The bus station at 4 AM is full of people running from something.\n"
                "You're just one more.\n\n"
                "Mexico first. Then further south. The dame's death goes unsolved.\n"
                "The conspiracy continues. Blackwood, Castellano, Morrison - they all win.\n"
                "But you're alive. Sometimes that's the only victory that matters.\n\n"
                "In your pocket, the photograph burns like a brand.\n"
                "Every night, you see her face. Every night, you wonder if you made the right choice.\n\n"
                "ENDING: THE COWARD'S EXIT"
            ),
            is_ending=True
        )
