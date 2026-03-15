# Pipeline Analysis & Optimization Guide

> Lessons learned from implementing 8 keypads (JS/Python/Rust)

---

## What Worked Well

### 1. Spec Structure

**The winning spec sections (in order of implementation usefulness):**

| Section | Usefulness | Why |
|---------|------------|-----|
| **Color Palette Table** | ⭐⭐⭐⭐⭐ | Exact hex values → direct code translation |
| **Timing/Animation Values** | ⭐⭐⭐⭐⭐ | Millisecond precision → no guessing |
| **Behavior Tables** | ⭐⭐⭐⭐ | State machines → clear implementation path |
| **CRT Effects Specs** | ⭐⭐⭐⭐ | Scanline spacing, opacity → reproducible |
| **Button State Tables** | ⭐⭐⭐⭐ | Default/Pressed/Active → visual consistency |
| **Typography Specs** | ⭐⭐⭐ | Font name + fallback chain |
| **Reference URLs** | ⭐⭐⭐ | Source verification |
| **Sound Design** | ⭐⭐ | Optional, often skipped |
| **Scene Context** | ⭐⭐ | Interesting but not code-relevant |

**Key insight**: Specs with precise numeric values (hex colors, ms timing, px dimensions) translated directly to code. Prose descriptions required interpretation.

### 2. Visual Effect Patterns That Worked

| Effect | Implementation | Cross-Platform Parity |
|--------|---------------|----------------------|
| **Phosphor glow (text-shadow)** | CSS shadowBlur | ✅ Easy |
| **Scan lines (gradient overlay)** | repeating-linear-gradient | ✅ Easy |
| **Ring-based thermal glow** | Concentric circles with falloff | ✅ Easy |
| **Chromatic aberration (edges)** | RGB channel offset | ⚠️ JS only |
| **Film grain (noise texture)** | Cached at 10fps | ✅ Medium |
| **CRT curvature** | Barrel distortion shader | ⚠️ JS heavy |
| **Screen flicker** | Random brightness dip | ✅ Easy |

**Key insight**: Simple 2D effects (glow, scanlines, flicker) achieved cross-platform parity. Complex shaders (curvature, chromatic aberration) stayed JS-only.

### 3. Code Structure Patterns

**JavaScript (winning pattern):**
```javascript
// Config object at top (matches spec)
const CONFIG = {
    colors: { /* from spec */ },
    timing: { /* from spec */ },
    effects: { /* from spec */ }
};

// Class-based state management
class Keypad {
    constructor(config = {}) { /* merge with defaults */ }
    handleInput(key) { /* state machine */ }
    render(ctx) { /* draw loop */ }
}

// Single animation loop
function gameLoop(timestamp) {
    // Clear, render, requestAnimationFrame
}
```

**Rust (winning pattern):**
```rust
// Config struct with Builder
pub struct Config {
    pub colors: Colors,
    pub timing: Timing,
}

impl Config {
    pub fn builder() -> ConfigBuilder { ... }
}

// Tests for timing/behavior (not visual)
#[test]
fn test_decay_formula() { ... }
```

**Python (winning pattern):**
```python
# Dataclass config
@dataclass
class Config:
    colors: dict = field(default_factory=lambda: {...})
    timing: dict = field(default_factory=lambda: {...})

# Class with render method
class Keypad:
    def __init__(self, config: Config = None): ...
    def render(self, surface: pygame.Surface): ...
```

### 4. Iteration Cycles (Thermal Keypad)

The 4-cycle approach produced measurable improvement:

| Cycle | Color Accuracy | Visual Effect | Cross-Platform |
|-------|---------------|---------------|----------------|
| 1 | 6.5/10 | 6.0/10 | 7.0/10 |
| 2 | 8.0/10 | 8.0/10 | 8.5/10 |
| 3 | 9.0/10 | 9.0/10 | 9.0/10 |
| 4 | 9.5/10 | 9.5/10 | 9.0/10 |

**Key insight**: Explicit discrepancy tracking per cycle prevented regression and documented progress.

---

## What Didn't Work Well

