#! Claude wrote this script. It just breaks a variable font into distinct weights for use with GPUI.

#!/usr/bin/env python3
"""Instance variable fonts into static weight faces.

GPUI's text system does not drive a variable font's `wght` axis: it registers
the file as a single face pinned at the font's *default* axis position, and
`.font_weight(...)` only selects between discrete registered faces. So a raw
variable font always renders at one weight (its default). To get real weight
selection we bake out static instances, one per weight, all sharing a single
family name with a distinct usWeightClass so GPUI can match them.

Usage:
    python instance_fonts.py            # instance every font in FONTS
    python instance_fonts.py satoshi    # only the entries whose key matches
"""

import sys
from pathlib import Path

from fontTools import ttLib
from fontTools.varLib.instancer import instantiateVariableFont

ASSETS = Path(__file__).resolve().parents[1] / "src/frontend/assets/fonts"

# Each entry: key -> config dict.
#   src:      variable font path (relative to ASSETS)
#   family:   shared family name applied to every instance
#   prefix:   filename/PostScript prefix for the output faces
#   extra:    fixed values for any non-weight axes (e.g. Cal Sans "GEOM")
#   weights:  list of (subfamily, wght, usWeightClass, output filename)
FONTS = {
    "cal-sans-ui": {
        "src": "cal-sans-ui/CalSansUI.wght.GEOM.ttf",
        "family": "Cal Sans UI",
        "prefix": "CalSansUI",
        "extra": {"GEOM": 0.0},  # 0=UI, 50=Text, 100=Geo
        "weights": [
            ("Light", 300, 300, "CalSansUI-Light.ttf"),
            ("Regular", 400, 400, "CalSansUI-Regular.ttf"),
            ("Medium", 490, 500, "CalSansUI-Medium.ttf"),
            ("SemiBold", 570, 600, "CalSansUI-SemiBold.ttf"),
            ("Bold", 700, 700, "CalSansUI-Bold.ttf"),
        ],
    },
    "satoshi": {
        "src": "satoshi/Satoshi-Variable.ttf",
        "family": "Satoshi",
        "prefix": "Satoshi",
        "extra": {},  # only a wght axis (300..900, default 900)
        "weights": [
            ("Light", 300, 300, "Satoshi-Light.ttf"),
            ("Regular", 400, 400, "Satoshi-Regular.ttf"),
            ("Medium", 500, 500, "Satoshi-Medium.ttf"),
            ("Bold", 700, 700, "Satoshi-Bold.ttf"),
            ("Black", 900, 900, "Satoshi-Black.ttf"),
        ],
    },
}


def set_name(name_table, name_id, value):
    name_table.setName(value, name_id, 3, 1, 0x409)  # Windows/Unicode/en-US
    name_table.setName(value, name_id, 1, 0, 0)      # Mac/Roman/en


def instance_font(key, cfg):
    src = ASSETS / cfg["src"]
    out_dir = src.parent
    family = cfg["family"]
    prefix = cfg["prefix"]
    extra = cfg.get("extra", {})

    for subfamily, wght, weight_class, out_name in cfg["weights"]:
        font = ttLib.TTFont(str(src))
        instantiateVariableFont(font, {"wght": wght, **extra}, inplace=True)

        # Force a single shared family so GPUI matches by weight, not family.
        name = font["name"]
        set_name(name, 1, family)                       # Family
        set_name(name, 2, subfamily)                    # Subfamily
        set_name(name, 4, f"{family} {subfamily}")      # Full name
        set_name(name, 6, f"{prefix}-{subfamily}")      # PostScript name
        set_name(name, 16, family)                      # Typographic family
        set_name(name, 17, subfamily)                   # Typographic subfamily

        if "OS/2" in font:
            font["OS/2"].usWeightClass = weight_class

        out_path = out_dir / out_name
        font.save(str(out_path))
        print(f"[{key}] wrote {out_name}  (wght={wght}, usWeightClass={weight_class})")


def main():
    filt = sys.argv[1] if len(sys.argv) > 1 else None
    for key, cfg in FONTS.items():
        if filt and filt not in key:
            continue
        instance_font(key, cfg)


if __name__ == "__main__":
    main()
