"""Make the README's demo, .github/media/lore.svg: an animated SVG of lore
itself, so it plays on GitHub (a video in the repository does not) and stays
small and sharp.

It runs app/lore in a private tmux server, in an empty home folder (no
history, no bookmarks, so nothing of yours is in it; internet on, for the
green dot), presses
the keys in SCENES, captures the screen with its colours after each, and
lays the captures out as frames shown one after another.

    cli/make && python3 scripts/demo/demo.py     # from the repository root

Needs tmux. Nothing else: the SVG is written by hand.
"""

from __future__ import annotations

import html
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LORE = ROOT / "app" / "lore"
OUT = ROOT / ".github" / "media" / "lore.svg"
COLS, ROWS = 100, 30
PALETTE = "tokyo-night"
SOCKET = "lore-demo"
CELL_W, CELL_H, FONT = 8.4, 17, 14  # px: one column, one row, the font size
PAD = 14
SETTLE = 0.6  # seconds after a key before the screen is captured

# (keys sent to tmux, seconds the result stays on screen); a key list of
# one literal string is typed letter by letter, a frame each
SCENES = [
    ([], 2.0),
    (["/"], 0.6),
    ("compress folder", 0.15),
    ([], 2.4),
    (["Escape"], 0.4),
    (["Enter"], 2.4),
    (["Tab"], 1.6),
    (["Tab"], 2.0),
    (["c"], 2.0),
    (["Down"], 0.8),
    (["Enter"], 2.0),
    (["Left"], 0.6),
    (["/"] + ["BSpace"] * 15 + ["Escape"], 0.6),  # the search emptied
    (["5"], 2.0),  # the tools topic
    (["Enter"], 2.0),
    (["Tab"], 1.4),
    (["Tab"], 2.0),
    (["?"], 3.0),
    (["Escape"], 0.4),
    (["Left"], 0.4),
    (["3"], 2.4),  # languages
]

SGR = re.compile(r"\x1b\[([0-9;]*)m")
BASIC = [
    "#15161e", "#f7768e", "#9ece6a", "#e0af68", "#7aa2f7", "#bb9af7", "#7dcfff", "#a9b1d6",
    "#414868", "#f7768e", "#9ece6a", "#e0af68", "#7aa2f7", "#bb9af7", "#7dcfff", "#c0caf5",
]


def tmux(*args: str) -> str:
    return subprocess.run(
        ["tmux", "-L", SOCKET, "-f", "/dev/null", *args],
        check=True, capture_output=True, text=True,
    ).stdout


