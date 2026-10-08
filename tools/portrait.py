"""Generate animated pixel-art portraits from a written description.

Each story folder may have a `cast.toml` describing its characters:

    [jack_malone]
    name = "Jack Malone"
    hair_style = "slick"        # messy | slick | short | long | bob | undercut | bald
    hat = "fedora"              # none | fedora | cap | police
    outfit = "trench"           # trench | suit | leather | uniform | dress | vest | rags
    extras = ["stubble", "cigarette"]
    skin = "#c9a58a"            # any of the colors below are optional
    hair = "#2b211b"
    ...

Usage:
    python3 tools/portrait.py stories/noir_detective [--preview out_dir]

Writes characters/<id>.toml for every entry. Hand-made sprites that are not in
cast.toml are left alone. Edit cast.toml and run again to tweak a character.
"""

import math
import os
import subprocess
import sys
import tomllib

W, H = 32, 40
CX = 15.5

STYLES = {
    "hair_style": ["messy", "slick", "short", "long", "bob", "undercut", "bald"],
    "hat": ["none", "fedora", "cap", "police"],
    "outfit": ["trench", "suit", "leather", "uniform", "dress", "vest", "rags"],
    "extras": ["stubble", "mustache", "beard", "scar", "cigarette", "glasses", "earrings", "cyber_eye",
               "neon_streak", "tattoo", "freckles", "lashes", "wrinkles", "pinstripe", "gray_temples"],
}

DEFAULTS = {
    "skin": "#d2b49c", "hair": "#2a2222", "eyes": "#6b7a8f", "lips": "#8a4a4a",
    "cloth": "#3a3a46", "shirt": "#d8d4c8", "accent": "#8a2a2a", "hat_color": "#3b342c",
    "metal": "#d4af37", "neon": "#00e5ff", "brows": None, "glasses": "#9a9aa6",
}


def hexrgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) for i in (0, 2, 4))


def tohex(c):
    return "#%02x%02x%02x" % tuple(max(0, min(255, int(v))) for v in c)


def shade(h, k):
    """k < 1 darkens, k > 1 lightens towards white."""
    r, g, b = hexrgb(h)
    if k <= 1:
        return tohex((r * k, g * k, b * k))
    t = k - 1
    return tohex((r + (255 - r) * t, g + (255 - g) * t, b + (255 - b) * t))


def head_hw(y):
    """Half-width of the face at row y (None outside the head)."""
    table = {7: 4.5, 8: 6.5, 9: 7.5, 19: 8.5, 20: 8.0, 21: 7.5, 22: 7.0, 23: 6.5, 24: 5.5, 25: 4.5, 26: 3.5, 27: 2.5}
    if 10 <= y <= 18:
        return 8.5
    return table.get(y)


class Canvas:
    def __init__(self):
        self.g = [["." for _ in range(W)] for _ in range(H)]

    def put(self, x, y, c):
        if 0 <= x < W and 0 <= y < H:
            self.g[y][x] = c

    def get(self, x, y):
        return self.g[y][x] if 0 <= x < W and 0 <= y < H else "."

    def fill(self, pred, c):
        for y in range(H):
            for x in range(W):
                if pred(x, y):
                    self.g[y][x] = c

    def outline(self, c="k"):
        add = []
        for y in range(H):
            for x in range(W):
                if self.g[y][x] == "." and any(self.get(x + dx, y + dy) not in ".k" for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))):
                    add.append((x, y))
        for x, y in add:
            # no outline along the bottom edge of the bust
            if y < H - 1:
                self.g[y][x] = c

    def rows(self):
        return ["".join(r) for r in self.g]


# ----------------------------------------------------------------------------- layers

