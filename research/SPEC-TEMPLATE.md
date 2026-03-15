# [SOURCE NAME] Keypad - Research Spec

> **Source**: [Movie/Game Name] ([Year]) - [Studio/Developer]
> **Context**: [Scene/Level/Context where keypad appears]
> **Effect**: [Brief description of visual style]
> **Purpose**: Reference document for accurate implementation

---

## 1. Design Philosophy

### Historical Context
- [When was it made? What era of technology?]
- [What real-world influences?]

### Key Design Principles
1. **[Principle 1]** - [Description]
2. **[Principle 2]** - [Description]
3. **[Principle 3]** - [Description]

---

## 2. Visual Characteristics

### Color Palette (REQUIRED)

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Background** | `#000000` | (0, 0, 0) | Screen background |
| **Primary Text** | `#00FF00` | (0, 255, 0) | Main text color |
| **Secondary** | `#008800` | (0, 136, 0) | Dim text, borders |
| **Accent** | `#FFFF00` | (255, 255, 0) | Highlights, alerts |
| **Error** | `#FF0000` | (255, 0, 0) | Error states |

### Typography (REQUIRED)

| Property | Value |
|----------|-------|
| **Primary Font** | 'FontName', 'Fallback1', 'Fallback2', monospace |
| **Size** | 16px |
| **Line-height** | 1.4 |
| **Weight** | Regular |

### Button Design

| Property | Value |
|----------|-------|
| **Shape** | [Square/Round/Hexagon/etc] |
| **Size** | 60x50 px |
| **Border radius** | 4px |
| **Border style** | 2px solid [color] |

### Button States (REQUIRED)

| State | Background | Border | Text | Effect |
|-------|------------|--------|------|--------|
| **Default** | `#000000` | `#008800` | `#00FF00` | None |
| **Hover** | `#002200` | `#00FF00` | `#00FF00` | Glow |
| **Pressed** | `#004400` | `#00FF00` | `#FFFFFF` | Flash |
| **Disabled** | `#111111` | `#333333` | `#666666` | None |

---

## 3. Effects Parameters (REQUIRED)

### Glow Effect

| Property | Value |
|----------|-------|
| **Radius** | 15px |
| **Intensity** | 0.5 (50%) |
| **Color** | Same as text |

### Scan Lines (if applicable)

| Property | Value |
|----------|-------|
| **Spacing** | 2px |
| **Opacity** | 0.15 |
| **Color** | rgba(0, 0, 0, 0.15) |

### Other Effects

| Effect | Value | Notes |
|--------|-------|-------|
| **Vignette** | 0.2 | Corner darkening |
| **Flicker** | 0.005 | Random chance per frame |
| **Noise** | 0.08 | Luminance variation |

---

## 4. Behavior & Animation

### Timing Values (REQUIRED)

| Event | Duration (ms) | Easing |
|-------|---------------|--------|
| **Keypress flash** | 100 | ease-out |
| **Cursor blink** | 530 | step (hard on/off) |
| **Transition fade** | 300 | ease-in-out |
| **Error flash** | 250 | linear |
| **Success flash** | 400 | ease-out |

### Input Behavior

| Action | Response |
|--------|----------|
| **Digit press** | Flash button, add to display |
| **Clear press** | Clear display, no flash |
| **Enter press** | Validate code |
| **Correct code** | Success flash, "ACCESS GRANTED" |
| **Wrong code** | Error flash, "ACCESS DENIED" |

### Authentication Flow

```
1. User enters code (digit by digit)
2. Display shows: **** (masked)
3. User presses Enter
4. Display: "VERIFYING..." (500ms)
5. Result:
   - SUCCESS: Green flash, "ACCESS GRANTED"
   - FAILURE: Red flash, "ACCESS DENIED", 1s lockout
```

---

## 5. Keypad Layout

```
┌─────┬─────┬─────┐
│  1  │  2  │  3  │
├─────┼─────┼─────┤
│  4  │  5  │  6  │
├─────┼─────┼─────┤
│  7  │  8  │  9  │
├─────┼─────┼─────┤
│  C  │  0  │  E  │
└─────┴─────┴─────┘

C = Clear
E = Enter
```

### Keyboard Mapping

| Key | Action |
|-----|--------|
| 0-9 | Enter digit |
| C / Backspace | Clear |
| E / Enter | Submit |

---

## 6. Display Area

```
┌─────────────────────────────┐
│  [TITLE]                    │
│  ─────────────────────      │
│  CODE: ████                 │
│  ─────────────────────      │
│  STATUS: AWAITING INPUT     │
└─────────────────────────────┘
```

| Element | Style |
|---------|-------|
| **Title** | Uppercase, primary color |
| **Separator** | Dashed line, secondary color |
| **Code display** | Monospace, masked with █ or * |
| **Status** | Lowercase, secondary color |

---

## 7. Implementation Notes

### JavaScript (Canvas)

```javascript
const CONFIG = {
    colors: {
        background: '#000000',
        primary: '#00FF00',
        secondary: '#008800',
        accent: '#FFFF00',
        error: '#FF0000'
    },
    timing: {
        keypressFlash: 100,
        cursorBlink: 530,
        transitionFade: 300,
        errorFlash: 250,
        successFlash: 400
    },
    effects: {
        glowRadius: 15,
        glowIntensity: 0.5,
        scanlineSpacing: 2,
        scanlineOpacity: 0.15
    }
};
```

### Python (pygame)

```python
COLORS = {
    'background': (0, 0, 0),
    'primary': (0, 255, 0),
    'secondary': (0, 136, 0),
    'accent': (255, 255, 0),
    'error': (255, 0, 0),
}

TIMING = {
    'keypress_flash_ms': 100,
    'cursor_blink_ms': 530,
    'transition_fade_ms': 300,
    'error_flash_ms': 250,
    'success_flash_ms': 400,
}
```

### Rust (embedded-graphics)

```rust
pub const BACKGROUND: Rgb888 = Rgb888::new(0, 0, 0);
pub const PRIMARY: Rgb888 = Rgb888::new(0, 255, 0);
pub const SECONDARY: Rgb888 = Rgb888::new(0, 136, 0);
pub const ACCENT: Rgb888 = Rgb888::new(255, 255, 0);
pub const ERROR: Rgb888 = Rgb888::new(255, 0, 0);

pub const TIMING: Timing = Timing {
    keypress_flash_ms: 100,
    cursor_blink_ms: 530,
    transition_fade_ms: 300,
    error_flash_ms: 250,
    success_flash_ms: 400,
};
```

---

## 8. Reference Materials

### Screenshots/Video
- [Link to reference images]
- [Link to video clips]

### Color Analysis
- [Link to color palette extraction]

### Technical References
- [Related documentation]

---

## 9. Success Criteria

Implementation should achieve:

1. **[Criterion 1]** - [Measurable standard]
2. **[Criterion 2]** - [Measurable standard]
3. **[Criterion 3]** - [Measurable standard]
4. **Cross-platform parity** - Same config in JS/Python/Rust
5. **Test coverage** - Minimum 5 Rust tests

---

## 10. Implementation Status

- [ ] Research complete
- [ ] Specification written
- [ ] JavaScript implementation
- [ ] Python implementation
- [ ] Rust implementation
- [ ] Screenshots captured
- [ ] Tests passing (5+)

---

*Spec template version: 1.0*
*Generated: 2026-03-15*
