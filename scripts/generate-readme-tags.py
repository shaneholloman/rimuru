#!/usr/bin/env python3
"""
Tempest UI -- Rimuru README design system.

Generates Tensura-themed "Unique Skill Notice" SVGs for the README.
Aesthetic: anime system-window frames with angular L-corner brackets,
diagonal stripe overlays, cyan-glow titles on indigo night-sky gradients.
Feature sections get character-specific glyphs + accent colors:

  Budget Engine       -> Benimaru (flame)        orange
  Runaway Detection   -> Great Sage (eye)        cyan
  Guard Wrapper       -> Shion (katana)          purple
  Output Compression  -> Shuna (cherry blossom)  pink
  Hardware Advisor    -> Veldora (lightning)     gold

Non-skill sections use Rimuru's chibi slime glyph on the default cyan.

Output
------
  docs/assets/rimuru-banner.svg        hero (chibi slime + wordmark)
  docs/assets/tags/*.svg               dark-bg variants (default in README)
  docs/assets/tags/light/*.svg         light-bg variants (shown on dark themes)

Run: python3 scripts/generate-readme-tags.py
"""
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ASSETS_DIR = ROOT / "docs" / "assets"
DARK_DIR = ASSETS_DIR / "tags"
LIGHT_DIR = DARK_DIR / "light"
BANNER_PATH = ASSETS_DIR / "rimuru-banner.svg"

SERIF = "Georgia, 'Crimson Text', 'Times New Roman', serif"
MONO = "'SF Mono', Menlo, Consolas, 'Courier New', monospace"


@dataclass
class Palette:
    variant: str
    bg_a: str
    bg_b: str
    border: str
    title: str
    subtitle: str
    accent: str
    glow: str
    purple: str
    gold: str
    shimmer: str
    common: str
    rare: str
    unique: str
    stripe_op: str
    fade_op: str


DARK = Palette(
    variant="dark",
    bg_a="#0A0E1A",
    bg_b="#141B2E",
    border="#1E293B",
    title="#F8FAFC",
    subtitle="#94A3B8",
    accent="#06B6D4",
    glow="#22D3EE",
    purple="#A78BFA",
    gold="#FBBF24",
    shimmer="#A5F3FC",
    common="#64748B",
    rare="#3B82F6",
    unique="#06B6D4",
    stripe_op="0.06",
    fade_op="0.9",
)

LIGHT = Palette(
    variant="light",
    bg_a="#F8FAFC",
    bg_b="#E0F2FE",
    border="#CBD5E1",
    title="#0F172A",
    subtitle="#475569",
    accent="#0891B2",
    glow="#06B6D4",
    purple="#7C3AED",
    gold="#D97706",
    shimmer="#0369A1",
    common="#64748B",
    rare="#2563EB",
    unique="#0891B2",
    stripe_op="0.05",
    fade_op="0.75",
)


# Character themes for the five "Unique Skill" sections.
# Each theme maps a glyph drawer + a pair of colors per palette variant.
THEMES: dict[str, dict] = {
    "budget": {
        "glyph": "flame",
        "char": "BENIMARU",
        "dark": ("#F97316", "#FB923C"),
        "light": ("#C2410C", "#EA580C"),
    },
    "runaway": {
        "glyph": "eye",
        "char": "GREAT SAGE",
        "dark": ("#06B6D4", "#22D3EE"),
        "light": ("#0891B2", "#06B6D4"),
    },
    "guard": {
        "glyph": "sword",
        "char": "SHION",
        "dark": ("#A78BFA", "#C4B5FD"),
        "light": ("#6D28D9", "#7C3AED"),
    },
    "compression": {
        "glyph": "blossom",
        "char": "SHUNA",
        "dark": ("#EC4899", "#F472B6"),
        "light": ("#BE185D", "#DB2777"),
    },
    "advisor": {
        "glyph": "bolt",
        "char": "VELDORA",
        "dark": ("#EAB308", "#FACC15"),
        "light": ("#A16207", "#CA8A04"),
    },
}


# ---------------------------------------------------------------------------
# Glyph drawers. Each returns an SVG <g> centered at (cx, cy), sized by s.
# ---------------------------------------------------------------------------


