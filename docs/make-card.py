#!/usr/bin/env python3
"""Generate a title card like the ones in assets/img/posts/ (dark, orange rings, Rubik title).

    cd blog
    python3 ../docs/make-card.py "My Article Title" --source example.com --out assets/img/posts/my-article-title.png

Needs Pillow (`pip install pillow`). The Rubik font is downloaded once to docs/.cache/. The design is the
same as docs/card-template.svg. --style picks another decoration (rings, blocks, grid, stripes) or "auto". Emoji are not in the font, so leave them out of the title.
"""
import argparse
import random
import re
import zlib
import urllib.request
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

FONT_URL = "https://raw.githubusercontent.com/googlefonts/rubik/main/fonts/variable/Rubik%5Bwght%5D.ttf"
FONT_PATH = Path(__file__).parent / ".cache" / "Rubik-var.ttf"
ORANGE = (249, 115, 22)  # primary colour of the "fire" scheme
BACKGROUND = (28, 25, 23)
# the shades of the home page Tetris wells (primary 300-700 of the "fire" scheme)
SHADES = {300: (253, 186, 116), 400: (251, 146, 60), 500: (249, 115, 22), 600: (234, 88, 12), 700: (194, 65, 12)}
SCALE = 2  # draw at 2x and downsample for smooth edges


def font(size, weight):
    if not FONT_PATH.exists():
        FONT_PATH.parent.mkdir(parents=True, exist_ok=True)
        urllib.request.urlretrieve(FONT_URL, FONT_PATH)
    f = ImageFont.truetype(str(FONT_PATH), size)
    f.set_variation_by_axes([weight])
    return f


def wrap(draw, text, fnt, max_width):
    lines, current = [], ""
    for word in text.split():
        trial = f"{current} {word}".strip()
        if draw.textlength(trial, font=fnt) <= max_width:
            current = trial
        else:
            lines.append(current)
            current = word
    return lines + [current]


def glow(img, k, cx, cy, size=1000, strength=0.30):
    """Orange radial glow centred on (cx, cy): `strength` at the centre, fading to nothing."""
    g = Image.radial_gradient("L").resize((size * k, size * k))
    layer = Image.new("RGBA", g.size, ORANGE + (0,))
    # radial_gradient is 0 at the centre and ~180 at the middle of each edge
    layer.putalpha(g.point(lambda v: max(0, int((180 - v) / 180 * 255 * strength))))
    img.alpha_composite(layer, (int((cx - size / 2) * k), int((cy - size / 2) * k)))


def overlay_for(img):
    layer = Image.new("RGBA", img.size, (0, 0, 0, 0))
    return layer, ImageDraw.Draw(layer)


# --- Styles. Each draws only the background decoration; the title and source are drawn afterwards. ---------

def style_rings(img, k, w, h, seed):
    """Glow and thinning rings on the right (the original look)."""
    cx, cy = w - 190, h // 2
    glow(img, k, cx, cy)
    layer, draw = overlay_for(img)
    for i, r in enumerate((90, 150, 210, 270, 330)):
        alpha = int(255 * (0.8 - i * 0.15))
        stroke = max(1, int((7 - i * 1.4) * k))
        draw.ellipse([(cx - r) * k, (cy - r) * k, (cx + r) * k, (cy + r) * k], outline=ORANGE + (alpha,), width=stroke)
    img.alpha_composite(layer)


TETROMINOES = {  # the seven pieces, as (column, row) cells in their spawn rotation
    "I": [(0, 0), (1, 0), (2, 0), (3, 0)],
    "O": [(0, 0), (1, 0), (0, 1), (1, 1)],
    "T": [(0, 0), (1, 0), (2, 0), (1, 1)],
    "S": [(1, 0), (2, 0), (0, 1), (1, 1)],
    "Z": [(0, 0), (1, 0), (1, 1), (2, 1)],
    "J": [(0, 0), (0, 1), (1, 1), (2, 1)],
    "L": [(2, 0), (0, 1), (1, 1), (2, 1)],
}


