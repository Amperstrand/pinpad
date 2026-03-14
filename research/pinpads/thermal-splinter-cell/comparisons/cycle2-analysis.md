# Cycle 2 Comparison Results

## Refinements Applied
- Added subtle scanline overlay
- Added sparse thermal noise/grain
- Added vignette edge darkening
- Updated Rust simulator to filled thermal blobs and scanline pass

## Improvement from Cycle 1
| Aspect | Cycle 1 | Cycle 2 | Improved |
|---|---:|---:|---|
| Color accuracy | 6.5/10 | 8/10 | ✓ |
| Glow resemblance | 6/10 | 8/10 | ✓ |
| Sensor feel | 4/10 | 7.5/10 | ✓ |
| Cross-platform consistency | 7/10 | 8.5/10 | ✓ |

## Remaining Discrepancies
1. Rust still slightly harsher in small-radius highlights
2. Python/Tk canvas blending differs from canvas and simulator renderers

## Learnings for Cycle 3
- Keep shared palette and decay fixed
- Fine-tune only blend intensity and presentation details
- Update docs to reflect evidence-backed approximations rather than exact capture parity
