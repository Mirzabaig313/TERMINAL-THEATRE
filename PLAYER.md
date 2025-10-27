# 🎮 Player Guide

## ⚡ Quick Start

```bash
# Install dependencies
pip install -r requirements.txt

# Run the game
python3 main.py
```

**Note**: The game works without Rich, but with Rich installed you get full color cinematics!

## 🎯 How to Play

### Controls

- **Number Keys (1-9)**: Select your choice during decision points
- **Enter**: Continue through dialogue and scenes
- **Ctrl+C**: Exit the game at any time

### Basic Gameplay

1. **Read the Scene**: Pay attention to descriptions - they contain clues
2. **Make a Choice**: Enter the number corresponding to your action
3. **Watch It Unfold**: See the consequences of your decision
4. **Continue**: Press Enter to proceed through dialogue

## 📖 Current Story: The Last Case

*"The rain hasn't stopped for three days. Neither has the blood."*

**The noir detective mystery where every choice matters.**

You play as **Jack Malone**, a private investigator framed for murder in a city drowning in corruption. Navigate a complex web of mob bosses, corrupt cops, and military conspiracies as you fight to clear your name.

**Story Details**
- **Choice Points**: 60+ decision moments
- **Endings**: 23 unique endings
- **Genre**: Film noir detective mystery
- **Themes**: Corruption, justice, moral ambiguity

## 💾 Save & Load System

Enjoy playing at your own pace:

- **Autosave**: Game automatically saves after major scenes
- **Manual Saves**: Select `[Save Game]` at any choice point
- **Multiple Slots**: Save up to 9 separate games
- **Continue**: Resume from your last save on the main menu
- **Load Game**: Browse all saves with details before loading
- **Save Location**: Files stored in `~/.terminal_theatre/saves/`

Each save tracks:
- Current scene
- Visited locations
- Inventory
- Choices made
- Playtime
- Completion percentage

## 🎯 Gameplay Tips

### Story Understanding
1. **Read Carefully**: Dialogue contains clues about best choices
2. **Consider Context**: Your earlier choices affect available options
3. **Character Knowledge**: Learn who to trust and who to avoid
4. **Environmental Clues**: Details in descriptions reveal secrets

### Strategic Gameplay
1. **Multiple Playthroughs**: Try different paths - no "correct" way
2. **Moral Choices**: Some endings are "good," some "bad," some ambiguous
3. **Risk vs. Reward**: Sometimes boldness wins, sometimes caution pays off
4. **Character Relationships**: Alliances matter - build them carefully

## 🎬 Your First Playthrough

### Opening Scene

You wake to find a dead woman in your office and a warm gun in your hand. Three immediate choices:

**Option 1: Search her body**  
Find crucial evidence, play it smart and methodical, gain information advantage

**Option 2: Check your gun**  
Discover who's framing you, get a major clue immediately, direct approach

**Option 3: Run**  
Take your chances on the street, escape and regroup, high-risk high-reward

### Early Decision Points

**Trust Question**: Do you call Eddie for help or go solo?  
With Eddie (have backup but owe him) vs. Alone (risk more but stay independent)

**Investigation Method**: How do you gather evidence?  
Sneak around (risky but quiet), talk to people (slow but informative), or use force (fast but creates enemies)

## 🗺️ Common Story Paths

### Path 1: The Detective Path (Official Victory)
1. Search body → Find evidence
2. Trust Rodriguez (detective)
3. Work with FBI
4. Gather all evidence
5. Confront conspirators in court
- **Ending**: Clear your name officially, justice prevails

### Path 2: The Lone Wolf Path (Personal Justice)
1. Check gun → Go solo
2. Refuse all help
3. Hunt down evidence yourself
4. Confront Castellano directly
5. Personal showdown
- **Ending**: On your own terms, but at a cost

### Path 3: The Alliance Path (Moral Ambiguity)
1. Run → Meet Castellano
2. Work with unlikely partners
3. Use mob connections
4. Take down the conspiracy
5. Become part of the underworld
- **Ending**: Problem solved, but you're compromised

