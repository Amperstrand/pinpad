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

### Color Palette (Extracted + Constrained)
| Element | Hex | Notes |
|---|---|---|
| Background cool base | `#050f1e` | dark tactical blue-black |
| Cold key body | `#0a1022` | unpressed key state |
| Old heat trace | `#0c2a6e` | earliest visible heat |
| Mid heat trace | `#1478aa` | cyan/teal phase |
| Warm heat trace | `#52be84` | green phase |
| Hot trace | `#dcda5a` | yellow phase |
| Peak trace | `#fffadc` | near-white highlight |

### Glow Effect Style
- Type: layered bloom (soft blob core + falloff aura)
- Radius: ~1.1x button max dimension (outer envelope)
- Falloff: exponential over time + layered alpha blending in space
- Rings/layers: 3 bloom layers (primary), optional ring helper in lower-level renderers

### Thermal Overlay Effects
- Scan lines: Yes (subtle)
- Noise/grain: Yes (light)
- Vignette: Yes (mild)
- Edge detection: No

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
