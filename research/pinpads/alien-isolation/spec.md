# Sevastolink Terminal Pinpad - Alien: Isolation (2014) Research Spec

> **Source**: Alien: Isolation (2014) by Creative Assembly / SEGA
> **Setting**: Sevastopol Station - a decommissioned space station (2137)
> **Operating System**: Seegson Sevastolink - retro-futuristic terminal interface
> **Purpose**: Reference document for accurate pinpad implementation

---

## 1. Design Philosophy & Aesthetic

### Historical Context
- Set shortly after the events of Alien (1979)
- Technology designed to match the 1979 film's aesthetic, not the 1986 Aliens sequel
- **Retro-futuristic period piece**: CRT monitors, beige plastic, beveled edges
- Inspired by 1970s/80s computer terminals (green phosphor monochrome displays)

### Key Design Principles
1. **Diegetic UI**: Interfaces exist within the game world
2. **Lo-fi aesthetic**: CRT scan lines, phosphor glow, screen curvature
3. **Monochrome palette**: Primarily green-on-black with amber/orange accents
4. **Industrial feel**: Worn, battered equipment showing station's decline
5. **Functional minimalism**: Simple menu structures, text-heavy interfaces

---

## 2. Visual Characteristics

### Color Palette

Based on the official Sevastolink color theme and game analysis:

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Xenomorph Skin** | `#0c290c` | `rgb(12, 41, 12)` | Background dark green / shadows |
| **Terminal Green** | `#134213` | `rgb(19, 66, 19)` | Secondary background, dim text |
| **Seegson Green** | `#05b669` | `rgb(5, 182, 105)` | Primary text, highlights, active elements |
| **Acid Blood** | `#f07826` | `rgb(240, 120, 38)` | Warnings, errors, accents |
| **Hypersleep White** | `#ccd5d4` | `rgb(204, 213, 212)` | Bright text, selection highlight |
| **Synthetic Skin** | `#7a807f` | `rgb(122, 128, 127)` | Muted text, disabled elements |
| **Pure Black** | `#000000` | `rgb(0, 0, 0)` | CRT screen background |
| **Dark Background** | `#0a0a0a` | `rgb(10, 10, 10)` | Screen border / bezel |

### Alternative Color Modes

**Amber Phosphor Mode** (some terminals in game):
- Primary: `#f0a000` (amber/orange)
- Secondary: `#c08000` (dim amber)
- Background: `#0a0800` (dark amber-black)

**Dual-Tone Mode** (rewire systems):
- Primary: `#05b669` (green)
- Secondary: `#f07826` (orange)
- Creates high-contrast dual-color displays

---

## 3. CRT Effects

### Screen Characteristics

The terminal displays simulate authentic CRT monitors with these effects:

#### Scan Lines
- **Horizontal lines**: Dark lines between each row of pixels
- **Line spacing**: Approximately 1-2 pixels between scan lines
- **Line opacity**: 15-25% darker than lit areas
- **Implementation**: CSS/Canvas overlay with repeating linear gradient

```css
/* CSS approximation */
.scanlines {
  background: repeating-linear-gradient(
    0deg,
    transparent,
    transparent 2px,
    rgba(0, 0, 0, 0.3) 2px,
    rgba(0, 0, 0, 0.3) 4px
  );
  pointer-events: none;
}
```

#### Screen Curvature
- **Subtle barrel distortion**: Edges curve slightly outward
- **Vignette effect**: Corners slightly darker than center
- **Radius**: ~2-5% of screen dimension

```glsl
// GLSL shader approximation
vec2 curved = uv * 2.0 - 1.0;
vec2 offset = abs(curved.yx) / vec2(10.0);
curved = curved + curved * offset * offset;
curved = curved * 0.5 + 0.5;
```

#### Phosphor Glow
- **Bloom effect**: Bright pixels bleed into neighbors
- **Glow radius**: 2-4 pixels
- **Glow intensity**: 20-40% of source brightness
- **Persistence**: Brief afterglow when pixels change (phosphor decay)