def body(cv, spec):
    outfit = spec["outfit"]
    extras = spec["extras"]

    def shoulders(x, y):
        return y >= 30 and abs(x - CX) <= min(16, 6 + (y - 29) * 2.4)

    cv.fill(shoulders, "c")
    # light from the upper left: right side and lower edge in shadow
    cv.fill(lambda x, y: shoulders(x, y) and (x - CX) > min(16, 6 + (y - 29) * 2.4) - 3, "C")

    def v_open(y, top, slope):
        return 1.5 + (y - top) * slope

    if outfit in ("trench", "suit", "uniform", "vest"):
        # shirt in a V, tie down the middle
        cv.fill(lambda x, y: 30 <= y and abs(x - CX) <= max(0, 3.2 - (y - 30) * 0.35) and y <= 38, "t")
        if outfit != "uniform":
            cv.fill(lambda x, y: y >= 30 and x in (15, 16), "a")
            cv.put(15, 30, "A"); cv.put(16, 30, "A")
        # lapels
        for y in range(30, 40):
            d = max(0, 3.2 - (y - 30) * 0.35)
            for side in (-1, 1):
                x = int(round(CX + side * (d + 1.0)))
                cv.put(x, y, "C" if side > 0 else "l")
    if outfit == "trench":
        # popped collar around the neck
        for y in range(26, 31):
            for side in (-1, 1):
                for w in range(2):
                    cv.put(int(CX + side * (4.5 + w + (30 - y) * 0.2)), y, "c" if side < 0 else "C")
        # belt line
        cv.fill(lambda x, y: y == 39 and abs(x - CX) > 4, "C")
    if outfit == "uniform":
        for y in (31, 33, 35, 37, 39):
            cv.put(15, y, "o")
        cv.put(9, 33, "o"); cv.put(10, 33, "o"); cv.put(9, 34, "O"); cv.put(10, 34, "O")
        cv.fill(lambda x, y: y in (29, 30) and 4 <= abs(x - CX) <= 6, "C")
    if outfit == "suit" and "pinstripe" in extras:
        cv.fill(lambda x, y: y >= 31 and cv.get(x, y) in "cC" and (x % 3 == 0), "l")
    if outfit == "suit":
        cv.put(9, 33, "a"); cv.put(10, 33, "a")
    if outfit == "leather":
        cv.fill(lambda x, y: y >= 30 and x == 15, "o")
        cv.fill(lambda x, y: y >= 31 and cv.get(x, y) == "c" and (x + y) % 7 == 0 and x < 13, "l")
        cv.fill(lambda x, y: y in (28, 29, 30) and 4 <= abs(x - CX) <= 5.5, "c")
    if outfit == "dress":
        # bare collarbone, high collar sides, necklace
        cv.fill(lambda x, y: 30 <= y <= 33 and abs(x - CX) <= 5 - (y - 30), "s")
        cv.fill(lambda x, y: y in (27, 28, 29, 30) and 4.5 <= abs(x - CX) <= 6, "c")
        for x, y in ((12, 31), (13, 32), (14, 33), (15, 33), (16, 33), (17, 33), (18, 32), (19, 31)):
            cv.put(x, y, "o")
        cv.put(15, 34, "a"); cv.put(16, 34, "a")
    if outfit == "vest":
        # shirt sleeves out to the sides, vest in the middle with buttons
        sleeve = lambda x, y: y >= 31 and abs(x - CX) > 8 and cv.get(x, y) != "."
        cv.fill(lambda x, y: sleeve(x, y), "t")
        cv.fill(lambda x, y: sleeve(x, y) and (x > CX + 11 or (x + 2 * y) % 6 == 0 or y == 31), "T")
        for y in (34, 37):
            cv.put(13, y, "o")
    if outfit == "rags":
        cv.fill(lambda x, y: y >= 31 and cv.get(x, y) == "c" and (x * 7 + y * 3) % 11 == 0, "l")


def neck(cv):
    cv.fill(lambda x, y: 24 <= y <= 31 and abs(x - CX) <= 3.5, "s")
    cv.fill(lambda x, y: 26 <= y <= 27 and abs(x - CX) <= 3.5, "S")
    cv.fill(lambda x, y: 24 <= y <= 31 and x == 19, "S")


