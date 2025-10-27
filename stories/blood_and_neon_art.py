"""ASCII Art Library for Blood and Neon Story
Noir-themed CLI art for the detective mystery"""

from engine.animation import Animation


class BloodAndNeonArt:
    """Collection of noir-themed ASCII art for Blood and Neon story"""
    
    @staticmethod
    def rain_city():
        """Noir cityscape with rain - Enhanced"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                  RAIN-SOAKED CITY                          ║
    ║                  [Midnight, Downtown]                      ║
    ╚════════════════════════════════════════════════════════════╝
    
         ║  ║  ║      ║  ║      ║  ║  ║      ║  ║  ║
            ║  ║  ║      ║  ║  ║      ║  ║  ║     ║
         ║     ║  ║  ║      ║  ║  ║      ║  ║  ║
                          
    ╔═══╗  ╔════╗  ╔═══╗  ╔════╗  ╔═══╗  ╔════╗  ╔═══╗
    ║▓▓▓║  ║▓▓▓▓║  ║▓▓▓║  ║▓▓▓▓║  ║▓▓▓║  ║▓▓▓▓║  ║▓▓▓║
    ║▓█▓║  ║▓██▓║  ║█▓█║  ║▓██▓║  ║▓█▓║  ║██▓▓║  ║▓█▓║
    ║▓█▓║  ║▓██▓║  ║█▓█║  ║▓██▓║  ║▓█▓║  ║██▓▓║  ║▓█▓║
    ║███║  ║████║  ║███║  ║████║  ║███║  ║████║  ║███║
    ║███║  ║████║  ║███║  ║████║  ║███║  ║████║  ║███║
    ╚═══╝  ╚════╝  ╚═══╝  ╚════╝  ╚═══╝  ╚════╝  ╚═══╝
    ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈
         [Wet streets reflect neon promises and broken dreams]
    """], frame_delay=0.0)
    
    @staticmethod
    def crime_scene():
        """Crime scene with body outline - Enhanced"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║          ⚠️  POLICE LINE - DO NOT CROSS  ⚠️                ║
    ║              [HOMICIDE INVESTIGATION]                      ║
    ╚════════════════════════════════════════════════════════════╝
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░  [3]  ░░░░░░░░░░░░╔═══╗░░░░░░░░░░░░░░░░░░░  [1]  ░░░▓
    ▓░░░EVIDENCE░░░░░░░░░░░║   ║░░░░░░░░░░░░░░░░░░EVIDENCE░░░▓
    ▓░░░MARKER░░░░░░  ═══╬═══╬═══  ░░░░░░░░░░░░░░░MARKER░░░░▓
    ▓░░░░░░░░░░░░░░░░░░░  ║   ║  ░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░░░ ╱     ╲ ░░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░░░░░░░░░░░╱           ╲░░░░░░░░░░░░░░░░░░░░░░░░░░▓
    ▓░░░░░░░  [2]  ░░░             ░░░░░  [4]  ░░░░░░░░░░░░░░▓
    ▓░░░░░EVIDENCE░░░░             ░░░EVIDENCE░░░░░░░░░░░░░░░▓
    ▓░░░░░MARKER░░░░░░             ░░░░MARKER░░░░░░░░░░░░░░░░▓
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
         [Time of Death: 2:47 AM | Cause: Exsanguination]
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
    
    @staticmethod
    def tarot_queen_swords():
        """Tarot card - Queen of Swords"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                  THE QUEEN OF SWORDS                       ║
    ╚════════════════════════════════════════════════════════════╝
    
                    ╔═══════════════════╗
                    ║  ┌─────────────┐  ║
                    ║  │    ___      │  ║
                    ║  │   /▓▓▓\     │  ║
                    ║  │  │ ◇ ◇ │    │  ║
                    ║  │   \___/     │  ║
                    ║  │   ═══╬═══   │  ║
                    ║  │   ⚔ ║ ⚔    │  ║
                    ║  │    ╱█╲      │  ║
                    ║  │   ╱███╲     │  ║
                    ║  │  ╱█████╲    │  ║
                    ║  │   ╱█╲       │  ║
                    ║  │  ╱   ╲      │  ║
                    ║  │  QUEEN      │  ║
                    ║  │ OF SWORDS   │  ║
                    ║  └─────────────┘  ║
                    ╚═══════════════════╝
    
            [Clarity through pain, truth through loss]
    """], frame_delay=0.0)
    
    @staticmethod
    def tarot_king_cups():
        """Tarot card - King of Cups"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                   THE KING OF CUPS                         ║
    ╚════════════════════════════════════════════════════════════╝
    
                    ╔═══════════════════╗
                    ║  ┌─────────────┐  ║
                    ║  │    ♕         │  ║
                    ║  │   ___        │  ║
                    ║  │  /▓▓▓\       │  ║
                    ║  │ │ ◆ ◆ │      │  ║
                    ║  │  \___/       │  ║
                    ║  │   ═╪═        │  ║
                    ║  │  ╱███╲       │  ║
                    ║  │ ╱█████╲      │  ║
                    ║  │   ╱█╲        │  ║
                    ║  │  ╱ ▓ ╲       │  ║
                    ║  │    ◡         │  ║
                    ║  │   KING       │  ║
                    ║  │  OF CUPS     │  ║
                    ║  └─────────────┘  ║
                    ╚═══════════════════╝
    
            [Emotional mastery, hidden depths]
    """], frame_delay=0.0)
    
    @staticmethod
    def tarot_knight_wands():
        """Tarot card - Knight of Wands"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                 THE KNIGHT OF WANDS                        ║
    ╚════════════════════════════════════════════════════════════╝
    
                    ╔═══════════════════╗
                    ║  ┌─────────────┐  ║
                    ║  │     ⚔       │  ║
                    ║  │    ╱█╲      │  ║
                    ║  │   ╱███╲     │  ║
                    ║  │  │ ◈ ◈ │    │  ║
                    ║  │   ═══════   │  ║
                    ║  │    ║║║      │  ║
                    ║  │   ══╬══     │  ║
                    ║  │    ║║║      │  ║
                    ║  │   ╱▓▓▓╲     │  ║
                    ║  │  ╱▓▓▓▓▓╲    │  ║
                    ║  │   ╱   ╲     │  ║
                    ║  │  KNIGHT     │  ║
                    ║  │ OF WANDS    │  ║
                    ║  └─────────────┘  ║
                    ╚═══════════════════╝
    
            [Action without thought, passion without control]
    """], frame_delay=0.0)
    
    @staticmethod
    def detective_silhouette():
        """Silhouette of detective in doorway"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                  DETECTIVE KANE ARRIVES                    ║
    ╚════════════════════════════════════════════════════════════╝
    
    ████████████████████████████████████████████████████████████
    ███                                                      ███
    ███  ╔═══════════════════════════════════════════════╗  ███
    ███  ║                                               ║  ███
    ███  ║                                               ║  ███
    ███  ║                    ▓▓▓                        ║  ███
    ███  ║                   ▓▓▓▓▓                       ║  ███
    ███  ║                  ▓▓▓▓▓▓▓                      ║  ███
    ███  ║                 ▓▓▓▓▓▓▓▓▓                     ║  ███
    ███  ║                ▓▓▓▓▓▓▓▓▓▓▓                    ║  ███
    ███  ║                  ▓▓▓▓▓▓▓                      ║  ███
    ███  ║                   ▓▓▓▓▓                       ║  ███
    ███  ║                    ▓▓▓                        ║  ███
    ███  ║                                               ║  ███
    ███  ║                                               ║  ███
    ███  ╚═══════════════════════════════════════════════╝  ███
    ███                                                      ███
    ████████████████████████████████████████████████████████████
    
              [A figure cuts through the darkness]
    """], frame_delay=0.0)
    
    @staticmethod
    def street_lamp():
        """Lonely street lamp in the rain"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    STREET CORNER - 4 AM                    ║
    ╚════════════════════════════════════════════════════════════╝
    
              ║  ║      ║  ║  ║      ║  ║      ║
                 ║  ║      ║  ║  ║      ║  ║  ║
    
                         ╔═══════════╗
                         ║  ░▒▒▒▒░  ║
                         ║ ░▒▒▒▒▒░  ║
                         ╚═══╦═══╦══╝
                             ║   ║
                             ║   ║
                             ║   ║
                             ║   ║
                             ║   ║
                             ║   ║
                             ║   ║
                             ║   ║
                        ═════╩═══╩═════
    
    ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈
    
            [The only witness stands silent and unmoved]
    """], frame_delay=0.0)
    
    @staticmethod
    def cassandra_portrait():
        """Portrait of Cassandra Westmore"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              DR. CASSANDRA WESTMORE - DECEASED             ║
    ║              [Or so the records claim...]                  ║
    ╚════════════════════════════════════════════════════════════╝
    
                        ╔═══════════════╗
                        ║   ,,,,,,,,    ║
                        ║  ▓▓▓▓▓▓▓▓▓▓   ║
                        ║  ▓▓▓▓▓▓▓▓▓▓   ║
                        ║  ▓▓▓▓▓▓▓▓▓▓   ║
                        ║    ◈    ◈     ║
                        ║       ▓        ║
                        ║     ─────     ║
                        ║               ║
                        ║   ═══════════ ║
                        ║   HEIGHT: 5'8"║
                        ║   EYES: GREEN ║
                        ║   HAIR: BLACK ║
                        ║   STATUS: ??? ║
                        ╚═══════════════╝
    
            [Beautiful. Brilliant. Believed dead.]
    """], frame_delay=0.0)
    
    @staticmethod
    def lab_equipment():
        """Chemical laboratory setup"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              PROMETHEUS INDUSTRIES - LAB 7                 ║
    ║              [RESTRICTED ACCESS - LEVEL 5]                 ║
    ╚════════════════════════════════════════════════════════════╝
    
         ╭───╮     ╭───╮     ╭───╮      ╔═══════════╗
         │ ▓ │     │ ▒ │     │ ░ │      ║  DANGER   ║
         │ ▓ │     │ ▒ │     │ ░ │      ║  BIOHAZARD║
         │▓▓▓│     │▒▒▒│     │░░░│      ╚═══════════╝
         ╰───╯     ╰───╯     ╰───╯
           ║         ║         ║         ┌───────────┐
    ═══════╬═════════╬═════════╬═════════│ NIGHTSHADE│
           ║         ║         ║         │ SYNTHESIS │
         ╭─┴─╮     ╭─┴─╮     ╭─┴─╮      │ PROTOCOL  │
         │ ~ │     │ ~ │     │ ~ │      └───────────┘
         │~~~│     │~~~│     │~~~│
         ╰───╯     ╰───╯     ╰───╯       [CLASSIFIED]
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
       [Where science became sin, and truth became weapon]
    """], frame_delay=0.0)
    
    @staticmethod
    def yacht_explosion():
        """Yacht explosion scene"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              COAST GUARD REPORT #47B-2019                  ║
    ║              VESSEL: "PROMETHEUS" - DESTROYED              ║
    ╚════════════════════════════════════════════════════════════╝
    
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
                     ░▒▓█  ◢█◣  █▓▒░
                    ░▒▓███ ◢███◣ ███▓▒░
                   ░▒▓█████◢█████◣█████▓▒░
                     ▒▓███  ◥█◤  ███▓▒
                       ▓█    ║    █▓
                        ▓    ║    ▓
                             ║
    ════════════════════════╩════════════════════════
                      DEBRIS FIELD
    ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈
    
      [TIME: 2:47 AM | SURVIVORS: 0 | CASUALTIES: 1 (PRESUMED)]
      [CAUSE: UNKNOWN | ACCELERANT DETECTED: MILITARY GRADE]
    """], frame_delay=0.0)
    
    @staticmethod
    def evidence_photos():
        """Wall of evidence photos"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                    EVIDENCE BOARD                          ║
    ╚════════════════════════════════════════════════════════════╝
    
    ╔════════╗  ╔════════╗  ╔════════╗  ╔════════╗  ╔════════╗
    ║ VICTIM ║  ║ VICTIM ║  ║ VICTIM ║  ║ SUSPECT║  ║ WEAPON ║
    ║   #1   ║  ║   #2   ║  ║   #3   ║  ║  ???   ║  ║ SYRINGE║
    ║ FOSTER ║  ║HARTLEY ║  ║ CROSS  ║  ║  ????  ║  ║NIGHTSHDE║
    ║ ◢████◣ ║  ║ ◢████◣ ║  ║ ◢████◣ ║  ║ ◢████◣ ║  ║ ╱╲     ║
    ║◢██████◣║  ║◢██████◣║  ║◢██████◣║  ║◢██████◣║  ║ ║║     ║
    ║████████║  ║████████║  ║████████║  ║████████║  ║═╬╬═════║
    ╚════════╝  ╚════════╝  ╚════════╝  ╚════════╝  ╚════════╝
    
    ╔════════╗  ╔════════╗  ╔════════╗  ╔════════╗  ╔════════╗
    ║ SCENE  ║  ║ SCENE  ║  ║ SCENE  ║  ║LOCATION║  ║  CARD  ║
    ║  ONE   ║  ║  TWO   ║  ║ THREE  ║  ║  CLUB  ║  ║ TAROT  ║
    ║ ALLEY  ║  ║COURT   ║  ║ PIER19 ║  ║CRIMSON ║  ║ QUEEN  ║
    ║████████║  ║████████║  ║████████║  ║████████║  ║ SWORDS ║
    ║████████║  ║████████║  ║████████║  ║████████║  ║   ♔    ║
    ╚════════╝  ╚════════╝  ╚════════╝  ╚════════╝  ╚════════╝
    
            [Every death tells a story. Every story has a price.]
    """], frame_delay=0.0)
    
    @staticmethod
    def interrogation_room():
        """Police interrogation room"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║              INTERROGATION ROOM 3 - PRECINCT 9             ║
    ╚════════════════════════════════════════════════════════════╝
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    ║                                                          ║
    ║                    ╔══════════╗                         ║
    ║                    ║   LIGHT  ║                         ║
    ║                    ╚════╦═════╝                         ║
    ║                         ║                               ║
    ║                         ▼                               ║
    ║                                                          ║
    ║         ╔═══════════════════════════╗                   ║
    ║         ║                           ║                   ║
    ║         ║      TABLE (METAL)        ║                   ║
    ║         ║                           ║                   ║
    ║         ║   [RECORDER]  [FILES]    ║                   ║
    ║         ╚═══════════════════════════╝                   ║
    ║                                                          ║
    ║    ╔═══════╗                      ╔═══════╗            ║
    ║    ║ CHAIR ║                      ║ CHAIR ║            ║
    ║    ║ (DET) ║                      ║(SUSPT)║            ║
    ║    ╚═══════╝                      ╚═══════╝            ║
    ║                                                          ║
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
           [Two chairs. One truth. But which truth?]
    """], frame_delay=0.0)
    
    @staticmethod
    def city_map():
        """Map of the city with crime locations marked"""
        return Animation([r"""
    ╔════════════════════════════════════════════════════════════╗
    ║                 CITY MAP - CRIME LOCATIONS                 ║
    ╚════════════════════════════════════════════════════════════╝
    
         NORTH DISTRICT        DOWNTOWN           PIER DISTRICT
    ┌──────────────────┬──────────────────┬──────────────────┐
    │                  │                  │                  │
    │     ░░░░░░       │   ████CITY████   │     ≈≈≈≈≈≈      │
    │     ░PARK░       │   ██HALL███      │    ≈WATER≈      │
    │     ░░░░░░       │   ████████       │   ≈≈≈≈[3]≈≈≈    │
    │                  │       [2]         │   ≈≈PIER≈≈≈     │
    ├──────────────────┼──────────────────┼──────────────────┤
    │  ARTS DISTRICT   │ FINANCIAL DIST   │  WAREHOUSE DIST  │
    │                  │                  │                  │
    │    ░THEATER░     │   ▓▓▓▓▓▓▓▓       │    ████████     │
    │    ░░[4]░░░      │   ▓BANKS▓▓       │    █WHSE██      │
    │    ░MUSEUM░      │   ▓▓▓▓▓▓▓▓       │    ██[5]███     │
    │                  │                  │    ████████      │
    ├──────────────────┼──────────────────┼──────────────────┤
    │   WEST SIDE      │   MEAT PACKING   │   INDUSTRIAL     │
    │                  │                  │                  │
    │   ▒▒▒▒▒▒▒▒       │    ╔═══════╗    │    ▓▓▓▓▓▓▓      │
    │   ▒RESID▒▒       │    ║CRIMSON║    │    ▓PLANT▓      │
    │   ▒▒[1]▒▒▒       │    ║ HOUR  ║    │    ▓▓▓▓▓▓▓      │
    │   ▒▒▒▒▒▒▒▒       │    ╚═══════╝    │                  │
    └──────────────────┴──────────────────┴──────────────────┘
    
    LEGEND: [1] Mercy Alley  [2] Courthouse  [3] Pier 19
            [4] Crimson Hour [5] Prometheus Labs
    """], frame_delay=0.0)


# Convenience function to get all art
def get_all_blood_and_neon_art():
    """Returns dictionary of all available art pieces"""
    return {
        'rain_city': BloodAndNeonArt.rain_city(),
        'crime_scene': BloodAndNeonArt.crime_scene(),
        'tarot_death': BloodAndNeonArt.tarot_card_death(),
        'tarot_fool': BloodAndNeonArt.tarot_card_fool(),
        'tarot_queen_swords': BloodAndNeonArt.tarot_queen_swords(),
        'tarot_king_cups': BloodAndNeonArt.tarot_king_cups(),
        'tarot_knight_wands': BloodAndNeonArt.tarot_knight_wands(),
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
        'detective_silhouette': BloodAndNeonArt.detective_silhouette(),
        'street_lamp': BloodAndNeonArt.street_lamp(),
        'cassandra_portrait': BloodAndNeonArt.cassandra_portrait(),
        'lab_equipment': BloodAndNeonArt.lab_equipment(),
        'yacht_explosion': BloodAndNeonArt.yacht_explosion(),
        'evidence_photos': BloodAndNeonArt.evidence_photos(),
        'interrogation_room': BloodAndNeonArt.interrogation_room(),
        'city_map': BloodAndNeonArt.city_map(),
    }