### Path 4: The Escape Path (Survival)
1. Choose defensive options
2. Avoid major confrontations
3. Gather escape resources
4. Leave the city
5. New life, unresolved past
- **Ending**: Alive but haunted

## 🎭 Ending Types

### Victory Endings (7 types)
- Solve the case completely
- Clear your name officially
- Bring down the conspiracy
- Variations based on methods

### Death Endings (3 types)
- Fall in combat
- Betrayed by allies
- Caught by enemies

### Moral Ambiguity Endings (5 types)
- Success at a cost
- Compromise your values
- Pyrrhic victories
- Partial solutions

### Escape Endings (3 types)
- Flee the city
- New identity
- Live in hiding

### Alliance Endings (5 types)
- Team with mob
- Join the cops
- Work with FBI
- Partner with rivals
- Unlikely combinations

## 🧩 Character Guide

### Jack Malone (You)
- Private detective framed for murder
- Smart and resourceful
- Can solve problems through investigation, persuasion, or force

### Eddie
- Your old informant friend
- Knows the street
- Can help or betray you

### Rodriguez
- Detective on your case
- Potentially ally or enemy
- By-the-book cop

### Castellano
- Mob boss
- Powerful and ruthless
- Can be negotiated with

### Blackwood
- FBI agent
- Hidden agenda
- Potentially trustworthy or dangerous

### Morrison
- Military intelligence
- Connected to Project Nightfall
- Mysterious intentions

## 🎨 Visual Features

**Color Moods**  
The game uses atmospheric colors for each scene:
- **Noir** (Blue/Amber): Detective atmosphere
- **Danger** (Red): Combat and threats
- **Alert** (Orange/Yellow): Tension and investigation
- **Calm** (Blue/Cyan): Safe zones and allies
- **Mystery** (Gray/Amber): Conspiracies and secrets

**ASCII Art**  
Key scenes feature atmospheric ASCII art for your office, city skyline, warehouse, police station, and other locations

## 🐛 Troubleshooting

**Controls not responding?**
- Make sure you press ENTER after entering your choice number
- Try pressing 1, 2, 3 etc. (1-based indexing)

**Screen looks messy?**
- Your terminal might not support clearing
- Try a different terminal emulator (iTerm2, Terminal.app, etc.)

**Text moving too fast?**
- This is intentional for cinematic effect
- Use pauses to catch up
- Text won't disappear - it's just displayed quickly

**Game crashed?**
- Try restarting: `python3 main.py`
- Report the bug if it happens consistently

**Want to exit?**
- Press `Ctrl+C` at any time (gracefully handled)

## 💡 Pro Tips

1. **Experiment**: The game is designed for multiple playthroughs
2. **Save Often**: Use manual saves to branch your story
3. **Read Everything**: Descriptions contain important information
4. **Trust Your Instincts**: Your first choices often lead to interesting paths
5. **Replay Different**: Each path reveals new dialogue and choices
6. **No Punishment**: Failed paths aren't "wrong" - they're just different

## 🎬 Replay Value

Terminal Theatre encourages multiple playthroughs:
- 23 different endings to discover
- 60+ choice points with varying outcomes
- Hidden dialogue based on your decisions
- Different character interactions based on your choices
- Completely different story arcs available

## ⏱️ Story Length

Terminal Theatre features stories of varying lengths:
- **Short Stories**: Quick playthroughs for casual sessions
- **Long Stories**: Epic adventures for deep immersion
- **Your Pace**: Save system lets you play at your own speed
- **Multiple Playthroughs**: Explore different paths and discover all endings

## 🎯 Achievement Ideas

While not built-in yet, try these playstyle challenges:

- **The Detective**: Solve using only investigation
- **The Lone Wolf**: Never accept help
- **The Alliance**: Work with everyone possible
- **The Survivor**: Reach every ending type
- **Speed Run**: Fastest time to an ending
- **Pacifist**: Solve without violence
- **Violent**: Solve through force only

---

**Ready to play? Start your investigation with `python3 main.py`!**

*"In the dark streets of Terminal Theatre, every choice matters. Choose wisely."* 🎭