def glyph_slime(cx: float, cy: float, s: float, pal: Palette, accent: str, glow: str) -> str:
    r1, r2, r3 = s * 0.9, s * 1.05, s * 0.4
    top = s * 0.85
    bot = s * 0.55
    eye_r = s * 0.14
    eye_dx = s * 0.3
    eye_dy = -s * 0.05
    hl_cx, hl_cy = -s * 0.35, -s * 0.55
    hl_rx, hl_ry = s * 0.26, s * 0.09
    body = (
        f"M -{r1},{s * 0.35} "
        f"C -{r2},0 -{r1},-{s * 0.6} -{r3},-{top} "
        f"C 0,-{s},{r3},-{top} {r1},-{s * 0.6} "
        f"C {r2},0 {r1},{s * 0.35} {s * 0.5},{bot} "
        f"C 0,{s * 0.65} -{s * 0.5},{bot} -{r1},{s * 0.35} Z"
    )
    return (
        f'  <g transform="translate({cx} {cy})">\n'
        f'    <path d="{body}" fill="{accent}" opacity="0.45" stroke="{glow}" stroke-width="1.2" stroke-linejoin="round"/>\n'
        f'    <ellipse cx="{hl_cx}" cy="{hl_cy}" rx="{hl_rx}" ry="{hl_ry}" fill="{pal.shimmer}" opacity="0.85"/>\n'
        f'    <circle cx="-{eye_dx}" cy="{eye_dy}" r="{eye_r}" fill="{pal.title}" opacity="0.92"/>\n'
        f'    <circle cx="{eye_dx}" cy="{eye_dy}" r="{eye_r}" fill="{pal.title}" opacity="0.92"/>\n'
        "  </g>\n"
    )


def glyph_flame(cx: float, cy: float, s: float, pal: Palette, accent: str, glow: str) -> str:
    outer = (
        f"M 0,-{s} "
        f"C {s * 0.35},-{s * 0.55} {s * 0.85},-{s * 0.15} {s * 0.6},{s * 0.45} "
        f"C {s * 0.4},{s * 0.75} 0,{s * 0.85} 0,{s * 0.85} "
        f"C 0,{s * 0.85} -{s * 0.4},{s * 0.75} -{s * 0.6},{s * 0.45} "
        f"C -{s * 0.85},-{s * 0.15} -{s * 0.35},-{s * 0.55} 0,-{s} Z"
    )
    inner = (
        f"M 0,-{s * 0.55} "
        f"C {s * 0.2},-{s * 0.2} {s * 0.4},{s * 0.15} {s * 0.25},{s * 0.4} "
        f"C 0,{s * 0.55} -{s * 0.25},{s * 0.4} -{s * 0.4},{s * 0.15} "
        f"C -{s * 0.2},-{s * 0.2} 0,-{s * 0.55} 0,-{s * 0.55} Z"
    )
    return (
        f'  <g transform="translate({cx} {cy})">\n'
        f'    <path d="{outer}" fill="{accent}" opacity="0.45" stroke="{glow}" stroke-width="1.2" stroke-linejoin="round"/>\n'
        f'    <path d="{inner}" fill="{glow}" opacity="0.9"/>\n'
        f'    <circle cx="0" cy="{s * 0.05}" r="{s * 0.1}" fill="{pal.shimmer}"/>\n'
        "  </g>\n"
    )


def glyph_eye(cx: float, cy: float, s: float, pal: Palette, accent: str, glow: str) -> str:
    return (
        f'  <g transform="translate({cx} {cy})">\n'
        f'    <circle cx="0" cy="0" r="{s * 0.95}" fill="none" stroke="{accent}" stroke-width="1" stroke-dasharray="2 2" opacity="0.65"/>\n'
        f'    <circle cx="0" cy="0" r="{s * 0.7}" fill="{accent}" opacity="0.35"/>\n'
        f'    <circle cx="0" cy="0" r="{s * 0.45}" fill="{glow}"/>\n'
        f'    <circle cx="0" cy="0" r="{s * 0.22}" fill="{pal.bg_a}"/>\n'
        f'    <circle cx="-{s * 0.1}" cy="-{s * 0.1}" r="{s * 0.08}" fill="{pal.shimmer}"/>\n'
        "  </g>\n"
    )


