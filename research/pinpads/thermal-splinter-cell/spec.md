# Thermal Pinpad - Splinter Cell (2002) Verified Spec

## Source Scope
- Game target: Tom Clancy's Splinter Cell (2002)
- Mechanic target: keypad thermal traces in Chinese Embassy sequence
- Reference set:
  - https://splintercell.fandom.com/wiki/Thermal_vision
  - https://splintercell.fandom.com/wiki/Chinese_Embassy_(Part_2)
  - https://splintercell.fandom.com/wiki/Keypad_lock/Codes
  - https://www.ign.com/wikis/tom-clancys-splinter-cell/Walkthrough:_Mission_8
  - https://pastebin.com/60WPpqbn
  - https://www.youtube.com/watch?v=lVNlggJECwc

## Verified Visual Characteristics

### Color Palette (HSV Interpolated, γ=1.3)
| Stop | HSV (h,s,v) | Hex | Notes |
|---|---|---|---|
| 0.00 | 230, 85, 12 | `#05081e` | deep blue-black |
| 0.20 | 225, 90, 43 | `#0c2a6b` | cobalt |
| 0.40 | 195, 88, 67 | `#1478b0` | cyan |
| 0.62 | 150, 58, 75 | `#52be87` | green |
| 0.80 | 55, 60, 86 | `#d8da58` | yellow |
| 0.92 | 48, 90, 94 | `#f0eb64` | warm yellow |
| 1.00 | 42, 88, 94 | `#f0d020` | yellow-orange peak |

**Key change**: Peak is now warm yellow-orange (#f0d020), not near-white (#fffadc).
This matches SC1 Xbox original thermal vision more accurately.

### Glow Effect Style
- Type: layered bloom (soft blob core + falloff aura)
- Radius: ~1.1x button max dimension (outer envelope)
- Falloff: exponential over time + layered alpha blending in space
- Rings/layers: 3 bloom layers (primary), optional ring helper in lower-level renderers

### Thermal Overlay Effects
- Scan lines: Yes (subtle)
- Noise/grain: Yes (film grain at ~10fps update rate)
- Vignette: Yes (mild)
- Chromatic aberration: Yes (JS only, on hot edges when intensity > 0.4)

## Core Mechanics

### Decay
```
intensity = exp(-(elapsed_ms / decay_time_ms) * 3)
```
- `decay_time_ms = 30000`
- min visible threshold: `0.02`

### Ordering Rule
- Brightest key = most recent press
- Faintest visible key = oldest press

## Keypad Layout
```
[1] [2] [3]
[4] [5] [6]
[7] [8] [9]
[*] [0] [#]
```

## Cross-Platform Contract
- JS/Python/Rust must share:
  - identical Splinter palette stop list
  - identical decay formula and timing
  - identical visibility threshold
  - equivalent glow style (soft thermal bloom)

## Implementation Targets

### JavaScript (`javascript/thermal-pinpad.html`)
- Palette stop update
- Layered bloom glow
- Noise + scanline + vignette post effects

### Python (`python/thermal_pinpad.py`)
- Palette stop update
- Layered bloom glow approximation
- Noise + scanline + vignette overlay

### Rust (`rust/src/color.rs`, `rust/src/bin/simulator.rs`)
- Palette stop update in shared mapper
- Simulator rendering closer to bloom-like fill and scanline texture
- Native screenshot support retained

## Validation Criteria
- Visual ranking target after cycle 3:
  - Color accuracy >= 9/10
  - Visual effect accuracy >= 9/10
  - Cross-platform consistency >= 9/10
  - Overall resemblance >= 8/10
