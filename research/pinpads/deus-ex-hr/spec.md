# Deus Ex: Human Revolution (2011) Hacking Keypad

> **Source**: Deus Ex: Human Revolution (2011) - Eidos Montreal / Square Enix
> **Style**: "Cyber-Renaissance" - Black and Gold aesthetic
> **Context**: Keypad entry panels and hacking mini-game overlay
> **Purpose**: Reference document for futuristic amber/gold keypad implementation

---

## 1. Design Philosophy

### Aesthetic: Cyber-Renaissance
- High-contrast **Black and Gold** palette
- Represents the "Golden Age" of human augmentation
- Sleek, futuristic but with Art Deco influences
- Geometric overlays and holographic projections

### Key Design Principles
1. **Black and Gold dominance** - Near-black backgrounds with amber/gold UI
2. **Geometric precision** - Clean lines, angular elements
3. **Holographic feel** - Translucent overlays, scan lines
4. **High contrast** - Bright gold against deep black
5. **Functional minimalism** - Every element serves a purpose

---

## 2. Visual Characteristics

### Color Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Primary Gold** | `#FFEA21` | (255, 234, 33) | High-brightness text, active highlights |
| **Amber/Gold** | `#E5AF2E` | (229, 175, 46) | Standard buttons, progress bars |
| **Dark Gold** | `#B49125` | (180, 145, 37) | Inactive elements, secondary text |
| **Deep Background** | `#131200` | (19, 18, 0) | Near-black with yellow tint |
| **Accent Cyan** | `#00FFFF` | (0, 255, 255) | I/O Port (starting node), friendly systems |
| **Alert Red** | `#FF0000` | (255, 0, 0) | Detection, trace progress, alerts |

### Keypad Layout

Standard 3x4 numeric grid:
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
```

### Button Design

| Property | Value |
|----------|-------|
| **Shape** | Square with soft rounded edges |
| **Border** | Amber border, glowing |
| **Background** | Dark/transparent |
| **Text** | Glowing digit, white-gold |
| **Press feedback** | Flash brighter, high-pitched beep |

### Display Area

- **Position**: Above keypad
- **Style**: Monospace text in holographic panel
- **Mask character**: `*` or `█` for entered digits
- **Max length**: 4-8 digits typically

---

## 3. Hacking Mini-Game Elements

### Node Types (for extended implementation)

| Node | Symbol | Color | Purpose |
|------|--------|-------|---------|
| **I/O Port** | Sphere | Cyan | Starting point |
| **Directory** | Folder | Gold | Standard nodes to capture |
| **Registry** | CPU | Gold | Target node - capture to win |
| **Datastore** | Hexagon | Gold | Optional - contains XP/credits |

### Visual Elements

| Element | Description |
|---------|-------------|
| **Paths** | Lines connecting nodes |
| **Capture progress** | Pulsing amber fill along paths |
| **Trace indicator** | Red progress bar from Registry to I/O |
| **Scanlines** | Constant horizontal scan effect |

---

## 4. Behavior & Animation

### Keypad Interaction

| Event | Visual | Duration |
|-------|--------|----------|
| **Boot-up** | Scanline effect, digital glitch resolve | 300-500ms |
| **Button press** | Flash brighter white-gold, beep | 100ms |
| **Correct digit** | Digit stays visible | - |
| **Submit** | "VERIFYING..." display | 500-1000ms |

### Authentication Result

| Result | Visual | Duration |
|--------|--------|----------|
| **Success** | Green flash, "ACCESS GRANTED" | 300-500ms |
| **Failure** | Red flash, "ACCESS DENIED" | 200-300ms |

### Ambient Effects

| Effect | Description |
|--------|-------------|
| **Scanlines** | Horizontal lines moving down at low opacity |
| **Phosphor glow** | Subtle bloom on bright elements |
| **Flicker** | Occasional brightness variation |

---

## 5. Implementation Notes

### JavaScript (Canvas)
```javascript
const COLORS = {
    background: '#131200',
    primaryGold: '#FFEA21',
    amber: '#E5AF2E',
    darkGold: '#B49125',
    cyan: '#00FFFF',
    red: '#FF0000'
};

const TIMING = {
    bootMs: 400,
    keypressFlashMs: 100,
    verifyMs: 800,
    successFlashMs: 400,
    errorFlashMs: 250
};

// Holographic button with glow
function drawHRButton(ctx, x, y, w, h, label, active) {
    const color = active ? COLORS.primaryGold : COLORS.amber;
    
    // Outer glow
    ctx.shadowColor = color;
    ctx.shadowBlur = active ? 15 : 8;
    
    // Button border
    ctx.strokeStyle = color;
    ctx.lineWidth = 2;
    roundRect(ctx, x, y, w, h, 4);
    ctx.stroke();
    
    // Label
    ctx.fillStyle = color;
    ctx.font = 'bold 24px monospace';
    ctx.textAlign = 'center';
    ctx.fillText(label, x + w/2, y + h/2 + 8);
    
    ctx.shadowBlur = 0;
}
```

### Python (pygame)
```python
COLORS = {
    'background': (19, 18, 0),
    'primary_gold': (255, 234, 33),
    'amber': (229, 175, 46),
    'dark_gold': (180, 145, 37),
    'cyan': (0, 255, 255),
    'red': (255, 0, 0),
}

TIMING = {
    'boot_ms': 400,
    'keypress_flash_ms': 100,
    'verify_ms': 800,
    'success_flash_ms': 400,
    'error_flash_ms': 250,
}
```

### Rust (embedded-graphics)
```rust
pub const BACKGROUND: Rgb888 = Rgb888::new(19, 18, 0);
pub const PRIMARY_GOLD: Rgb888 = Rgb888::new(255, 234, 33);
pub const AMBER: Rgb888 = Rgb888::new(229, 175, 46);
pub const DARK_GOLD: Rgb888 = Rgb888::new(180, 145, 37);
pub const CYAN: Rgb888 = Rgb888::new(0, 255, 255);
pub const ALERT_RED: Rgb888 = Rgb888::new(255, 0, 0);
```

---

## 6. Reference Materials

### Screenshots & Documentation
- **Deus Ex Wiki**: [Interface Images](https://deusex.fandom.com/wiki/Category:Deus_Ex:_Human_Revolution_interface_images)
- **Keycodes Page**: [DXHR Keycodes](https://deusex.fandom.com/wiki/Keycodes_(DXHR))
- **Design Analysis**: [HUDS+GUIS Review](https://www.hudsandguis.com/home/2014/05/07/deus-ex-human-revolution-ui)
- **UI Database**: [Game UI Database - DX:HR](https://www.gameuidatabase.com/gameData.php?id=283)

---

## 7. Success Criteria

Implementation should achieve:
1. **Color accuracy** - Black and gold Cyber-Renaissance palette
2. **Holographic feel** - Glow effects, scan lines
3. **Geometric precision** - Clean, angular design
4. **Animation timing** - Match game's boot/keypress timing
5. **High contrast** - Bright gold on near-black

---

## 8. Implementation Status

- [x] Research complete
- [x] Specification written
- [ ] JavaScript implementation
- [ ] Python implementation
- [ ] Rust implementation
- [ ] Screenshots captured

---

*Research source: Deus Ex: Human Revolution gameplay, Eidos Montreal design, cyberpunk UI analysis*