def colour_256(n: int) -> str:
    if n < 16:
        return BASIC[n]
    if n < 232:
        n -= 16
        steps = [0, 95, 135, 175, 215, 255]
        return "#%02x%02x%02x" % (steps[n // 36], steps[n // 6 % 6], steps[n % 6])
    grey = 8 + (n - 232) * 10
    return "#%02x%02x%02x" % (grey, grey, grey)


def cells(capture: str, fg0: str, bg0: str) -> list[list[tuple[str, str, str, bool]]]:
    """Each row's cells: (character, foreground, background, bold)."""
    rows = []
    for line in capture.split("\n")[:ROWS]:
        fg, bg, bold, reverse = fg0, bg0, False, False
        row: list[tuple[str, str, str, bool]] = []
        at = 0
        for m in SGR.finditer(line + "\x1b[m"):
            for ch in line[at:m.start()]:
                f, b = (bg, fg) if reverse else (fg, bg)
                row.append((ch, f, b, bold))
            at = m.end()
            codes = [int(c) if c else 0 for c in m.group(1).split(";")]
            i = 0
            while i < len(codes):
                c = codes[i]
                if c == 0:
                    fg, bg, bold, reverse = fg0, bg0, False, False
                elif c == 1:
                    bold = True
                elif c == 22:
                    bold = False
                elif c == 7:
                    reverse = True
                elif c == 27:
                    reverse = False
                elif 30 <= c <= 37 or 90 <= c <= 97:
                    fg = BASIC[c - 30 if c < 90 else c - 82]
                elif 40 <= c <= 47 or 100 <= c <= 107:
                    bg = BASIC[c - 40 if c < 100 else c - 92]
                elif c == 39:
                    fg = fg0
                elif c == 49:
                    bg = bg0
                elif c in (38, 48) and i + 1 < len(codes):
                    if codes[i + 1] == 5 and i + 2 < len(codes):
                        value = colour_256(codes[i + 2])
                        i += 2
                    elif codes[i + 1] == 2 and i + 4 < len(codes):
                        value = "#%02x%02x%02x" % tuple(codes[i + 2:i + 5])
                        i += 4
                    else:
                        value = fg0 if c == 38 else bg0
                    if c == 38:
                        fg = value
                    else:
                        bg = value
                i += 1
        row = row[:COLS]
        row += [(" ", fg0, bg0, False)] * (COLS - len(row))
        rows.append(row)
    while len(rows) < ROWS:
        rows.append([(" ", fg0, bg0, False)] * COLS)
    return rows


def frame_svg(rows, bg0: str) -> str:
    """One screen: background runs as rects, text runs as tspans."""
    parts = []
    for y, row in enumerate(rows):
        top = y * CELL_H
        x = 0
        while x < COLS:  # backgrounds, joined while the colour holds
            end = x
            while end < COLS and row[end][2] == row[x][2]:
                end += 1
            if row[x][2] != bg0:
                parts.append(
                    f'<rect x="{x * CELL_W:.1f}" y="{top}" width="{(end - x) * CELL_W:.1f}" '
                    f'height="{CELL_H}" fill="{row[x][2]}"/>'
                )
            x = end
        spans = []
        x = 0
        while x < COLS:  # text, joined while colour and weight hold
            end = x
            while end < COLS and row[end][1:4:2] == row[x][1:4:2]:
                end += 1
            text = "".join(c[0] for c in row[x:end])
            if text.strip():
                weight = ' font-weight="bold"' if row[x][3] else ""
                spans.append(
                    f'<tspan x="{x * CELL_W:.1f}" fill="{row[x][1]}"{weight}>'
                    f"{html.escape(text)}</tspan>"
                )
            x = end
        if spans:
            parts.append(f'<text y="{top + CELL_H - 4}" xml:space="preserve">{"".join(spans)}</text>')
    return "".join(parts)


def palette_colours() -> tuple[str, str]:
    text = (ROOT / "palettes" / f"{PALETTE}.palette").read_text(encoding="utf-8")
    found = dict(re.findall(r"^\s*(\w+)\s*=\s*(#[0-9a-fA-F]{6})", text, re.M))
    return found.get("text", "#c0caf5"), found.get("background", "#1a1b26")


def capture_scenes() -> list[tuple[str, float]]:
    home = Path(tempfile.mkdtemp(prefix="lore-demo-"))
    (home / ".config" / "lore").mkdir(parents=True)
    (home / ".config" / "lore" / "config").write_text("history = off\n")
    env = (
        f"env -i HOME={home} PATH=/usr/local/bin:/usr/bin:/bin TERM=xterm-256color "
        f"COLORTERM=truecolor LANG=C.UTF-8 LORE_PALETTE={PALETTE} {LORE}"
    )
    frames = []
    try:
        tmux("new-session", "-d", "-s", "demo", "-x", str(COLS), "-y", str(ROWS), env)
        time.sleep(1.5)
        for keys, hold in SCENES:
            if isinstance(keys, str):
                for letter in keys:
                    tmux("send-keys", "-t", "demo", "-l", letter)
                    time.sleep(0.15)
                    frames.append((tmux("capture-pane", "-e", "-p", "-t", "demo"), hold))
                continue
            for key in keys:
                tmux("send-keys", "-t", "demo", key)
                time.sleep(0.05)
            time.sleep(SETTLE)
            frames.append((tmux("capture-pane", "-e", "-p", "-t", "demo"), hold))
    finally:
        subprocess.run(["tmux", "-L", SOCKET, "kill-server"], capture_output=True)
        shutil.rmtree(home, ignore_errors=True)
    return frames


def main() -> int:
    if not LORE.exists():
        print("app/lore is missing: run cli/make first", file=sys.stderr)
        return 1
    fg0, bg0 = palette_colours()
    frames = capture_scenes()
    width, height = COLS * CELL_W, ROWS * CELL_H
    total = sum(hold for _, hold in frames)
    # every frame stacked; one animation slides the stack, a step a frame
    stops, at = [], 0.0
    for i, (_, hold) in enumerate(frames):
        stops.append(f"{at / total * 100:.3f}%{{transform:translateY({-i * height}px)}}")
        at += hold
    body = "".join(
        f'<g transform="translate(0 {i * height})">{frame_svg(cells(text, fg0, bg0), bg0)}</g>'
        for i, (text, _) in enumerate(frames)
    )
    svg = (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width + 2 * PAD:.0f}" '
        f'height="{height + 2 * PAD}" viewBox="0 0 {width + 2 * PAD:.0f} {height + 2 * PAD}">'
        f"<style>#s{{animation:play {total:.2f}s step-end infinite}}"
        f"@keyframes play{{{''.join(stops)}}}"
        f"text{{font-family:'JetBrains Mono','DejaVu Sans Mono',Menlo,Consolas,monospace;"
        f"font-size:{FONT}px}}</style>"
        f'<rect width="100%" height="100%" rx="10" fill="{bg0}"/>'
        f'<svg x="{PAD}" y="{PAD}" width="{width:.0f}" height="{height}">'
        f'<g id="s">{body}</g></svg></svg>'
    )
    OUT.write_text(svg, encoding="utf-8")
    print(f"{OUT.relative_to(ROOT)}: {len(frames)} frames, {total:.0f} s, {len(svg) // 1024} KB")
    return 0


if __name__ == "__main__":
    sys.exit(main())
