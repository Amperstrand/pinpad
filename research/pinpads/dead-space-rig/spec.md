# Dead Space (2008) RIG Interface

> **Source**: Dead Space (2008) - Visceral Games / EA
> **Style**: Diegetic UI - Interface exists in game world
> **Context**: Resource Integration Gear (RIG) - health, stasis, inventory
> **Purpose**: Reference document for holographic control surface implementation

---

## 1. Design Philosophy

### Diegetic UI Principle
- All UI elements exist **physically in the game world**
- Health bar on character's spine (visible to player)
- Holographic menus projected from suit
- No traditional HUD overlays

### Key Design Principles
1. **Dead Space Blue** - Signature cyan glow
2. **Holographic projection** - 3D floating elements
3. **Transparency** - Environment visible through UI
4. **Jitter/instability** - Imperfect projection
5. **Scan lines** - Constant refresh effect

---

## 2. Visual Characteristics

### Color Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Primary Cyan** | `#00FFFF` | (0, 255, 255) | Primary glow, highlights |
| **Deep Blue** | `#0047AB` | (0, 71, 171) | Darker backing elements |
| **Health Teal** | `#00FFCC` | (0, 255, 204) | Health bar (full) |
| **Stasis Blue** | `#3399FF` | (51, 153, 255) | Stasis module gauge |
| **Alert Red** | `#FF3300` | (255, 51, 0) | Critical/damage states |
| **Text Blue** | `#A0E6FF` | (160, 230, 255) | Legible text overlay |
| **Deep Black** | `#0A0A0A` | (10, 10, 10) | Background (rare) |

### UI Element Design

#### Spine Health Bar
- **Orientation**: Vertical, segmented
- **Position**: Character's back/spine
- **Segments**: 3-5 visible tubes
- **Colors**: Cyan (full) → Yellow (mid) → Red (critical)
- **Animation**: Slow pulse when full, erratic when low

#### Stasis Module
- **Shape**: Circular gauge
- **Position**: Right shoulder/upper back
- **Fill**: Rotating light ring
- **Color**: `#3399FF`

#### Holographic Menus
- **Projection source**: Chest plate
- **3D space**: Slightly curved, exists in world
- **Transparency**: 60-80% opacity
- **Size**: Floating ~1m in front of character

---

## 3. Holographic Effects

### Transparency & Depth
```css
.holographic {
    opacity: 0.75;
    background: rgba(0, 255, 255, 0.1);
    border: 1px solid rgba(0, 255, 255, 0.5);
}
```

### Chromatic Aberration
- **Red/blue fringing** at edges of text and icons
- **More visible** at periphery of holographic elements

### Jitter Effect
- **High-frequency position offset**: 1-2 pixels
- **Simulates** slightly unstable projection
- **CSS**: `transform: translate(${Math.random()*2}px, ${Math.random()*2}px)`

### Volumetric Light
- **"God rays"** from projection source
- **Cone of light** connecting chest to floating UI

---

## 4. Animation Patterns

### Boot-up Sequence
| Phase | Duration | Visual |
|-------|----------|--------|
| **Initialize** | 200ms | Single point of light |
| **Expand** | 300ms | Folds out / "rezes" in |
| **Glitch** | 100ms | Digital artifact |
| **Ready** | - | Full UI visible |

### Scan Lines
- **Direction**: Vertical or horizontal
- **Speed**: Slow drift across surface
- **Opacity**: 5-10% darker lines

### Pulse Effect
| Health State | Pulse Speed |
|--------------|-------------|
| **Full** | Slow, rhythmic (2s cycle) |
| **Mid** | Faster (1s cycle) |
| **Critical** | Erratic, fast (0.5s) |

---

## 5. Implementation Notes

### JavaScript (Canvas)
```javascript
const COLORS = {
    primaryCyan: '#00FFFF',
    deepBlue: '#0047AB',
    healthTeal: '#00FFCC',
    stasisBlue: '#3399FF',
    alertRed: '#FF3300',
    textBlue: '#A0E6FF'
};

// Holographic glow
function drawHolographic(ctx, x, y, w, h) {
    ctx.globalAlpha = 0.75;
    ctx.strokeStyle = COLORS.primaryCyan;
    ctx.shadowColor = COLORS.primaryCyan;
    ctx.shadowBlur = 15;
    ctx.strokeRect(x, y, w, h);
    ctx.globalAlpha = 1;
    ctx.shadowBlur = 0;
}

// Jitter effect
function applyJitter(x, y) {
    return {
        x: x + (Math.random() - 0.5) * 2,
        y: y + (Math.random() - 0.5) * 2
    };
}
```

### Python (pygame)
```python
COLORS = {
    'primary_cyan': (0, 255, 255),
    'deep_blue': (0, 71, 171),
    'health_teal': (0, 255, 204),
    'stasis_blue': (51, 153, 255),
    'alert_red': (255, 51, 0),
    'text_blue': (160, 230, 255),
}
```

### Rust (embedded-graphics)
```rust
pub const PRIMARY_CYAN: Rgb888 = Rgb888::new(0, 255, 255);
pub const DEEP_BLUE: Rgb888 = Rgb888::new(0, 71, 171);
pub const HEALTH_TEAL: Rgb888 = Rgb888::new(0, 255, 204);
pub const ALERT_RED: Rgb888 = Rgb888::new(255, 51, 0);
```

---

## 6. Control Surface Adaptation

**Note**: Dead Space RIG is not a traditional keypad. For implementation:

### Option A: Status Display
- Show health/stasis gauges
- Holographic projection aesthetic
- Read-only display

### Option B: Inventory Grid
- 3x3 or 4x4 item slots
- Holographic selection highlight
- Navigate with directional input

### Option C: Locator Panel
- Directional guide display
- Objective markers
- Map overlay

---

## 7. Reference Materials

### Game References
- **Dead Space Wiki**: [Resource Integration Gear](https://deadspace.fandom.com/wiki/Resource_Integration_Gear)
- **Game UI Database**: [Dead Space UI](https://www.gameuidatabase.com/gameData.php?id=581)
- **Ars Technica**: [Producer Interview on UI](https://arstechnica.com/gaming/2008/05/guest-writer-dead-space-producer-chuck-beaver-on-story-ui/)

---

## 8. Success Criteria

1. **Dead Space Blue** - Signature cyan glow
2. **Holographic feel** - Transparency, projection
3. **Jitter/instability** - Imperfect projection effect
4. **Scan lines** - Constant refresh
5. **Chromatic aberration** - Edge color fringing

---

## 9. Implementation Status

- [x] Research complete
- [x] Specification written
- [x] JavaScript implementation
- [x] Python implementation
- [x] Rust implementation
- [x] Screenshots captured

---

*Research source: Dead Space (2008), Visceral Games design, diegetic UI analysis*