def rotations(cells):
    """The distinct rotations of a piece, each moved to the top left corner."""
    seen, out = set(), []
    for _ in range(4):
        cells = [(-r, c) for c, r in cells]  # rotate 90 degrees
        min_c, min_r = min(c for c, _ in cells), min(r for _, r in cells)
        cells = sorted((c - min_c, r - min_r) for c, r in cells)
        if tuple(cells) not in seen:
            seen.add(tuple(cells))
            out.append(cells)
    return out


def play_tetris(rnd, cols, rows, pieces, max_height):
    """Plays up to `pieces` real tetrominoes into a well, picking each landing spot with a little noise on top of
    "few holes, low and flat" (the same idea as the home page player). Spots that would complete a line or rise
    above `max_height` are skipped, so the stack stays a low, readable pile. Returns grid[row][col] = piece id
    (or None) and the shade of each piece id."""
    grid = [[None] * cols for _ in range(rows)]
    shades = {}

    def fits(cells, x, y):
        return all(0 <= x + c < cols and y + r < rows and (y + r < 0 or grid[y + r][x + c] is None) for c, r in cells)

    def evaluate(g):
        heights = [next((rows - r for r in range(rows) if g[r][c] is not None), 0) for c in range(cols)]
        holes = sum(1 for c in range(cols) for r in range(rows - heights[c], rows) if g[r][c] is None)
        bump = sum(abs(heights[c] - heights[c - 1]) for c in range(1, cols))
        return -0.5 * sum(heights) - 1.2 * holes - 0.25 * bump + rnd.random() * 1.2

    for pid in range(pieces):
        options = []
        for cells in rotations(TETROMINOES[rnd.choice(list(TETROMINOES))]):
            width = max(c for c, _ in cells) + 1
            for x in range(cols - width + 1):
                y = -4
                if not fits(cells, x, y):
                    continue
                while fits(cells, x, y + 1):
                    y += 1
                if y < 0:
                    continue
                trial = [row[:] for row in grid]
                for c, r in cells:
                    trial[y + r][x + c] = pid
                if any(all(v is not None for v in row) for row in trial) or rows - (y + min(r for _, r in cells)) > max_height:
                    continue
                options.append((evaluate(trial), cells, x, y))
        if not options:
            break
        _, cells, x, y = max(options, key=lambda o: o[0])
        for c, r in cells:
            grid[y + r][x + c] = pid
        # a shade different from every piece it touches, so the pieces read as separate shapes
        touching = {grid[y + r + dr][x + c + dc]
                    for c, r in cells for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1))
                    if 0 <= y + r + dr < rows and 0 <= x + c + dc < cols}
        taken = {shades[t] for t in touching if t in shades}
        shades[pid] = rnd.choice([s for s in SHADES if s not in taken] or list(SHADES))
    return grid, shades


def style_blocks(img, k, w, h, seed):
    """Minimal: real tetrominoes stacked on the right, placed by a seeded player (so each title gets its own
    stack), in the shades of the home page wells. No frame."""
    rnd = random.Random(seed)
    cell, cols, rows = 44, 7, 8
    x0, y0 = w - 80 - cols * cell, (h - rows * cell) // 2  # 80 px margin: clear of the title (wraps at 700 px)
    glow(img, k, x0 + cols * cell / 2, y0 + rows * cell * 0.7, size=1100, strength=0.16)
    layer, draw = overlay_for(img)
    grid, shades = play_tetris(rnd, cols, rows, rnd.randint(8, 11), max_height=5)
    for r in range(rows):
        for c in range(cols):
            pid = grid[r][c]
            if pid is None:
                continue

            # every block is its own rounded square; the four squares of a piece share one shade and sit closer to
            # each other (1 px gap) than to other pieces (4 px), so each piece reads as a unit
            def gap(dc, dr):
                same = 0 <= r + dr < rows and 0 <= c + dc < cols and grid[r + dr][c + dc] == pid
                return 1 if same else 3
            px, py = x0 + c * cell, y0 + r * cell
            draw.rounded_rectangle([(px + gap(-1, 0)) * k, (py + gap(0, -1)) * k, (px + cell - gap(1, 0)) * k,
                                    (py + cell - gap(0, 1)) * k], radius=6 * k, fill=SHADES[shades[pid]] + (225,))
    img.alpha_composite(layer)