def head(cv, spec):
    cv.fill(lambda x, y: head_hw(y) is not None and abs(x - CX) <= head_hw(y), "s")
    # ears
    for y in range(15, 19):
        cv.put(6, y, "s"); cv.put(25, y, "S")
    cv.put(6, 16, "S")
    # shading: right edge, jaw underside, nose
    for y in range(7, 28):
        hw = head_hw(y)
        cv.put(int(CX + hw), y, "S")
    for x in range(11, 21):
        if cv.get(x, 27) == "s":
            cv.put(x, 27, "S")
    for x, y in ((16, 17), (16, 18), (15, 18)):
        cv.put(x, y, "S")
    cv.put(16, 19, "d")
    extras = spec["extras"]
    if "freckles" in extras:
        for x, y in ((10, 18), (12, 18), (11, 19), (19, 18), (21, 18), (20, 19)):
            cv.put(x, y, "S")
    if "wrinkles" in extras:
        for x, y in ((9, 15), (9, 16), (22, 15), (22, 16), (12, 10), (13, 10), (14, 10), (17, 10), (18, 10), (19, 10)):
            cv.put(x, y, "S")
    if "stubble" in extras or "beard" in extras:
        dense = "beard" in extras
        for y in range(20, 28):
            for x in range(W):
                if cv.get(x, y) in "sS" and (y >= 22 or abs(x - CX) > 4.5):
                    if dense or (x + y) % 2 == 0:
                        cv.put(x, y, "h" if dense else "u")
    if "mustache" in extras:
        for x in range(13, 19):
            cv.put(x, 19, "h")
        cv.put(12, 20, "h"); cv.put(19, 20, "h")
    if "scar" in extras:
        for x, y in ((9, 14), (10, 15), (10, 16), (11, 17), (11, 18), (12, 19)):
            cv.put(x, y, "r")


def hair_back(cv, spec):
    style = spec["hair_style"]
    if style == "long":
        cv.fill(lambda x, y: 8 <= y <= 34 and abs(x - CX) <= 11.5 - max(0, y - 30) * 0.8, "j")
    if style == "bob":
        cv.fill(lambda x, y: 8 <= y <= 25 and abs(x - CX) <= 10.5, "j")


