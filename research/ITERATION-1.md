# ITERATION 1 — Interaction & Accessibility Foundation

Scope: all 8 demos in `javascript/*.html` (NOT docs/demos — synced after verification)
plus `docs/index.html`. No new external resources (CI loads demos via file:// with no
network). Keep `#randomBtn` id in thermal (CI clicks it).

## 1. Shared Sound module (identical API in every demo, per-theme config)

Paste near top of each demo's script. Per-theme TUNING table at the bottom.

```js
const Sound = (() => {
  const AC = window.AudioContext || window.webkitAudioContext;
  let ctx = null, master = null, noiseBuf = null;
  let enabled = (() => { try { const v = localStorage.getItem('pinpad:sound'); return v === null ? true : v === '1'; } catch (e) { return true; } })();
  function save() { try { localStorage.setItem('pinpad:sound', enabled ? '1' : '0'); } catch (e) {} }
  function unlock() {
    if (!AC) return;
    if (!ctx) {
      ctx = new AC();
      master = ctx.createGain(); master.gain.value = 0.5; master.connect(ctx.destination);
      const len = Math.floor(ctx.sampleRate * 0.05);
      noiseBuf = ctx.createBuffer(1, len, ctx.sampleRate);
      const d = noiseBuf.getChannelData(0);
      for (let i = 0; i < len; i++) d[i] = Math.random() * 2 - 1;
    }
    if (ctx.state === 'suspended') ctx.resume();
  }
  function tone(freq, dur, type, peak, slideTo) {
    if (!enabled || !ctx || ctx.state !== 'running') return;
    const t = ctx.currentTime, o = ctx.createOscillator(), g = ctx.createGain();
    o.type = type; o.frequency.setValueAtTime(freq, t);
    if (slideTo) o.frequency.exponentialRampToValueAtTime(slideTo, t + dur * 0.8);
    g.gain.setValueAtTime(0.0001, t);
    g.gain.exponentialRampToValueAtTime(peak, t + 0.004);
    g.gain.exponentialRampToValueAtTime(0.0001, t + dur);
    o.connect(g).connect(master); o.start(t); o.stop(t + dur + 0.02);
  }
  function thud(peak, cutoff) { // filtered noise burst = mechanical clunk
    if (!enabled || !ctx || ctx.state !== 'running' || !noiseBuf) return;
    const t = ctx.currentTime, s = ctx.createBufferSource(); s.buffer = noiseBuf;
    const f = ctx.createBiquadFilter(); f.type = 'lowpass'; f.frequency.value = cutoff;
    const g = ctx.createGain();
    g.gain.setValueAtTime(0.0001, t);
    g.gain.exponentialRampToValueAtTime(peak, t + 0.002);
    g.gain.exponentialRampToValueAtTime(0.0001, t + 0.06);
    s.connect(f).connect(g).connect(master); s.start(t); s.stop(t + 0.07);
  }
  return {
    unlock,
    key()    { TUNING.key(); },     // digit press
    action() { TUNING.action(); },  // submit/enter
    error()  { TUNING.error(); },
    success(){ TUNING.success(); },
    toggle() { enabled = !enabled; save(); return enabled; },
    get enabled() { return enabled; }
  };
})();
```

Per-theme TUNING (call `Sound.unlock()` in EVERY pointer/keyboard handler first):

| demo | key() | action() | error() | success() |
|---|---|---|---|---|
| thermal | tone(1180,0.045,'square',0.08) | tone(880,0.09,'square',0.10) | tone(160,0.28,'sawtooth',0.10) | tone(1319,0.12,'square',0.10) |
| sevastolink | tone(1900,0.04,'sine',0.10)+tone(2400,0.03,'sine',0.05) | tone(1568,0.10,'sine',0.10) | tone(196,0.2,'square',0.09) x2 (150ms apart) | tone(1047,0.09,'sine',0.10)+tone(1568,0.12) |
| mr-robot | tone(1000,0.045,'sine',0.07) | tone(1245,0.06,'sine',0.08) | tone(220,0.08,'square',0.08) x2 | tone(880,0.15,'sine',0.08) |
| nostromo | thud(0.22,520)+tone(880,0.05,'sine',0.05) | tone(660,0.10,'sine',0.09) | tone(180,0.30,'square',0.09) | tone(988,0.12,'sine',0.09) |
| wargames | tone(1050,0.09,'square',0.07) | tone(1050,0.09,'square',0.07); tone(1400,0.09) 120ms later | tone(240,0.18,'square',0.08) | tone(523,0.1,'square',0.08)+tone(659,0.14) |
| deusex | tone(520,0.06,'sine',0.09) | tone(660,0.10,'sine',0.10) | tone(200,0.22,'triangle',0.10) | tone(784,0.10,'sine',0.10)+tone(1047,0.14) |
| deadspace | tone(600,0.08,'sine',0.09,900) | tone(700,0.09,'sine',0.10,1000) | tone(320,0.20,'sawtooth',0.09,180) | tone(659,0.09,'sine',0.10)+tone(830,0.13) |
| tron | tone(740,0.08,'square',0.09) detune feel: tone(756,0.08,'square',0.05) same call | tone(988,0.10,'square',0.09) | tone(185,0.25,'sawtooth',0.09) | tone(1319,0.10,'square',0.08) |

