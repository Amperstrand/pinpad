# Initial Task: Thermal Pinpad Implementation

## Overview

Implement the thermal vision keypad effect from Splinter Cell (2002). This is the first pinpad in our collection and will serve as the reference implementation for all future platforms.

## Source Reference

**Game:** Tom Clancy's Splinter Cell (2002)  
**Mission:** Chinese Embassy (Part 2)  
**Mechanic:** Sam Fisher uses thermal goggles to see heat signatures left by guards on keypad buttons

### How It Works In-Game
1. Guards enter codes on keypads
2. Their body heat transfers to the buttons they press
3. Thermal goggles reveal the heat signatures
4. Heat fades over ~30 seconds (exponential decay)
5. More recent presses are brighter
6. Player must check quickly before signatures fade

### Visual Effect Characteristics
- Pressed buttons glow with orange/yellow thermal signature
- Glow uses concentric circles (rings) radiating outward
- Outer rings are dimmer (quadratic falloff)
- Cold buttons are dark blue/black
- Background is dark thermal blue

---

## Implementation Requirements

### Core Thermal Logic (Platform-Agnostic)

```
Decay Formula:
  intensity = e^(-decay_progress * 3)
  
Where:
  decay_progress = elapsed_time_ms / decay_time_ms
  decay_time_ms = 30000 (30 seconds, configurable)
  
Ring Intensity:
  ring_intensity = base_intensity * (1 - ring_index/total_rings)^2

Minimum visible intensity: 0.02 (skip rendering below this)
```

### Keypad Layout

Standard 12-button telephone layout:

```
[1] [2] [3]
[4] [5] [6]
[7] [8] [9]
[*] [0] [#]
```

### Color Palette Mapping

**Splinter Cell Palette (default):**
```
Intensity → RGB
0.0-0.2   → dark blue → cyan
0.2-0.4   → cyan → green  
0.4-0.6   → green → yellow
0.6-0.8   → yellow → orange
0.8-1.0   → orange → bright yellow
```

---

## Deliverables

### 1. JavaScript (`javascript/thermal-pinpad.html`)

Self-contained single HTML file:
- Canvas-based rendering (320x400 minimum)
- Click on buttons to simulate presses
- **Demo Mode** button - auto-enters random codes every 8 seconds
- **Reset** button - clear all heat signatures
- **Random Code** button - simulate a 4-digit code entry
- Palette dropdown - switch between color schemes
- Smooth 60fps animation loop
- Status text showing current action

**No external dependencies** - pure vanilla JavaScript

### 2. Python (`python/thermal_pinpad.py`)

Core logic module + Tkinter demo:
- `ThermalKeypad` class - state management for all 12 buttons
- `ThermalButton` class - individual button state (intensity, pressed_at)
- `ThermalConfig` dataclass - configurable parameters
- `ThermalPalette` enum - color scheme options
- `ThermalColorMapper` class - intensity → RGB conversion
- Tkinter demo matching JavaScript visuals
- `if __name__ == "__main__"` demo block

**Designed for easy porting to LVGL/MicroPython**

### 3. Rust (`rust/`)

embedded-graphics library:
- `no_std` compatible core library
- `ThermalKeypad` struct with builder pattern
- `ThermalConfig` struct
- `ThermalPalette` enum
- `render_thermal_keypad()` function for embedded-graphics
- PC simulator binary with "simulator" feature flag
- Uses embedded-graphics 0.8, embedded-graphics-simulator 0.5

---

## File Structure After Completion

```
pinpad/
├── MISSION.md
├── README.md
├── prompts/
│   └── INITIAL.md
├── javascript/
│   └── thermal-pinpad.html
├── python/
│   └── thermal_pinpad.py
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── thermal.rs
│       ├── color.rs
│       └── bin/
│           └── simulator.rs
└── docs/
    └── PORTING.md
```

---

## Success Criteria

1. ✅ All three implementations produce identical visual effect
2. ✅ Thermal decay timing is consistent (30 seconds)
3. ✅ Color palettes match across platforms  
4. ✅ Interactive demos work in all three versions
5. ✅ Code is well-documented
6. ✅ README.md is updated with usage instructions
7. ✅ Git commit with descriptive message

---

## Notes for AI Agent

- **Reference implementation:** JavaScript version is the visual reference
- **Prioritize code clarity** over cleverness
- **Document assumptions** you make during implementation
- **Keep core logic portable** - avoid platform-specific optimizations
- **Test visually** - make sure it looks right before committing
- **Match the Splinter Cell aesthetic** - dark, moody, thermal imaging look
