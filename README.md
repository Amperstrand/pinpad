# Pinpad

A collection of pinpad implementations from movies, video games, and popular culture, built through agentic AI development.

## Current Project

**Thermal Pinpad** - Splinter Cell (2002) thermal vision effect

See [MISSION.md](MISSION.md) for project overview andSee [research/pinpads/thermal-splinter-cell/spec.md](research/pinpads/thermal-splinter-cell/spec.md) for detailed specifications.

## Implementations

| Platform | Location | Status |
|----------|----------|--------|
| JavaScript | `javascript/thermal-pinpad.html` | ✅ Complete |
| Python | `python/thermal_pinpad.py` | ✅ Complete |
| Rust | `rust/` | ✅ Complete |

## Quick Start

### JavaScript (Browser Demo)

```bash
# Just open in browser
open javascript/thermal-pinpad.html
```

Or serve locally:
```bash
cd javascript
python3 -m http.server 8080
# Open http://localhost:8080/thermal-pinpad.html
```

### Python (Tkinter Demo)

```bash
python3 python/thermal_pinpad.py
```

### Rust (PC Simulator)

```bash
cd rust
cargo run --features simulator
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
| Splinter Cell | Dark blue → cyan → green → yellow → orange (default) |
| Classic | Blue → cyan → yellow → orange → red |
| Ironbow | Black → purple → red → orange → yellow → white |
| Hot/Cold | Blue → white → red |

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
