# WarGames (1983) NORAD Console / IMSAI 8080 Terminal

> **Source**: WarGames (1983) - MGM/United Artists
> **Computers**: IMSAI 8080 microcomputer, Electrohome 17" monitor
> **Context**: David Lightman's bedroom terminal, NORAD war room displays
> **Purpose**: Reference document for 1980s green/amber phosphor terminal implementation

---

## 1. Design Philosophy

### Historical Context
- Peak of 8-bit home computing era
- IMSAI 8080: Real S-100 bus computer (1975-1978)
- Electrohome 17" monitor: CRT with green phosphor
- Authentic command-line interface (no GUI)

### Key Design Principles
1. **Single phosphor color** - Green or amber, never both
2. **Blocky monospace** - 8x8 or 8x16 pixel fonts
3. **CRT authenticity** - Scan lines, phosphor glow, curvature
4. **High contrast** - Bright text on near-black background
5. **Command-line interaction** - Text prompts, not buttons

---

## 2. Visual Characteristics

### Color Palette

#### Green Phosphor (Primary - WarGames)
| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Phosphor Green** | `#00AA00` | (0, 170, 0) | Primary text color |
| **Bright Green** | `#18E699` | (24, 230, 153) | Highlights, active elements |
| **Dim Green** | `#10540E` | (16, 84, 14) | Low-brightness elements |
| **Dark Background** | `#051A05` | (5, 26, 5) | Very dark green-black |
| **Pure Black** | `#000000` | (0, 0, 0) | CRT off areas |

#### Amber Phosphor (Alternative)
| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Phosphor Amber** | `#B8860B` | (184, 134, 11) | Primary text color |
| **Bright Amber** | `#FFD814` | (255, 216, 20) | Highlights |
| **Dark Amber** | `#8B6914` | (139, 105, 20) | Low-brightness |

### Typography

**Recommended Fonts:**
| Font | Size | Source |
|------|------|--------|
| **Unscii-8** | 8x8 px | [unscii.fi](http://viznut.fi/unscii/) |
| **VT323** | 8x16 px | [GitHub](https://github.com/phoikoi/VT323) |
| **BlockZone** | 9x16 px | [GitHub](https://github.com/ansilove/BlockZone) |
| **80s PXL** | Variable | [Baseline Fonts](https://baselinefonts.com/) |

**Font Characteristics:**
- Monospace only
- No anti-aliasing (crisp pixels)
- Blocky, angular glyphs
- Code page 437 support (DOS characters)

---

## 3. CRT Effects

### Scan Lines
```css
.crt-scanlines::before {
  content: " ";
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: linear-gradient(
    rgba(18, 16, 16, 0) 50%,
    rgba(0, 0, 0, 0.25) 50%
  );
  background-size: 100% 4px;
  pointer-events: none;
}
```

### Phosphor Glow
```css
.phosphor-glow {
  color: #00AA00;
  text-shadow: 
    0 0 2px #00AA00,
    0 0 4px #00AA00,
    0 0 8px #00AA00;
}
```

### Screen Curvature
```css
.crt-curve {
  box-shadow: inset 0 0 100px rgba(0, 0, 0, 0.9);
  border-radius: 20px;
}
```

---

## 4. UI Elements

### Prompt Pattern
```
C:\> _
```

### WarGames Menu Style
```
GAMES LIST:

1. GLOBAL THERMONUCLEAR WAR
2. POKER
3. CHESS
4. FIGHTER COMBAT

SELECT: _
```

### Blinking Cursor
```css
.cursor {
  display: inline-block;
  width: 8px;
  height: 16px;
  background-color: #00AA00;
  animation: blink 1s step-end infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}
```

---

## 5. Behavior & Animation

### Text Typing
| Property | Value |
|----------|-------|
| **Character delay** | 30-50ms |
| **Variance** | ±15ms for human feel |
| **Newline pause** | 150-200ms |

### CRT Effects
| Effect | Timing |
|--------|--------|
| **Cursor blink** | 1s on/off |
| **Scan line drift** | Very slow (10-20s cycle) |
| **Phosphor persistence** | Brief afterglow on changes |

### Boot Sequence
```
1. Blank screen
2. Single cursor appears
3. Text begins typing
4. System ready prompt
```

---

## 6. Implementation Notes

### JavaScript (Canvas)
```javascript
const COLORS = {
    phosphorGreen: '#00AA00',
    brightGreen: '#18E699',
    dimGreen: '#10540E',
    background: '#051A05',
    black: '#000000'
};

const TIMING = {
    cursorBlinkMs: 1000,
    typeDelayMs: 40,
    typeVarianceMs: 15
};

// Phosphor glow text
function drawGlowText(ctx, text, x, y) {
    ctx.shadowColor = COLORS.phosphorGreen;
    ctx.shadowBlur = 8;
    ctx.fillStyle = COLORS.phosphorGreen;
    ctx.fillText(text, x, y);
    ctx.shadowBlur = 0;
}
```

### Python (pygame)
```python
COLORS = {
    'phosphor_green': (0, 170, 0),
    'bright_green': (24, 230, 153),
    'dim_green': (16, 84, 14),
    'background': (5, 26, 5),
    'black': (0, 0, 0),
}

TIMING = {
    'cursor_blink_ms': 1000,
    'type_delay_ms': 40,
    'type_variance_ms': 15,
}
```

### Rust (embedded-graphics)
```rust
pub const PHOSPHOR_GREEN: Rgb888 = Rgb888::new(0, 170, 0);
pub const BRIGHT_GREEN: Rgb888 = Rgb888::new(24, 230, 153);
pub const DIM_GREEN: Rgb888 = Rgb888::new(16, 84, 14);
pub const BACKGROUND: Rgb888 = Rgb888::new(5, 26, 5);
```

---

## 7. Reference Materials

### Film References
- **Starring the Computer**: [IMSAI 8080 in WarGames](https://www.starringthecomputer.com/appearance.php?f=10&c=10)
- **PC-Museum**: [WarGames IMSAI](https://pc-museum.com/046-imsai8080/wargames.htm)
- **Reddit**: [NORAD Command Center](https://www.reddit.com/r/RetroFuturism/comments/1mhppol/)

### Font Resources
- **Unscii**: [unscii.fi](http://viznut.fi/unscii/)
- **BlockZone**: [GitHub](https://github.com/ansilove/BlockZone)
- **VT323**: [GitHub](https://github.com/phoikoi/VT323)

---

## 8. Success Criteria

1. **Single phosphor color** - Green or amber only
2. **Blocky typography** - 8x8 or 8x16 pixel fonts
3. **CRT effects** - Scan lines, glow, curvature
4. **Command-line interface** - Text prompts, not buttons
5. **High contrast** - Bright text on near-black

---

## 9. Implementation Status

- [x] Research complete
- [x] Specification written
- [x] JavaScript implementation
- [x] Python implementation
- [x] Rust implementation
- [x] Screenshots captured

---

*Research source: WarGames (1983), IMSAI 8080 documentation, CRT phosphor references*