### 1. Inconsistent Spec Depth

| Keypad | Spec Lines | Quality | Issue |
|--------|------------|---------|-------|
| Mr. Robot | 482 | ⭐⭐⭐⭐⭐ | Excellent - exact timing, colors, commands |
| Sevastolink | 484 | ⭐⭐⭐⭐⭐ | Excellent - complete CRT specs |
| Nostromo | 197 | ⭐⭐⭐ | Good - missing some timing values |
| WarGames | 255 | ⭐⭐⭐ | Good - terminal not keypad, needed adaptation |
| Deus Ex HR | 233 | ⭐⭐⭐ | Good - missing exact glow formula |
| Dead Space | 219 | ⭐⭐⭐ | Good - inventory grid not traditional keypad |
| Tron | 219 | ⭐⭐⭐ | Good - hex button implementation unclear |
| Thermal | 92 | ⭐⭐⭐⭐ | Verified spec + separate visual-analysis.md |

**Key insight**: Specs created early (Mr. Robot, Sevastolink) had more detail. Later specs got compressed. Need template enforcement.

### 2. Missing Values in Specs

Implementers had to guess these:

| Missing Value | Frequency | Example |
|---------------|-----------|---------|
| Exact glow radius | 5/8 | "soft bloom" → what px radius? |
| Animation easing | 6/8 | Linear? Ease-out? Exponential? |
| Font fallback chain | 4/8 | Only primary font specified |
| Error state duration | 3/8 | How long does "ACCESS DENIED" show? |
| Sound timing | 7/8 | Beep duration rarely specified |

### 3. Test Coverage Inconsistency

| Rust Module | Tests | Coverage Type |
|-------------|-------|---------------|
| thermal.rs | 13 | ✅ Timing, decay, colors |
| mr_robot.rs | 16 | ✅ Timing, typing, demo |
| sevastolink.rs | 14 | ✅ Auth flow, timing |
| tron.rs | 6 | ⚠️ Basic only |
| deusex.rs | 5 | ⚠️ Basic only |
| nostromo.rs | 0 | ❌ No tests |
| deadspace.rs | 0 | ❌ No tests |
| wargames.rs | 0 | ❌ No tests |

**Key insight**: First implementations (thermal, mr_robot, sevastolink) got thorough tests. Later implementations skimped.

### 4. Cross-Platform Drift

Effects that didn't achieve parity:

| Effect | JS | Python | Rust | Issue |
|--------|----|--------|------|-------|
| Chromatic aberration | ✅ | ❌ | ❌ | Shader-heavy, skipped |
| CRT curvature | ✅ | ❌ | ❌ | GLSL-only, skipped |
| Film grain | ✅ | ✅ | ⚠️ | Rust uses simpler noise |
| Vignette | ✅ | ✅ | ❌ | Radial gradient expensive |

---

## Optimized Pipeline

### Phase 1: Research Spec Template

Every spec MUST include these sections with precise values:

```markdown
## Color Palette (REQUIRED)
| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| Primary | #00FF41 | (0, 255, 65) | Main text |

## Timing Values (REQUIRED)
| Event | Duration (ms) | Easing |
|-------|---------------|--------|
| Keypress flash | 100 | ease-out |
| Cursor blink | 530 | step |

## Effect Parameters (REQUIRED)
| Effect | Value | Notes |
|--------|-------|-------|
| Glow radius | 15px | From button edge |
| Scanline spacing | 2px | Horizontal |
| Scanline opacity | 0.15 | rgba(0,0,0,0.15) |

## Button States (REQUIRED)
| State | Background | Border | Text |
|-------|------------|--------|------|
| Default | #0c290c | #134213 | #05b669 |
| Pressed | #134213 | #05b669 | #ccd5d4 |

## Font Stack (REQUIRED)
Primary: 'FontName', 'Fallback1', 'Fallback2', monospace
Size: 16px
Line-height: 1.4
```

### Phase 2: Implementation Checklist

Before marking a keypad complete:

