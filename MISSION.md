# PINPAD - Agentic Development Experiment

## Mission

Build a collection of pinpad implementations from movies, video games, and popular culture using agentic AI development. Each pinpad should capture the visual and interactive essence of its source material.

## Philosophy

This repository explores how AI agents can:
1. Understand visual/interactive references from media
2. Research and synthesize information about specific effects
3. Implement consistent code across multiple platforms
4. Document their reasoning and decision-making process

---

## Current Target: Thermal Pinpad (Splinter Cell)

**Source:** Tom Clancy's Splinter Cell (2002) - Chinese Embassy mission  
**Effect:** Thermal vision reveals heat signatures on keypad buttons after guards enter codes

### Visual Characteristics
- Heat signatures appear as orange/yellow glow on pressed buttons
- Intensity fades over ~30 seconds (exponential decay)
- More recent presses are brighter
- Thermal color gradient: cold (blue) → warm (orange/yellow)

### Implementation Targets

| Platform | Purpose | Status |
|----------|---------|--------|
| JavaScript | Browser demo, reference implementation | Pending |
| Python (LVGL) | Embedded systems (ESP32, etc.) | Pending |
| Rust (embedded-graphics) | Embedded systems (STM32, etc.) | Pending |

### Core Components

1. **Thermal Logic** (platform-agnostic)
   - Heat intensity calculation (0.0 to 1.0)
   - Exponential decay formula: `intensity = e^(-decay_progress * 3)`
   - Button state management

2. **Rendering Layer** (platform-specific)
   - Concentric circle glow effect (8-12 rings)
   - Color palette mapping
   - Animation loop

3. **Color Palettes**
   - **Splinter Cell** (default): dark blue → cyan → green → yellow → orange
   - **Classic**: blue → cyan → yellow → orange → red
   - **Ironbow**: professional thermal camera look
   - **Hot/Cold**: blue → white → red

---

## Future Pinpads (Backlog)

Ideas for future implementations:

- **Alien: Isolation** - Door panel interfaces
- **Mr. Robot** - Terminal keypads
- **Blade Runner 2049** - DNA sequencer interface
- **The Matrix** - Code entry screens
- **Fallout** - Vault terminal interface
- **Star Trek** - LCARS panels
- **Minority Report** - Gesture interfaces

---

## Repository Structure

```
pinpad/
├── MISSION.md              # This file - project definition
├── README.md               # User documentation
├── prompts/                # Task definitions for AI agents
│   └── INITIAL.md
├── keypads/                # Reference images/screenshots
├── javascript/             # JS implementations
├── python/                 # Python implementations  
├── rust/                   # Rust implementations
└── docs/                   # Additional documentation
```

## How to Use This Repository with AI Agents

1. Point the AI agent to `MISSION.md` for context
2. Use `prompts/INITIAL.md` for the first task
3. Create new prompt files for additional features
4. Document decisions in `docs/` as the project evolves
