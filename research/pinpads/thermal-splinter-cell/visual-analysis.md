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

## Verified Gameplay Behavior
- Heat marks are a short-lived interaction aid for keypad reconstruction
- Brightest mark indicates most recent keypress
- Faintest mark indicates earliest keypress
- The effect is stylized game thermal imaging, not scientific camera output

## Color Findings (Reference-Constrained)
- Confirmed palette ordering from references: dark cool background -> cyan/green mids -> yellow/highlights
- Red appears in generic thermal descriptions, but keypad interactions in this sequence read cleaner in blue/cyan/green/yellow with near-white peaks
- Final implementation palette for Splinter Cell mode:
  - `#050820` cool base
  - `#0c2a6e` cold-to-mid ramp
  - `#1478aa` active warm-up
  - `#52be84` warm key footprint
  - `#dcda5a` hot
  - `#ffeb8c` very hot
  - `#fffadc` peak

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

## Open Uncertainties
- Exact frame-accurate 2002 color values vary by platform/version/capture pipeline
- YouTube recompression and post-processing alter sampled colors
- Current values are evidence-backed approximations tuned for recognition fidelity