```markdown
## JS Implementation
- [ ] Config object matches spec exactly
- [ ] All colors from spec present
- [ ] All timing values from spec used
- [ ] Demo mode button functional
- [ ] Keyboard support (0-9, C, E)
- [ ] Screenshot captured

## Python Implementation
- [ ] Same config as JS
- [ ] Tkinter demo works
- [ ] Visual parity with JS (checked)

## Rust Implementation
- [ ] Config struct with Builder
- [ ] Minimum 5 tests (timing, state, colors)
- [ ] Simulator runs
- [ ] Visual parity with JS (checked)

## Cross-Platform Verification
- [ ] Same color palette in all 3
- [ ] Same timing values in all 3
- [ ] Same visual effects (where supported)
```

### Phase 3: Quality Gates

**Before commit:**
1. `cargo test` passes (all modules)
2. Python syntax check (`python -m py_compile *.py`)
3. JS LSP diagnostics clean
4. Visual comparison (screenshot vs reference)

**Before marking complete:**
1. All 3 platforms implemented
2. At least 5 Rust tests
3. Spec status updated to [x]
4. Docs screenshot captured

---

## Gas Town Integration

### What Worked

1. **Parallel librarian agents** - 5 specs researched simultaneously
2. **Parallel implementation agents** - 5 keypads implemented simultaneously  
3. **Improvement cycle agents** - 8 agents polished all keypads in parallel
4. **Session continuity** - Using `session_id` for follow-up fixes

### What Could Improve

1. **Prompt specificity** - Earlier prompts were vague ("make it look good")
2. **Verification step** - No automated visual verification
3. **Test requirement** - Tests weren't mandatory for completion
4. **Spec template** - No enforced structure for research agents

### Recommended Delegation Pattern

```
1. Research Phase:
   - Fire 5 librarian agents in parallel (background)
   - Each produces spec.md with REQUIRED sections
   - Mayor reviews for completeness before proceeding

2. Implementation Phase:
   - Fire 5 deep agents in parallel (background)
   - Each implements JS + Python + Rust
   - Mandatory: 5+ Rust tests per keypad
   - Mandatory: Screenshot for visual verification

3. Verification Phase:
   - Run cargo test (all must pass)
   - Visual check: JS screenshot vs reference image
   - Mayor approves or sends back for fixes

4. Polish Phase:
   - Fire improvement agents for each keypad
   - Focus on: timing precision, glow accuracy, effect consistency
   - Re-verify tests and visuals
```

---

## Prompt Templates

### Research Agent Prompt

```
[CONTEXT]: Creating a keypad implementation for [SOURCE] ([YEAR])
[GOAL]: Produce a spec.md with precise values for implementation
[DOWNSTREAM]: Implementers will translate this directly to code

[REQUEST]:
1. Research [SOURCE] keypad/terminal interface
2. Find reference images, videos, color extractions
3. Extract EXACT values:
   - Color palette (hex + RGB)
   - Timing (ms with easing)
   - Effect parameters (px, opacity, radius)
   - Font names with fallback chain
4. Return spec.md with ALL REQUIRED sections filled

Sources to check:
- Official game/movie stills
- Fan wikis (color palettes, screenshots)
- YouTube clips (for timing measurement)
- GitHub themes (color extraction)
```

### Implementation Agent Prompt

```
[CONTEXT]: Implementing [SOURCE] keypad from spec at [PATH/spec.md]
[GOAL]: Create JS/Python/Rust implementations with visual parity
[DOWNSTREAM]: Will be verified against spec and screenshot

[TASK]: Create [SOURCE]-pinpad.html, [source]_pinpad.py, [source].rs

[MUST DO]:
1. Read spec.md FIRST - all values must match
2. Copy config structure from existing keypads
3. Implement all 3 platforms with same config
4. Add minimum 5 Rust tests (timing, state, colors)
5. Test keyboard support (0-9, C, E)
6. Add demo mode button

[MUST NOT DO]:
1. Guess values - use spec or ask
2. Skip tests
3. Implement effects not in spec
4. Use external dependencies (JS must be self-contained)
```

---

*Generated: 2026-03-15*
*Based on analysis of 8 implemented keypads*