def glyph_sword(cx: float, cy: float, s: float, pal: Palette, accent: str, glow: str) -> str:
    return (
        f'  <g transform="translate({cx} {cy})">\n'
        f'    <rect x="-{s * 0.14}" y="-{s * 0.9}" width="{s * 0.28}" height="{s * 1.28}" fill="{accent}" opacity="0.55" stroke="{glow}" stroke-width="1"/>\n'
        f'    <polygon points="-{s * 0.14},-{s * 0.9} 0,-{s * 1.05} {s * 0.14},-{s * 0.9}" fill="{glow}"/>\n'
        f'    <rect x="-{s * 0.55}" y="{s * 0.38}" width="{s * 1.1}" height="{s * 0.18}" rx="{s * 0.05}" ry="{s * 0.05}" fill="{glow}"/>\n'
        f'    <circle cx="0" cy="{s * 0.47}" r="{s * 0.14}" fill="{pal.shimmer}" opacity="0.85"/>\n'
        f'    <rect x="-{s * 0.09}" y="{s * 0.58}" width="{s * 0.18}" height="{s * 0.35}" fill="{accent}" opacity="0.7"/>\n'
        "  </g>\n"
    )


def glyph_blossom(cx: float, cy: float, s: float, pal: Palette, accent: str, glow: str) -> str:
    petals: list[str] = []
    for angle in (0, 72, 144, 216, 288):
        petals.append(
            f'    <ellipse cx="0" cy="-{s * 0.55}" rx="{s * 0.3}" ry="{s * 0.48}" '
            f'fill="{accent}" opacity="0.55" stroke="{glow}" stroke-width="0.9" '
            f'transform="rotate({angle})"/>'
        )
    return (
        f'  <g transform="translate({cx} {cy})">\n'
        + "\n".join(petals)
        + f'\n    <circle cx="0" cy="0" r="{s * 0.2}" fill="{glow}"/>\n'
        + f'    <circle cx="0" cy="0" r="{s * 0.09}" fill="{pal.shimmer}"/>\n'
        "  </g>\n"
    )


def glyph_bolt(cx: float, cy: float, s: float, pal: Palette, accent: str, glow: str) -> str:
    bolt = (
        f"M -{s * 0.35},-{s} "
        f"L {s * 0.25},-{s * 0.2} "
        f"L -{s * 0.05},-{s * 0.05} "
        f"L {s * 0.35},{s} "
        f"L -{s * 0.25},{s * 0.2} "
        f"L {s * 0.05},{s * 0.05} Z"
    )
    return (
        f'  <g transform="translate({cx} {cy})">\n'
        f'    <circle cx="0" cy="0" r="{s * 0.95}" fill="none" stroke="{accent}" stroke-width="1" stroke-dasharray="2 2" opacity="0.5"/>\n'
        f'    <path d="{bolt}" fill="{accent}" opacity="0.55" stroke="{glow}" stroke-width="1.2" stroke-linejoin="round"/>\n'
        f'    <circle cx="0" cy="0" r="{s * 0.1}" fill="{pal.shimmer}"/>\n'
        "  </g>\n"
    )


GLYPHS = {
    "slime": glyph_slime,
    "flame": glyph_flame,
    "eye": glyph_eye,
    "sword": glyph_sword,
    "blossom": glyph_blossom,
    "bolt": glyph_bolt,
}


# ---------------------------------------------------------------------------
# Shared SVG helpers
# ---------------------------------------------------------------------------


def _defs(pal: Palette) -> str:
    return f"""  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="{pal.bg_a}"/>
      <stop offset="1" stop-color="{pal.bg_b}"/>
    </linearGradient>
    <linearGradient id="titleGrad" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{pal.title}"/>
      <stop offset="1" stop-color="{pal.shimmer}"/>
    </linearGradient>
    <linearGradient id="shim" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{pal.accent}" stop-opacity="0"/>
      <stop offset="0.5" stop-color="{pal.glow}" stop-opacity="{pal.fade_op}"/>
      <stop offset="1" stop-color="{pal.accent}" stop-opacity="0"/>
    </linearGradient>
    <pattern id="grid" patternUnits="userSpaceOnUse" width="10" height="10" patternTransform="rotate(45)">
      <line x1="0" y1="0" x2="0" y2="10" stroke="{pal.accent}" stroke-width="1.4" opacity="{pal.stripe_op}"/>
    </pattern>
  </defs>"""