def style_grid(img, k, w, h, seed):
    """Dotted grid, a warm glow from the bottom right, an orange tab above the title and a big thin ring."""
    glow(img, k, w - 120, h + 20, size=1300, strength=0.34)
    layer, draw = overlay_for(img)
    step = 40
    for gx in range(step // 2, w, step):
        for gy in range(step // 2, h, step):
            fade = min(1, 0.35 + gx / w)  # dots get stronger towards the right
            r = 2 * k
            draw.ellipse([gx * k - r, gy * k - r, gx * k + r, gy * k + r], fill=(255, 255, 255, int(46 * fade)))
    draw.ellipse([(w - 120 - 300) * k, (h - 300 + 20) * k, (w - 120 + 300) * k, (h + 300 + 20) * k],
                 outline=ORANGE + (150,), width=3 * k)
    draw.rounded_rectangle([80 * k, 70 * k, 170 * k, 78 * k], radius=4 * k, fill=ORANGE + (255,))
    img.alpha_composite(layer)


def style_stripes(img, k, w, h, seed):
    """Diagonal orange bands on the right that fade out towards the title."""
    glow(img, k, w - 60, 40, size=1200, strength=0.28)
    layer, draw = overlay_for(img)
    run = int(h * 0.6)  # horizontal run of each band: keeps them clear of the title and the source line
    for i in range(6):
        offset = w - 30 - i * 66
        width = max(8, 36 - i * 5)
        alpha = int(255 * max(0.14, 0.85 - i * 0.13))
        pts = [(offset, -10), (offset + width, -10), (offset + width - run - 10, h + 10), (offset - run - 10, h + 10)]
        draw.polygon([(x * k, y * k) for x, y in pts], fill=ORANGE + (alpha,))
    img.alpha_composite(layer)


STYLES = {"rings": style_rings, "blocks": style_blocks, "grid": style_grid, "stripes": style_stripes}


def make_card(title, source, width, height, style="rings"):
    if style == "auto":  # the same title always gets the same style; different titles spread over all of them
        style = list(STYLES)[zlib.crc32(title.encode()) % len(STYLES)]
    k = SCALE
    img = Image.new("RGBA", (width * k, height * k), BACKGROUND + (255,))
    STYLES[style](img, k, width, height, zlib.crc32(title.encode()))

    img = img.resize((width, height), Image.LANCZOS).convert("RGB")
    draw = ImageDraw.Draw(img)

    title_font = font(68, 700)
    lines = wrap(draw, title, title_font, 700)
    line_height = 84
    y = height / 2 - line_height * len(lines) / 2 - 20
    for line in lines:
        draw.text((80, y), line, font=title_font, fill=(255, 255, 255))
        y += line_height
    if source:
        draw.text((80, height - 95), source, font=font(34, 500), fill=ORANGE)
    return img


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("title")
    p.add_argument("--source", default="", help='small label under the title, e.g. "example.com"')
    p.add_argument("--out", required=True, help="output PNG path")
    p.add_argument("--style", default="rings", choices=[*STYLES, "auto"],
                   help="background decoration (default rings); auto picks one from the title so covers differ")
    p.add_argument("--size", default="1200x600", help="WIDTHxHEIGHT (default 1200x600, the 2:1 card ratio)")
    args = p.parse_args()
    if re.search(r"[\U0001F000-\U0001FAFF☀-➿]", args.title):
        p.error("the title contains emoji, which Rubik does not have; remove them")
    w, h = (int(n) for n in args.size.lower().split("x"))
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    make_card(args.title, args.source, w, h, args.style).save(out, optimize=True)
    print(f"wrote {out} ({w}x{h})")


if __name__ == "__main__":
    main()
