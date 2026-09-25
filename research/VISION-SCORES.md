# Vision Scores — z.ai vision iteration log

Rubric per surface: 5 criteria x 10 → OVERALL /10.
Screenshots: Playwright, 1280x800 @2x, 2.5s after load (desktop), 390x844 (mobile index).

## BASELINE (pre-iteration-1) — 2026-09-25

| Surface | Overall | Top defects (vision judge) |
|---|---|---|
| index (landing) | 4.5 | card preview PNG reads as black void; no theme captions/nav/footer; 4 competing accent hues; hero eats 45% viewport; content clipped at fold |
| index mobile | — | (captured, folded into landing fixes) |
| thermal | 3.0 | no visible ironbow ramp in static frame; no bloom/halo; no sensor fiction (scanlines/noise/vignette); dead zones |
| sevastolink | 4.5 | keys unlabeled; scanlines applied to physical keys; no curvature/bloom; white text breaks phosphor purity |
| mr-robot | 5.5 | cropped artifact at keypad top; no blinking cursor / typed proof; panel misalignment; diegesis-breaking caption |
| nostromo | 3.0 | glossy blue Vista-style buttons; zero amber/CRT; reads 2000s web not 1979 industrial |
| wargames | 4.0 | green not amber (vs spec); no barrel curvature; "TRAININ" mid-word truncation; cursor orphaned from prompt; no WOPR hook line |
| deusex | 6.0 | rounded rects (DXHR = sharp chamfers); cyan off-palette; flat single-plane layering |
| deadspace | 5.5 | opaque flat panels (not holo); no chromatic aberration/scan sweep; generic gradient health bar; 2-letter codes meaningless |
| tron | 6.5 | key glyphs drowned by own glow; dead circuit trace; no press feedback states |

**Average: 4.72/10**

### Cross-cutting findings

1. **Frozen-frame liveliness**: judges & CI screenshots see a static frame. Demos
   must show visible heat/glow/entered-digits within ~2s of load (demo mode
   should act immediately, not wait 8s).
2. **Landing page previews**: CI-generated PNGs catch empty/dark frames →
   cards look broken. Fix generation timing + add CSS-drawn mini previews as
   fallback.
3. **Diegesis**: modern web chrome (gradient buttons, rounded cards) inside
   themed demos is the #1 fidelity killer across thermal/nostromo/wargames.