def _themed_shim(accent: str, glow: str, op: str) -> str:
    return f"""    <linearGradient id="themeShim" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{accent}" stop-opacity="0"/>
      <stop offset="0.5" stop-color="{glow}" stop-opacity="{op}"/>
      <stop offset="1" stop-color="{accent}" stop-opacity="0"/>
    </linearGradient>"""


def _corner_brackets(w: int, h: int, color: str, arm: int = 10) -> str:
    return f"""  <path d="M 3 {arm + 3} L 3 3 L {arm + 3} 3" stroke="{color}" stroke-width="1.5" fill="none" stroke-linecap="square"/>
  <path d="M {w - arm - 3} 3 L {w - 3} 3 L {w - 3} {arm + 3}" stroke="{color}" stroke-width="1.5" fill="none" stroke-linecap="square"/>
  <path d="M 3 {h - arm - 3} L 3 {h - 3} L {arm + 3} {h - 3}" stroke="{color}" stroke-width="1.5" fill="none" stroke-linecap="square"/>
  <path d="M {w - arm - 3} {h - 3} L {w - 3} {h - 3} L {w - 3} {h - arm - 3}" stroke="{color}" stroke-width="1.5" fill="none" stroke-linecap="square"/>"""


def _char_px_serif(size: int, spacing: float) -> float:
    return size * 0.72 + spacing


def _char_px_mono(size: int, spacing: float) -> float:
    return size * 0.62 + spacing


# ---------------------------------------------------------------------------
# Section header (skill notice)
# ---------------------------------------------------------------------------


def skill_notice(
    title: str,
    subtitle: str,
    notice_id: str,
    rank: str,
    pal: Palette,
    glyph_key: str = "slime",
    accent: str | None = None,
    glow: str | None = None,
) -> str:
    accent = accent or pal.accent
    glow = glow or pal.glow

    title_upper = title.upper()
    subtitle_upper = subtitle.upper()
    notice_upper = notice_id.upper()
    rank_upper = rank.upper()

    title_w = len(title_upper) * _char_px_serif(15, 2.4)
    subtitle_w = len(subtitle_upper) * _char_px_mono(9, 1.3)
    notice_w = len(notice_upper) * _char_px_mono(8, 1.2)
    rank_w = len(rank_upper) * _char_px_serif(13, 1.4)

    content_w = max(title_w, subtitle_w)
    right_w = max(notice_w, rank_w) + 10

    crest_area = 54
    width = int(crest_area + 10 + content_w + 14 + right_w + 16)
    width = max(360, width)
    height = 64

    crest_cx = crest_area / 2 + 4
    crest_cy = height / 2
    left_sep_x = crest_area + 4
    content_x = crest_area + 14
    right_sep_x = width - right_w - 14
    right_text_x = right_sep_x + 6

    draw_glyph = GLYPHS[glyph_key]

    defs_block = f"""  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="{pal.bg_a}"/>
      <stop offset="1" stop-color="{pal.bg_b}"/>
    </linearGradient>
    <linearGradient id="titleGrad" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{pal.title}"/>
      <stop offset="1" stop-color="{pal.shimmer}"/>
    </linearGradient>
{_themed_shim(accent, glow, pal.fade_op)}
    <pattern id="grid" patternUnits="userSpaceOnUse" width="10" height="10" patternTransform="rotate(45)">
      <line x1="0" y1="0" x2="0" y2="10" stroke="{accent}" stroke-width="1.4" opacity="{pal.stripe_op}"/>
    </pattern>
  </defs>"""

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        defs_block,
        f'  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" fill="url(#bg)" stroke="{pal.border}"/>',
        f'  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" fill="url(#grid)"/>',
        _corner_brackets(width, height, accent),
        draw_glyph(crest_cx, crest_cy, 15, pal, accent, glow),
        f'  <line x1="{left_sep_x}" y1="14" x2="{left_sep_x}" y2="{height - 14}" stroke="{pal.border}" stroke-width="1" opacity="0.7"/>',
        f'  <text x="{content_x}" y="29" font-family="{SERIF}" font-size="15" font-weight="700" font-style="italic" letter-spacing="2.4" fill="url(#titleGrad)">{title_upper}</text>',
        f'  <text x="{content_x}" y="47" font-family="{MONO}" font-size="9" letter-spacing="1.3" fill="{pal.subtitle}">{subtitle_upper}</text>',
        f'  <line x1="{right_sep_x}" y1="14" x2="{right_sep_x}" y2="{height - 14}" stroke="{pal.border}" stroke-width="1" opacity="0.7"/>',
        f'  <text x="{right_text_x}" y="26" font-family="{MONO}" font-size="8" letter-spacing="1.2" fill="{pal.subtitle}">{notice_upper}</text>',
        f'  <text x="{right_text_x}" y="44" font-family="{SERIF}" font-size="13" font-weight="700" font-style="italic" letter-spacing="1.4" fill="{glow}">{rank_upper}</text>',
        f'  <line x1="14" y1="{height - 4}" x2="{width - 14}" y2="{height - 4}" stroke="url(#themeShim)" stroke-width="1"/>',
        "</svg>",
        "",
    ]
    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Stat card