#### Chromatic Aberration
- **RGB channel offset**: 1-2 pixels separation at edges
- **Most visible at screen edges**: Subtle center, stronger at periphery
- **Implementation**: Separate R/G/B channels with slight offset

### Static/Noise Effects

| Effect | Description | Intensity |
|--------|-------------|-----------|
| **Static noise** | Random pixel variation | 5-10% luminance variation |
| **Screen flicker** | Occasional brightness dip | 5% every 2-4 seconds |
| **Horizontal tear** | Rare scan line disruption | Once per 30-60 seconds |
| **Signal glitch** | Momentary distortion | Random, ~0.5% chance per frame |

### Animation Timing Values

| Effect | Duration | Notes |
|--------|----------|-------|
| **Text typing** | 30-50ms per character | Terminal text output |
| **Cursor blink** | 530ms on/off | Block cursor |
| **Screen flicker** | 50-100ms | Random intervals |
| **Transition fade** | 200-400ms | Screen changes |
| **Button flash** | 100-150ms | Key press feedback |
| **Static burst** | 50-200ms | Random interference |

---

## 4. Typography

### Primary Font: Sevastopol Interface

- **Source**: Created by RNRCFan, inspired by the game UI
- **Download**: https://www.dafont.com/sevastopol-interface.font
- **Type**: Bitmap font (16px base)
- **Style**: Monospace, pixel-based
- **Character count**: 180 glyphs

### Font Specifications

| Property | Value |
|----------|-------|
| **Base size** | 16px |
| **Line height** | 1.2-1.5 |
| **Character width** | Monospace (8-10px) |
| **Weight** | Regular (single weight) |
| **Anti-aliasing** | None (crisp pixels) |

### Fallback Fonts

For systems without Sevastopol Interface:
```css
font-family: 'Sevastopol Interface', 'OCR-A', 'Courier New', monospace;
```

### Text Styling

- **Uppercase preference**: Headings often ALL CAPS
- **No italic**: Font doesn't support italics
- **No bold variant**: Use color/brightness for emphasis
- **Text shadow**: Optional 1px glow in primary color

```css
text-shadow: 0 0 5px currentColor;
```

---

## 5. Pinpad-Specific Elements

### Keypad Layout

Standard 12-button telephone layout:

```
┌─────────────────────────────┐
│  ╔═══╗  ╔═══╗  ╔═══╗       │
│  ║ 1 ║  ║ 2 ║  ║ 3 ║       │
│  ╚═══╝  ╚═══╝  ╚═══╝       │
│  ╔═══╗  ╔═══╗  ╔═══╗       │
│  ║ 4 ║  ║ 5 ║  ║ 6 ║       │
│  ╚═══╝  ╚═══╝  ╚═══╝       │
│  ╔═══╗  ╔═══╗  ╔═══╗       │
│  ║ 7 ║  ║ 8 ║  ║ 9 ║       │
│  ╚═══╝  ╚═══╝  ╚═══╝       │
│  ╔═══╗  ╔═══╗  ╔═══╗       │
│  ║ * ║  ║ 0 ║  ║ # ║       │
│  ╚═══╝  ╚═══╝  ╚═══╝       │
└─────────────────────────────┘
```

### Button Dimensions (Reference)

| Property | Value | Notes |
|----------|-------|-------|
| **Button size** | 60x50 px | Approximate |
| **Button gap** | 8-12 px | Between buttons |
| **Border radius** | 2-4 px | Slightly rounded corners |
| **Border style** | 1-2px solid | Single color outline |

### Button Visual States

| State | Appearance |
|-------|------------|
| **Default** | Dark background (`#0c290c`), green border (`#134213`) |
| **Hover** | Brighter border (`#05b669`), subtle glow |
| **Pressed** | Fill with dim green (`#134213`), text bright (`#05b669`) |
| **Disabled** | Muted colors (`#7a807f`), no glow |
| **Active/Input** | Bright green fill (`#05b669`), white text (`#ccd5d4`) |

### Input Display

- **Position**: Above keypad
- **Style**: Monospace text in display area
- **Mask character**: `*` or `█` for entered digits
- **Max length**: Typically 4-8 digits
- **Display format**: `****` or `████`

