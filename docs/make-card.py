#!/usr/bin/env python3
"""Generate a title card like the ones in assets/img/posts/ (dark, orange rings, Rubik title).

    cd blog
    python3 ../docs/make-card.py "My Article Title" --source example.com --out assets/img/posts/my-article-title.png

Needs Pillow (`pip install pillow`). The Rubik font is downloaded once to docs/.cache/. The design is the
same as docs/card-template.svg. Emoji are not in the font, so leave them out of the title.
"""
import argparse
import re
import urllib.request
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

FONT_URL = "https://raw.githubusercontent.com/googlefonts/rubik/main/fonts/variable/Rubik%5Bwght%5D.ttf"
FONT_PATH = Path(__file__).parent / ".cache" / "Rubik-var.ttf"
ORANGE = (249, 115, 22)  # primary colour of the "fire" scheme
BACKGROUND = (28, 25, 23)
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


def make_card(title, source, width, height):
    k = SCALE
    cx, cy = width - 190, height // 2  # ring centre
    img = Image.new("RGBA", (width * k, height * k), BACKGROUND + (255,))

    # glow: radial_gradient is 0 at the centre and ~180 at the middle of each edge, so map 0..180 -> 30%..0
    size = 1000
    g = Image.radial_gradient("L").resize((size * k, size * k))
    glow = Image.new("RGBA", g.size, ORANGE + (0,))
    glow.putalpha(g.point(lambda v: max(0, int((180 - v) / 180 * 255 * 0.30))))
    img.alpha_composite(glow, (int((cx - size / 2) * k), int((cy - size / 2) * k)))

    # rings: radius 90..330, thinner and fainter outwards
    overlay = Image.new("RGBA", img.size, (0, 0, 0, 0))
    draw = ImageDraw.Draw(overlay)
    for i, r in enumerate((90, 150, 210, 270, 330)):
        alpha = int(255 * (0.8 - i * 0.15))
        stroke = max(1, int((7 - i * 1.4) * k))
        draw.ellipse([(cx - r) * k, (cy - r) * k, (cx + r) * k, (cy + r) * k], outline=ORANGE + (alpha,), width=stroke)
    img.alpha_composite(overlay)

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
    p.add_argument("--size", default="1200x600", help="WIDTHxHEIGHT (default 1200x600, the 2:1 card ratio)")
    args = p.parse_args()
    if re.search(r"[\U0001F000-\U0001FAFF☀-➿]", args.title):
        p.error("the title contains emoji, which Rubik does not have; remove them")
    w, h = (int(n) for n in args.size.lower().split("x"))
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    make_card(args.title, args.source, w, h).save(out, optimize=True)
    print(f"wrote {out} ({w}x{h})")


if __name__ == "__main__":
    main()