# ---------------------------------------------------------------------------


def stat_card(value: str, label: str, pal: Palette, color_key: str) -> str:
    width = 184
    height = 80
    color = getattr(pal, color_key)

    num_w = len(value) * 17 + 8

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        _defs(pal),
        f'  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" fill="url(#bg)" stroke="{pal.border}"/>',
        f'  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" fill="url(#grid)"/>',
        _corner_brackets(width, height, pal.accent),
        glyph_slime(width - 24, 22, 9, pal, pal.accent, pal.glow),
        f'  <text x="16" y="24" font-family="{MONO}" font-size="8" letter-spacing="1.6" fill="{pal.subtitle}">SKILL LVL</text>',
        f'  <text x="16" y="54" font-family="{SERIF}" font-style="italic" font-size="28" font-weight="700" fill="{color}">{value}</text>',
        f'  <line x1="16" y1="60" x2="{16 + num_w}" y2="60" stroke="{color}" stroke-width="1.5"/>',
        f'  <text x="16" y="73" font-family="{MONO}" font-size="9" letter-spacing="1.2" fill="{pal.subtitle}">{label.upper()}</text>',
        "</svg>",
        "",
    ]
    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Skill tag (pill)
# ---------------------------------------------------------------------------


def skill_tag(text: str, rarity: str, pal: Palette) -> str:
    rarity_colors = {
        "common": pal.common,
        "rare": pal.rare,
        "unique": pal.unique,
        "legendary": pal.gold,
    }
    color = rarity_colors.get(rarity, pal.accent)

    height = 28
    text_w = len(text) * _char_px_mono(10, 1)
    width = int(text_w + 42)
    cy = height / 2

    hex_pts = f"7,0 {width - 7},0 {width},{cy} {width - 7},{height} 7,{height} 0,{cy}"

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        _defs(pal),
        f'  <polygon points="{hex_pts}" fill="url(#bg)" stroke="{color}" stroke-width="1.2"/>',
        f'  <polygon points="16,7 22,{cy} 16,{height - 7} 10,{cy}" fill="{color}"/>',
        f'  <text x="28" y="18" font-family="{MONO}" font-size="10" font-weight="600" letter-spacing="1.2" fill="{pal.title}">{text}</text>',
        "</svg>",
        "",
    ]
    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Divider
# ---------------------------------------------------------------------------


def divider(pal: Palette) -> str:
    width = 820
    height = 16
    cy = height // 2
    cx = width // 2

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        "  <defs>",
        '    <linearGradient id="line" x1="0" y1="0" x2="1" y2="0">',
        f'      <stop offset="0" stop-color="{pal.accent}" stop-opacity="0"/>',
        f'      <stop offset="0.5" stop-color="{pal.accent}" stop-opacity="0.7"/>',
        f'      <stop offset="1" stop-color="{pal.accent}" stop-opacity="0"/>',
        "    </linearGradient>",
        "  </defs>",
        f'  <line x1="0" y1="{cy}" x2="{width}" y2="{cy}" stroke="url(#line)" stroke-width="1"/>',
        f'  <circle cx="{cx - 20}" cy="{cy}" r="1" fill="{pal.accent}" opacity="0.5"/>',
        f'  <circle cx="{cx - 12}" cy="{cy}" r="1.5" fill="{pal.accent}" opacity="0.7"/>',
        glyph_slime(cx, cy, 4, pal, pal.accent, pal.glow),
        f'  <circle cx="{cx + 12}" cy="{cy}" r="1.5" fill="{pal.accent}" opacity="0.7"/>',
        f'  <circle cx="{cx + 20}" cy="{cy}" r="1" fill="{pal.accent}" opacity="0.5"/>',
        "</svg>",
        "",
    ]
    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Hero banner
