# Pinpad

[![CI](https://github.com/Amperstrand/pinpad/actions/workflows/screenshots.yml/badge.svg)](https://github.com/Amperstrand/pinpad/actions/workflows/screenshots.yml)
[![Demo](https://img.shields.io/badge/demo-GitHub%20Pages-blue)](https://amperstrand.github.io/pinpad/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A collection of pinpad implementations from movies, video games, and popular culture, built through agentic AI development.

## Quick Links

- [Live Demo (GitHub Pages)](https://amperstrand.github.io/pinpad/)
- [Screenshots Gallery](https://amperstrand.github.io/pinpad/#screenshots)
- [CI Pipeline](https://github.com/Amperstrand/pinpad/actions/workflows/screenshots.yml)
- [Thermal Visual Analysis](research/pinpads/thermal-splinter-cell/visual-analysis.md)
- [Cycle Comparisons](research/pinpads/thermal-splinter-cell/comparisons/)

## Current Project

**Thermal Pinpad** - Splinter Cell (2002) thermal vision effect

See [MISSION.md](MISSION.md) for project overview and [research/pinpads/thermal-splinter-cell/spec.md](research/pinpads/thermal-splinter-cell/spec.md) for detailed specifications.

## Implementations

| Platform | Location | Status |
|----------|----------|--------|
| JavaScript | `javascript/thermal-pinpad.html` | ✅ Complete |
| Python | `python/thermal_pinpad.py` | ✅ Complete |
| Rust | `rust/` | ✅ Complete |

## Platform Instructions

### JavaScript (Web Demo)
**No installation required** - Simply open `javascript/thermal-pinpad.html` in any modern browser.

```bash
# Direct open
open javascript/thermal-pinpad.html

# Or serve locally
cd javascript && python3 -m http.server 8080
# Visit http://localhost:8080/thermal-pinpad.html
```

### Python (Embedded + Desktop)
**Target**: Raspberry Pi/touch targets via LVGL, plus desktop parity demo via pygame.

```bash
# Install dependencies
pip install -r python/requirements.txt

# Run LVGL demo (embedded Linux with touchscreen)
python python/lvgl_demo.py

# Or run pygame demo (desktop parity testing)
python python/thermal_pinpad.py
```

### Rust (embedded-graphics)
**Target**: STM32F469-DISCO or similar ARM Cortex boards with displays.

```bash
cd rust

# PC simulator (requires SDL2)
cargo run --features simulator

# For actual hardware, use your preferred flash tool
# The library is no_std compatible and uses fixed-point math
cargo build --release --target thumbv7em-none-eabihf
```

## Usage

### JavaScript Controls
- **Click** buttons to simulate key presses
- **Demo Mode** - Auto-enters random codes every 8 seconds
- **Reset** - Clear all heat signatures
- **Random Code** - Enter a single random 4-digit code
- **Palette** dropdown - Switch color schemes

### Python Controls
- **Click** buttons to simulate key presses
- **D key** - Toggle demo mode
- **R key** - Reset all heat
- **P key** - Cycle through palettes

### Rust Controls
- **Click** buttons to simulate key presses
- **D key** - Toggle demo mode (auto-enters codes every 8s)
- **R key** - Reset all heat signatures
- **P key** - Cycle through color palettes
- **ESC** - Exit simulator

## Color Palettes

| Palette | Description |
|---------|-------------|
| Splinter Cell | Deep blue-black -> cobalt -> cyan -> green -> yellow -> near-white (default) |
| Classic | Blue → cyan → yellow → orange → red |
| Ironbow | Black → purple → red → orange → yellow → white |
| Hot/Cold | Blue → white → red |

## Visual Accuracy Notes

- The Splinter Cell palette and glow style were refined through a 3-cycle process documented in `research/pinpads/thermal-splinter-cell/visual-analysis.md`.
- Cycle-by-cycle artifacts and analyses are in `research/pinpads/thermal-splinter-cell/comparisons/`.
- Final target preserves gameplay clarity first: brightest key = most recent press, faintest key = oldest press.

## Thermal Effect Details

- **Decay Time**: 30 seconds (configurable)
- **Decay Formula**: `intensity = e^(-decay_progress * 3)`
- **Ring Intensity**: `ring_intensity = base * (1 - ring_idx/total_rings)^2`
- **Minimum Visible**: 0.02 intensity threshold

## For Embedded Developers

See [docs/PORTING.md](docs/PORTING.md) for instructions on porting to ESP32, STM32, and other embedded platforms.

## Development

Read [MISSION.md](MISSION.md) for project overview.
Read [prompts/INITIAL.md](prompts/INITIAL.md) for implementation requirements.

## License

MIT

> **STATUS: ARCHIVED EXPERIMENT (2026-09-04)** — agentic-AI development scaffold, no
> completed implementation. Kept as a workflow artifact only.
