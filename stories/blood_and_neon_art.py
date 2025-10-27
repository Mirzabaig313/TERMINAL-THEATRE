"""ASCII Art Library for Blood and Neon Story
Noir-themed CLI art for the detective mystery"""

from engine.animation import Animation


class BloodAndNeonArt:
    """Collection of noir-themed ASCII art for Blood and Neon story"""
    
    @staticmethod
    def rain_city():
        """Noir cityscape with rain"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                  RAIN-SOAKED CITY                          ║
    ╚════════════════════════════════════════════════════════════╝
    
            ║  ║  ║         ║  ║       ║  ║  ║
         ║  ║  ║  ║  ║   ║  ║  ║    ║  ║  ║  ║  ║
            ║  ║  ║      ║  ║  ║  ║    ║  ║  ║
                          
    ╔═══╗  ╔════╗  ╔═══╗  ╔════╗  ╔═══╗  ╔════╗
    ║▓▓▓║  ║▓▓▓▓║  ║▓▓▓║  ║▓▓▓▓║  ║▓▓▓║  ║▓▓▓▓║
    ║▓█▓║  ║▓██▓║  ║█▓█║  ║▓██▓║  ║▓█▓║  ║██▓▓║
    ║███║  ║████║  ║███║  ║████║  ║███║  ║████║
    ║███║  ║████║  ║███║  ║████║  ║███║  ║████║
    ╚═══╝  ╚════╝  ╚═══╝  ╚════╝  ╚═══╝  ╚════╝
    """], frame_delay=0.0)
    
    @staticmethod
    def crime_scene():
        """Crime scene with body outline"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              ⚠️  CRIME SCENE - DO NOT CROSS  ⚠️            ║
    ╚════════════════════════════════════════════════════════════╝
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░  ╔═══╗  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░  ║   ║  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░  ═══╬═══╬═══  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░  ║   ║  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░ ╱     ╲ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░╱         ╲░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░           ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░           ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    """], frame_delay=0.0)
    
    @staticmethod
    def tarot_card_death():
        """Tarot card - Death"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                     THE DEATH CARD                         ║
    ╚════════════════════════════════════════════════════════════╝
    
                    ╔═══════════════════╗
                    ║  ┌─────────────┐  ║
                    ║  │    ___      │  ║
                    ║  │   /   \     │  ║
                    ║  │  │ ☠️  │    │  ║
                    ║  │   \___/     │  ║
                    ║  │    ║║║      │  ║
                    ║  │   ══╬══     │  ║
                    ║  │    ║║║      │  ║
                    ║  │   ╱   ╲     │  ║
                    ║  │  ╱     ╲    │  ║
                    ║  │            │  ║
                    ║  │   XIII.     │  ║
                    ║  │   DEATH     │  ║
                    ║  └─────────────┘  ║
                    ╚═══════════════════╝
    """], frame_delay=0.0)
    
    @staticmethod
    def tarot_card_fool():
        """Tarot card - The Fool"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                     THE FOOL CARD                          ║
    ╚════════════════════════════════════════════════════════════╝
    
                    ╔═══════════════════╗
                    ║  ┌─────────────┐  ║
                    ║  │     ☺       │  ║
                    ║  │    /│\      │  ║
                    ║  │   / │ \     │  ║
                    ║  │     │       │  ║
                    ║  │    / \      │  ║
                    ║  │   /   \     │  ║
                    ║  │            │  ║
                    ║  │    /\      │  ║
                    ║  │   /  \     │  ║
                    ║  │  /    \    │  ║
                    ║  │            │  ║
                    ║  │    0.       │  ║
                    ║  │  THE FOOL   │  ║
                    ║  └─────────────┘  ║
                    ╚═══════════════════╝
    """], frame_delay=0.0)
    
    @staticmethod
    def neon_sign():
        """Neon sign for the city"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                   THE CRIMSON HOUR                         ║
    ╚════════════════════════════════════════════════════════════╝
    
         ╔═══════════════════════════════════════════════╗
         ║  ╔═══╗ ┏━┓ ┏━┓ ┏┳┓ ┏━┓ ┏━┓ ┏┓┏   ╔═══╗  ║
         ║  ║     ┃ ┃ ┃ ┃ ┃┃┃ ┗━┓ ┃ ┃ ┃┗┫   ║      ║
         ║  ╚═══╝ ┗━┛ ┗━┛ ┻ ┻ ┗━┛ ┗━┛ ┻ ┻   ╚═══╝  ║
         ║                                            ║
         ║  ╦ ╦ ┏━┓ ╦ ╦ ┏━┓                         ║
         ║  ╠═╣ ┃ ┃ ┃ ┃ ┣┳┛                         ║
         ║  ╩ ╩ ┗━┛ ┗━┛ ┻┗━                         ║
         ╚═══════════════════════════════════════════════╝
              ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    """], frame_delay=0.0)
    
    @staticmethod
    def detective_badge():
        """Detective badge"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    DETECTIVE KANE                          ║
    ╚════════════════════════════════════════════════════════════╝
    
                        ╱▔▔▔▔▔▔▔▔▔╲
                       │  ★  ★  ★  │
                      │   DETECTIVE  │
                      │              │
                      │  HOMICIDE    │
                      │              │
                      │   M. KANE    │
                      │    SHIELD    │
                       │   #4517    │
                        ╲___________╱
    
                    ═══════════════════
    """], frame_delay=0.0)
    
    @staticmethod
    def gun_and_badge():
        """Detective's tools"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                   TOOLS OF THE TRADE                       ║
    ╚════════════════════════════════════════════════════════════╝
    
              ╔════════╗                    ___
              ║  ★★★  ║                   /   \
              ║ SHIELD ║         ┌────────────────┐
              ║  4517  ║         │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│
              ╚════════╝         └───────┐        │
                                         └────────┘
                                              ║
         [BADGE]                          [GUN]
    """], frame_delay=0.0)
    
    @staticmethod
    def pier_at_night():
        """Dark pier scene"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    PIER 19 - 3:00 AM                       ║
    ╚════════════════════════════════════════════════════════════╝
    
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ════════════════════════════════════════════════════════════
          ║    ║    ║    ║    ║    ║    ║    ║    ║    ║
          ║    ║    ║    ║    ║    ║    ║    ║    ║    ║
          ║    ║    ║    ║    ║    ║    ║    ║    ║    ║
          ║    ║    ║    ║    ║    ║    ║    ║    ║    ║
          ║    ║    ║    ║    ║    ║    ║    ║    ║    ║
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
    """], frame_delay=0.0)
    
    @staticmethod
    def ouroboros_symbol():
        """Ouroboros - snake eating its tail"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              OUROBOROS PHARMACEUTICALS                     ║
    ╚════════════════════════════════════════════════════════════╝
    
                        ╱───────────╲
                      ╱   ___   ___  ╲
                     │   (o o) (o o)  │
                    │    ╲___________╱ │
                    │    ╱           ╲ │
                   │    │  OUROBOROS │ │
                    │    ╲___________╱ │
                     │   ___         ___ │
                      ╲ ╱   ╲═════╱   ╲╱
                       ╲_______________╱
    
              [The snake that devours itself eternally]
    """], frame_delay=0.0)
    
    @staticmethod
    def nightshade_vial():
        """Vial of Nightshade compound"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                     NIGHTSHADE COMPOUND                    ║
    ╚════════════════════════════════════════════════════════════╝
    
                        ╔═══════════╗
                        ║  ⚠️  ⚠️   ║
                        ║   TOXIC   ║
                        ╚═══════════╝
                           ┃┃┃┃┃
                        ╔═════════╗
                        ║▓▓▓▓▓▓▓▓▓║
                        ║▓▓▓▓▓▓▓▓▓║
                        ║▒▒▒▒▒▒▒▒▒║
                        ║▒▒▒▒▒▒▒▒▒║
                        ║░░░░░░░░░║
                        ╚═════════╝
                        
                    [NIGHTSHADE - DO NOT TOUCH]
    """], frame_delay=0.0)
    
    @staticmethod
    def rain_window():
        """Rain streaming down a window"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                  3 AM - KANE'S APARTMENT                   ║
    ╚════════════════════════════════════════════════════════════╝
    
    ╔════════════════════════════════════════════════════════════╗
    ║ ║  ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║              ║
    ║    ║  ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║           ║
    ║ ║     ║  ║  ║     ║  ║  ║     ║  ║  ║     ║  ║           ║
    ║  ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║     ║          ║
    ║     ║  ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║          ║
    ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║     ║  ║          ║
    ║ ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║     ║           ║
    ║    ║  ║     ║  ║  ║     ║  ║  ║     ║  ║  ║     ║        ║
    ╚════════════════════════════════════════════════════════════╝
    
             [Rain hammers the glass like accusations]
    """], frame_delay=0.0)
    
    @staticmethod
    def police_lights():
        """Police car lights flashing"""
        return Animation([
            r"""
    ╔════════════════════════════════════════════════════════════╗
    ║  🔴 🔴 🔴                               🔵 🔵 🔵          ║
    ╚════════════════════════════════════════════════════════════╝
            """,
            r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                                                            ║
    ╚════════════════════════════════════════════════════════════╝
            """,
            r"""
    ╔════════════════════════════════════════════════════════════╗
    ║  🔴 🔴 🔴                               🔵 🔵 🔵          ║
    ╚════════════════════════════════════════════════════════════╝
            """
        ], frame_delay=0.3)
    
    @staticmethod
    def blood_splatter():
        """Blood splatter pattern"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    EVIDENCE MARKER #3                      ║
    ╚════════════════════════════════════════════════════════════╝
    
                  ░     ▓         ░
                     ▒  ▓▓▓  ░  ▒
               ░   ▒▓▓▓▓███▓▓▓▒   ░
                  ▒▓████████▓▓▒
                ░ ▓▓███████████▓▓ ░
                  ▒▓███████████▓▒
                    ▒▓▓▓███▓▓▓▒
                   ░  ▒▓▓▓▓▒  ░
                        ▒░
                      ░   ▒
    
                  [Arterial spray pattern]
    """], frame_delay=0.0)
    
    @staticmethod
    def morgue():
        """Morgue scene"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                  CITY MORGUE - BASEMENT                    ║
    ╚════════════════════════════════════════════════════════════╝
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ║                                                          ║
    ║  ╔═══════════════╗  ╔═══════════════╗  ╔═══════════════╗ ║
    ║  ║  [ DRAWER 1 ] ║  ║  [ DRAWER 2 ] ║  ║  [ DRAWER 3 ] ║ ║
    ║  ╚═══════════════╝  ╚═══════════════╝  ╚═══════════════╝ ║
    ║                                                          ║
    ║  ╔═══════════════╗  ╔═══════════════╗  ╔═══════════════╗ ║
    ║  ║  [ DRAWER 4 ] ║  ║  [ DRAWER 5 ] ║  ║  [ DRAWER 6 ] ║ ║
    ║  ╚═══════════════╝  ╚═══════════════╝  ╚═══════════════╝ ║
    ║                                                          ║
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
              [The scent of formaldehyde and secrets]
    """], frame_delay=0.0)
    
    @staticmethod
    def whiskey_glass():
        """Whiskey glass - detective's companion"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    LIQUID COURAGE                          ║
    ╚════════════════════════════════════════════════════════════╝
    
                          ╱▔▔▔▔▔╲
                         │       │
                         │       │
                        │  ░▒▒░  │
                        │  ▒▒▒▒  │
                         │ ▒▒▒▒ │
                          ╲▁▁▁▁╱
                          ▕████▏
                           ▔▔▔▔
    
                    [Some nights require help]
    """], frame_delay=0.0)
    
    @staticmethod
    def case_files():
        """Stack of case files"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    CASE FILES - SEALED                     ║
    ╚════════════════════════════════════════════════════════════╝
    
          ╔═══════════════════════════════════════════╗
          ║  CONFIDENTIAL - WESTMORE CASE             ║
          ║  ════════════════════════════════════     ║
          ║  [REDACTED]                               ║
          ║  [REDACTED]                               ║
          ║  Evidence: DESTROYED                      ║
          ║  Status: CLASSIFIED                       ║
          ╠═══════════════════════════════════════════╣
          ║  WITNESS STATEMENTS - RECANTED            ║
          ╠═══════════════════════════════════════════╣
          ║  CHEMICAL ANALYSIS - SEALED               ║
          ╠═══════════════════════════════════════════╣
          ║  CASE OUTCOME: DISMISSED                  ║
          ╚═══════════════════════════════════════════╝
    """], frame_delay=0.0)
    
    @staticmethod
    def phone_ringing():
        """Ringing telephone - animated"""
        return Animation([
            r"""
                        ╔═══════╗
                        ║ ☎️📞 ║
                        ║ RING! ║
                        ╚═══════╝
            """,
            r"""
                        ╔═══════╗
                        ║  ☎️📞  ║
                        ║       ║
                        ╚═══════╝
            """,
            r"""
                        ╔═══════╗
                        ║ ☎️📞 ║
                        ║ RING! ║
                        ╚═══════╝
            """
        ], frame_delay=0.5)
    
    @staticmethod
    def red_door():
        """The red door to The Crimson Hour"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              THE CRIMSON HOUR - ENTRANCE                   ║
    ╚════════════════════════════════════════════════════════════╝
    
    ████████████████████████████████████████████████████████████
    ████████████████████████████████████████████████████████████
    ███                                                      ███
    ███  ╔══════════════════════════════════════════════╗  ███
    ███  ║▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓║  ███
    ███  ║▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓║  ███
    ███  ║▓▓▓                                    ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓    [No sign. Just the door.]      ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓                                    ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓    [Blood-red paint, expensive]    ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓                                    ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓          ╔═════╗                  ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓          ║  ●  ║                  ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓          ╚═════╝                  ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓                                    ▓▓▓▓▓▓║  ███
    ███  ║▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓║  ███
    ███  ╚══════════════════════════════════════════════╝  ███
    ███                                                      ███
    ████████████████████████████████████████████████████████████
    """], frame_delay=0.0)
    
    @staticmethod
    def syringe():
        """Syringe with Nightshade"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                   WEAPON OF CHOICE                         ║
    ╚════════════════════════════════════════════════════════════╝
    
                    ║
                    ║
                  ╔═╩═╗
                  ║ ⚠ ║
                  ╚═╦═╝
                    ║
            ════════╬════════
            ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
            ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
            ░░░░░░░░░░░░░░░░
            ════════════════
                    ▼
    
            [One dose rewrites memory]
            [Two doses create madness]
            [Three doses... death]
    """], frame_delay=0.0)
    
    @staticmethod
    def chess_pieces():
        """Chess pieces - the game metaphor"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              THE GAME IS ALWAYS IN MOTION                  ║
    ╚════════════════════════════════════════════════════════════╝
    
         ♜    ♞    ♝    ♛    ♚    ♝    ♞    ♜
         ♟    ♟    ♟    ♟    ♟    ♟    ♟    ♟
         
         
         
         
         ♙    ♙    ♙    ♙    ♙    ♙    ♙    ♙
         ♖    ♘    ♗    ♕    ♔    ♗    ♘    ♖
    
            [Who is the player? Who is the piece?]
    """], frame_delay=0.0)
    
    @staticmethod
    def conspiracy_board():
        """Detective's conspiracy board with red string"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    CONNECTING THE DOTS                     ║
    ╚════════════════════════════════════════════════════════════╝
    
    ╔══════════════════════════════════════════════════════════╗
    ║  [PHOTO]     ╱╲    [PHOTO]    ╱╲╲   [REPORT]            ║
    ║  FOSTER  ═══╱══╲═══HARTLEY═══╱══╲══NIGHTSHADE           ║
    ║            ╱    ╲            ╱    ╲                      ║
    ║  [CARD]  ╱      ╲  [CARD]  ╱      ╲  [EVIDENCE]        ║
    ║  KING   ╱        ╲ KNIGHT ╱        ╲ DESTROYED          ║
    ║         ╲        ╱        ╱          ╲                   ║
    ║          ╲      ╱  [DOC] ╱   [YACHT]  ╲                 ║
    ║           ╲════╱═══CROSS════EXPLOSION══╲═[WESTMORE]     ║
    ║            ╲  ╱          ╲            ╱                  ║
    ║  [VICTIM]  ╲╱  [LATIN]   ╲          ╱  [ALIVE?]        ║
    ║  SEVEN      ║   PHRASES   ╲        ╱   CASSANDRA        ║
    ║             ║              ╲      ╱                      ║
    ║  [YOU]═════════════════════╲════╱═══[OUROBOROS]        ║
    ╚══════════════════════════════════════════════════════════╝
    
              [The pattern is emerging from chaos]
    """], frame_delay=0.0)


# Convenience function to get all art
def get_all_blood_and_neon_art():
    """Returns dictionary of all available art pieces"""
    return {
        'rain_city': BloodAndNeonArt.rain_city(),
        'crime_scene': BloodAndNeonArt.crime_scene(),
        'tarot_death': BloodAndNeonArt.tarot_card_death(),
        'tarot_fool': BloodAndNeonArt.tarot_card_fool(),
        'neon_sign': BloodAndNeonArt.neon_sign(),
        'detective_badge': BloodAndNeonArt.detective_badge(),
        'gun_and_badge': BloodAndNeonArt.gun_and_badge(),
        'pier': BloodAndNeonArt.pier_at_night(),
        'ouroboros': BloodAndNeonArt.ouroboros_symbol(),
        'nightshade': BloodAndNeonArt.nightshade_vial(),
        'rain_window': BloodAndNeonArt.rain_window(),
        'police_lights': BloodAndNeonArt.police_lights(),
        'blood_splatter': BloodAndNeonArt.blood_splatter(),
        'morgue': BloodAndNeonArt.morgue(),
        'whiskey': BloodAndNeonArt.whiskey_glass(),
        'case_files': BloodAndNeonArt.case_files(),
        'phone_ringing': BloodAndNeonArt.phone_ringing(),
        'red_door': BloodAndNeonArt.red_door(),
        'syringe': BloodAndNeonArt.syringe(),
        'chess': BloodAndNeonArt.chess_pieces(),
        'conspiracy_board': BloodAndNeonArt.conspiracy_board(),
    }
