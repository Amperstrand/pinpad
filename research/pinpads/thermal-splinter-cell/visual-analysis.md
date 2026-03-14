# Splinter Cell Thermal Vision Visual Analysis

## Scope
- Target: original Splinter Cell (2002), keypad heat-trace mechanic in Chinese Embassy missions
- Goal: improve resemblance while keeping JS/Python/Rust outputs visually consistent
- Method: 3 iterative cycles with explicit discrepancy tracking

## Evidence Used
- Splinter Cell Wiki thermal vision overview: https://splintercell.fandom.com/wiki/Thermal_vision
- Chinese Embassy (Part 2) mission notes and keypad code entries: https://splintercell.fandom.com/wiki/Chinese_Embassy_(Part_2)
- Keypad code reference page: https://splintercell.fandom.com/wiki/Keypad_lock/Codes
- IGN Mission 8 walkthrough text (explicit thermal keypad interaction flow): https://www.ign.com/wikis/tom-clancys-splinter-cell/Walkthrough:_Mission_8
- Thermal keypad clip entry page: https://www.youtube.com/watch?v=lVNlggJECwc
- Mission run references:
  - https://www.youtube.com/watch?v=0q76li3FaRM
  - https://www.youtube.com/watch?v=0pmCchkYOdA
- Novel excerpt (luminescence ordering): https://pastebin.com/60WPpqbn
- **Cycle 4 additions**:
  - Unity URP thermal vision (Fraser Hutchison - fraserh.dev): HSV temperature mapping
  - Unity thermal post-processing shader (mert-dev-acc/ThermalVisionPostProcessingShader)
  - GLSL Predator thermal vision (Geeks3D)
  - Roblox Splinter Cell goggles (pixeldippz): HSV interpolation reference
  - SCCT Versus Reloaded mod: https://allypal.github.io/SCCT_Versus_Reloaded/

## Verified Gameplay Behavior
- Heat marks are a short-lived interaction aid for keypad reconstruction
- Brightest mark indicates most recent keypress
- Faintest mark indicates earliest keypress
- The effect is stylized game thermal imaging, not scientific camera output

## Color Findings (Reference-Constrained)
- Confirmed palette ordering from references: dark cool background -> cyan/green mids -> yellow/highlights
- **CRITICAL UPDATE (Cycle 4)**: Peak color should be warm yellow-orange (#f0d020), NOT near-white (#fffadc)
  - Source: SC1 Xbox original footage analysis - thermal peaks at yellow-orange, not white
  - Reference: Unity URP thermal vision (Fraser Hutchison), Roblox SC goggles (pixeldippz)
- **NEW**: HSV interpolation replaces RGB for perceptually smoother thermal gradients
  - RGB interpolation causes muddy transitions; HSV maintains hue continuity
  - Hue wraparound handled for blue→cyan→green→yellow→orange progression
- **NEW**: Gamma curve (γ=1.3) applied for better perceptual separation between key intensities
- Final HSV palette stops for Splinter Cell mode (h: 0-360, s: 0-100, v: 0-100):
  - `t=0.00`: h=230, s=85, v=12 (deep blue-black #05081e)
  - `t=0.20`: h=225, s=90, v=43 (cobalt #0c2a6b)
  - `t=0.40`: h=195, s=88, v=67 (cyan #1478b0)
  - `t=0.62`: h=150, s=58, v=75 (green #52be87)
  - `t=0.80`: h=55,  s=60, v=86 (yellow #d8da58)
  - `t=0.92`: h=48,  s=90, v=94 (warm yellow #f0eb64)
  - `t=1.00`: h=42,  s=88, v=94 (yellow-orange peak #f0d020)

## Effect Findings
- Reference behavior reads as soft heat bloom over key surfaces more than hard ring-only rendering
- Best match for readability and nostalgia: blob-like glow core + softer outer aura
- Light post-process noise and horizontal scanline texture improves "goggle/sensor" presentation
- Mild vignette helps focus and match low-light tactical framing

## Cross-Platform Rules
- Keep decay math exactly aligned: `exp(-(elapsed/decay_time)*3)`
- Keep visibility threshold aligned (`0.02`)
- Keep palette stop list identical across JS/Python/Rust
- Keep glow style aligned: layered bloom + softened key tint

## Cycle Notes

### Cycle 1
- Re-baselined palette toward deep blue/cyan/yellow/near-white
- Replaced ring-dominant look with layered bloom in JS/Python
- Kept decay timing at 30s

### Cycle 2
- Added subtle thermal sensor texture: noise + scanlines + vignette
- Adjusted button base/tint to improve hot-vs-cold readability
- Updated Rust simulator rendering style toward filled heat blobs and scanline overlay

### Cycle 3
- Consolidated cross-platform consistency on final Splinter palette stops
- Updated docs and comparison scoring
- Preserved optional alternate palettes without changing core Splinter mode behavior

### Cycle 4
- **Peak color correction**: Changed from near-white (#fffadc) to warm yellow-orange (#f0d020)
  - Source: Oracle consultation (session ses_31241dcb1ffeCz6VqXVTdYpHrA)
  - Reference: SC1 Xbox original peaks at yellow-orange, not white
- **HSV interpolation**: Replaced RGB linear interpolation with HSV-based
  - Perceptually smoother color transitions
  - Proper hue wraparound handling (blue→cyan→green→yellow→orange)
- **Gamma curve**: Added γ=1.3 for better perceptual separation between intensities
- **Precomputed LUT**: 256-entry lookup table for performance (especially Rust embedded)
- **Chromatic aberration** (JS only): RGB channel offset on hot button edges
  - Triggers at intensity > 0.4
  - Subtle lens distortion effect on high-heat areas
- **Film grain**: Noise now updates at ~10fps instead of 60fps
  - More authentic thermal camera feel
  - Reduces visual noise while maintaining texture

## Open Uncertainties
- Exact frame-accurate 2002 color values vary by platform/version/capture pipeline
- YouTube recompression and post-processing alter sampled colors
- Current values are evidence-backed approximations tuned for recognition fidelity
