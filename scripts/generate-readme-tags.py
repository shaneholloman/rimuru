#!/usr/bin/env python3
"""
Generate the rimuru README SVG tag set.

Produces two parallel trees under docs/assets/tags/:
  - docs/assets/tags/*.svg          (dark-bg variant, default)
  - docs/assets/tags/light/*.svg    (light-bg variant, shown on dark themes)

Tag types:
  - section-*   44px section headers with title + subtitle, 6px accent strip
  - stat-*      140x48 stat cards with big number + label
  - pill-*      rounded pills with dot indicator
  - divider     thin accent line across full width

Run: python3 scripts/generate-readme-tags.py
"""
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DARK_DIR = ROOT / "docs" / "assets" / "tags"
LIGHT_DIR = DARK_DIR / "light"


@dataclass
class Palette:
    bg_start: str
    bg_end: str
    border: str
    title: str
    subtitle: str
    accent: str
    stat_blue: str
    stat_green: str
    stat_orange: str


DARK = Palette(
    bg_start="#1A1A1A",
    bg_end="#0F0F0F",
    border="#2A2A2A",
    title="#FFFFFF",
    subtitle="#9CA3AF",
    accent="#6366F1",
    stat_blue="#818CF8",
    stat_green="#34D399",
    stat_orange="#FB923C",
)

LIGHT = Palette(
    bg_start="#FFFFFF",
    bg_end="#F8F9FB",
    border="#E5E7EB",
    title="#0F172A",
    subtitle="#64748B",
    accent="#4F46E5",
    stat_blue="#4F46E5",
    stat_green="#059669",
    stat_orange="#EA580C",
)


def _measure_text(text: str, avg_char_px: float) -> float:
    return len(text) * avg_char_px


def section_header(title: str, subtitle: str, pal: Palette) -> str:
    height = 44
    title_px = _measure_text(title.upper(), 7.2)
    subtitle_px = _measure_text(subtitle, 5.6)
    content_px = max(title_px, subtitle_px)
    width = max(200, int(content_px + 60))
    accent_w = 6
    text_x = accent_w + 14

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="{pal.bg_start}"/>
      <stop offset="1" stop-color="{pal.bg_end}"/>
    </linearGradient>
  </defs>
  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" rx="6" ry="6" fill="url(#bg)" stroke="{pal.border}"/>
  <rect x="0.5" y="0.5" width="{accent_w}" height="{height - 1}" rx="3" ry="3" fill="{pal.accent}"/>
  <text x="{text_x}" y="19" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif" font-size="13" font-weight="700" letter-spacing="0.8" fill="{pal.title}">{title.upper()}</text>
  <text x="{text_x}" y="34" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif" font-size="11" font-weight="400" fill="{pal.subtitle}">{subtitle}</text>
</svg>
"""


def stat_card(value: str, label: str, color: str, pal: Palette) -> str:
    width = 160
    height = 56

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="{pal.bg_start}"/>
      <stop offset="1" stop-color="{pal.bg_end}"/>
    </linearGradient>
  </defs>
  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" rx="8" ry="8" fill="url(#bg)" stroke="{pal.border}"/>
  <text x="14" y="30" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif" font-size="22" font-weight="700" fill="{color}">{value}</text>
  <text x="14" y="45" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif" font-size="10" font-weight="500" letter-spacing="0.8" fill="{pal.subtitle}">{label.upper()}</text>
</svg>
"""


def pill(text: str, pal: Palette, dot_color: str | None = None) -> str:
    height = 24
    text_px = _measure_text(text, 6.5)
    width = int(text_px + 36)
    dot = dot_color or pal.accent

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="{pal.bg_start}"/>
      <stop offset="1" stop-color="{pal.bg_end}"/>
    </linearGradient>
  </defs>
  <rect x="0.5" y="0.5" width="{width - 1}" height="{height - 1}" rx="12" ry="12" fill="url(#bg)" stroke="{pal.border}"/>
  <circle cx="14" cy="12" r="3.5" fill="{dot}"/>
  <text x="24" y="16" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif" font-size="11" font-weight="600" fill="{pal.title}">{text}</text>
</svg>
"""


def divider(pal: Palette) -> str:
    width = 820
    height = 8
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <line x1="0" y1="4" x2="{width}" y2="4" stroke="{pal.border}" stroke-width="1"/>
  <circle cx="{width // 2}" cy="4" r="3" fill="{pal.accent}"/>
</svg>
"""


SECTIONS = [
    ("why", "Why Rimuru", "Cost control for AI coding agents"),
    ("agents", "Works with every agent", "Claude Code, Cursor, Codex, and more"),
    ("budget", "Budget engine", "Monthly, daily, session, per-agent caps"),
    ("runaway", "Runaway detection", "Spot loops before they burn tokens"),
    ("guard", "Guard wrapper", "Kill any agent process at a cost limit"),
    ("compression", "Output compression", "Six strategies, token-bloat off"),
    ("interfaces", "Four interfaces", "CLI, Web UI, TUI, Desktop"),
    ("advisor", "Hardware advisor", "Run models locally, see savings"),
    ("quickstart", "Quick start", "Install in 30 seconds"),
    ("architecture", "Architecture", "Three primitives, one engine"),
    ("api", "API reference", "HTTP + iii trigger"),
    ("development", "Development", "Build, test, ship"),
    ("license", "License", "Apache 2.0"),
]

STATS = [
    ("agents", "8", "agents tracked", "blue"),
    ("functions", "60+", "iii functions", "blue"),
    ("interfaces", "4", "interfaces", "blue"),
    ("caps", "4", "budget cap levels", "green"),
    ("strategies", "6", "compression strategies", "green"),
    ("endpoints", "45+", "http endpoints", "orange"),
]

PILLS = [
    ("rust", "Rust 1.85"),
    ("apache2", "Apache-2.0"),
    ("v040", "v0.4.0 Benimaru"),
    ("iii", "iii-engine"),
    ("tauri", "Tauri v2"),
]


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


def generate_variant(pal: Palette, out_dir: Path) -> int:
    count = 0

    for key, title, subtitle in SECTIONS:
        write(out_dir / f"section-{key}.svg", section_header(title, subtitle, pal))
        count += 1

    color_map = {
        "blue": pal.stat_blue,
        "green": pal.stat_green,
        "orange": pal.stat_orange,
    }
    for key, value, label, color_key in STATS:
        write(out_dir / f"stat-{key}.svg", stat_card(value, label, color_map[color_key], pal))
        count += 1

    for key, text in PILLS:
        write(out_dir / f"pill-{key}.svg", pill(text, pal))
        count += 1

    write(out_dir / "divider.svg", divider(pal))
    count += 1

    return count


def main() -> None:
    dark_count = generate_variant(DARK, DARK_DIR)
    light_count = generate_variant(LIGHT, LIGHT_DIR)
    total = dark_count + light_count
    print(f"wrote {dark_count} dark tags -> {DARK_DIR.relative_to(ROOT)}")
    print(f"wrote {light_count} light tags -> {LIGHT_DIR.relative_to(ROOT)}")
    print(f"total: {total} svg files")


if __name__ == "__main__":
    main()