def hair_front(cv, spec):
    style = spec["hair_style"]
    hat = spec["hat"] != "none"

    def cap(top, hairline, grow):
        """Dome over the skull from row `top` down to `hairline`."""
        rx, ry, cy = 9.0 + grow, 13.0 - top, 13.0

        def f(x, y):
            if y < top or y > hairline:
                return False
            return ((x - CX) / rx) ** 2 + ((y + 0.5 - cy) / ry) ** 2 <= 1.0
        return f

    def sheen(rows, x0, x1, step=2):
        """Light band on the upper-left of the hair."""
        for y in rows:
            for x in range(x0, x1):
                if cv.get(x, y) == "h" and (x + y) % step == 0:
                    cv.put(x, y, "H")

    if style == "bald":
        # just a fringe of hair above the ears
        cv.fill(lambda x, y: 12 <= y <= 16 and 7.5 <= abs(x - CX) <= 9.5, "h")
        return
    if style == "short":
        cv.fill(cap(5, 9, 0.5), "h")
        cv.fill(lambda x, y: 10 <= y <= 14 and 7.5 <= abs(x - CX) <= 9, "h")
        sheen((6, 7), 9, 15, 3)
    elif style == "slick":
        cv.fill(cap(5, 9, 0.8), "h")
        cv.fill(lambda x, y: 10 <= y <= 15 and 7.5 <= abs(x - CX) <= 9, "h")
        # combed-back sheen
        sheen((6,), 9, 16, 1)
        sheen((7,), 8, 13, 1)
        sheen((8,), 22, 24, 1)
    elif style == "undercut":
        cv.fill(cap(3, 10, 0.3), "h")
        # swept fringe falling to the right
        for i, y in enumerate(range(10, 14)):
            for x in range(16 + i, 23):
                cv.put(x, y, "h")
        cv.fill(lambda x, y: 10 <= y <= 16 and 7.5 <= abs(x - CX) <= 9, "u")
        sheen((4, 5), 10, 17, 1)
        sheen((6,), 9, 13, 2)
    elif style in ("messy", "long", "bob"):
        cv.fill(cap(4, 10, 1.2), "h")
        # spikes on top, only where they grow out of the dome
        for x in range(9, 23, 3):
            if cv.get(x, 4) == "h":
                cv.put(x, 3, "h"); cv.put(x + 1, 3, "h"); cv.put(x, 2, "h")
        # fringe strands
        for x in range(8, 24):
            if x % 3 != 1:
                cv.put(x, 11, "h")
            if x % 4 == 0:
                cv.put(x, 12, "h")
        cv.fill(lambda x, y: 11 <= y <= 18 and 7.5 <= abs(x - CX) <= 9.5, "h")
        sheen((5, 6), 9, 15, 2)
        if style == "long":
            cv.fill(lambda x, y: 12 <= y <= 33 and 8 <= abs(x - CX) <= 11 - max(0, y - 29) * 0.7, "h")
            cv.fill(lambda x, y: 12 <= y <= 33 and 10 <= (x - CX) <= 11, "j")
        if style == "bob":
            cv.fill(lambda x, y: 11 <= y <= 24 and 8 <= abs(x - CX) <= 10.5, "h")
            cv.fill(lambda x, y: y == 11 and abs(x - CX) <= 8, "h")
    if "gray_temples" in spec["extras"]:
        cv.fill(lambda x, y: 9 <= y <= 15 and abs(x - CX) >= 7.5 and cv.get(x, y) == "h", "H")
    if "neon_streak" in spec["extras"]:
        for i, y in enumerate(range(4, 13)):
            cv.put(11 + i // 3, y, "n")
    if hat:
        # hats cover the top of the head
        cv.fill(lambda x, y: y <= 8, ".")


def hat(cv, spec):
    kind = spec["hat"]
    if kind == "fedora":
        cv.fill(lambda x, y: 2 <= y <= 7 and abs(x - CX) <= 6.5 - (1 if y == 2 else 0), "f")
        cv.put(15, 2, "."); cv.put(16, 2, ".")
        cv.fill(lambda x, y: 2 <= y <= 7 and x - CX > 4, "F")
        cv.fill(lambda x, y: y == 7 and abs(x - CX) <= 6.5, "e")
        cv.fill(lambda x, y: y == 8 and abs(x - CX) <= 12.5, "f")
        cv.fill(lambda x, y: y == 9 and abs(x - CX) <= 11.5, "F")
        # brim shadow across the eyes: that noir look
        cv.fill(lambda x, y: y == 10 and cv.get(x, y) in "sSh", "d")
    elif kind == "cap":
        cv.fill(lambda x, y: 3 <= y <= 8 and abs(x - CX) <= 9.5 - max(0, 5 - y) * 1.5, "f")
        cv.fill(lambda x, y: 3 <= y <= 8 and x - CX > 6, "F")
        cv.fill(lambda x, y: y == 9 and 6 <= x <= 22, "F")
        cv.put(15, 3, "e"); cv.put(16, 3, "e")
    elif kind == "police":
        cv.fill(lambda x, y: 2 <= y <= 7 and abs(x - CX) <= 9 - max(0, 4 - y), "f")
        cv.fill(lambda x, y: y == 7 and abs(x - CX) <= 8, "e")
        cv.fill(lambda x, y: y == 8 and abs(x - CX) <= 8.5, "k")
        cv.fill(lambda x, y: y == 9 and abs(x - CX) <= 7, "k")
        for x, y in ((15, 4), (16, 4), (15, 5), (16, 5)):
            cv.put(x, y, "o")


def front_extras(cv, spec):
    extras = spec["extras"]
    if "glasses" in extras:
        for x0 in (9, 17):
            for x in range(x0, x0 + 6):
                cv.put(x, 17, "q")
            for y in range(14, 17):
                cv.put(x0, y, "q"); cv.put(x0 + 5, y, "q")
        cv.put(15, 14, "q"); cv.put(16, 14, "q")
    if "earrings" in extras:
        cv.put(6, 19, "o"); cv.put(25, 19, "o")
    if "tattoo" in extras:
        for x, y in ((13, 28), (14, 29), (13, 30), (18, 27), (19, 28)):
            cv.put(x, y, "n")
    if "cyber_eye" in extras:
        for x, y in ((23, 12), (24, 13), (24, 14), (23, 18), (24, 17)):
            cv.put(x, y, "n")
    if "cigarette" in extras:
        for x in range(19, 23):
            cv.put(x, 21, "x")
        cv.put(23, 21, "y")
        for x, y in ((24, 19), (25, 17), (24, 15)):
            cv.put(x, y, "z")


# ----------------------------------------------------------------------------- face parts

def eyes_parts(spec):
    lash = "lashes" in spec["extras"]
    right_iris = "n" if "cyber_eye" in spec["extras"] else "i"

    def pair(rows, iris=True):
        out = []
        for r in rows:
            left = r
            right = r[::-1]
            if iris:
                right = right.replace("i", right_iris)
            out.append(left + ".." + right)
        return out

    parts = {
        "open": pair([".kkkk", "kwipw", ".SSS."]),
        "half": pair(["kkkkk", "kwipw", ".SSS."]),
        "closed": pair(["sssss", "kkkkk", ".SSS."]),
        "wide": pair(["kkkkk", "wwpww", "kwwwk"]),
        "narrow": pair(["kkkkk", "kipik", ".kkk."]),
        "happy": pair([".kkk.", "k...k", "....."]),
    }
    if lash:
        for k in ("open", "wide", "half"):
            r = parts[k][0]
            parts[k][0] = "k" + r[1:-1] + "k"
    # pupils: mirror would put them both towards the nose; keep the same glance
    return {k: ((10, 14), v) for k, v in parts.items()}


def brows_parts():
    pair = lambda a, b: [a + ".." + a[::-1], b + ".." + b[::-1]]
    return {
        "neutral": ((10, 12), pair(".....", "vvvvv")),
        "angry": ((10, 12), pair("vv...", "..vvv")),
        "worried": ((10, 12), pair("...vv", "vvv..")),
        "raised": ((10, 12), pair("vvvvv", ".....")),
    }


def mouth_parts():
    return {
        "closed": ((13, 21), ["......", ".mmmm."]),
        "open": ((13, 20), [".kkkk.", ".kbbk.", "..kk.."]),
        "grim": ((13, 21), ["......", "kmmmmk"]),
        "smirk": ((13, 20), [".....m", ".mmmk."]),
        "gasp": ((13, 20), ["..kk..", ".kbbk.", "..kk.."]),
        "smile": ((13, 20), ["m....m", ".mmmm."]),
        "frown": ((13, 21), [".mmmm.", "m....m"]),
    }


EXPRESSIONS = {
    "neutral": {"brows": "neutral", "eyes": "open", "mouth": "closed", "talk": "open"},
    "happy": {"brows": "raised", "eyes": "happy", "mouth": "smile", "talk": "open"},
    "smirk": {"brows": "neutral", "eyes": "half", "mouth": "smirk", "talk": "open"},
    "angry": {"brows": "angry", "eyes": "narrow", "mouth": "grim", "talk": "open"},
    "afraid": {"brows": "worried", "eyes": "wide", "mouth": "gasp", "talk": "open"},
    "sad": {"brows": "worried", "eyes": "half", "mouth": "frown", "talk": "open"},
    "pained": {"brows": "worried", "eyes": "closed", "mouth": "grim", "talk": "gasp"},
    "shocked": {"brows": "raised", "eyes": "wide", "mouth": "gasp", "talk": "gasp"},
}


# ----------------------------------------------------------------------------- assemble

def build(cid, raw):
    spec = {"hair_style": "messy", "hat": "none", "outfit": "suit", "extras": []}
    spec.update(raw)
    for key, allowed in STYLES.items():
        vals = spec[key] if isinstance(spec[key], list) else [spec[key]]
        for v in vals:
            if v not in allowed:
                sys.exit(f"{cid}: {key} '{v}' is not one of {', '.join(allowed)}")
    col = dict(DEFAULTS)
    col.update({k: v for k, v in raw.items() if k in DEFAULTS})
    if col["brows"] is None:
        col["brows"] = col["hair"]

    cv = Canvas()
    hair_back(cv, spec)
    body(cv, spec)
    neck(cv)
    head(cv, spec)
    hair_front(cv, spec)
    hat(cv, spec)
    cv.outline()
    front_extras(cv, spec)
    base = cv.rows()

    palette = {
        "k": "#0b0a10",
        "s": col["skin"], "S": shade(col["skin"], 0.8), "d": shade(col["skin"], 0.55), "u": shade(col["skin"], 0.72),
        "h": col["hair"], "H": shade(col["hair"], 1.35) if "gray_temples" not in spec["extras"] else "#9a9aa2",
        "j": shade(col["hair"], 0.7), "v": col["brows"],
        "w": "#ecebf2", "i": col["eyes"], "p": "#0f0f18", "m": col["lips"], "b": shade(col["lips"], 0.55),
        "c": col["cloth"], "C": shade(col["cloth"], 0.72), "l": shade(col["cloth"], 1.18),
        "t": col["shirt"], "T": shade(col["shirt"], 0.8),
        "a": col["accent"], "A": shade(col["accent"], 0.7),
        "f": col["hat_color"], "F": shade(col["hat_color"], 0.72), "e": shade(col["accent"], 0.8),
        "o": col["metal"], "O": shade(col["metal"], 0.7),
        "r": shade(col["skin"], 0.62), "x": "#e8e4dc", "q": col["glasses"],
    }
    neon = col["neon"]
    cycles = {
        "n": (160, [shade(neon, 0.55), shade(neon, 0.8), neon, shade(neon, 1.3), neon, shade(neon, 0.8)]),
        "y": (120, ["#ff6a00", "#ff3b00", "#ffb000", "#ff3b00"]),
        "z": (300, ["#5a5a62", "#3a3a40", "#26262c", "#3a3a40"]),
    }
    used = set("".join(base))
    parts = {"brows": brows_parts(), "eyes": eyes_parts(spec), "mouth": mouth_parts()}
    for group in parts.values():
        for _, rows in group.values():
            used |= set("".join(rows))
    palette = {k: v for k, v in palette.items() if k in used}
    cycles = {k: v for k, v in cycles.items() if k in used}
    return {
        "name": raw.get("name", cid),
        "base": base,
        "palette": palette,
        "cycles": cycles,
        "parts": parts,
    }


def to_toml(s):
    q = lambda v: '"' + v + '"'
    L = [
        "# Generated by tools/portrait.py from cast.toml. Edit cast.toml and re-run,",
        "# or edit this file by hand (and remove it from cast.toml so it is not overwritten).",
        f"name = {q(s['name'])}",
        f"size = [{W}, {H}]",
        'base = """', *s["base"], '"""', "", "[palette]",
    ]
    L += [f"{k} = {q(v)}" for k, v in s["palette"].items()]
    if s["cycles"]:
        L += ["", "[cycles]"]
        for k, (ms, cols) in s["cycles"].items():
            L.append(f"{k} = {{ ms = {ms}, colors = [{', '.join(q(c) for c in cols)}] }}")
    L += ["", "[anim]", 'blink_eyes = "closed"', "blink_ms = [2200, 5200]", "breathe_ms = 1500",
          "breathe_from_row = 26", "talk_ms = 110"]
    for group, variants in s["parts"].items():
        for name, (at, rows) in variants.items():
            L += ["", f"[parts.{group}.{name}]", f"at = [{at[0]}, {at[1]}]", 'grid = """', *rows, '"""']
    for name, e in EXPRESSIONS.items():
        L += ["", f"[expressions.{name}]"] + [f"{k} = {q(v)}" for k, v in e.items()]
    return "\n".join(L) + "\n"


def compose(s, expr):
    g = [list(r) for r in s["base"]]
    for group in ("brows", "eyes", "mouth"):
        at, rows = s["parts"][group][EXPRESSIONS[expr][group]]
        for dy, row in enumerate(rows):
            for dx, ch in enumerate(row):
                if ch != ".":
                    g[at[1] + dy][at[0] + dx] = ch
    return ["".join(r) for r in g]


def preview(path, sprites, scale=6):
    pal_of = lambda s: {**s["palette"], **{k: c[1][2] for k, c in s["cycles"].items()}}
    exprs = list(EXPRESSIONS)
    cols, rows = len(exprs), len(sprites)
    w, h = cols * (W + 2), rows * (H + 2)
    img = [[(18, 18, 24)] * w for _ in range(h)]
    for r, s in enumerate(sprites):
        pal = pal_of(s)
        for c, e in enumerate(exprs):
            for y, line in enumerate(compose(s, e)):
                for x, ch in enumerate(line):
                    if ch in pal:
                        img[r * (H + 2) + y][c * (W + 2) + x] = hexrgb(pal[ch])
    ppm = path + ".ppm"
    with open(ppm, "wb") as f:
        f.write(f"P6 {w * scale} {h * scale} 255\n".encode())
        for row in img:
            f.write(b"".join(bytes(p) * scale for p in row) * scale)
    subprocess.run(["sips", "-s", "format", "png", ppm, "--out", path + ".png"], capture_output=True)
    os.remove(ppm)


def main():
    story = sys.argv[1]
    prev = sys.argv[sys.argv.index("--preview") + 1] if "--preview" in sys.argv else None
    with open(os.path.join(story, "cast.toml"), "rb") as f:
        cast = tomllib.load(f)
    os.makedirs(os.path.join(story, "characters"), exist_ok=True)
    built = []
    for cid, raw in cast.items():
        s = build(cid, raw)
        with open(os.path.join(story, "characters", f"{cid}.toml"), "w") as f:
            f.write(to_toml(s))
        built.append(s)
        print(f"  {cid}")
    if prev:
        os.makedirs(prev, exist_ok=True)
        preview(os.path.join(prev, os.path.basename(story.rstrip("/"))), built)


if __name__ == "__main__":
    main()