### Display Area

```
┌─────────────────────────────┐
│  SEEGSON SEVASTOLINK        │
│  ─────────────────────      │
│  ACCESS CODE:               │
│  ████                       │
│  ─────────────────────      │
│  [KEYPAD]                   │
│                             │
│  STATUS: AWAITING INPUT     │
└─────────────────────────────┘
```

---

## 6. Behavior & Animation

### Keypress Feedback

1. **Visual**:
   - Button fill changes to bright green
   - Brief glow pulse (100-150ms)
   - Text changes to white/bright

2. **Audio** (see Sound Design section):
   - Soft click/beep sound
   - 50-100ms duration

### Authentication Sequence

```
1. User enters code (digit by digit)
   - Each press: visual flash + audio click
   - Display updates with mask character

2. User submits (Enter/Confirm)
   - Display shows "VERIFYING..."
   - Brief processing animation (500-1000ms)

3. Result feedback:
   SUCCESS:
   - Green flash
   - "ACCESS GRANTED" message
   - Optional: door/unlock sound
   
   FAILURE:
   - Red/orange flash
   - "ACCESS DENIED" message
   - Error beep sound
   - 1-2 second lockout before retry
```

### Idle State Animations

| Animation | Interval | Description |
|-----------|----------|-------------|
| **Cursor blink** | 530ms | Block cursor toggles |
| **Screen flicker** | 2-4s | Brief brightness variation |
| **Static noise** | Continuous | Subtle pixel noise overlay |
| **Scan line drift** | 10-20s | Very slow horizontal drift |

### Error State

- **Flash color**: Orange/amber (`#f07826`)
- **Flash duration**: 200-300ms
- **Error message**: "ACCESS DENIED" or "INVALID CODE"
- **Lockout time**: 1-2 seconds before retry allowed
- **Retry counter**: Optional - limit attempts

### Success State

- **Flash color**: Bright green (`#05b669`)
- **Flash duration**: 300-500ms
- **Success message**: "ACCESS GRANTED"
- **Transition**: Fade to next screen or unlock animation

---

## 7. Sound Design Notes

### UI Sound Categories

Based on game audio analysis:

| Sound Type | Description | Duration |
|------------|-------------|----------|
| **Keypress** | Soft mechanical click | 50-100ms |
| **Confirm** | Subtle beep/tone | 100-150ms |
| **Success** | Rising tone sequence | 300-500ms |
| **Error** | Low buzz or descending tone | 200-400ms |
| **Ambient** | Low hum, occasional static | Continuous |

### Audio Characteristics

- **Sample rate**: 44.1kHz (game standard)
- **Style**: Lo-fi, slightly distorted (matches CRT aesthetic)
- **Tone quality**: Synthetic, electronic
- **Keypress**: Short click, not a pure beep - more mechanical
- **Success tone**: Rising pitch, positive feedback
- **Error tone**: Lower pitch, harsh buzz

### Sound Implementation Notes

- Consider using Web Audio API for JS implementation
- Python: pygame.mixer for simple sounds
- Rust: rodio crate for audio playback
- Pre-generate or synthesize sounds rather than using samples

---

## 8. Implementation Notes

### JavaScript (Canvas/WebGL)

```javascript
// CRT shader parameters
const crtConfig = {
  scanlineIntensity: 0.15,
  scanlineGap: 2,
  curvature: 0.03,
  vignette: 0.2,
  chromaticAberration: 0.002,
  flickerChance: 0.005,
  noiseIntensity: 0.08
};

// Animation timing
const timing = {
  cursorBlink: 530,
  keypressFlash: 150,
  screenTransition: 300,
  errorFlash: 250,
  successFlash: 400
};
```

### Python (pygame)

```python
# Color definitions
COLORS = {
    'background': (12, 41, 12),      # Xenomorph Skin
    'dim_green': (19, 66, 19),       # Terminal Green  
    'primary': (5, 182, 105),        # Seegson Green
    'accent': (240, 120, 38),        # Acid Blood
    'bright': (204, 213, 212),       # Hypersleep White
    'muted': (122, 128, 127),        # Synthetic Skin
}

# Timing (milliseconds)
TIMING = {
    'cursor_blink': 530,
    'keypress_flash': 150,
    'error_duration': 1500,
    'success_duration': 500,
}
```

