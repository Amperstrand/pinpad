# Alien (1979) Nostromo Door Control Panel

> **Source**: Alien (1979) - 20th Century Fox
> **Designer**: Ron Cobb (Semiotic Standard)
> **Context**: Nostromo commercial towing vehicle door control panels
> **Purpose**: Reference document for industrial sci-fi keypad implementation

---

## 1. Design Philosophy

### Historical Context
- Part of the "used universe" aesthetic - technology that looks lived-in
- Ron Cobb's "Semiotic Standard" icon system for spacecraft
- Industrial, practical design - not magical or overly futuristic
- Kitbashed from real industrial components

### Key Design Principles
1. **Industrial realism** - Functional, mechanical feel
2. **Wear and aging** - Grease, grime, scuffs visible
3. **High-contrast labeling** - Clear, readable text
4. **Tactile feedback** - Physical button depression
5. **Multiple modules** - Components bolted together, not seamless

---

## 2. Visual Characteristics

### Color Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Panel Background** | `#E8E6E1` | (232, 230, 225) | Off-white/cream plastic |
| **Primary Button (Blue)** | `#005A9C` | (0, 90, 156) | Standard operation/execute |
| **Emergency Button (Red)** | `#B00000` | (176, 0, 0) | Lock, emergency, stop |
| **Indicator (Amber)** | `#FFB000` | (255, 176, 0) | In progress, busy status |
| **Indicator (Green)** | `#00FF41` | (0, 255, 65) | Ready, open, success |
| **Text/Labels** | `#1A1A1A` | (26, 26, 26) | Charcoal black, semi-bold |
| **Wear/Dark Wash** | `#3A3530` | (58, 53, 48) | Grease/grime in crevices |

### Button Design

| Property | Value | Notes |
|----------|-------|-------|
| **Shape** | Square/rectangular | Slightly rounded corners (2-4px radius) |
| **Material** | Translucent plastic | "Jewel" covers over incandescent bulbs |
| **Depth** | Physical depression | Visible click depth when pressed |
| **Labeling** | Above or on button | All caps, sans-serif |
| **Glow** | Internal illumination | Visible filament/LED pattern when unlit |

### Layout Patterns
- **Vertical strips**: Groups of 7 buttons in columns
- **Horizontal grids**: 3x3 arrangements
- **Mixed**: Various module sizes bolted together

### Panel Design

| Property | Value |
|----------|-------|
| **Material** | Vacuform plastic with visible seams |
| **Texture** | Slightly textured, matte finish |
| **Borders** | Inset grooves and raised ridges |
| **Wear** | Dark wash in crevices, scuffs on edges |
| **Assembly** | Multiple modules bolted together |

---

## 3. Semiotic Standard Icons

Ron Cobb's icon set for labeling:

| Icon | Description | Usage |
|------|-------------|-------|
| **Access/Entry** | Square with vertical bar | Door controls |
| **Hazard/Pressure** | Yellow triangle, 3 vertical lines | Warning states |
| **Emergency** | Red circle with horizontal slash | Emergency stop |
| **Manual** | Hand symbol | Manual override |
| **Cycle** | Circular arrow | Door cycle operation |

---

## 4. Behavior & Animation

### Button Interaction
| State | Visual |
|-------|--------|
| **Default** | Button raised, indicator may be lit |
| **Pressed** | Physical depression, stays down if latching |
| **Active** | Button stays depressed, indicator glows |

### Light Behavior
| Pattern | Duration | Usage |
|---------|----------|-------|
| **Blink** | 1 second intervals | Waiting/processing |
| **Flicker** | Rapid, irregular | Power surge, override |
| **Steady** | Constant | Ready/success state |

### Sound Design
| Event | Sound |
|-------|-------|
| **Button press** | Heavy mechanical "clunk" |
| **Active indicator** | Low-frequency electronic hum |

---

## 5. Implementation Notes

### JavaScript (Canvas)
```javascript
const COLORS = {
    panel: '#E8E6E1',
    buttonBlue: '#005A9C',
    buttonRed: '#B00000',
    indicatorAmber: '#FFB000',
    indicatorGreen: '#00FF41',
    text: '#1A1A1A',
    wear: '#3A3530'
};

// Button with internal glow effect
function drawNostromoButton(ctx, x, y, w, h, color, lit) {
    // Button body
    ctx.fillStyle = color;
    roundRect(ctx, x, y, w, h, 3);
    ctx.fill();
    
    // Internal glow when lit
    if (lit) {
        ctx.shadowColor = color;
        ctx.shadowBlur = 10;
        ctx.fillStyle = lighten(color, 30);
        roundRect(ctx, x+2, y+2, w-4, h-4, 2);
        ctx.fill();
        ctx.shadowBlur = 0;
    }
}
```

### Python (pygame)
```python
COLORS = {
    'panel': (232, 230, 225),
    'button_blue': (0, 90, 156),
    'button_red': (176, 0, 0),
    'indicator_amber': (255, 176, 0),
    'indicator_green': (0, 255, 65),
    'text': (26, 26, 26),
    'wear': (58, 53, 48),
}
```

### Rust (embedded-graphics)
```rust
pub const PANEL_BG: Rgb888 = Rgb888::new(232, 230, 225);
pub const BUTTON_BLUE: Rgb888 = Rgb888::new(0, 90, 156);
pub const BUTTON_RED: Rgb888 = Rgb888::new(176, 0, 0);
pub const INDICATOR_AMBER: Rgb888 = Rgb888::new(255, 176, 0);
pub const INDICATOR_GREEN: Rgb888 = Rgb888::new(0, 255, 65);
```

---

## 6. Reference Materials

### Production Resources
- **Propstore**: [Light-Up Nostromo Door Control Panel](https://propstore.com/product/alien-1979/lot-4-light-up-nostromo-door-control-panel/)
- **Ron Cobb Icons**: [Semiotic Standard Gallery](https://wharferj.wordpress.com/2012/05/25/ron-cobbs-alien-semiotic-standards/)
- **Interactive Icons**: [SemioticStandard.org](https://SemioticStandard.org)

### Analysis
- **Sci-Fi Interfaces**: [Alien (1979) Analysis](https://scifiinterfaces.com/category/alien-1979/)

---

## 7. Success Criteria

Implementation should achieve:
1. **Industrial authenticity** - Looks like real spacecraft hardware
2. **Color accuracy** - Off-white panels with correct indicator colors
3. **Wear details** - Visible aging and grime effects
4. **Tactile feel** - Buttons appear to have physical depth
5. **Icon consistency** - Semiotic standard for labels

---

## 8. Implementation Status

- [x] Research complete
- [x] Specification written
- [ ] JavaScript implementation
- [ ] Python implementation
- [ ] Rust implementation
- [ ] Screenshots captured

---

*Research source: Ron Cobb production design, prop analysis, sci-fi interface studies*