# ---------------------------------------------------------------------------


def hero_banner(pal: Palette) -> str:
    """720x240 hero: chibi slime glyph on the left, RIMURU wordmark stacked."""
    width = 720
    height = 240

    slime_cx = 118
    slime_cy = 120
    slime_scale = 1.35

    text_cx = 446

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        "  <defs>",
        '    <radialGradient id="sky" cx="0.5" cy="0.4" r="0.85">',
        f'      <stop offset="0" stop-color="{pal.bg_b}"/>',
        f'      <stop offset="1" stop-color="{pal.bg_a}"/>',
        "    </radialGradient>",
        '    <radialGradient id="aura" cx="0.5" cy="0.5" r="0.5">',
        f'      <stop offset="0" stop-color="{pal.glow}" stop-opacity="0.35"/>',
        f'      <stop offset="1" stop-color="{pal.glow}" stop-opacity="0"/>',
        "    </radialGradient>",
        '    <linearGradient id="rim" x1="0" y1="0" x2="1" y2="1">',
        f'      <stop offset="0" stop-color="{pal.glow}"/>',
        f'      <stop offset="0.5" stop-color="{pal.shimmer}"/>',
        f'      <stop offset="1" stop-color="{pal.purple}"/>',
        "    </linearGradient>",
        '    <linearGradient id="shimTop" x1="0" y1="0" x2="1" y2="0">',
        f'      <stop offset="0" stop-color="{pal.accent}" stop-opacity="0"/>',
        f'      <stop offset="0.5" stop-color="{pal.glow}" stop-opacity="0.9"/>',
        f'      <stop offset="1" stop-color="{pal.accent}" stop-opacity="0"/>',
        "    </linearGradient>",
        '    <pattern id="stars" patternUnits="userSpaceOnUse" width="60" height="60">',
        f'      <circle cx="8" cy="12" r="0.8" fill="{pal.shimmer}" opacity="0.35"/>',
        f'      <circle cx="34" cy="28" r="0.5" fill="{pal.glow}" opacity="0.45"/>',
        f'      <circle cx="52" cy="8" r="0.6" fill="{pal.shimmer}" opacity="0.25"/>',
        f'      <circle cx="22" cy="48" r="0.7" fill="{pal.glow}" opacity="0.35"/>',
        f'      <circle cx="46" cy="44" r="0.5" fill="{pal.shimmer}" opacity="0.45"/>',
        "    </pattern>",
        '    <pattern id="grid" patternUnits="userSpaceOnUse" width="12" height="12" patternTransform="rotate(45)">',
        f'      <line x1="0" y1="0" x2="0" y2="12" stroke="{pal.accent}" stroke-width="1.2" opacity="0.04"/>',
        "    </pattern>",
        "  </defs>",
        f'  <rect x="0" y="0" width="{width}" height="{height}" fill="url(#sky)"/>',
        f'  <rect x="0" y="0" width="{width}" height="{height}" fill="url(#stars)"/>',
        f'  <rect x="0" y="0" width="{width}" height="{height}" fill="url(#grid)"/>',
        f'  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" fill="none" stroke="{pal.border}"/>',
        _corner_brackets(width, height, pal.accent, arm=22),
        # Slime aura
        f'  <circle cx="{slime_cx}" cy="{slime_cy}" r="88" fill="url(#aura)"/>',
        # Slime shadow
        f'  <ellipse cx="{slime_cx}" cy="{slime_cy + int(62 * slime_scale)}" rx="{int(54 * slime_scale)}" ry="5" fill="{pal.bg_a}" opacity="0.55"/>',
        _draw_banner_slime(slime_cx, slime_cy, slime_scale, pal),
        # Decorative crest top-right of banner
        f'  <g transform="translate({width - 60} 52)">',
        f'    <polygon points="-14,-6 -6,-14 6,-14 14,-6 14,6 6,14 -6,14 -14,6" fill="none" stroke="{pal.accent}" stroke-width="1" stroke-dasharray="2 2" opacity="0.55"/>',
        f'    <polygon points="0,-8 8,0 0,8 -8,0" fill="{pal.accent}" opacity="0.35"/>',
        f'    <polygon points="0,-5 5,0 0,5 -5,0" fill="{pal.glow}"/>',
        "  </g>",
        # Wordmark
        f'  <text x="{text_cx}" y="118" text-anchor="middle" font-family="{SERIF}" font-size="78" font-weight="700" font-style="italic" letter-spacing="9" fill="url(#rim)">RIMURU</text>',
        f'  <text x="{text_cx}" y="148" text-anchor="middle" font-family="{MONO}" font-size="11" letter-spacing="6" fill="{pal.subtitle}">TEMPEST SYSTEM</text>',
        f'  <line x1="{text_cx - 100}" y1="162" x2="{text_cx + 100}" y2="162" stroke="url(#shimTop)" stroke-width="1"/>',
        f'  <text x="{text_cx}" y="182" text-anchor="middle" font-family="{MONO}" font-size="10" letter-spacing="2.6" fill="{pal.glow}">UNIQUE SKILL -- COST CONTROL</text>',
        f'  <text x="{text_cx}" y="198" text-anchor="middle" font-family="{MONO}" font-size="9" letter-spacing="1.8" fill="{pal.subtitle}">FOR AI CODING AGENTS</text>',
        "</svg>",
        "",
    ]
    return "\n".join(parts)


