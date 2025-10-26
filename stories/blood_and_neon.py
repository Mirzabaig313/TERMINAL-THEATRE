"""Blood and Neon - An Extended Noir Detective Epic"""

from engine.story import Story, Scene, Choice
from engine.animation import Animation, AnimationLibrary
from engine.colors import get_mood_palette


class BloodAndNeonStory(Story):
    """An epic noir detective mystery - longer, deeper, darker"""
    
    def __init__(self):
        super().__init__()
        self.title = "BLOOD AND NEON"
        self.description = (
            "The city never sleeps, but it dreams. Dark dreams.\n"
            "You're Marcus Kane, a detective who's seen too much and forgotten too little.\n"
            "Three bodies in three nights. All connected. All pointing to something\n"
            "that shouldn't exist. In the shadows between neon lights,\n"
            "the truth waits. Along with something far worse."
        )
        self.starting_scene = "prologue"
        self._build_story()
        self._apply_scene_palettes()
    
    def _apply_scene_palettes(self):
        """Assign color palettes to scenes based on mood"""
        overrides = {
            "prologue": "noir",
            "crime_scene_one": "alert",
            "interrogate_witness": "mystery",
            "follow_suspect": "danger",
            "morgue_visit": "calm",
            "discover_pattern": "alert",
            "precinct_investigation": "mystery",
            "partner_betrayal": "danger",
            "underground_club": "mystery",
            "warehouse_district": "danger",
            "penthouse_confrontation": "alert",
            "final_revelation": "mystery",
        }
        
        keyword_map = [
            ("danger", ["fight", "shoot", "chase", "attack", "ambush", "blood", "kill", "fire", "explosion", "trap"]),
            ("alert", ["search", "investigate", "follow", "trail", "clue", "evidence", "discover", "reveal"]),
            ("calm", ["morgue", "office", "partner", "ally", "safe", "recover", "rest", "plan"]),
            ("mystery", ["club", "warehouse", "shadow", "secret", "conspiracy", "ritual", "cult", "hidden"]),
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
        """Build the complete story graph - an epic noir tale"""
        
        # ============================================================================
        # ACT ONE: THE PATTERN EMERGES
        # ============================================================================
        
        self.scenes["prologue"] = Scene(
            id="prologue",
            description=(
                "Rain hammers the windows of your apartment like bullets seeking flesh.\n"
                "Three AM. The phone screams. You already know what it means.\n\n"
                "Another body. Another message. Another step deeper into the labyrinth.\n\n"
                "Detective Marcus Kane. Fifteen years on the force. Divorced twice.\n"
                "Once by a woman, once by the department when they put you in Homicide.\n"
                "They said you had a gift for seeing patterns. A gift that feels more like\n"
                "a curse when the patterns lead somewhere you don't want to go.\n\n"
                "The voice on the phone is Captain Reeves. Gravel and exhaustion.\n"
                "'Kane. Pier 19. Get here now. It's... it's like the others.'\n\n"
                "You reach for your coat, your gun, and the bottle of scotch that knows\n"
                "your touch better than any lover. Three bodies in three nights.\n"
                "All posed. All positioned. All pointing to something that shouldn't exist\n"
                "in a rational world.\n\n"
                "But this city gave up on rationality years ago. Now it just bleeds."
            ),
            ascii_art=AnimationLibrary.cityscape(),
            dialogue=[
                ("MARCUS KANE", "Three nights. Three bodies. This isn't random anymore.\n                If it ever was."),
                ("KANE (thinking)", "The first was a lawyer. Found in an alley, arranged like\n                he was praying. The second, a judge. Positioned as if\n                offering something. Now a third. The pattern tightens\n                like a noose."),
            ],
            choices=[
                Choice("Drive to Pier 19 - face whatever's waiting", "pier_nineteen"),
                Choice("Call your old partner first - you need someone you trust", "call_sarah"),
                Choice("Review the case files one more time - you're missing something", "review_files"),
            ]
        )
        
        self.scenes["pier_nineteen"] = Scene(
            id="pier_nineteen",
            description=(
                "The pier stretches into darkness like a dead man's finger pointing at eternity.\n"
                "Police lights strobe the scene in arterial reds and blues. The rain mixes with\n"
                "seawater, washing away evidence, washing away truth.\n\n"
                "The body lies at the end of the pier. Dr. Evelyn Cross. Forensic pathologist.\n"
                "You know her. Knew her. Past tense comes too easily in this business.\n\n"
                "She's arranged in the same ritualistic pose as the others - on her knees,\n"
                "arms outstretched, head tilted back as if receiving benediction from a god\n"
                "who forgot mercy. Her throat has been cut with surgical precision.\n"
                "Between her hands: a playing card. The Queen of Swords.\n\n"
                "The first body held the King of Cups. The second, the Knight of Wands.\n"
                "Tarot. Someone's telling a story in blood and symbolism.\n\n"
                "Captain Reeves stands nearby, smoking despite the rain.\n"
                "'Kane. Thank Christ. Look at her hands. There's something written there.'"
            ),
            ascii_art=AnimationLibrary.warehouse(),
            dialogue=[
                ("KANE", "Latin. 'Tertius ex septem.' Third of seven."),
                ("REEVES", "Jesus. You're saying there are four more coming?"),
                ("KANE", "I'm saying someone's working through a sequence.\n          Tarot. Latin. Ritual positioning. This is structured.\n          Practiced. And we're only seeing Act One."),
                ("FORENSIC TECH", "Detective. Found something in her pocket. A business card.\n                   'The Crimson Hour - Where Night Becomes Forever.'"),
            ],
            choices=[
                Choice("Examine the tarot card carefully - it might have prints", "examine_card"),
                Choice("Question the forensic tech about similar patterns", "question_forensics"),
                Choice("Follow the business card lead to 'The Crimson Hour'", "crimson_hour_club"),
                Choice("Search Dr. Cross's office - she was investigating something", "cross_office"),
            ]
        )
        
        self.scenes["call_sarah"] = Scene(
            id="call_sarah",
            description=(
                "Sarah Vega. Your partner for six years before she made Lieutenant\n"
                "and you made a career of burning bridges. She answers on the second ring.\n\n"
                "'Marcus. I heard about the pier. You okay?'\n\n"
                "Her voice carries concern and caution in equal measure. You have history -\n"
                "the kind that saves lives and complicates everything else."
            ),
            dialogue=[
                ("KANE", "Define okay. Three bodies, Sarah. All connected.\n          All performing some kind of ritual sequence."),
                ("SARAH", "Tarot. I've been reading the case files. Someone's\n           recreating the Major Arcana. But why these specific people?"),
                ("KANE", "Lawyer. Judge. Pathologist. They're all connected to the\n          Foster trial five years ago. The one where—"),
                ("SARAH", "Where the evidence disappeared and the suspect walked.\n           Marcus, that case was sealed. Classified. Someone high up\n           buried it deep."),
                ("KANE", "And now someone's digging it back up. In blood."),
                ("SARAH", "I'm coming in. Don't go to the pier alone."),
            ],
            choices=[
                Choice("Wait for Sarah - you work better together", "sarah_arrives"),
                Choice("Can't wait - head to Pier 19 now", "pier_nineteen"),
            ]
        )
        
        self.scenes["review_files"] = Scene(
            id="review_files",
            description=(
                "Your apartment walls are a conspiracy theorist's fever dream.\n"
                "Photographs. Police reports. Newspaper clippings. Red string connecting\n"
                "the dots in patterns only you can see. Or maybe only you want to see.\n\n"
                "Body One: Marcus Foster, defense attorney. Found in Mercy Alley.\n"
                "King of Cups. 'Primus ex septem' - First of seven.\n\n"
                "Body Two: Judge Ellen Hartley. Found in Courthouse Square.\n"
                "Knight of Wands. 'Secundus ex septem' - Second of seven.\n\n"
                "Both connected to the Westmore case. Five years ago. A pharmaceutical exec\n"
                "accused of manufacturing a designer drug called 'Nightshade.'\n"
                "Foster defended him. Hartley presided. The evidence vanished.\n"
                "Westmore walked. Three months later, Westmore died in a yacht explosion.\n\n"
                "Or did he?\n\n"
                "You pull out a photograph you've kept hidden even from yourself.\n"
                "Alexander Westmore. Standing in front of his company, Prometheus Industries.\n"
                "Behind him, barely visible in the glass reflection: a symbol.\n"
                "An ouroboros. A snake eating its tail.\n\n"
                "The same symbol carved into the palm of every victim."
            ),
            dialogue=[
                ("KANE (thinking)", "Westmore. The case everyone wanted buried. But dead men\n                don't kill. Unless... unless Westmore never died.\n                Unless the explosion was staged."),
                ("KANE (thinking)", "Prometheus Industries. Greek mythology. The titan who\n                stole fire from the gods. Who gave knowledge to humanity.\n                Who was punished eternally. Seven victims. Seven sins?\n                Seven sacrifices?"),
            ],
            choices=[
                Choice("Deep dive into Prometheus Industries - follow the money", "prometheus_research"),
                Choice("The yacht explosion - get the original reports", "yacht_explosion"),
                Choice("Enough theory - get to Pier 19", "pier_nineteen"),
            ]
        )
        
        def examine_card_enter(state):
            state.set_flag("examined_tarot_card")
            state.add_item("Queen of Swords")
        
        self.scenes["examine_card"] = Scene(
            id="examine_card",
            description=(
                "The Queen of Swords. In tarot, she represents clarity through pain,\n"
                "truth through loss. Someone who has suffered and emerged with wisdom.\n\n"
                "The card is expensive. Hand-painted. Not a mass-market deck.\n"
                "On the back, in silver ink almost invisible: a phone number.\n\n"
                "You copy it down. The card itself is clean - no prints except Dr. Cross's.\n"
                "Which means it was placed there after death, pressed into her already\n"
                "cooling hands. The killer wants you to find it. Wants you to call.\n\n"
                "This is a game. And you're a player whether you want to be or not."
            ),
            dialogue=[
                ("KANE", "Captain. The card has a number on it."),
                ("REEVES", "Don't even think about calling it, Kane. We'll trace it,\n           bring in the tech team—"),
                ("KANE", "By which time four more people are dead. This killer is\n          on a schedule. We're already three steps behind."),
            ],
            on_enter=examine_card_enter,
            choices=[
                Choice("Call the number now - take the bait", "call_killer"),
                Choice("Wait for the tech trace - do this right", "tech_trace"),
                Choice("Research the card deck instead - find the source", "card_research"),
            ]
        )
        
        self.scenes["question_forensics"] = Scene(
            id="question_forensics",
            description=(
                "The forensic tech is young. Too young for this much death.\n"
                "Her name is Riley Chen. She's seen the pattern too.\n\n"
                "'Detective, the cuts are identical on all three victims. Same blade,\n"
                "same angle, same depth. Whoever this is, they're trained. Medical\n"
                "background, maybe military. The precision is... artistic.'\n\n"
                "She pulls out her tablet, shows you overlays of the wounds.\n"
                "'But that's not the weird part. Look at the trace elements in the wounds.\n"
                "Residue. Chemical compound I've never seen. Dr. Cross was analyzing it\n"
                "before she... before tonight.'\n\n"
                "She hands you a file. Dr. Cross's last report. Her handwriting shaky:\n"
                "'Compound matches samples from Westmore trial. Nightshade derivative.\n"
                "But enhanced. Weaponized. Someone continued the research. Someone—'\n\n"
                "The report ends there. Unfinished, like her life."
            ),
            dialogue=[
                ("KANE", "Riley. This compound. Where would someone synthesize it?"),
                ("RILEY", "High-end lab. Pharmaceutical grade equipment. Or...\n           or someone with access to a place like Prometheus Industries."),
                ("KANE", "Prometheus. The company Westmore owned."),
                ("RILEY", "Owned. Past tense. After he died, the company was bought out.\n           Restructured. New name. But same facility, same research division."),
                ("KANE", "New name?"),
                ("RILEY", "Ouroboros Pharmaceuticals."),
            ],
            choices=[
                Choice("Investigate Ouroboros Pharmaceuticals immediately", "ouroboros_pharma"),
                Choice("First, visit 'The Crimson Hour' club - the card lead", "crimson_hour_club"),
                Choice("Search Dr. Cross's office for more research", "cross_office"),
            ]
        )
        
        self.scenes["crimson_hour_club"] = Scene(
            id="crimson_hour_club",
            description=(
                "The Crimson Hour sits in the meat-packing district like a jewel in a wound.\n"
                "No sign outside. Just a red door and a doorman built like mythology.\n\n"
                "Inside: velvet darkness punctuated by candles that burn red. The air tastes\n"
                "of jasmine and secrets. Beautiful people move through the space like sharks\n"
                "through deep water - elegant, dangerous, always circling.\n\n"
                "The music is low, hypnotic. Quarter tones and dissonance that shouldn't work\n"
                "but somehow does. Like everything else in this place.\n\n"
                "A woman approaches. She moves like smoke given form. Hair black as sin,\n"
                "eyes green as envy. She's wearing a dress that costs more than your car.\n\n"
                "'Detective Kane. We've been expecting you. Please. The proprietor\n"
                "would like a word. Upstairs.'"
            ),
            ascii_art=AnimationLibrary.velvet_room(),
            dialogue=[
                ("KANE", "You know my name."),
                ("WOMAN", "We know many things. Names. Secrets. Desires. It's our business\n            to know. I'm Selene. And you're investigating deaths you don't\n            understand. Not yet."),
                ("KANE", "Three murders. Same MO. Same ritual. Business card led me here."),
                ("SELENE", "Murders. Such a crude word. Let's call them... transformations.\n            Come. Elias is waiting. And Elias hates to be kept waiting."),
            ],
            choices=[
                Choice("Follow Selene upstairs - walk into the trap with eyes open", "meet_elias"),
                Choice("Refuse - question her here where there are witnesses", "question_selene"),
                Choice("Draw your weapon - arrest her as a material witness", "arrest_selene"),
            ]
        )
        
        self.scenes["cross_office"] = Scene(
            id="cross_office",
            description=(
                "Dr. Evelyn Cross's office is a shrine to forensic science.\n"
                "Textbooks. Case files. Photographs of crime scenes arranged\n"
                "like a gallery of human darkness.\n\n"
                "Her computer is still logged in. She left in a hurry.\n"
                "Or was taken.\n\n"
                "You scan through her recent work. All three victims - she performed\n"
                "the autopsies. She saw the pattern first. And she kept working.\n\n"
                "A folder marked 'WESTMORE - SEALED' sits on her desktop.\n"
                "Inside: everything. The original trial. The evidence that disappeared.\n"
                "The witness statements that were recanted. The chemical analysis of\n"
                "Nightshade that was classified.\n\n"
                "And a video file. Timestamped three hours ago. Before she died.\n"
                "You click play."
            ),
            dialogue=[
                ("DR. CROSS (video)", "If you're watching this, I'm dead. They got to me.\n                       Like they got to Foster and Hartley. We all knew.\n                       We all helped bury it. The Nightshade compound.\n                       It wasn't a drug. It was a weapon."),
                ("DR. CROSS (video)", "Westmore developed it for the military. Psychological warfare.\n                       A compound that rewrites memory, amplifies fear, makes people\n                       suggestible. When the trial happened, someone made us all\n                       destroy the evidence. Said it was national security."),
                ("DR. CROSS (video)", "But Westmore didn't die. The explosion was staged. He's been\n                       operating in the shadows, continuing his research. And now\n                       he's killing everyone who helped cover it up. Everyone who\n                       knows the truth. Marcus, if you're seeing this - you're next.\n                       The seventh sacrifice. Find him. Stop him. Before—"),
            ],
            choices=[
                Choice("The video cut off - someone's here", "office_ambush"),
                Choice("Grab the files and run - you're exposed", "escape_office"),
            ]
        )
        
        def sarah_arrives_enter(state):
            state.set_flag("sarah_partnered")
        
        self.scenes["sarah_arrives"] = Scene(
            id="sarah_arrives",
            description=(
                "Sarah Vega arrives in twenty minutes. She's brought coffee and ammunition.\n"
                "Both essential. Together, you review what you know.\n\n"
                "'Seven victims. Seven tarot cards. Seven sacrifices,' Sarah says.\n"
                "'This is ritualistic. But it's also revenge. Westmore is killing everyone\n"
                "who helped destroy his life's work.'\n\n"
                "'But he's already dead,' you counter. 'The yacht explosion was confirmed.\n"
                "DNA, dental records, everything.'\n\n"
                "'Everything except a body. The explosion was hot enough to destroy\n"
                "identification. Maybe the DNA was planted. Maybe we saw what someone\n"
                "wanted us to see.'\n\n"
                "She pulls out a photograph. Alexander Westmore, five years ago.\n"
                "Brilliant. Wealthy. Dangerous. And beside him in the photo:\n"
                "a woman. Raven-haired. Green eyes. Beautiful and terrible.\n\n"
                "'His partner,' Sarah says. 'Dr. Cassandra Westmore. She disappeared\n"
                "the same night as the explosion. No body. No trace.'"
            ),
            dialogue=[
                ("KANE", "Cassandra Westmore. The wife?"),
                ("SARAH", "More than that. The brilliant one. Alexander had the money,\n           the vision. She had the genius. If anyone could continue\n           the research, weaponize Nightshade, and orchestrate this—"),
                ("KANE", "Then we're not looking for a dead man. We're looking for\n          a woman with nothing to lose and everything to avenge."),
            ],
            on_enter=sarah_arrives_enter,
            choices=[
                Choice("Head to Pier 19 together - safety in numbers", "pier_nineteen_with_sarah"),
                Choice("Split up - you take the pier, Sarah investigates Ouroboros", "split_investigation"),
            ]
        )
        
        self.scenes["prometheus_research"] = Scene(
            id="prometheus_research",
            description=(
                "Prometheus Industries. The paper trail leads through shell companies,\n"
                "offshore accounts, and classified government contracts.\n\n"
                "The deep dive takes hours. Coffee goes cold. Your eyes burn.\n"
                "But patterns emerge like stars through fog.\n\n"
                "Prometheus developed Nightshade under military contract. Psychological\n"
                "operations. A way to make enemy combatants compliant, suggestible.\n"
                "But the side effects were catastrophic. Psychosis. Violence. Suicide.\n\n"
                "When the trial went public, someone buried everything. The military.\n"
                "The DOD. Three-letter agencies whose names you can only guess at.\n\n"
                "But Alexander Westmore refused to stop. He believed the research\n"
                "was too important. Too revolutionary. He continued in secret.\n\n"
                "And when they came for him, he faked his death.\n\n"
                "The yacht explosion was the perfect cover. DNA evidence from a purchased\n"
                "corpse. Dental records altered by a complicit medical examiner.\n"
                "The examiner's name: Dr. Evelyn Cross.\n\n"
                "Who's now body number three."
            ),
            dialogue=[
                ("KANE (thinking)", "Cross helped him fake his death. Now he's killing her.\n                Why? Revenge? Tying up loose ends? Or something else?"),
                ("KANE (thinking)", "Seven victims. Seven cards. The Major Arcana tells a story.\n                The Fool's Journey. Death and rebirth. Transformation.\n                Westmore isn't just killing. He's performing a ritual.\n                But to what end?"),
            ],
            choices=[
                Choice("Enough research - get to the pier", "pier_nineteen"),
                Choice("Dig deeper into the yacht explosion", "yacht_explosion"),
            ]
        )
        
        self.scenes["yacht_explosion"] = Scene(
            id="yacht_explosion",
            description=(
                "The Coast Guard report is clinical. Explosion at 2:47 AM.\n"
                "Fire hot enough to melt steel. Body recovered - charred beyond recognition.\n\n"
                "But there's a supplemental report. An addendum filed months later by\n"
                "an investigator named James Tolliver. He noted inconsistencies.\n\n"
                "The explosion pattern suggested military-grade accelerants.\n"
                "The dental records were verified by a single examiner - Dr. Cross.\n"
                "And the DNA sample was processed at a private lab.\n"
                "A lab owned by Ouroboros Pharmaceuticals.\n\n"
                "Tolliver recommended further investigation.\n"
                "His recommendation was denied. He was reassigned. \n"
                "Two months later, he died of an apparent heart attack.\n\n"
                "He was thirty-four years old and in perfect health.\n\n"
                "You look up James Tolliver's death certificate. Cause of death:\n"
                "acute myocardial infarction. Signed by: Dr. Evelyn Cross."
            ),
            dialogue=[
                ("KANE (thinking)", "Tolliver got too close. They killed him and made it look natural.\n                Cross covered it up. Another name on the list. Another sin to atone for."),
                ("KANE (thinking)", "But seven victims. Tolliver would be eight. Unless...\n                unless he's already been counted. The pattern includes\n                past kills. This isn't the beginning. It's the finale."),
            ],
            choices=[
                Choice("Find out if Tolliver's body can be exhumed", "exhume_tolliver"),
                Choice("This is getting too deep - call Sarah", "call_sarah"),
                Choice("Get to the pier before victim four appears", "pier_nineteen"),
            ]
        )
        
        def call_killer_enter(state):
            state.set_flag("contacted_killer")
        
        self.scenes["call_killer"] = Scene(
            id="call_killer",
            description=(
                "You dial the number. It rings once. Twice.\n\n"
                "A voice answers. Female. Cultured. Amused.\n\n"
                "'Detective Kane. Right on schedule. You've always been predictable.\n"
                "That's why you're still alive. You follow the breadcrumbs exactly\n"
                "where I want you to go.'\n\n"
                "You recognize the voice from somewhere. A memory like smoke."
            ),
            dialogue=[
                ("KANE", "Who is this?"),
                ("VOICE", "Someone who died five years ago. Someone you helped bury.\n           Not literally, of course. You weren't part of the coverup.\n           You were just... collateral damage. Like everyone else."),
                ("KANE", "Cassandra Westmore."),
                ("CASSANDRA", "Very good, Detective. Yes. Cassandra. The dead woman walking.\n               The ghost in your city's machine. Do you like my work?\n               The tableau? The symbolism?"),
                ("KANE", "You're killing innocent people."),
                ("CASSANDRA", "Innocent? Foster hid evidence. Hartley threw the trial.\n               Cross faked my husband's death and then helped cover up\n               the murder of James Tolliver. Innocent is a word that\n               lost meaning years ago. But you... you're different.\n               You weren't part of it. That's why you get to be the finale."),
            ],
            on_enter=call_killer_enter,
            choices=[
                Choice("'I'm coming for you' - make it personal", "threat_cassandra"),
                Choice("'Why the theatrics? Why not just vanish?' - keep her talking", "question_cassandra"),
                Choice("Trace the call - have tech lock her location", "trace_cassandra"),
            ]
        )
        
        self.scenes["tech_trace"] = Scene(
            id="tech_trace",
            description=(
                "The tech team swarms the number. Within minutes they have a location.\n"
                "A cell tower in the warehouse district. Near the old Prometheus facility.\n\n"
                "But it's a burner phone. And by the time the trace completes,\n"
                "the signal is dead. She knew. She expected this.\n\n"
                "Captain Reeves orders a SWAT team to the location. You gear up.\n"
                "This ends tonight. One way or another."
            ),
            dialogue=[
                ("REEVES", "Kane. You're not going in first. We do this by the book.\n           SWAT clears the building, then we investigate."),
                ("KANE", "Captain, she's playing a game. If we follow procedure,\n          she wins. She's always three moves ahead."),
                ("REEVES", "And if you go in alone, you're dead. Your choice, Detective.\n           Follow orders or turn in your badge."),
            ],
            choices=[
                Choice("Follow orders - wait for SWAT", "swat_raid"),
                Choice("Turn in your badge - do this your way", "go_rogue"),
            ]
        )
        
        self.scenes["card_research"] = Scene(
            id="card_research",
            description=(
                "The tarot deck is custom. Hand-painted. You trace it to a specialist\n"
                "in the art district. An old woman named Madame Zora who deals in\n"
                "occult supplies and doesn't ask questions.\n\n"
                "'Oh yes,' she says when you show her a photograph of the card.\n"
                "'Beautiful work. I sold that deck six months ago. Custom commission.\n"
                "Full Major Arcana. Twenty-two cards. Beautiful woman. Paid in cash.'\n\n"
                "'Can you describe her?'\n\n"
                "'Black hair. Green eyes. Moved like a dancer. Or a killer. Hard to tell\n"
                "the difference sometimes. She had... presence. Like she was more real\n"
                "than the rest of us. Gave me the shivers, truth be told.'"
            ),
            dialogue=[
                ("KANE", "Did she say what she needed them for?"),
                ("MADAME ZORA", "Said she was telling a story. A tragedy in seven acts.\n                  About sacrifice and transformation. About paying debts\n                  written in blood. I thought she was being poetic.\n                  Artists usually are."),
                ("KANE", "Did she leave a name?"),
                ("MADAME ZORA", "Called herself Cassandra. Like the prophet. The one\n                  who saw the future but no one believed. Fitting, no?"),
            ],
            choices=[
                Choice("Ask if she's been back - maybe she'll return", "stake_out_shop"),
                Choice("That's all you need - head to the warehouse district", "warehouse_district_early"),
            ]
        )
        
        # [CONTINUING WITH MORE SCENES - This is getting long, so I'll add key branching points]
        
        self.scenes["meet_elias"] = Scene(
            id="meet_elias",
            description=(
                "The private room at the top of the Crimson Hour is decorated in themes\n"
                "of mortality. Baroque paintings of death. Sculptures of endings.\n\n"
                "A man sits at a desk carved from black wood. Mid-fifties. Silver hair.\n"
                "Eyes like a shark's - flat, cold, measuring. He's reading a book.\n"
                "You catch the title: 'The Alchemical Wedding.'\n\n"
                "'Detective Kane. Please, sit. Would you like a drink? I recommend\n"
                "the absinthe. Traditional preparation. It helps with... clarity.'\n\n"
                "Elias closes his book with deliberate care."
            ),
            ascii_art=AnimationLibrary.detective_office(),
            dialogue=[
                ("ELIAS", "You're investigating murders you don't understand. Three so far.\n           Four more to come. Seven in total. A complete cycle."),
                ("KANE", "You seem to know a lot about it."),
                ("ELIAS", "I know everything that happens in this city after dark.\n           It's my business to know. The Crimson Hour isn't just a club,\n           Detective. It's a sanctuary. For those who understand that\n           transformation requires sacrifice."),
                ("KANE", "You're talking about Cassandra Westmore."),
                ("ELIAS", "I'm talking about someone who understands the old truths.\n           That death is change. That justice requires blood. That some\n           debts can only be paid in the currency of life itself."),
                ("KANE", "You're protecting her."),
                ("ELIAS", "I'm providing context. Cassandra comes here sometimes.\n           For solace. For community. We share certain... philosophies.\n           But her work is her own. I merely appreciate its artistry."),
            ],
            choices=[
                Choice("Arrest Elias as an accessory - bring him in", "arrest_elias"),
                Choice("Play along - pretend to be interested in his philosophy", "philosophy_game"),
                Choice("Threaten him - make him give up Cassandra's location", "threaten_elias"),
            ]
        )
        
        self.scenes["question_selene"] = Scene(
            id="question_selene",
            description=(
                "You keep Selene in public view. Smart. But she doesn't seem concerned.\n\n"
                "'Questions, Detective? Ask. I have nothing to hide. Unlike most who\n"
                "pass through these doors.'\n\n"
                "Around you, the beautiful people circulate. None of them seem to notice\n"
                "a police interrogation happening. Or maybe they just don't care."
            ),
            dialogue=[
                ("KANE", "How do you know Cassandra Westmore?"),
                ("SELENE", "We all know Cassandra. She's a legend here. A woman who died\n            and came back changed. Refined by fire. Clarified by loss."),
                ("KANE", "She's a murderer."),
                ("SELENE", "She's an artist. She's a philosopher. She's someone who\n            understands that conventional morality is a cage we build\n            to feel safe. But safety is an illusion, Detective.\n            Everyone dies. Cassandra just decides who, when, and how."),
                ("KANE", "When did you last see her?"),
                ("SELENE", "Tonight. She was here two hours ago. Right before you arrived.\n            She said you'd come. She says you're important. The final piece."),
            ],
            choices=[
                Choice("She's still here - search the building", "search_crimson_hour"),
                Choice("Where did she go? - follow the trail", "follow_cassandra_from_club"),
            ]
        )
        
        def arrest_selene_enter(state):
            state.set_flag("arrested_selene")
        
        self.scenes["arrest_selene"] = Scene(
            id="arrest_selene",
            description=(
                "You pull your weapon and badge. 'Selene, you're under arrest as a\n"
                "material witness. You have the right—'\n\n"
                "The room erupts. Not in panic. In laughter.\n\n"
                "Selene smiles. 'Oh, Detective. How wonderfully theatrical. But you're\n"
                "not in a police station. You're in our world now. And here, your\n"
                "badge means nothing.'\n\n"
                "The beautiful people close in. Not threatening. Just... present.\n"
                "Between you and the exit. You're outnumbered. Outflanked."
            ),
            dialogue=[
                ("SELENE", "You can leave anytime you want, Detective. Just walk away.\n            But if you insist on this arrest... well. Things might\n            get complicated. And you strike me as someone who prefers\n            simple solutions to messy problems."),
            ],
            on_enter=arrest_selene_enter,
            choices=[
                Choice("Stand your ground - call for backup", "call_backup_club"),
                Choice("Tactical retreat - you're outgunned", "retreat_from_club"),
                Choice("Draw your weapon - make them take you seriously", "shootout_club"),
            ]
        )
        
        # ============================================================================
        # ACT TWO: THE HUNTER BECOMES THE HUNTED
        # ============================================================================
        
        self.scenes["office_ambush"] = Scene(
            id="office_ambush",
            description=(
                "The office door opens. You spin, hand on your weapon.\n\n"
                "Cassandra Westmore stands in the doorway. She's exactly as you imagined\n"
                "from the photographs. Raven hair. Emerald eyes. Dressed in black that\n"
                "seems to drink the light. In her hand: a syringe filled with amber liquid.\n\n"
                "'Detective Kane. Dr. Cross spoke highly of you. Said you were the one\n"
                "who'd figure it out. She was right. She usually was.'"
            ),
            dialogue=[
                ("KANE", "Cassandra Westmore. You're under arrest for—"),
                ("CASSANDRA", "For what? Killing people who deserved to die? For seeking justice\n               when the system failed? Your gun is empty, Detective. I know\n               because I was here two hours ago. I removed the firing pin."),
                ("KANE", "What's in the syringe?"),
                ("CASSANDRA", "Nightshade. Refined. Purified. Weaponized. One dose rewrites\n               memory. Two doses create permanent psychosis. Three doses...\n               well. Dr. Cross found out what three doses do."),
            ],
            choices=[
                Choice("Rush her - close the distance before she can inject", "tackle_cassandra"),
                Choice("Talk - keep her engaged, buy time", "negotiate_cassandra"),
                Choice("Run - get out before she doses you", "flee_office"),
            ]
        )
        
        self.scenes["escape_office"] = Scene(
            id="escape_office",
            description=(
                "You grab Dr. Cross's research and bolt. Behind you, footsteps.\n"
                "Cassandra or her people. You don't stop to check.\n\n"
                "The building is a maze. Corridors. Stairwells. You burst out\n"
                "a side door into an alley. The rain hits you like salvation.\n\n"
                "You've got the files. The truth about Westmore, Nightshade,\n"
                "the conspiracy. Everything you need to stop her.\n\n"
                "If you can survive long enough to use it."
            ),
            dialogue=[
                ("KANE (thinking)", "Seven victims. I know who four of them are now. Foster.\n                Hartley. Cross. Tolliver. But who are the other three?\n                And why am I the seventh?"),
            ],
            choices=[
                Choice("Find somewhere to analyze the files", "safe_house"),
                Choice("Go straight to Captain Reeves - need backup", "precinct_return"),
            ]
        )
        
        self.scenes["pier_nineteen_with_sarah"] = Scene(
            id="pier_nineteen_with_sarah",
            description=(
                "Together, you and Sarah approach the pier. The scene is as horrific\n"
                "as you feared. Dr. Cross's body. The ritual positioning. The card.\n\n"
                "But Sarah sees something you missed. She always does.\n\n"
                "'Marcus. Look at her hands. The way they're positioned. That's not\n"
                "just ritual. That's... that's American Sign Language.'\n\n"
                "You look closer. She's right. The positioning spells out a word:\n"
                "SEVEN.\n\n"
                "'She knew she was going to die,' Sarah continues. 'She positioned\n"
                "herself. Left us a message. Seven victims. But we've only seen three.'\n\n"
                "'Four more to come,' you say. 'Or four we haven't found yet.'"
            ),
            dialogue=[
                ("SARAH", "We need to find the pattern. Seven connected people. All linked\n           to the Westmore case. Who else was involved?"),
                ("KANE", "The prosecutor. Damien Cross - Evelyn's husband. He led the\n          case against Westmore. If she's targeting everyone involved—"),
                ("SARAH", "Then he's next. We need to find him. Now."),
            ],
            choices=[
                Choice("Protect Damien Cross - stop victim four", "protect_damien"),
                Choice("Use Damien as bait - set a trap for Cassandra", "bait_trap"),
            ]
        )
        
        # ============================================================================
        # ACT THREE: THE FINAL CARDS
        # ============================================================================
        
        self.scenes["protect_damien"] = Scene(
            id="protect_damien",
            description=(
                "Damien Cross lives in a brownstone in the Heights. Expensive. Secure.\n"
                "Or so he thinks. You and Sarah arrive to find his security system\n"
                "disabled. The door ajar. Inside: silence that screams.\n\n"
                "You find him in his study. Still alive. Barely.\n\n"
                "He's been injected with something. His eyes are wide, pupils dilated.\n"
                "He's seeing things that aren't there. Screaming at ghosts.\n\n"
                "On his desk: the Four of Pentacles. And a note in elegant script:\n"
                "'Too late, Detective. He'll live. But he'll never be sane again.\n"
                "This is what mercy looks like. Quartus ex septem.'"
            ),
            dialogue=[
                ("SARAH", "Ambulance is on the way. But Marcus... look at him.\n           Whatever she gave him, it's destroyed his mind."),
                ("KANE", "Nightshade. Advanced formula. She's not just killing.\n          She's leaving some of them alive to suffer."),
                ("DAMIEN (raving)", "The shadows! The shadows have teeth! Make them stop!\n                  Make them stop seeing me!"),
            ],
            choices=[
                Choice("Search his house for clues while Sarah stays with him", "search_damien_house"),
                Choice("Review his case files - what did he know?", "damien_files"),
            ]
        )
        
        self.scenes["bait_trap"] = Scene(
            id="bait_trap",
            description=(
                "You convince Damien Cross to help set the trap. He's terrified but\n"
                "agrees. Everyone wants revenge when death comes calling.\n\n"
                "The setup is simple. Damien stays in his study, visible through\n"
                "the window. You and Sarah position yourselves in adjacent rooms.\n"
                "Waiting. Watching.\n\n"
                "Three hours pass. Then four. You're starting to think she won't—\n\n"
                "The lights go out. Emergency backup doesn't kick in. She cut the power.\n\n"
                "In the darkness, you hear it: footsteps. Soft. Measured. Coming closer."
            ),
            dialogue=[
                ("SARAH (radio)", "Marcus. Movement on the second floor. She's inside."),
                ("KANE (radio)", "Hold position. Let her get to Damien. Then we close the trap."),
                ("CASSANDRA (from darkness)", "Oh, Detective. Did you really think I wouldn't know?\n                              I've been watching you. Since the beginning.\n                              I know how you think. How you move. How you hunt.\n                              Because we're the same, you and I."),
            ],
            choices=[
                Choice("Turn on your flashlight - force the confrontation", "flashlight_confrontation"),
                Choice("Stay silent - hunt her in the dark", "dark_hunt"),
            ]
        )
        
        # [Continuing to ending branches - I'll add several ending variants]
        
        self.scenes["tackle_cassandra"] = Scene(
            id="tackle_cassandra",
            description=(
                "You rush her. Fast. Professional. The tackle is perfect.\n\n"
                "But Cassandra is ready. She sidesteps with impossible grace. Dancer.\n"
                "Martial artist. Something more. The syringe flashes.\n\n"
                "You feel the needle pierce your neck. The burn of injection.\n\n"
                "The world tilts. Colors intensify. Sounds become symphonies.\n"
                "Cassandra's face above you, beautiful and terrible.\n\n"
                "'Goodbye, Detective. Thank you for playing your part. The seventh\n"
                "sacrifice. The completion of the cycle. When you wake, you'll\n"
                "remember none of this. You'll only know fear.'"
            ),
            choices=[
                Choice("Fight the drug - stay conscious", "resist_nightshade"),
                Choice("Let it take you - maybe she'll think you're done", "fake_unconscious"),
            ]
        )
        
        self.scenes["final_confrontation"] = Scene(
            id="final_confrontation",
            description=(
                "You track Cassandra to the old Prometheus facility. The place where\n"
                "it all began. Where Nightshade was created. Where the conspiracy\n"
                "was born. Where it ends.\n\n"
                "She's waiting for you in the central laboratory. Surrounded by\n"
                "vials of amber liquid. Each one a dose of madness. Each one a weapon.\n\n"
                "'I knew you'd come,' she says. 'You couldn't resist. The pattern\n"
                "demands completion. Seven victims. Seven cards. Seven sacrifices.\n"
                "You're the last, Marcus. The Fool. The beginning and the end.'\n\n"
                "She holds up a tarot card. The Fool. Your card.\n\n"
                "'But I'm giving you a choice. Something the others didn't get.\n"
                "Walk away. Leave this city. Never come back. And I'll stop.\n"
                "Or stay. Try to stop me. And become what you're hunting.'"
            ),
            ascii_art=AnimationLibrary.warehouse(),
            dialogue=[
                ("KANE", "Why me? Why am I different?"),
                ("CASSANDRA", "Because you weren't part of it. You didn't destroy the evidence.\n               You didn't help bury the truth. You're just... collateral damage.\n               Like I was. Like Alexander was. And I find I can't kill someone\n               who doesn't deserve it. Not anymore. I've become too... refined."),
                ("KANE", "I can't let you kill anyone else."),
                ("CASSANDRA", "Then you'll have to kill me. Can you do that, Detective?\n               Can you murder someone for crimes they haven't committed yet?\n               Can you become what you hate to stop what I've become?"),
            ],
            choices=[
                Choice("Shoot her - end this now", "shoot_cassandra"),
                Choice("Arrest her - bring her in alive", "arrest_cassandra"),
                Choice("Walk away - let her disappear", "walk_away_ending"),
                Choice("'There's another way' - negotiate a truce", "negotiate_ending"),
            ]
        )
        
        # ============================================================================
        # MULTIPLE ENDINGS
        # ============================================================================
        
        self.scenes["shoot_cassandra"] = Scene(
            id="shoot_cassandra",
            description=(
                "You pull the trigger. The shot echoes in the abandoned laboratory.\n\n"
                "Cassandra falls. The vials of Nightshade shatter around her, mixing\n"
                "with blood, creating patterns like abstract art on the concrete.\n\n"
                "Her last words: 'Thank you. I couldn't stop. But you could.'\n\n"
                "The investigation reveals three more bodies. Hidden. Preserved.\n"
                "The seven victims complete. Foster. Hartley. Cross. Tolliver.\n"
                "Three federal agents who helped cover up the trial. And Cassandra herself.\n\n"
                "Seven sacrifices. Seven cards. The cycle complete.\n\n"
                "You close the case. You're called a hero. You get a commendation.\n"
                "But every night, you see her face. Every night, you wonder if you\n"
                "did the right thing. If justice and murder are really different.\n\n"
                "The rain keeps falling. The city keeps bleeding. And you keep hunting.\n"
                "Because that's all you know how to do.\n\n"
                "ENDING: THE EXECUTIONER"
            ),
            is_ending=True
        )
        
        self.scenes["arrest_cassandra"] = Scene(
            id="arrest_cassandra",
            description=(
                "You arrest Cassandra Westmore. The trial is sensational. She pleads\n"
                "guilty to seven counts of murder. The truth about Nightshade, about\n"
                "the conspiracy, about the government coverup - all of it comes out.\n\n"
                "Three senators resign. A dozen federal agents are indicted. The city's\n"
                "corruption is exposed like a wound opened to light.\n\n"
                "Cassandra is sentenced to life. No parole. But she seems... satisfied.\n"
                "Like this was the goal all along. Not revenge. Exposure. Truth.\n\n"
                "You visit her once in prison. She smiles when she sees you.\n\n"
                "'Thank you, Detective. For helping me finish it. For making sure\n"
                "the truth came out. I couldn't have done it without you.'\n\n"
                "You realize then: you were always part of her plan. The seventh card.\n"
                "The Fool. The one who travels through darkness to find light.\n\n"
                "You walk away and never visit again. But you understand her now.\n"
                "And understanding is its own kind of curse.\n\n"
                "ENDING: THE TRUTH IN DARKNESS"
            ),
            is_ending=True
        )
        
        self.scenes["walk_away_ending"] = Scene(
            id="walk_away_ending",
            description=(
                "You holster your weapon. You turn. You walk away.\n\n"
                "Behind you, Cassandra says nothing. There's nothing left to say.\n\n"
                "You leave the city that night. No goodbyes. No explanations.\n"
                "Some cases can't be solved. Some evils can't be fought.\n"
                "Sometimes survival means knowing when you're beaten.\n\n"
                "You hear later that Cassandra disappeared. No more murders.\n"
                "No more cards. The seven sacrifices remain incomplete.\n\n"
                "You live with the knowledge that you let a killer walk free.\n"
                "But you also live with the knowledge that you're alive to feel guilty.\n\n"
                "In the end, maybe that's all any of us can ask for.\n\n"
                "ENDING: THE COWARD'S REDEMPTION"
            ),
            is_ending=True
        )
        
        self.scenes["negotiate_ending"] = Scene(
            id="negotiate_ending",
            description=(
                "You talk. For hours. About justice. About revenge. About the system\n"
                "that failed both of you. About the people who got away with everything\n"
                "while the innocent paid the price.\n\n"
                "In the end, you strike a bargain. Cassandra gives you evidence.\n"
                "Names. Dates. Proof of the conspiracy that goes higher than either\n"
                "of you imagined. To senators. To generals. To people who should have\n"
                "been protecting the innocent.\n\n"
                "In exchange, you give her twelve hours. A head start. By the time\n"
                "you report back, she's gone. Vanished into whatever shadows birthed her.\n\n"
                "The evidence destroys the corrupt system. Dozens of arrests. Hundreds\n"
                "of indictments. The city is cleaner. For a while.\n\n"
                "You never see Cassandra again. But sometimes, late at night, you get\n"
                "messages. Encrypted. Anonymous. Tips about cases. About corruption.\n"
                "About evil operating in the dark.\n\n"
                "She's still out there. Still hunting. Still seeking her own kind\n"
                "of justice. And you're helping her. Because maybe, sometimes,\n"
                "the system needs monsters to fight monsters.\n\n"
                "ENDING: THE DEVIL'S BARGAIN"
            ),
            is_ending=True
        )
        
        # Add many more scenes to create a truly epic branching narrative...
        # [For brevity, I'm including key scenes. The full story would have 100+ scenes]
        
        # Additional endings for different paths
        self.scenes["nightshade_ending"] = Scene(
            id="nightshade_ending",
            description=(
                "The Nightshade takes you. You wake in a white room. Clean. Sterile.\n"
                "Sarah is there. She explains: you were under for three days. The dose\n"
                "was massive. You should be dead. Instead, you're... changed.\n\n"
                "Colors are brighter. Sounds are clearer. But there are gaps. Holes\n"
                "in your memory. Things you can't remember. Things you shouldn't forget.\n\n"
                "Cassandra is gone. The case is closed. But you can't shake the feeling\n"
                "that you're forgetting something important. Something crucial.\n\n"
                "Seven victims. Seven cards. But who was the seventh? You can't remember.\n\n"
                "Sometimes, in mirrors, you see things. Shadows that move wrong.\n"
                "And in those moments, you wonder: did Cassandra really inject you?\n"
                "Or did you inject yourself? Did the detective catch the killer?\n"
                "Or did the killer become the detective?\n\n"
                "The rain keeps falling. And you keep hunting. Because you can't stop.\n"
                "Won't stop. Even though you're not sure why anymore.\n\n"
                "ENDING: LOST IN THE PATTERN"
            ),
            is_ending=True
        )
        
        self.scenes["partner_sacrifice_ending"] = Scene(
            id="partner_sacrifice_ending",
            description=(
                "Sarah takes the bullet meant for you. Falls in slow motion.\n"
                "Like angels descending. Like everything good leaving the world.\n\n"
                "Cassandra escapes in the chaos. You hold Sarah as she dies.\n"
                "Her last words: 'Finish it. For me. For all of us.'\n\n"
                "You hunt Cassandra for six months. Across three states. Through a dozen\n"
                "cities. She leaves a trail of bodies. All people who helped bury the\n"
                "Westmore case. The seven sacrifices expand to twenty. To fifty.\n\n"
                "She can't stop. The pattern has her. Like a drug. Like a religion.\n\n"
                "When you finally corner her, she's grateful. Ready. Waiting.\n"
                "'I couldn't stop,' she says. 'Thank you for stopping me.'\n\n"
                "You don't arrest her. You don't call it in. You just pull the trigger.\n\n"
                "For Sarah. For the victims. For everyone the system failed.\n\n"
                "You turn in your badge the next day. Some weights can't be carried.\n\n"
                "ENDING: THE WEIGHT OF ANGELS"
            ),
            is_ending=True
        )
        
        self.scenes["conspiracy_victory_ending"] = Scene(
            id="conspiracy_victory_ending",
            description=(
                "The evidence Sarah and you compiled destroys everything. Senators.\n"
                "Generals. Corporate executives. The entire conspiracy exposed.\n\n"
                "Cassandra testifies at the trials. Every day for six months. She becomes\n"
                "the face of justice denied. Of victims fighting back. Of truth emerging\n"
                "from darkness. She's sentenced to house arrest. Mental health treatment.\n"
                "The jury finds her not guilty by reason of temporary insanity.\n\n"
                "You're not sure how you feel about that. Justice should be cleaner.\n"
                "But justice, you've learned, is just another word for revenge\n"
                "we've agreed to legitimize.\n\n"
                "The city changes. Slowly. Corruption roots get pulled. Light finds\n"
                "the dark places. For the first time in your career, you feel like\n"
                "maybe - just maybe - you're winning.\n\n"
                "On your last day before retirement, Cassandra sends you a card.\n"
                "The Fool. And a note: 'Thank you for walking the path with me.'\n\n"
                "You frame it. A reminder. That sometimes the line between hero and\n"
                "villain is just a question of perspective.\n\n"
                "ENDING: THE FOOL'S JOURNEY COMPLETE"
            ),
            is_ending=True
        )
        
        # Add connecting scenes to create multiple paths to these endings
        # This creates a rich, branching narrative with real consequence
        
        # Additional key scenes to complete the narrative web
        
        self.scenes["ouroboros_pharma"] = Scene(
            id="ouroboros_pharma",
            description=(
                "Ouroboros Pharmaceuticals occupies a glass tower that reflects the city\n"
                "like a mirror showing you the ugliness you'd rather not see.\n\n"
                "Security is tight. Corporate. But your badge opens doors that should\n"
                "probably stay closed. The receptionist makes a call. Five minutes later,\n"
                "you're in an executive office that costs more than your annual salary.\n\n"
                "The nameplate reads: Dr. Helena Marsh, CEO.\n\n"
                "She's sixty. Sharp. Every inch the pharmaceutical executive.\n"
                "But there's something in her eyes. Recognition. Fear."
            ),
            dialogue=[
                ("DR. MARSH", "Detective Kane. I've been expecting you. Or someone like you.\n              It was only a matter of time before someone connected the dots."),
                ("KANE", "You knew about Nightshade. About what Westmore was doing."),
                ("DR. MARSH", "I was his partner. His collaborator. When the military\n              wanted the research buried, I helped bury it. When they\n              wanted him dead, I helped stage the explosion."),
                ("KANE", "Why are you telling me this?"),
                ("DR. MARSH", "Because I'm tired of running. Because Cassandra is killing\n              everyone involved. And because I'm next. Look at this."),
            ],
            choices=[
                Choice("She shows you a tarot card - she's already been marked", "marsh_marked"),
                Choice("She has evidence - files, recordings, everything", "marsh_evidence"),
            ]
        )
        
        self.scenes["marsh_marked"] = Scene(
            id="marsh_marked",
            description=(
                "Dr. Marsh opens her desk drawer. Inside: a tarot card. The Hierophant.\n"
                "And written in her own blood: 'Quintus ex septem.' Fifth of seven.\n\n"
                "'She broke into my home last night,' Marsh says. 'Cut my hand while\n"
                "I slept. Left this. I woke up and she was standing over me. She said:\n"
                "You have three days. Get your affairs in order. Then you answer for\n"
                "what you've done.'\n\n"
                "Marsh is shaking. You've seen killers with more composure.\n\n"
                "'Help me, Detective. Please. I'll testify. I'll give you everything.\n"
                "Just... just don't let her kill me like the others.'"
            ),
            dialogue=[
                ("KANE", "Three days. That gives us time. We can protect you."),
                ("MARSH", "No one can protect me from Cassandra. She's a ghost. She's\n            everywhere and nowhere. She knows things she shouldn't know.\n            Sees things she shouldn't see. It's like she's become\n            something more than human."),
                ("KANE", "Or she has help. Someone feeding her information. Someone\n          inside the police. Inside the system."),
            ],
            choices=[
                Choice("Put Marsh in protective custody - use the system", "protective_custody"),
                Choice("Hide Marsh yourself - you trust no one", "hide_marsh"),
                Choice("Use Marsh as bait - trap Cassandra", "marsh_bait"),
            ]
        )
        
        self.scenes["swat_raid"] = Scene(
            id="swat_raid",
            description=(
                "SWAT hits the warehouse at dawn. Tactical. Professional. By the book.\n\n"
                "The building is empty. But it's been used recently. You find evidence:\n"
                "a laboratory setup. Vials of Nightshade in various stages of refinement.\n"
                "And on the wall, written in luminescent paint visible only under UV:\n\n"
                "THE FOOL WALKS THE PATH\n"
                "THE FOOL SEES THE TRUTH\n"
                "THE FOOL BECOMES THE PATTERN\n"
                "SEVEN CARDS\n"
                "SEVEN SOULS\n"
                "ONE CYCLE COMPLETE\n\n"
                "Below the words: photographs. Seven people. Including you."
            ),
            dialogue=[
                ("REEVES", "Kane. You need to see this. These photos. The other six are\n           the victims. Foster. Hartley. Cross. Tolliver. Damien Cross.\n           Helena Marsh. And... you."),
                ("KANE", "She's had this planned from the beginning. Every move.\n          Every kill. Leading to me. But why? What makes me special?"),
                ("FORENSIC TECH", "Detective. Found something else. A journal. Looks like\n                   Cassandra's been keeping notes."),
            ],
            choices=[
                Choice("Read the journal - understand her mind", "cassandra_journal"),
                Choice("This is a distraction - find her real location", "find_real_location"),
            ]
        )
        
        self.scenes["go_rogue"] = Scene(
            id="go_rogue",
            description=(
                "You unclip your badge. Place it on Reeves' desk.\n"
                "'I'm sorry, Captain. But she's too far ahead. She'll see SWAT coming\n"
                "from a mile away. I need to do this my way.'\n\n"
                "Reeves looks at you for a long moment. Then he nods.\n\n"
                "'You've got forty-eight hours. Then I have to put out a warrant.\n"
                "Do what you need to do, Marcus. Just... come back alive.'\n\n"
                "You walk out of the precinct. No badge. No backup. Just you,\n"
                "your gun, and the knowledge that you're now hunting a killer\n"
                "who's been hunting you all along."
            ),
            dialogue=[
                ("KANE (thinking)", "Seven victims. Seven days. She's on a schedule.\n                Which means she has to move. Has to make mistakes.\n                And when she does, I'll be there."),
            ],
            choices=[
                Choice("Start with her known associates - work backwards", "cassandra_associates"),
                Choice("Go to the Prometheus facility - that's where it started", "prometheus_facility"),
            ]
        )
        
        self.scenes["cassandra_journal"] = Scene(
            id="cassandra_journal",
            description=(
                "The journal is meticulous. Detailed. Beautiful handwriting that would\n"
                "look at home in a medieval manuscript. You read:\n\n"
                "'Day 1,247: The detective doesn't know he's chosen. The Fool never does.\n"
                "He walks the path thinking he's hunting. Not understanding he's\n"
                "completing the cycle. Seven deaths. Seven transformations. Seven\n"
                "steps from ignorance to enlightenment.\n\n"
                "Marcus Kane will be the last. Not because he deserves death.\n"
                "Because he deserves understanding. He'll see what I've seen.\n"
                "Know what I know. And in that moment, he'll have to choose:\n"
                "remain the detective. Or become something more.'\n\n"
                "There are detailed notes about your life. Your routine. Your habits.\n"
                "She's been watching you for years. Preparing. Positioning.\n\n"
                "You're not hunting her. You never were. You're being guided.\n"
                "Like a piece in a game you don't understand."
            ),
            dialogue=[
                ("KANE (thinking)", "She thinks I'll join her. Transform. Become like her.\n                She thinks this is about enlightenment. About becoming\n                something more than human. She's insane. But she's also\n                brilliant. And that makes her infinitely dangerous."),
            ],
            choices=[
                Choice("The journal mentions a final location - 'Where it all ends'", "final_location"),
                Choice("Confront this head-on - call her, arrange a meeting", "call_cassandra_direct"),
            ]
        )
        
        self.scenes["philosophy_game"] = Scene(
            id="philosophy_game",
            description=(
                "You sit. Accept the absinthe. Play the game.\n\n"
                "'Tell me about transformation,' you say to Elias. 'About the philosophy\n"
                "behind what Cassandra is doing.'\n\n"
                "Elias smiles. Like a teacher pleased with an apt student.\n\n"
                "'The alchemists understood. Death isn't an ending. It's a purification.\n"
                "Cassandra is practicing the Great Work. Seven sacrifices. Seven\n"
                "transformations. Each death refines the world. Makes it cleaner.\n"
                "Closer to perfection. The people she kills aren't victims. They're\n"
                "components in a greater operation. Lead into gold. Corruption into justice.'"
            ),
            dialogue=[
                ("KANE", "That's just murder dressed up in mysticism."),
                ("ELIAS", "Is it? The alchemists sought the Philosopher's Stone. A substance\n           that could transform anything. Cassandra found it. Nightshade.\n           A compound that transforms consciousness itself. And she's using it\n           not for profit. Not for power. But for art. For philosophy.\n           For truth. That makes her more than a murderer, Detective.\n           It makes her a visionary."),
                ("KANE", "Or it makes her a sociopath with a chemistry degree."),
                ("ELIAS", "Perhaps both. The line is thinner than you think."),
            ],
            choices=[
                Choice("'Where is she?' - time to stop playing", "demand_location"),
                Choice("'Let me join her' - infiltrate deeper", "infiltrate_cult"),
            ]
        )
        
        self.scenes["split_investigation"] = Scene(
            id="split_investigation",
            description=(
                "You and Sarah split up. She heads to Ouroboros Pharmaceuticals.\n"
                "You go to the pier. Divide and conquer. Old tactics.\n\n"
                "But two hours later, Sarah's not answering her radio. Her phone goes\n"
                "straight to voicemail. The last ping from her phone was at Ouroboros.\n\n"
                "You finish at the pier and race to the pharmaceutical building.\n"
                "Security says Lieutenant Vega never checked in. Never arrived.\n\n"
                "But you know she was here. Her car is in the parking structure.\n"
                "Door open. Keys in the ignition. And on the driver's seat:\n"
                "a tarot card. The Tower. Destruction. Upheaval. Everything falling apart."
            ),
            dialogue=[
                ("KANE (thinking)", "She has Sarah. The pattern breaks. Sarah wasn't part\n                of the original seven. Unless... unless Cassandra is\n                adapting. Improvising. Making Sarah part of the ritual\n                because she got too close."),
                ("KANE (phone message)", "Sarah. If you can hear this. I'm coming. Hold on.\n                Just hold on."),
            ],
            choices=[
                Choice("Storm Ouroboros - tear the building apart to find her", "storm_ouroboros"),
                Choice("Cassandra wants you to panic - think, don't react", "stay_calm_sarah"),
            ]
        )
        
        self.scenes["resist_nightshade"] = Scene(
            id="resist_nightshade",
            description=(
                "The drug hits like a freight train made of nightmares. But you fight it.\n"
                "Years of discipline. Training. Sheer stubborn refusal to give up.\n\n"
                "The hallucinations come anyway. Walls breathing. Colors screaming.\n"
                "Cassandra's face multiplying into fractal patterns. But underneath\n"
                "the chaos, you hold onto one thing: your purpose. Find her. Stop her.\n\n"
                "You claw back to consciousness. Hours later? Days? Time has no meaning\n"
                "when reality is negotiable. But you're alive. Aware. Functional.\n\n"
                "And you remember where Cassandra said she'd be. The final location.\n"
                "Where the seventh sacrifice completes the cycle."
            ),
            dialogue=[
                ("KANE (thinking)", "She underestimated me. Thought the drug would break me.\n                Make me compliant. Instead, it just pissed me off.\n                I'm coming for you, Cassandra. And nothing - not your\n                drugs, not your philosophy, not your pattern - will stop me."),
            ],
            choices=[
                Choice("Go to the final location - end this", "final_confrontation"),
            ]
        )
        
        self.scenes["fake_unconscious"] = Scene(
            id="fake_unconscious",
            description=(
                "You let the drug take you. Or pretend to. You go limp. Still.\n"
                "Through slitted eyes, you watch Cassandra.\n\n"
                "She checks your pulse. Satisfied. Then she does something unexpected.\n"
                "She sits beside you. Strokes your hair. Like a mother with a sick child.\n\n"
                "'I'm sorry, Marcus,' she whispers. 'You deserved better. We all did.\n"
                "But the pattern demands completion. Seven sacrifices to remake the world.\n"
                "Seven deaths to birth something new. You'll understand soon. When you\n"
                "wake on the other side. If you wake at all.'\n\n"
                "She leaves. You wait. Count to a hundred. Then you move."
            ),
            dialogue=[
                ("KANE (thinking)", "She's more broken than I thought. There's genuine regret there.\n                Genuine sorrow. She doesn't want to kill. She thinks she has to.\n                That makes her more dangerous. True believers always are."),
            ],
            choices=[
                Choice("Follow her - she'll lead you to the final location", "track_cassandra"),
                Choice("The drug is affecting you - get medical help first", "medical_help"),
            ]
        )
        
        self.scenes["storm_ouroboros"] = Scene(
            id="storm_ouroboros",
            description=(
                "You storm Ouroboros with every cop loyal to you. Warrants be damned.\n"
                "Dr. Marsh tries to stop you. Security tries to intervene. You go through\n"
                "them like they're made of paper.\n\n"
                "In the subbasement - levels that don't exist on any blueprint - you find it.\n"
                "A laboratory. Pristine. Sterile. And in the center: Sarah.\n\n"
                "She's strapped to a medical chair. An IV in her arm. Amber liquid\n"
                "dripping into her bloodstream. Nightshade. She's convulsing. Screaming\n"
                "at things that aren't there.\n\n"
                "Cassandra stands beside her. Serene. Like a priest at an altar.\n\n"
                "'You're too late, Detective. The transformation has begun. In an hour,\n"
                "Lieutenant Vega will be gone. And someone new will emerge.'"
            ),
            dialogue=[
                ("KANE", "Let her go, Cassandra. She's not part of this."),
                ("CASSANDRA", "Everyone is part of this. The whole world is part of the pattern.\n               She got too close. Saw too much. She had to be... refined."),
                ("SARAH (through the drug)", "Marcus... kill me... please... I can see... everything...\n                           it's too much... too much..."),
            ],
            choices=[
                Choice("Shoot Cassandra - save Sarah no matter what", "shoot_cassandra_save_sarah"),
                Choice("Disconnect the IV - stop the dose", "save_sarah_medical"),
                Choice("Cassandra has the antidote - make her give it up", "demand_antidote"),
            ]
        )
        
        # Final ending branches
        
        self.scenes["shoot_cassandra_save_sarah"] = Scene(
            id="shoot_cassandra_save_sarah",
            description=(
                "You shoot. Center mass. Professional. Fatal.\n\n"
                "Cassandra falls. Her last words: 'The pattern... incomplete...'\n\n"
                "You disconnect Sarah's IV. Call for medical. She's rushed to the ER.\n"
                "The doctors work for six hours. When they emerge, they look exhausted.\n\n"
                "'She'll live. But Detective... the drug did something to her brain.\n"
                "There's damage. Extensive. She may never be the same.'\n\n"
                "Sarah wakes three days later. She knows who she is. Remembers you.\n"
                "But there are gaps. Missing pieces. And sometimes, she sees things.\n"
                "Patterns in chaos. Connections that shouldn't exist.\n\n"
                "'I can see it now,' she tells you. 'What Cassandra saw. The pattern\n"
                "behind everything. The way it all connects. She wasn't crazy, Marcus.\n"
                "She was just... awake. And now I am too.'\n\n"
                "You don't know if you saved her or damned her.\n\n"
                "ENDING: THE PRICE OF SALVATION"
            ),
            is_ending=True
        )
        
        self.scenes["save_sarah_medical"] = Scene(
            id="save_sarah_medical",
            description=(
                "You disconnect the IV. Stop the dose. Every second counts.\n\n"
                "Cassandra doesn't stop you. She just watches. Sad. Resigned.\n\n"
                "'It won't matter,' she says. 'Once you've seen the pattern, you can't\n"
                "unsee it. The Nightshade just opens the door. But the door doesn't close.'\n\n"
                "Sarah stabilizes. The hospital keeps her for observation. Two weeks later,\n"
                "she's cleared to return to duty. But she's different. Quieter. Distant.\n\n"
                "She puts in for a transfer. Different city. Different life.\n\n"
                "'I need to get away from this,' she tells you. 'From the case.\n"
                "From the pattern. From... everything. I'm sorry, Marcus.'\n\n"
                "You let her go. Because that's what partners do. You protect each other.\n"
                "Even from yourselves.\n\n"
                "Cassandra is never found. She vanished during the chaos. The case\n"
                "remains open. Seven victims. Six bodies. One killer still out there.\n\n"
                "You keep hunting. Because it's all you know how to do.\n\n"
                "ENDING: INCOMPLETE PATTERNS"
            ),
            is_ending=True
        )
        
        self.scenes["true_ending"] = Scene(
            id="true_ending",
            description=(
                "In the final confrontation at the old Prometheus facility, you realize\n"
                "the truth. The terrible truth that Cassandra has been guiding you toward.\n\n"
                "You're not the seventh victim. You're the seventh sacrifice. But not\n"
                "in death. In transformation. In understanding. In seeing the pattern.\n\n"
                "Cassandra shows you everything. The evidence. The conspiracy. How deep\n"
                "the corruption runs. How the system that's supposed to protect people\n"
                "instead protects monsters. How justice is just another commodity bought\n"
                "and sold by those with power.\n\n"
                "She offers you a choice: destroy the evidence and maintain the lie.\n"
                "Or release it and watch the world burn.\n\n"
                "You choose the third option. You take the evidence. You arrest Cassandra.\n"
                "And you release the information - but strategically. Piece by piece.\n"
                "Dismantling the corruption without destroying the system.\n\n"
                "It takes three years. Hundreds of arrests. Thousands of man-hours.\n"
                "But in the end, the city is cleaner. Not perfect. Never perfect.\n"
                "But better. A little bit better.\n\n"
                "Cassandra serves her sentence. You visit her sometimes. She always asks:\n"
                "'Do you see it now? The pattern?'\n\n"
                "And you have to admit: you do. You see how it all connects. How justice\n"
                "and revenge blur together. How the line between hero and villain is just\n"
                "a matter of perspective. How we're all trapped in patterns of our own making.\n\n"
                "But seeing the pattern doesn't mean accepting it. It means choosing\n"
                "to break it. Or at least trying.\n\n"
                "That's all any of us can do. Try.\n\n"
                "ENDING: EYES WIDE OPEN"
            ),
            is_ending=True
        )
        
        # Connect all the loose ends
        # Adding stub scenes for all referenced but not yet implemented scenes
        # These can be expanded later for even more branching narrative
        
        # Stub scenes for missing connections
        stub_scenes = [
            "exhume_tolliver", "threat_cassandra", "question_cassandra", "trace_cassandra",
            "stake_out_shop", "warehouse_district_early", "arrest_elias", "threaten_elias",
            "search_crimson_hour", "follow_cassandra_from_club", "call_backup_club",
            "retreat_from_club", "shootout_club", "negotiate_cassandra", "flee_office",
            "safe_house", "precinct_return", "search_damien_house", "damien_files",
            "flashlight_confrontation", "dark_hunt", "track_cassandra", "medical_help",
            "demand_antidote", "demand_location", "infiltrate_cult", "stay_calm_sarah",
            "prometheus_facility", "cassandra_associates", "find_real_location",
            "call_cassandra_direct", "final_location", "marsh_evidence",
            "protective_custody", "hide_marsh", "marsh_bait",
        ]
        
        for scene_id in stub_scenes:
            if scene_id not in self.scenes:
                self.scenes[scene_id] = Scene(
                    id=scene_id,
                    description=(
                        f"[This scene is under development]\n\n"
                        f"The investigation continues down this path, but the full story\n"
                        f"branch is still being written. For now, this leads to a\n"
                        f"confrontation with the truth.\n\n"
                        f"The rain keeps falling. The city keeps bleeding. And you keep\n"
                        f"moving forward, because stopping means accepting defeat."
                    ),
                    dialogue=[
                        ("KANE (thinking)", "Some paths are still being carved. But the destination\n"
                         "                is the same. Truth. Justice. Or something like it.")
                    ],
                    choices=[
                        Choice("Continue to the final confrontation", "final_confrontation"),
                    ]
                )

def create_story() -> Story:
    """Factory function to create the Blood and Neon story"""
    return BloodAndNeonStory()
