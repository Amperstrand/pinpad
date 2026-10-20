# ITERATION 1 — Landing page checklist (docs/index.html)

Constraints: static file, must work file:// and https://amperstrand.github.io/pinpad/.
Inline CSS stays inline. No frameworks.

## Head block (replace/augment existing head)
```html
<title>Pinpad — Movie & Game Keypad Effects</title>
<meta name="description" content="...(keep existing, fix: says 8, names 7 — add Alien: Isolation Sevastolink)">
<link rel="canonical" href="https://amperstrand.github.io/pinpad/">
<meta name="theme-color" content="#030810">
<link rel="icon" href="favicon.svg" type="image/svg+xml">
<link rel="icon" href="favicon-32.png" sizes="32x32" type="image/png">
<link rel="apple-touch-icon" href="icon-192.png">
<meta property="og:type" content="website">
<meta property="og:site_name" content="Pinpad">
<meta property="og:title" content="Pinpad — Movie & Game Keypad Effects">
<meta property="og:description" content="8 iconic keypad interfaces from movies and games, in JavaScript, Python and Rust.">
<meta property="og:url" content="https://amperstrand.github.io/pinpad/">
<meta property="og:image" content="https://amperstrand.github.io/pinpad/og-image.png">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta property="og:image:alt" content="Gallery of 8 retro keypad interfaces — thermal, CRT terminals and holograms">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="Pinpad — Movie & Game Keypad Effects">
<meta name="twitter:description" content="Splinter Cell thermal vision, Alien Sevastolink, WOPR, Sarif, RIG, Tron — playable in-browser.">
<meta name="twitter:image" content="https://amperstrand.github.io/pinpad/og-image.png">
```

## Images (8 card PNGs; 5 are 1462x879, 3 are 400x500)
- Add intrinsic `width`/`height` attributes matching actual file dims (kills CLS).
- `loading="lazy" decoding="async"` on all EXCEPT the first card's img (LCP).
  First card img gets `fetchpriority="high"` (no lazy).
- Keep existing onerror fallback.

## A11y
- Skip link as first focusable element in body:
  `<a class="skip-link" href="#main">Skip to content</a>` + CSS (hidden until focus)
  + `id="main" tabindex="-1"` on `<main>`.
- `:focus-visible { outline: 2px solid var(--text-cyan); outline-offset: 3px; }`
- Reduced motion global gate (@media block, 0.01ms pattern).
- Unique accessible names per card CTA:
  `<a ... class="demo-btn"><span aria-hidden="true">▶</span> Try Demo<span class="sr-only">: Thermal Vision Pinpad</span></a>`
  + `.sr-only` CSS. 8 unique names.
- `.demo-btn { min-height: 44px; }` touch target.
- `aria-hidden="true"` on placeholder emojis + footer/hero decorative glyphs.
- Footer external links: add `rel="noopener noreferrer"` (target=_blank ones lacking it).
- GitHub SVG: `aria-hidden="true"` (link already has text).

## Perf
- Below-fold sections: `content-visibility: auto; contain-intrinsic-size: auto 600px;`
  on the Cross-Platform section + github-section + footer (NOT the gallery).
- Copy accuracy: "Pure HTML5 Canvas with 60fps animation" → "Canvas & CSS animations, zero dependencies" (5 of 8 are DOM/CSS).
- Fix meta description 8-vs-7 drift (add Alien: Isolation).

## Assets to create (I will generate after your edit): favicon.svg, favicon-32.png,
icon-192.png, icon-512.png, og-image.png — reference them as above; do NOT create them.