def _draw_banner_slime(cx: float, cy: float, scale: float, pal: Palette) -> str:
    """Cute chibi slime for the hero banner. Bigger and more detailed than the
    inline crest version -- adds cheek blushes, eye shines, and a smile."""
    s = scale
    body = (
        f"M {cx - 44 * s},{cy + 20 * s} "
        f"C {cx - 52 * s},{cy - 2 * s} {cx - 48 * s},{cy - 32 * s} {cx - 28 * s},{cy - 44 * s} "
        f"C {cx - 16 * s},{cy - 54 * s} {cx - 4 * s},{cy - 58 * s} {cx},{cy - 60 * s} "
        f"C {cx + 4 * s},{cy - 58 * s} {cx + 16 * s},{cy - 54 * s} {cx + 28 * s},{cy - 44 * s} "
        f"C {cx + 48 * s},{cy - 32 * s} {cx + 52 * s},{cy - 2 * s} {cx + 44 * s},{cy + 20 * s} "
        f"C {cx + 36 * s},{cy + 36 * s} {cx + 16 * s},{cy + 42 * s} {cx},{cy + 44 * s} "
        f"C {cx - 16 * s},{cy + 42 * s} {cx - 36 * s},{cy + 36 * s} {cx - 44 * s},{cy + 20 * s} Z"
    )
    return f"""  <path d="{body}" fill="{pal.accent}" opacity="0.55" stroke="{pal.glow}" stroke-width="1.8" stroke-linejoin="round"/>
  <ellipse cx="{cx - 22 * s}" cy="{cy - 30 * s}" rx="{13 * s}" ry="{6 * s}" fill="{pal.shimmer}" opacity="0.65"/>
  <ellipse cx="{cx - 26 * s}" cy="{cy - 34 * s}" rx="{5 * s}" ry="{2 * s}" fill="{pal.title}" opacity="0.9"/>
  <ellipse cx="{cx - 14 * s}" cy="{cy - 6 * s}" rx="{4.5 * s}" ry="{6 * s}" fill="{pal.bg_a}"/>
  <ellipse cx="{cx + 14 * s}" cy="{cy - 6 * s}" rx="{4.5 * s}" ry="{6 * s}" fill="{pal.bg_a}"/>
  <circle cx="{cx - 13 * s}" cy="{cy - 8 * s}" r="{1.8 * s}" fill="{pal.title}"/>
  <circle cx="{cx + 15 * s}" cy="{cy - 8 * s}" r="{1.8 * s}" fill="{pal.title}"/>
  <circle cx="{cx - 15 * s}" cy="{cy - 4 * s}" r="{0.9 * s}" fill="{pal.title}" opacity="0.7"/>
  <circle cx="{cx + 13 * s}" cy="{cy - 4 * s}" r="{0.9 * s}" fill="{pal.title}" opacity="0.7"/>
  <ellipse cx="{cx - 26 * s}" cy="{cy + 5 * s}" rx="{5 * s}" ry="{2.5 * s}" fill="{pal.purple}" opacity="0.35"/>
  <ellipse cx="{cx + 26 * s}" cy="{cy + 5 * s}" rx="{5 * s}" ry="{2.5 * s}" fill="{pal.purple}" opacity="0.35"/>
  <path d="M {cx - 5 * s},{cy + 8 * s} Q {cx},{cy + 13 * s} {cx + 5 * s},{cy + 8 * s}" stroke="{pal.bg_a}" stroke-width="{1.6 * s}" fill="none" stroke-linecap="round"/>"""


