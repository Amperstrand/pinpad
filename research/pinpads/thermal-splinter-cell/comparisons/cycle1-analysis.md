# Cycle 1 Comparison Results

## Inputs
- Reference set: mission/wiki/walkthrough/novel evidence
- Implementations compared: JS, Python, Rust simulator

## Color Accuracy
| Aspect | Reference Direction | Cycle 1 Output | Match |
|---|---|---|---|
| Cool background | deep blue-black | corrected to deep blue-black | ✓ |
| Old heat | blue/cyan | cyan-blue | ✓ |
| Recent heat | yellow to near-white | yellow-white peak | ✓ |
| Over-warm orange bias | low | reduced | ✓ |

## Visual Effect Accuracy
| Effect | Reference Signal | Cycle 1 | Match |
|---|---|---|---|
| Soft heat bloom | present | added layered bloom | ✓ |
| Hard concentric ring dominance | weak | reduced | ✓ |
| Readability ordering | required | maintained | ✓ |

## Scores
- Color accuracy: 6.5/10
- Visual effect accuracy: 6/10
- Cross-platform consistency: 7/10
- Overall resemblance: 6/10

## Key Discrepancies
1. Sensor texture still too clean compared to legacy thermal display feel
2. Rust simulator still looked more geometric than JS/Python
3. Contrast at edges needed more tactical framing

## Learnings for Cycle 2
- Add light scanline/noise/vignette overlay
- Push Rust simulator away from stroke-only rings
- Keep palette locked while improving texture layers
