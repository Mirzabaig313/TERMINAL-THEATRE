"""Quick checks for a story folder without building the game.

Usage: python3 tools/check_story.py stories/<id>

Checks scene links, speakers, line moods (against the speaker's sprite
expressions), effects, sounds, and reports unreachable scenes. `cargo test` runs the
full validation; this is a fast helper while writing.
"""

import glob
import os
import sys
import tomllib

FX = {"shake", "glitch", "flash", "blood", "blackout", "lightning", "heartbeat", "dizzy", "chill", "static", "reveal"}
TRANSITIONS = {"dissolve", "fade", "sweep", "rise", "coalesce", "cut"}
MOODS = {"noir", "danger", "alert", "calm", "mystery"}
SOUNDS = {"gunshot", "thunder", "knock", "phone", "siren", "glass", "heartbeat", "impact",
          "static", "sting", "whoosh", "warble", "chill", "chime", "footsteps", "silence"}
AMBIENCE = {"rain", "storm", "city", "neon", "wind", "drone", "dream", "silence"}
AUDIO_EXT = (".ogg", ".wav", ".mp3", ".flac")


def check_audio(folder, where, value, names, kind, errors):
    """A built-in name, or a playable audio file inside the story folder."""
    if "/" in value or "." in value:
        path = os.path.join(folder, value)
        if not os.path.isfile(path):
            errors.append(f"{where}: {kind} file '{value}' not found")
        elif not value.lower().endswith(AUDIO_EXT):
            errors.append(f"{where}: {kind} '{value}' must be one of {', '.join(AUDIO_EXT)}")
    elif value not in names:
        errors.append(f"{where}: {kind} '{value}' not in {sorted(names)}")


def main(folder):
    errors, warnings = [], []
    meta = tomllib.load(open(os.path.join(folder, "story.toml"), "rb"))
    speakers = meta.get("speakers", {})
    scenes = {}
    for path in sorted(glob.glob(os.path.join(folder, "scenes", "*.toml"))):
        for sid, scene in tomllib.load(open(path, "rb")).items():
            if sid in scenes:
                errors.append(f"{sid}: defined twice ({os.path.basename(path)})")
            scenes[sid] = scene

    expressions = {}
    for sid, spk in speakers.items():
        sprite = spk.get("sprite")
        if sprite:
            path = os.path.join(folder, "characters", f"{sprite}.toml")
            if not os.path.exists(path):
                errors.append(f"speaker {sid}: missing sprite file {path}")
                continue
            expressions[sid] = set(tomllib.load(open(path, "rb"))["expressions"])

    if "ambience" in meta:
        check_audio(folder, "story", meta["ambience"], AMBIENCE, "ambience", errors)
    for mood, value in meta.get("ambience_moods", {}).items():
        if mood not in MOODS:
            errors.append(f"ambience_moods: unknown mood '{mood}'")
        check_audio(folder, f"ambience_moods.{mood}", value, AMBIENCE, "ambience", errors)
    for sid, scene in scenes.items():
        if "sound" in scene:
            check_audio(folder, sid, scene["sound"], SOUNDS, "sound", errors)
        if "ambience" in scene:
            check_audio(folder, sid, scene["ambience"], AMBIENCE, "ambience", errors)
        for line in scene.get("dialogue", []):
            if "sound" in line:
                check_audio(folder, sid, line["sound"], SOUNDS, "sound", errors)

    if meta["start"] not in scenes:
        errors.append(f"start scene '{meta['start']}' missing")
    for sid, scene in scenes.items():
        if "image" in scene:
            img = os.path.join(folder, scene["image"])
            if not os.path.isfile(img):
                errors.append(f"{sid}: image '{scene['image']}' not found")
            elif not img.lower().endswith((".png", ".jpg", ".jpeg", ".svg")):
                errors.append(f"{sid}: image '{scene['image']}' must be .png, .jpg or .svg")
        if "mood" in scene and scene["mood"] not in MOODS:
            errors.append(f"{sid}: scene mood '{scene['mood']}' not in {sorted(MOODS)}")
        if scene.get("fx") and scene["fx"] not in FX:
            errors.append(f"{sid}: scene fx '{scene['fx']}' not in {sorted(FX)}")
        if scene.get("transition") and scene["transition"] not in TRANSITIONS:
            errors.append(f"{sid}: transition '{scene['transition']}' not in {sorted(TRANSITIONS)}")
        for line in scene.get("dialogue", []):
            who = line["who"]
            if who not in speakers:
                errors.append(f"{sid}: unknown speaker '{who}'")
                continue
            mood = line.get("mood")
            if mood and who in expressions and mood not in expressions[who]:
                errors.append(f"{sid}: '{who}' has no expression '{mood}' (has {sorted(expressions[who])})")
            if mood and who not in expressions:
                warnings.append(f"{sid}: '{who}' has no portrait, mood '{mood}' is ignored")
            if line.get("fx") and line["fx"] not in FX:
                errors.append(f"{sid}: fx '{line['fx']}' not in {sorted(FX)}")
        for c in scene.get("choices", []):
            if c["goto"] not in scenes:
                errors.append(f"{sid}: choice '{c['text']}' goes to missing scene '{c['goto']}'")
        if not scene.get("choices") and not scene.get("ending"):
            warnings.append(f"{sid}: no choices and not marked ending = true")

    seen, todo = set(), [meta["start"]]
    while todo:
        s = todo.pop()
        if s in seen or s not in scenes:
            continue
        seen.add(s)
        todo += [c["goto"] for c in scenes[s].get("choices", [])]
    for s in sorted(set(scenes) - seen):
        warnings.append(f"{s}: unreachable from the start")

    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("ERROR:", e)
    print(f"{len(scenes)} scenes, {len(errors)} errors, {len(warnings)} warnings")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