### Rust (embedded-graphics)

```rust
// Color definitions (RGB565 where applicable)
pub const XENOMORPH_SKIN: Rgb565 = Rgb565::new(3, 10, 3);
pub const TERMINAL_GREEN: Rgb565 = Rgb565::new(4, 16, 4);
pub const SEEGSON_GREEN: Rgb565 = Rgb565::new(2, 45, 26);
pub const ACID_BLOOD: Rgb565 = Rgb565::new(60, 30, 9);
pub const HYPERSLEEP_WHITE: Rgb565 = Rgb565::new(51, 53, 53);
```

### Cross-Platform Considerations

1. **Font rendering**: Use bitmap/rasterized font for consistency
2. **CRT effects**: May need to be simplified for embedded systems
3. **Animation timing**: Use platform-specific timing functions
4. **Color accuracy**: Test on target display hardware

---

## 9. Reference Materials

### Video References
- YouTube: "Alien: Isolation [UI Sounds]" (Bond Factory)
  - https://www.youtube.com/watch?v=VBlbKCk8DC8
- YouTube: "ASMR | Alien: Isolation | Spaceflight Terminal Computer Sounds"
  - https://www.youtube.com/watch?v=WeDvpZXP13k

### Font Resources
- Sevastopol Interface Font
  - https://www.dafont.com/sevastopol-interface.font
  - https://fontmeme.com/alien-isolation-font/

### Color Theme References
- paulopacitti/sevastolink (GitHub)
  - https://github.com/paulopacitti/sevastolink
  - Official color palette extraction

### CRT Shader References
- gingerbeardman/webgl-crt-shader
  - https://github.com/gingerbeardman/webgl-crt-shader
- Retro CRT Shader (Babylon.js)
  - https://babylonjs.medium.com/retro-crt-shader-a-post-processing-effect-study-1cb3f783afbc

### Game Wiki References
- Orcz: Alien Isolation Wiki
  - https://orcz.com/Alien_Isolation:_Welcome_to_Sevastopol

### Design Analysis
- Lucas Pettersson: ALIEN Main Menu Terminal UI
  - https://www.lucaspettersson.net/alienterminal.html
- Sci-Fi Interfaces: Alien (1979) analysis
  - https://scifiinterfaces.com/category/alien-1979/

---

## 10. Success Criteria

Implementation should achieve:

1. **Visual fidelity** - Recognizable as Sevastolink terminal
2. **CRT authenticity** - Scan lines, glow, subtle curvature
3. **Color accuracy** - Match the green/amber palette
4. **Responsive feedback** - Keypress visual/audio timing
5. **Atmospheric feel** - Convey the isolation/tension of the game
6. **Smooth animation** - 60fps rendering where possible

---

## 11. Future Enhancements

Potential additions after core implementation:

- **Multiple terminal styles**: Different station terminals (medical, engineering)
- **Corrupted display mode**: Glitchy, damaged terminal effects
- **Ambient audio**: Background station sounds
- **APOLLO interface**: The station's main computer aesthetic
- **Working Joe interaction**: Android interface elements
- **Save station**: The distinctive save terminal design

---

## 12. Notes & Estimations

### Values Requiring Verification

The following values are estimated from visual analysis and may need adjustment:

- [ ] Exact scan line spacing (estimated 2px)
- [ ] Precise button dimensions (estimated 60x50px)
- [ ] Animation timing values (measured from video)
- [ ] Sound effect frequencies/durations

### Implementation Priorities

1. **Core pinpad functionality** - Layout, input, validation
2. **CRT effects** - Scan lines, glow, curvature
3. **Visual polish** - Colors, typography, states
4. **Audio feedback** - Keypress, success, error sounds
5. **Advanced effects** - Static, flicker, chromatic aberration

---

*Document created: 2024*
*Research sources: Game footage, community resources, official color themes*
