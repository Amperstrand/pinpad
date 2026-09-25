# RESEARCH & IMPROVE — Pinpad

> Reusable agentic prompt. Point an agent (with z.ai zread + web search + vision
> access) at this file alongside the repo. Repeat one full pass per iteration.

## Goal

Continuously improve https://amperstrand.github.io/pinpad/ — visual fidelity,
interaction quality, accessibility, performance, and code quality — using
research-first iteration. Never change a demo without a researched reason.

## Tools you MUST use each iteration

1. **z.ai zread** (`zread_get_repo_structure`, `zread_read_file`, `zread_search_doc`)
   - Mine high-quality OSS for concrete patterns we can adopt:
     CRT/retro-terminal CSS+canvas implementations, web-audio UI feedback,
     game-style UI HUDs, showcase/landing pages of creative-coding projects.
   - Read actual source files, not summaries. Steal patterns, cite the repo.
2. **Web search**
   - Current best practices for: canvas rendering (DPR scaling, rAF + Page
     Visibility), Web Audio API UI tones, `prefers-reduced-motion`, WCAG 2.2
     for interactive canvas widgets, PWA on GitHub Pages, SEO/OG tags,
     lazy-loading, modern CSS (2025+ baseline).
   - Prefer primary sources: MDN, web.dev, WCAG spec, library docs.
3. **z.ai vision** (screenshot analysis)
   - After each iteration, screenshot every demo + landing page with Playwright.
   - Score each surface 0–10 on: visual fidelity to source material, layout &
     spacing, typographic hierarchy, color/contrast, perceived polish.
   - Record scores in `research/VISION-SCORES.md` with per-surface notes and
     the specific fixes queued for the next iteration.

## Non-negotiable constraints

- Demos stay dependency-free single HTML files (open-in-browser portability is
  a core feature). No build step for `javascript/*.html` or `docs/demos/*`.
- `docs/` mirrors deployed Pages content; keep it in sync with `javascript/`.
- CI (`.github/workflows/screenshots.yml`) must stay green: `cargo test` in
  `rust/`, `py_compile` on `python/*.py`, screenshot regeneration.
- Every iteration ends with: verified screenshots → vision scores logged →
  conventional commit → push to `main` (Pages auto-deploys) → live URL check.

## Iteration protocol

1. RESEARCH: one zread mining pass + one web-search best-practice pass.
   Output: a ranked fix list in `research/ITERATION-N.md`.
2. IMPLEMENT: apply the ranked fixes (delegate parallel work where possible).
3. VERIFY: `npx playwright screenshot` (or equivalent) on every surface;
   check console errors; check mobile viewport (390px) and desktop (1280px).
4. SCORE: z.ai vision on every screenshot; append to `research/VISION-SCORES.md`.
5. SHIP: sync `javascript/` → `docs/demos/`, commit, push, confirm CI + live.

## Standing improvement backlog (seed list)

- Keyboard input (digits, Backspace, Enter) on every demo + visible hint.
- Web Audio keypress tones with per-theme character + mute toggle persisted
  in `localStorage`; respect autoplay policies (init on first gesture).
- `prefers-reduced-motion`: pause/gate decorative animation loops.
- Pause rAF loops when tab hidden (Page Visibility API).
- DPR-aware canvas sizing for crisp rendering on HiDPI.
- A11y: canvas `role="application"` + `aria-label`, `aria-live` status,
  focusable controls, visible focus rings, contrast-checked palettes.
- Landing page: favicon + `<meta theme-color>` + Open Graph/Twitter cards,
  lazy-loaded previews with correct `width`/`height`, `content-visibility`,
  keyboard-skip link, footer year, per-demo "view source" links.
- PWA: `manifest.webmanifest` + tiny offline-caching service worker.
- README: drop stale "ARCHIVED" banner once revived; document new features.
