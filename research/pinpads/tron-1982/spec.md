# Tron (1982) Control Panel

> **Source**: Tron (1982) - Walt Disney Productions
> **Designers**: Syd Mead, Jean "Moebius" Giraud
> **Style**: "Digital Frontier" - Neon on black, angular geometry
> **Purpose**: Reference document for neon circuit keypad implementation

---

## 1. Design Philosophy

### Historical Context
- Pioneering CGI visualization
- "Backlit animation" using Kodalith cels
- Self-illuminated world (no external lighting)
- Technical brutalism with geometric precision

### Key Design Principles
1. **Self-illumination** - Everything glows, no shadows
2. **45°/90° angles only** - No curves in circuit traces
3. **Neon palette** - Blue primary, orange enemy, white hot
4. **Geometric buttons** - Hexagons, octagons, beveled rects
5. **Circuit integration** - Keypad is part of the Grid

---

## 2. Visual Characteristics

### Color Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Neon Blue** | `#2AD2FF` | (42, 210, 255) | Primary hero circuits |
| **Neon Orange** | `#FF9D00` | (255, 157, 0) | Enemy/MCP elements |
| **Neon White** | `#E0F7FF` | (224, 247, 255) | Hot center of glow |
| **Deep Black** | `#030504` | (3, 5, 4) | Background void |
| **Grid Cyan** | `#008CA3` | (0, 140, 163) | Secondary structure |

### Button Design

| Property | Value |
|----------|-------|
| **Shape** | Hexagons, octagons, beveled rectangles |
| **Bevel** | 45° angle on edges |
| **Layout** | Staggered patterns, follow bus lines |
| **Grouping** | Asymmetrical clusters |
| **Core** | Near-white bright center |

### Circuit Traces

| Property | Value |
|----------|-------|
| **Angle constraint** | 45° or 90° only |
| **Line weights** | Main bus (thick) → sub-routines (thin) |
| **Terminators** | Small squares/circles at line ends |
| **Symmetry** | Asymmetrical (complex architecture) |

---

## 3. Neon Glow Effects

### Inner Glow (Button Core)
- **Color**: Near-white (`#E0F7FF`)
- **Appearance**: Bright, almost solid center

### Outer Bloom
- **Color**: Primary neon (blue or orange)
- **Extent**: ~20% of button width
- **Blur**: Gaussian soft edge

### 1982 Backlit Technique
```css
.tron-glow {
    background: #030504;
    border: 2px solid #2AD2FF;
    box-shadow: 
        0 0 10px #2AD2FF,
        0 0 20px #2AD2FF,
        inset 0 0 10px rgba(42, 210, 255, 0.5);
}

/* NO drop shadows - everything is self-illuminated */
```

### Flicker Effect
- **Variation**: 90-100% opacity
- **Frequency**: High (subtle power hum)
- **CSS**: `animation: flicker 0.1s infinite`

---

## 4. Animation Patterns

### Boot-up
| Phase | Duration | Visual |
|-------|----------|--------|
| **Dark** | - | All off |
| **Trace** | 500ms | Circuits light sequentially |
| **Buttons** | 200ms | Keypad elements illuminate |
| **Ready** | - | Full glow, subtle flicker |

### Button Press
| Event | Visual |
|-------|--------|
| **Press** | Flash to white core |
| **Hold** | Brighter glow |
| **Release** | Return to normal |

### Circuit Animation
- **Flow**: Subtle pulse along traces
- **Direction**: From power source to elements
- **Speed**: Slow, rhythmic

---

## 5. Implementation Notes

### JavaScript (Canvas)
```javascript
const COLORS = {
    neonBlue: '#2AD2FF',
    neonOrange: '#FF9D00',
    neonWhite: '#E0F7FF',
    deepBlack: '#030504',
    gridCyan: '#008CA3'
};

// Self-illuminated button - NO drop shadows
function drawTronButton(ctx, x, y, w, h, color) {
    // Outer glow
    ctx.shadowColor = color;
    ctx.shadowBlur = 15;
    
    // Border
    ctx.strokeStyle = color;
    ctx.lineWidth = 2;
    ctx.strokeRect(x, y, w, h);
    
    // Inner glow
    ctx.fillStyle = COLORS.neonWhite;
    ctx.globalAlpha = 0.3;
    ctx.fillRect(x + 4, y + 4, w - 8, h - 8);
    ctx.globalAlpha = 1;
    
    ctx.shadowBlur = 0;
}

// Hexagonal button
function drawHexButton(ctx, cx, cy, radius, color) {
    ctx.beginPath();
    for (let i = 0; i < 6; i++) {
        const angle = (Math.PI / 3) * i - Math.PI / 2;
        const x = cx + radius * Math.cos(angle);
        const y = cy + radius * Math.sin(angle);
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
    }
    ctx.closePath();
    
    ctx.shadowColor = color;
    ctx.shadowBlur = 15;
    ctx.strokeStyle = color;
    ctx.stroke();
    ctx.shadowBlur = 0;
}
```

### Python (pygame)
```python
COLORS = {
    'neon_blue': (42, 210, 255),
    'neon_orange': (255, 157, 0),
    'neon_white': (224, 247, 255),
    'deep_black': (3, 5, 4),
    'grid_cyan': (0, 140, 163),
}
```

### Rust (embedded-graphics)
```rust
pub const NEON_BLUE: Rgb888 = Rgb888::new(42, 210, 255);
pub const NEON_ORANGE: Rgb888 = Rgb888::new(255, 157, 0);
pub const NEON_WHITE: Rgb888 = Rgb888::new(224, 247, 255);
pub const DEEP_BLACK: Rgb888 = Rgb888::new(3, 5, 4);
```

---

## 6. Reference Materials

### Production References
- **Syd Mead Concept Art**: Tron production sketches
- **MCP Interface**: [Movie screencaps](https://movie-screencaps.com/tron-1982/)
- **Arcade Cabinet**: 1982 Bally Midway (real-world implementation)
- **Prop Decals**: [Movie Reliquary](https://movie-reliquary.com/)

---

## 7. Success Criteria

1. **Self-illumination** - NO shadows, only glow
2. **Geometric buttons** - Hexagons, octagons, beveled
3. **45°/90° circuits** - Angular traces only
4. **Neon palette** - Blue primary, orange enemy
5. **Backlit feel** - Inner white core, outer bloom

---

## 8. Implementation Status

- [x] Research complete
- [x] Specification written
- [ ] JavaScript implementation
- [ ] Python implementation
- [ ] Rust implementation

---

*Research source: Tron (1982), Syd Mead design, backlit animation technique*
