# Future Keypads Roadmap

> Reference document for potential keypad implementations

---

## Candidate Pool

### Tier 1: Strong Candidates (Distinct Visual Styles)

| # | Source | Year | Visual Style | Unique Element | Difficulty |
|---|--------|------|--------------|----------------|------------|
| 1 | **Blade Runner** | 1982 | Neon noir CRT | Amber phosphor, Esper photo analyzer, rain-slicked | Medium |
| 2 | **Fallout (Pip-Boy)** | 1997+ | Green monochrome CRT | 1950s retro-future, vacuum tube glow, Vault-Tec branding | Medium |
| 3 | **2001: A Space Odyssey (HAL 9000)** | 1968 | Red lens interface | Single glowing red eye, soft pulse, clinical minimalism | Easy |
| 4 | **BioShock** | 2007 | Art Deco brass | Rivets, brass panels, 1940s underwater, incandescent bulbs | Medium |
| 5 | **Portal (Aperture Science)** | 2007 | Clean white/orange | Minimalist scientific, orange aperture logo, sterile | Easy |
| 6 | **The Matrix (Nebuchadnezzar)** | 1999 | Operator console | Green code rain, ship interface, analog+digital hybrid | Medium |
| 7 | **Jurassic Park (IRIX/SGI)** | 1993 | SGI workstation | 90s Unix, IRIX indigo magic, "It's a Unix system!" | Medium |
| 8 | **Star Trek LCARS** | 1987+ | Touch panel | Rounded rectangles, color-coded zones, Okudagram | Medium |

### Tier 2: Interesting But Challenging

| # | Source | Year | Visual Style | Challenge |
|---|--------|------|--------------|-----------|
| 9 | **System Shock (SHODAN)** | 1994 | Corrupted interface | Glitch effects, face integration, early cyberpunk |
| 10 | **Mass Effect (Omni-tool)** | 2007 | Holographic orange | Circular radial menus, arm projection |
| 11 | **Minority Report** | 2002 | Gesture interface | 3D spatial, hand tracking visualization |
| 12 | **Ghost in the Shell** | 1995 | Cyberpunk thermoptic | Japanese text, holographic overlays |
| 13 | **Ex Machina** | 2014 | Clean AI interface | Minimalist, search queries, data visualization |
| 14 | **Her** | 2013 | Soft AI OS | Warm colors, conversational, no traditional UI |
| 15 | **Iron Man (JARVIS)** | 2008 | Holographic blue | 3D projections, multi-panel, gesture |
| 16 | **Prometheus (David)** | 2012 | Engineer interface | Alien symbols, holographic maps |

### Tier 3: Games with Distinct Keypads

| # | Source | Year | Visual Style | Notes |
|---|--------|------|--------------|-------|
| 17 | **Doom (classic)** | 1993 | Industrial keycard | Simple colored keycards, no keypad per se |
| 18 | **Half-Life (retinal scanner)** | 1998 | Black Mesa | Scan animation, scientific facility |
| 19 | **Metal Gear Solid** | 1998 | Codec/Terminal | Green codec screen, frequency dial |
| 20 | **Resident Evil (save room)** | 1996 | Typewriter | Ink ribbon aesthetic, not keypad |
| 21 | **System Shock 2** | 1999 | Cyber-psionic | Upgrade stations, body modification |
| 22 | **Prey (2006)** | 2006 | Alien interface | Sphere integration, living technology |
| 23 | **Cyberpunk 2077** | 2020 | Future tech | Brain dance, netrunning, AR overlays |
| 24 | **Observation** | 2019 | SAM interface | AI controlling station, hexagonal grids |

---

## Selection Criteria for Next Batch

When choosing the next 5 keypads, prioritize:

1. **Visual distinctness** - Must look different from existing 8
2. **Source recognition** - Iconic enough that people recognize it
3. **Research availability** - Enough reference material exists
4. **Implementation feasibility** - Effects can be replicated in Canvas/LVGL/embedded-graphics
5. **Cross-platform parity** - Same effect achievable in JS/Python/Rust

---

## Recommended Next 5

Based on distinct visual styles and implementation feasibility:

1. **Blade Runner (1982)** - Amber CRT, neon noir, Esper terminal
2. **Fallout Pip-Boy** - Green vacuum tube CRT, Vault-Tec branding
3. **HAL 9000** - Single red eye, clinical minimalism
4. **BioShock** - Art Deco brass, rivets, incandescent
5. **Portal** - Clean white/orange, Aperture Science

This gives us:
- 2x CRT-based (Blade Runner, Fallout)
- 1x Minimalist (HAL)
- 1x Mechanical/Physical (BioShock)
- 1x Clean Modern (Portal)

---

*Generated: 2026-03-15*
