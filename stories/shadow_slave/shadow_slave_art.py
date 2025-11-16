"""ASCII Art Library for Shadow Slave Story

This module contains all ASCII art used in the Shadow Slave story.
Add your own art here or modify the existing templates.
"""

from engine.animation import Animation


class ShadowSlaveArt:
    """Collection of ASCII art for Shadow Slave story"""

    @staticmethod
    def nightmare_gate():
        """The entrance to the Dream Realm"""
        return """
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║              ▄████████▄    ▄████████▄                    ║
    ║            ▄██▀      ▀██▄▄██▀      ▀██▄                  ║
    ║           ███   ░░░   ████   ░░░   ███                   ║
    ║           ███   ▒▒▒   ████   ▒▒▒   ███                   ║
    ║            ▀██▄      ▄██▀▀██▄      ▄██▀                  ║
    ║              ▀████████▀    ▀████████▀                    ║
    ║                   ████████████                           ║
    ║                   ██ SPELL ██                            ║
    ║                   ████████████                           ║
    ║              ░░░░░░░░░░░░░░░░░░░░░                       ║
    ║            ░░ THE NIGHTMARE GATE ░░                      ║
    ║              ░░░░░░░░░░░░░░░░░░░░░                       ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
        """

    @staticmethod
    def shadow_creature():
        """A nightmare creature made of shadows"""
        return """
                       ▄▄▄▄▄▄▄
                   ▄▄██▀▀░░░▀▀██▄▄
                 ▄█▀░░  ◆  ◆  ░░▀█▄
                ██░░   ▄███▄   ░░██
                █░░  ▄█▀▀▀▀▀█▄  ░░█
                █░░ ██  ▄▄▄  ██ ░░█
                ▀█░░▀█▄▀   ▀▄█▀░░█▀
                 ▀█▄░░▀█████▀░░▄█▀
                   ▀██▄▄░░░▄▄██▀
                  ░░░░▀▀███▀▀░░░░
                 ░░░  ███████  ░░░
                ░░   ██     ██   ░░
               ░░   ██       ██   ░░
              ░░   ██         ██   ░░
                  ██           ██
        """

    @staticmethod
    def dark_city():
        """The corrupted city in the Dream Realm"""
        return """
    ═══════════════════════════════════════════════════════════════
         ▄▄▄                        ▄▄▄           ▄▄▄▄
        █▓▓▓█       ▄▄▄▄          █▓▓▓█         █▓▓▓▓█
        █▓▓▓█      █▓▓▓▓█         █▓▓▓█         █▓▓▓▓█
        █▓▓▓█      █▓▓▓▓█  ▄▄▄    █▓▓▓█   ▄▄▄   █▓▓▓▓█
        █▓▓▓█▄▄▄▄  █▓▓▓▓█ █▓▓▓█   █▓▓▓█  █▓▓▓█  █▓▓▓▓█
        █▓▓▓▓▓▓▓█  █▓▓▓▓█ █▓▓▓█   █▓▓▓█  █▓▓▓█  █▓▓▓▓█
        █▓▓▓▓▓▓▓█  █▓▓▓▓█ █▓▓▓█▄▄▄█▓▓▓█▄▄█▓▓▓█▄▄█▓▓▓▓█
        ▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀ ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
    ═══════════════════════════════════════════════════════════════
              THE DARK CITY - DREAM REALM OUTSKIRTS
    ═══════════════════════════════════════════════════════════════
        """

    @staticmethod
    def victory():
        """Victory/survival scene"""
        return """
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║                    ✦ NIGHTMARE SURVIVED ✦                 ║
    ║                                                           ║
    ║                         ▄▄▄▄▄                             ║
    ║                       ▄█▀▀░▀▀█▄                           ║
    ║                      ██  ░█░  ██                          ║
    ║                      ██░░███░░██                          ║
    ║                       ▀██▄▄▄██▀                           ║
    ║                      ░░░░███░░░░                          ║
    ║                        ░░███░░                            ║
    ║                         ░███░                             ║
    ║                          ███                              ║
    ║                                                           ║
    ║              "The Spell acknowledges your strength"       ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
        """

    @staticmethod
    def death():
        """Death/failure scene"""
        return """
    ═══════════════════════════════════════════════════════════════

                    ░░░  YOU HAVE DIED  ░░░

                         ▄▄▄▄▄▄▄▄▄
                       ▄█▀░░░░░░░▀█▄
                      ██░  ▄▄▄▄▄  ░██
                      ██░ █░░X░░█ ░██
                      ██░ █░░X░░█ ░██
                      ██░  ▀▀▀▀▀  ░██
                       ▀█▄░░░░░░░▄█▀
                         ▀▀▀▀▀▀▀▀▀

                "The nightmare claims another soul"

    ═══════════════════════════════════════════════════════════════
        """

    @staticmethod
    def memory_fragment():
        """A recovered memory"""
        return """
            ╭───────────────────────────────────────╮
            │    ✧ MEMORY FRAGMENT RECOVERED ✧     │
            │                                       │
            │         ░░░▒▒▒▓▓▓███▓▓▓▒▒▒░░░         │
            │       ░░▒▒▓▓██         ██▓▓▒▒░░       │
            │      ░▒▓███                ███▓▒░     │
            │     ░▒▓██                    ██▓▒░    │
            │     ░▒▓██      MEMORY        ██▓▒░    │
            │     ░▒▓██                    ██▓▒░    │
            │      ░▒▓███                ███▓▒░     │
            │       ░░▒▒▓▓██         ██▓▓▒▒░░       │
            │         ░░░▒▒▒▓▓▓███▓▓▓▒▒▒░░░         │
            │                                       │
            ╰───────────────────────────────────────╯
        """

    @staticmethod
    def spell_runes():
        """Mystical spell runes"""
        return """
            ◈━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━◈

                ᚱ ᚢ ᚾ ᛖ ᛋ   ᛟ ᚠ   ᛈ ᛟ ᚹ ᛖ ᚱ

                      ◆ ◈ ◆ ◈ ◆

                    ▓▒░ SPELL ░▒▓

                      ◆ ◈ ◆ ◈ ◆

            ◈━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━◈
        """

    @staticmethod
    def nightmare_creature_animation():
        """Animated shadow creature (multiple frames)"""
        frames = [
            # Frame 1
            """
                       ▄▄▄▄▄▄▄
                   ▄▄██▀▀░░░▀▀██▄▄
                 ▄█▀░░  ◆  ◆  ░░▀█▄
                ██░░   ▄███▄   ░░██
                █░░  ▄█▀▀▀▀▀█▄  ░░█
                █░░ ██  ▄▄▄  ██ ░░█
                ▀█░░▀█▄▀   ▀▄█▀░░█▀
                 ▀█▄░░▀█████▀░░▄█▀
            """,
            # Frame 2
            """
                       ▄▄▄▄▄▄▄
                   ▄▄██▀▀░░░▀▀██▄▄
                 ▄█▀░░  ◇  ◇  ░░▀█▄
                ██░░   ▄███▄   ░░██
                █░░  ▄█▀▀▀▀▀█▄  ░░█
                █░░ ██ ▀▀▀▀▀ ██ ░░█
                ▀█░░▀█▄▀   ▀▄█▀░░█▀
                 ▀█▄░░▀█████▀░░▄█▀
            """,
            # Frame 3
            """
                       ▄▄▄▄▄▄▄
                   ▄▄██▀▀░░░▀▀██▄▄
                 ▄█▀░░  ◆  ◆  ░░▀█▄
                ██░░   ▄███▄   ░░██
                █░░  ▄█▀▀▀▀▀█▄  ░░█
                █░░ ██  ---  ██ ░░█
                ▀█░░▀█▄▀   ▀▄█▀░░█▀
                 ▀█▄░░▀█████▀░░▄█▀
            """,
        ]
        return Animation(frames=frames, delay=0.3)

    # ========================================================================
    # ADD YOUR OWN ASCII ART BELOW
    # ========================================================================
    #
    # Tips for creating ASCII art:
    # 1. Use https://www.asciiart.eu/ for inspiration
    # 2. Use box drawing characters: ═ ║ ╔ ╗ ╚ ╝ ─ │ ┌ ┐ └ ┘
    # 3. Use shading: ░ ▒ ▓ █
    # 4. Use symbols: ◆ ◇ ◈ ● ○ ◉ ◎ ✦ ✧ ✶ ✷ ⚔ ⚡ ☠
    # 5. Keep width under 65 characters for best display
    # 6. Test your art in the terminal to ensure it displays correctly
    #
    # Example of adding new art:
    #
    # @staticmethod
    # def your_art_name():
    #     """Description of your art"""
    #     return """
    #     Your ASCII art here
    #     """
    #
    # For animations, use this format:
    #
    # @staticmethod
    # def your_animation_name():
    #     """Description of your animation"""
    #     frames = [
    #         "Frame 1 art here",
    #         "Frame 2 art here",
    #         "Frame 3 art here",
    #     ]
    #     return Animation(frames=frames, delay=0.5)
    #
    # ========================================================================
