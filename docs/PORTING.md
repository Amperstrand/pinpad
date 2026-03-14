# Porting Guide

This document describes how to port the thermal pinpad library to embedded systems.

## Supported Platforms

- **ESP32** with LVGL/MicroPython
- **STM32** with embedded-graphics
- **nRF52** with embedded-graphics
- **Any embedded-graphics compatible display**

## Core Concepts

The thermal pinpad library is platform-agnostic:

### 1. Thermal Logic (no dependencies)

```rust
use thermal_pinpad::{ThermalConfig, ThermalKeypad};

// Create keypad with custom config
let config = ThermalConfig::new()
    .decay_time_ms(30000)  // 30 seconds
    .min_visible_intensity(0.02)
    .num_rings(10);

let mut keypad = ThermalKeypad::with_config(config);

// Press buttons
keypad.press('1', 0);    // timestamp 0
keypad.press('2', 500);   // timestamp 500ms
keypad.press('3', 1000);   // timestamp 1000ms

// Get intensities at current time
let intensities = keypad.intensities(5000);  // 5 seconds later
for (label, intensity) in intensities {
    if intensity > 0.02 {
        println!("Button {} has intensity {:.2}", label, intensity);
    }
}
```

### 2. Color Mapping (requires embedded-graphics)

```rust
use embedded_graphics::pixelcolor::Rgb888;
use thermal_pinpad::{ThermalPalette, ThermalColorMapper};

let mapper = ThermalColorMapper::with_palette(ThermalPalette::SplinterCell);
let color = mapper.intensity_to_rgb(0.8);  // Get RGB for 80% intensity
```

### 3. Ring Intensity Calculation

```rust
use thermal_pinpad::ring_intensity;

let base = 0.8;
let ring_idx = 3;
let total_rings = 10;
let ring_int = ring_intensity(base, ring_idx, total_rings);
```

## Platform-Specific Integration

### ESP32 with LVGL

```c
// 1. Create LVGL display buffer
// 2. Port ThermalKeypad logic (copy from thermal.rs)
// 3. Implement rendering using LVGL canvas API
// 4. Use LVGL timer for animation loop
```

### STM32 with embedded-graphics

```rust
use embedded_graphics::prelude::*;
use thermal_pinpad::{ThermalKeypad, ThermalColorMapper, ring_intensity};

// In your main loop:
loop {
    let now = get_system_tick_ms();
    let intensities = keypad.intensities(now);
    
    // Clear display
    display.clear(Rgb565::BLACK)?;
    
    // Draw each button
    for row in 0..4 {
        for col in 0..3 {
            let (label, intensity) = get_intensity_for_button(row, col, &intensities);
            draw_thermal_button(&mut display, row, col, intensity, &color_mapper)?;
        }
    }
    
    // Flush to hardware display
    display.flush()?;
}
```

## Memory Considerations

- ThermalKeypad: ~200 bytes (12 buttons × ~16 bytes each)
- ThermalConfig: 12 bytes
- Intensity array: 96 bytes (12 × 8 bytes)
- No heap allocations required

## Performance Tips

1. **Update rate**: 30-60fps is sufficient for thermal decay
2. **Ring rendering**: Skip rings below min_visible_intensity
3. **Color caching**: Cache intensity_to_rgb results for common values

## Troubleshooting
- **Buttons not showing heat**: Check timestamp units (ms vs µs)
- **Decay too fast/slow**: Verify decay_time_ms setting
- **Colors look wrong**: Check palette selection and intensity mapping
