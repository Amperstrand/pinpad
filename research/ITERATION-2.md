# ITERATION 2 — Visual Fidelity Upgrades

Driven by: baseline z.ai vision scores (research/VISION-SCORES.md) + CRT/holographic
technique research (scanline params, phosphor persistence, bloom layering, ironbow
stops, chamfered DXHR geometry). All changes canvas-2D/CSS only, no WebGL, no new
network dependencies (system font stacks only).

## thermal (3.0 → target 6.5+)
1. Frozen-frame heat: raise ring-glow alpha caps + draw a faint permanent "residual
   heat" hint so any frame ≥2s shows color ramp (ironbow feel). Ensure auto-entry
   at ~800ms leaves 3-4 keys visibly hot at 2.2s.
2. Sensor fiction: pre-rendered fixed-pattern noise overlay (one ImageData, cached,
   alpha 0.04-0.06) + radial vignette on canvas edges; light blue-black ambient.
3. Two-tap bloom: after scene draw, ctx.filter blur(4px) alpha 0.35 self-draw +
   blur(12px) alpha 0.18 (composite 'lighter').
4. Status/readout: small "TEMP" column or heat legend bar right of keypad (ironbow
   gradient strip) — fills dead zone, sells thermal fiction.

## sevastolink (4.5 → target 7+)
1. Keys must read as physical backlit buttons: digit labels bright green, key
   surface slightly 3D (top-light bevel line), NO scanlines on keys (scanlines on
   display area only).
2. Phosphor purity: all display text in green shades (no white/gray) + text glow
   (shadowBlur 6-10 on display text).
3. Chunky bezel: thicker panel border, corner screws (4 small circles), "SEEGSON"
   brand plate text.
4. Subtle CRT curvature on display: border-radius on display rect + corner vignette.

## mr-robot (5.5 → target 7+)
1. Kill cropped artifact at keypad top (misaligned element) — align keypad panel
   width to terminal width exactly.
2. Prompt line: bright fsociety prompt + dim [*] lines + live block cursor on the
   active line.
3. Remove meta caption under device (diegesis).

## nostromo (3.0 → target 6.5+) — BIGGEST gap
1. Kill glossy blue Vista buttons: matte charcoal-olive keycaps (flat fill, 1px
   top highlight, dark bottom edge — no gradients/shine), amber legend text.
2. Display: amber phosphor (#FFB000) on near-black screen, scanline overlay on the
   display rect only, soft glow on display text, blinking READY indicator.
3. Industrial chrome: corner screws on panel, worn metal edge (2px darker inset
   border), "ACCESS WAY A/03" style label plate, subtle grime (existing radial
   grime kept).
4. Warm panel: shift panel background from white/beige to dark olive-brown metal.

## wargames (4.0 → target 6.5+)
1. AMBER palette per spec: phosphor #FFB000/#FFAA00 everywhere (text, cursor,
   scanlines tint), not green.
2. CRT curvature: screen container border-radius ~26px + strong corner vignette
   (radial-gradient transparent 52% → rgba(0,0,0,0.55) 84% → 0.9 100%) + subtle
   perspective(1000px) rotateX(1.5deg).
3. Fix truncation bug ("TRAININ") — typing buffer/width math.
4. Cursor sits at prompt line (NORAD> ▮), blinking; boot text completes fast.

## deusex (6.0 → target 7.5+)
1. Chamfered geometry: clip-path polygon corners (8-10px 45° cuts) on panel +
   keys; concentric double frame (outer hairline + inner panel).
2. Palette discipline: kill cyan accent → gold #D4AF37 family only; header corner
   brackets + small diamond motif.
3. Layering: 1px hairline inner strokes, translucent display sub-panel over
   chassis, faint diagonal texture on panel.
4. Key press state: amber fill flash (CSS transition) — pair with Sound.key().

## deadspace (5.5 → target 7+)
1. Holo opacity: panels to 0.5-0.6 + backdrop-filter blur(2px) (keep text ≥0.85).
2. Chromatic aberration visible: strengthen CSS channel-offset box-shadows on
   panel edges + add ::before/::after RGB-split text-shadow on the header title.
3. Segmented health/stasis meters (discrete ticks, color-by-value) replacing
   gradient bar.
4. Populate left column: objective detail lines; slot two-letter codes get a
   legend line.

## tron (6.5 → target 7.5+)
1. Key glyphs: near-white core (#EAFEFF) with tight cyan bloom — glyph luminance
   always above key fill.
2. Live circuit trace: CSS animation — traveling light pulse along the trace
   (background-position or box-shadow keyframes); on keypress fire one pulse.
3. Display echo: entered digits with blinking cursor block on the display line.

## Landing page (4.5 → target 7+)
1. Card previews: after CI regenerates 8 screenshots (live-state frames), verify
   cards show glowing/active pads; placeholder icons stay as fallback.
2. Palette discipline: intro paragraph → neutral gray; cyan = single accent;
   orange reserved for CTAs only.
3. Hero compression: title block ~25-30% viewport; one-sentence tagline; chip-row
   index of 8 themes above the grid (anchor links).
4. Footer: year, license, keyboard hint ("every demo works with your keyboard").

## Verification gate
- Playwright screenshots (desktop + 390px) → z.ai vision re-score all 9 surfaces.
- Target: average ≥6.5 (from 4.72). Any surface <5.0 gets an inline follow-up fix.
