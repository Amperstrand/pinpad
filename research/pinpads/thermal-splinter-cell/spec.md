# Thermal Pinpad - Splinter Cell (2002) Research Spec

> **Source**: Tom Clancy's Splinter Cell (2002) - Chinese Embassy Part 2 mission
> **Effect**: Thermal vision reveals heat signatures on keypad buttons after guards enter codes
> **Purpose**: Reference document for accurate implementation

---

## 1. Game Mechanics Overview

### How It Works In-Game
1. Guards enter codes on keypads during patrol
2. Body heat transfers to the buttons they press
3. Sam Fisher activates thermal goggles to see heat signatures
4. Heat fades over time (~30 seconds)
5. Player must check quickly before signatures fade completely
6. Player deduces code order by brightness intensity

### Player Strategy (from novel excerpt)
> "The trick is to press them in the correct order. Logically the key that's the faintest would be the first one and the brightest key would be the last. Distinguishing the differences of luminescence on the three keys in-between is the hard part."

**Key Insight**: Brightness indicates recency:
- **Faintest** = pressed first (oldest)
- **Brightest** = pressed last (most recent)

---

## 2. Visual Characteristics

### Heat Signature Appearance
- **Pressed buttons glow** with thermal signature
- **Glow style**: Concentric circles radiating outward from center
- **Outer rings are dimmer** (quadratic falloff)
- **Unpressed buttons**: Dark, cold appearance
- **Background**: Dark thermal blue/black

### Color Palette (Splinter Cell Style)

From the Splinter Cell Wiki thermal vision description:
> "Lower temperature objects in the environment will be a dark, purplish color while higher temperatures will be displayed as a brighter color from green, to yellow and then red (indicating the highest temperature)."

**Recommended palette mapping**:

| Intensity | Color | RGB Approximation |
|-----------|-------|-------------------|
| 0.0 - 0.1 | Dark blue/black | `#0a0a1a` |
| 0.1 - 0.3 | Purple/dark blue | `#1a0a2e` → `#2a1a4e` |
| 0.3 - 0.5 | Cyan/teal | `#0a4a5a` → `#1a7a8a` |
| 0.5 - 0.7 | Green | `#2a8a4a` → `#4aaa6a` |
| 0.7 - 0.85 | Yellow | `#8aaa2a` → `#caca4a` |
| 0.85 - 1.0 | Orange/bright yellow | `#ea8a2a` → `#faca5a` |

### Alternative Palettes

**Classic Thermal**:
- Blue → Cyan → Yellow → Orange → Red

**Ironbow** (professional thermal camera):
- Black → Purple → Red → Orange → Yellow → White

**Hot/Cold**:
- Blue → White → Red

---

## 3. Thermal Decay Mechanics

### Decay Timing
- **Full decay time**: ~30 seconds (configurable)
- **Minimum visible intensity**: 0.02 (below this, skip rendering)
- **Decay formula**: Exponential decay

### Mathematical Model

```
intensity = e^(-decay_progress * 3)

Where:
  decay_progress = elapsed_time_ms / decay_time_ms
  decay_time_ms = 30000 (30 seconds)
```

This creates a curve where:
- Initial intensity drops quickly
- Then gradually fades to near-zero
- Total fade time is approximately 30 seconds

### Ring Intensity Falloff

For concentric circle glow effect:
```
ring_intensity = base_intensity * (1 - ring_index/total_rings)^2
```

This creates quadratic falloff where outer rings are significantly dimmer.

---

## 4. Keypad Layout

Standard 12-button telephone layout:

```
[1] [2] [3]
[4] [5] [6]
[7] [8] [9]
[*] [0] [#]
```

### Button Dimensions (Reference)
- **Canvas size**: 320x400 minimum
- **Button size**: ~80x60 pixels each
- **Gap between buttons**: ~10 pixels
- **Margin**: ~20 pixels

---

## 5. Implementation Requirements

### Core Components

1. **Thermal Logic** (platform-agnostic)
   - Heat intensity calculation (0.0 to 1.0)
   - Exponential decay formula
   - Button state management
   - Press timestamp tracking

2. **Rendering Layer** (platform-specific)
   - Concentric circle glow effect (8-12 rings)
   - Color palette mapping
   - 60fps animation loop
   - Background rendering

3. **Interaction**
   - Click/tap to simulate button press
   - Demo mode (auto-enters random codes)
   - Reset button (clear all heat)
   - Palette selector

### Visual Style Guidelines

- **Dark, moody aesthetic** - match Splinter Cell's tone
- **High contrast** between hot and cold buttons
- **Smooth gradients** for thermal glow
- **No harsh edges** on glow effect

---

## 6. Reference Materials

### Video References
- YouTube: "Tom Clancy's Splinter Cell - Thermal Keypad" (JULIOHHH, 2022)
  - https://www.youtube.com/watch?v=lVNlggJECwc
- YouTube: "Splinter Cell: Complete Stealth Walkthrough | Part 9 Chinese Embassy" (Centerstrain01)
  - https://www.youtube.com/watch?v=6Eou4MfwJNA

### Wiki References
- Splinter Cell Wiki: Thermal Vision
  - https://splintercell.fandom.com/wiki/Thermal_vision
- Splinter Cell Wiki: Keypad Lock
  - https://splintercell.fandom.com/wiki/Keypad_lock
- Splinter Cell Wiki: Chinese Embassy (Part 2)
  - https://splintercell.fandom.com/wiki/Chinese_Embassy_(Part_2)

### Novel Reference
- *Splinter Cell* novel, Chapter 16 - describes thermal keypad mechanic in detail

---

## 7. Success Criteria

Implementation should achieve:

1. **Visual fidelity** - looks like the game effect
2. **Accurate decay timing** - ~30 second fade
3. **Correct brightness ordering** - recent = bright, old = faint
4. **Smooth animation** - 60fps rendering
5. **Intuitive interaction** - clear button feedback

---

## 8. Future Enhancements

Potential additions after core implementation:

- **OPSAT mode**: Screenshot analysis with contrast adjustment
- **Difficulty modes**: Faster decay, more digits
- **Sound effects**: Button press audio
- **Multiple keypads**: Different keypad styles from other games
- **Mobile support**: Touch-friendly interface