Wire sound calls into existing press/submit/deny/grant code paths.

Mute toggle button appended to each demo's controls row:
`<button class="btn" id="soundBtn" type="button" aria-pressed="true">Sound: On</button>`
JS: toggles Sound, updates label + aria-pressed. Initialize label from Sound.enabled.

## 2. Keyboard hygiene
- In thermal + sevastolink `handleKeyDown`: call `e.preventDefault()` for every key the
  handler consumes (fixes Enter double-firing focused buttons).
- Guard `if (e.ctrlKey || e.metaKey || e.altKey) return;` at top of key handlers.

## 3. Page Visibility pause
```js
document.addEventListener('visibilitychange', () => {
  if (document.hidden) { /* cancel rAF via stored id, clearInterval demo timers */ }
  else { /* restart loop, reset lastT, resume demo timers if they were active */ }
});
```
Cancel paths required: thermal/sevastolink/mr-robot rAF (use the stored animationId),
demo-mode setIntervals (all demos with demo mode), deadspace raw rAF loop, wargames
cursor blink chain. CSS animations pause automatically in background tabs — no action.

## 4. prefers-reduced-motion
JS: `const REDUCED = matchMedia('(prefers-reduced-motion: reduce)');`
Gate ONLY decorative motion: flicker/jitter amplitude, scanline drift, glitch offsets,
boot glitch, random jitter. KEEP functional updates (heat decay, typed text, cursor
blink slowed to 1.2s is fine to keep).
CSS append per demo:
```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after { animation-duration: 0.01ms !important; animation-iteration-count: 1 !important; transition-duration: 0.01ms !important; }
}
```

## 5. DPR-aware canvas (thermal, sevastolink, mr-robot ONLY)
Canvas currently: fixed attribute size (320x400 etc.), CSS none. Add:
```js
function fitCanvas() {
  const dpr = Math.min(window.devicePixelRatio || 1, 2);
  const w = canvas.width / currentScaleIfAny, h = canvas.height; // use existing logical size
  canvas.style.width = w + 'px'; canvas.style.height = h + 'px';
  // store logical size once at init; then:
  canvas.width = Math.round(w * dpr); canvas.height = Math.round(h * dpr);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
}
```
All existing draw code uses logical (CSS px) coordinates — unchanged. Click math
(getBoundingClientRect + clientX/Y) unchanged. Call once at init.

## 6. A11y
Canvas demos: wrap canvas in
`<div role="application" tabindex="0" aria-label="<THEME> keypad. Type digits 0-9, Backspace to delete, Enter to submit." aria-roledescription="keypad">`
+ `<p id="a11yStatus" role="status" class="visually-hidden"></p>` updated on state
changes (digits entered, granted, denied). Add :focus-visible outline CSS.
DOM demos: ensure status elements have aria-live="polite" (some have it already);
add visually-hidden announcements if missing.

## 7. Frozen-frame liveliness (CI screenshots + vision judges see 2.5s-old frames)
- thermal: fire one demo code entry ~800ms after load (before the 8s interval starts).
- sevastolink: fire one entry ~600ms after load.
- mr-robot: type one terminal command ~600ms after load (if demo mode not user-stopped).
- nostromo: first demo digit ~700ms after load (interval 2.2s stays).
- wargames: after boot text completes, auto-type `SHALL WE PLAY A GAME?` then leave
  cursor blinking at prompt.
- deusex/deadspace/tron: verify a meaningful state is visible at t=2.5s (boot done,
  something entered or status displayed); if boot is longer, speed it to <2s.

## 8. Keyboard hint
One short line under each demo's controls, styled like existing `.subtitle`:
`Keyboard: 0-9 · Backspace clear · Enter submit · D demo · S sound`
(adapt to actual keys supported by that demo).

## 9. Landing page (docs/index.html) — separate checklist
See research/ITERATION-1-LANDING.md.