# ---------------------------------------------------------------------------
# Content
# ---------------------------------------------------------------------------


SECTIONS: list[tuple[str, str, str, str, str, str | None]] = [
    # (key, title, subtitle, notice_id, rank, theme_key)
    ("why", "Rimuru", "Cost control for AI coding agents", "NOTICE 000", "PROLOGUE", None),
    ("quickstart", "Quick Start", "One line install - four binaries", "NOTICE 001", "RANK C", None),
    ("agents", "Works With Every Agent", "Six tools - one dashboard", "NOTICE 002", "RANK A", None),
    ("budget", "Budget Engine", "Hard caps - four enforcement levels", "BENIMARU", "RANK SS", "budget"),
    ("runaway", "Runaway Detection", "Four patterns - severity scoring", "GREAT SAGE", "RANK S", "runaway"),
    ("guard", "Guard Wrapper", "Kill agents at the cost limit", "SHION", "RANK S", "guard"),
    ("compression", "Output Compression", "Six strategies - auto routing", "SHUNA", "RANK A", "compression"),
    ("interfaces", "Four Interfaces", "CLI - Web UI - TUI - Desktop", "NOTICE 007", "RANK B", None),
    ("advisor", "Hardware Advisor", "Run models locally - see savings", "VELDORA", "RANK B", "advisor"),
    ("architecture", "Architecture", "Worker - function - trigger", "NOTICE 009", "RANK S", None),
    ("api", "API Reference", "HTTP + iii trigger - same functions", "NOTICE 010", "RANK A", None),
    ("development", "Development", "Build - test - ship", "NOTICE 011", "RANK C", None),
    ("license", "License", "Apache 2.0 - made in public", "NOTICE 012", "RANK -", None),
]

STATS = [
    ("agents", "6", "agents tracked", "glow"),
    ("functions", "60+", "iii functions", "glow"),
    ("interfaces", "4", "interfaces", "purple"),
    ("caps", "4", "budget cap levels", "glow"),
    ("strategies", "6", "compression modes", "purple"),
    ("endpoints", "46", "http endpoints", "glow"),
]

PILLS = [
    ("rust", "RUST 1.85", "unique"),
    ("apache2", "APACHE 2.0", "common"),
    ("v040", "BENIMARU", "legendary"),
    ("iii", "III-ENGINE", "rare"),
    ("tauri", "TAURI V2", "rare"),
]


def generate(pal: Palette, out_dir: Path) -> int:
    out_dir.mkdir(parents=True, exist_ok=True)
    n = 0
    for key, title, subtitle, notice_id, rank, theme_key in SECTIONS:
        if theme_key is not None:
            theme = THEMES[theme_key]
            accent, glow = theme[pal.variant]
            glyph = theme["glyph"]
        else:
            accent, glow, glyph = pal.accent, pal.glow, "slime"
        (out_dir / f"section-{key}.svg").write_text(
            skill_notice(title, subtitle, notice_id, rank, pal, glyph, accent, glow)
        )
        n += 1
    for key, value, label, color_key in STATS:
        (out_dir / f"stat-{key}.svg").write_text(stat_card(value, label, pal, color_key))
        n += 1
    for key, text, rarity in PILLS:
        (out_dir / f"pill-{key}.svg").write_text(skill_tag(text, rarity, pal))
        n += 1
    (out_dir / "divider.svg").write_text(divider(pal))
    n += 1
    return n


def main() -> None:
    dark_n = generate(DARK, DARK_DIR)
    light_n = generate(LIGHT, LIGHT_DIR)
    BANNER_PATH.write_text(hero_banner(DARK))
    print(f"wrote {dark_n} dark tags -> {DARK_DIR.relative_to(ROOT)}")
    print(f"wrote {light_n} light tags -> {LIGHT_DIR.relative_to(ROOT)}")
    print(f"wrote hero banner -> {BANNER_PATH.relative_to(ROOT)}")
    print(f"total: {dark_n + light_n + 1} svg files")


if __name__ == "__main__":
    main()
