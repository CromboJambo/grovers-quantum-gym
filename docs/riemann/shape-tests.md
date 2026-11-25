# Riemann Hypothesis Station — Shape Tests

Computational exploration of the geometric structure of |ζ(1/2 + it)| using
the 100-foot perpendicular drill protocol. Each test is a different drilling
direction into the same hill (the zeta magnitude trajectory), revealing
structure that other directions miss.

## The Hill Drilling Analogy

The Riemann-Siegel theta function produces a trajectory of peaks and troughs
in |ζ(1/2 + it)| as t increases. We treat this as a hill:

- **Values** are measurements (drill depths)
- **Types** are constraints on what shapes are possible
- **Functions** are transformations between representations
- **Proofs** are evidence that independent drilling directions agree

The 100-foot drill constraint: we can only measure perpendicular to the local
grade, with a fixed tool length (half_window=5 in zero-space). This prevents
cherry-picking easy near-peak points and forces connected walks through the
entire sequence.

**Key principle:** Boreholes don't need to physically intersect — they connect
through logical consistency. If all drilling directions describe the same
underlying object from different angles, the map is valid.

## Running the Tests

```bash
cargo run --example debug_multiscale
```

Runs all 11 shape tests with t_max=200. Individual tests can be called:

```rust
use shor_grover_exploration::riemann::zeta_riemann_siegel::*;
let report = shape_test_gradient_zero_finding(200.0);
```

## The 11 Shape Tests

### SHAPE TEST 1 — Return Map (R_n vs R_{n+1})

Plots each residual against the next one. Looks for attractors or repellers
in the trajectory dynamics. If points cluster along a curve, the system has
memory; if they scatter randomly, it's chaotic.

**What it drills:** Vertical (local dynamics at fixed depth).

### SHAPE TEST 2 — Log-Polar (ln N_smooth vs R)

Transforms to log-polar coordinates to reveal spiral or radial structure in
the trajectory. Tests whether the hill has rotational symmetry when viewed
from above.

**What it drills:** Diagonal (combines depth and magnitude).

### SHAPE TEST 3 — Folded Phase (N_smooth vs R mod 1)

Folds the residual modulo 1 to look for periodic structure that might be
hidden by the overall trend. If the folded points form bands, there's hidden
periodicity in how the hill rises and falls.

**What it drills:** Horizontal (magnitude only, depth-agnostic).

### SHAPE TEST 4 — Residual Acceleration (ΔR_n vs ΔR_{n+1})

Plots consecutive changes in residual against each other. Looks for
acceleration patterns — does the hill speed up or slow down as it approaches
peaks? Tests whether the dynamics are consistent across scales.

**What it drills:** Vertical (local acceleration).

### SHAPE TEST 5 — Peak Width Analysis (Self-Similarity Test)

Measures the width of each peak at half its height and compares to the peak
height. Tests whether W/H is constant (self-similar hill) or varies with
depth. This was the first test to establish that the system is NOT one fixed
triangle but a family of triangles evolving with depth.

**Key finding:** W/H ≈ 7.8 for early peaks (wide, shallow), W/H ≈ 2.5 for
late peaks (narrower, proportional). The geometric constraint itself evolves.

**What it drills:** Horizontal (width vs height relationship).

### SHAPE TEST 6 — Stepwise Counter (Steps Since Trough vs Height)

Counts steps since the last trough and correlates with current height. Tests
whether there's a predictable "ramp" structure to the hill — do peaks always
rise over the same number of steps? Looks for integer-quantized structure in
the ascent.

**What it drills:** Diagonal (temporal position vs magnitude).

### SHAPE TEST 7 — Pythagorean Constraint Test

Tests whether W² + H² correlates with depth (N_smooth) in a way that suggests
a geometric constraint. If W² + H² ≈ c·H² for constant c, then W/H = √c and
the system is constrained to a specific shape family.

**Key finding:** Corr(W²+H², H²) = 0.98 — the Pythagorean relationship holds
with near-perfect correlation. The system behaves like it's bounded by a
Pythagorean constraint where width and height are legs of a right triangle.

**What it drills:** Diagonal (relates two horizontal dimensions to depth).

### SHAPE TEST 8 — Depth-Dependent Constraint (Rigorous)

The rigorous version of the W/H analysis. Fits Model A (W/H constant) vs
Model B (W/H depends on N_smooth) and computes R² for each. Also measures
variance collapse across early/middle/late depth partitions.

**Key findings:**
- Model B wins: slope = -0.70, 67% of W/H variance explained by depth alone
- Variance collapse: early std=2.29, middle std=2.42, late std=0.94
- Detrended Corr(W²+H², N_smooth) = 0.72 (not a shared-trend artifact)
- Phase transition at ~peak 10–12 (T ≈ 100–150): W/H drops from ~6 to ~2.5

**What it drills:** Diagonal (depth-dependent constraint intersecting width and height).

### SHAPE TEST 9 — Edge Partition & Distance Mapping

Classifies zeros by position relative to peaks/troughs using local extrema
detection, then compares spacing statistics across groups. Measures "distance
to connection" — how far each dimension must drill before it stabilizes.

**Key findings:**
- Dimensions connect at different depths (layered discovery):
  - Correlation connects at peak 20 (N_smooth = 0.73)
  - Edge partition at T = 33.05
  - W/H ratio stabilizes at peak 47 (N_smooth = 4.53)
- Strong level repulsion near peaks: all spacings < 0.5
- Mean spacing near peaks 0.178 vs troughs 0.205

**What it drills:** Edge partition (positional structure).

### SHAPE TEST 10 — Perpendicular to Grade (Fixed 100ft Drill)

The core protocol test. At each zero, fits a local linear trend over
half_window=5 on either side, then measures the perpendicular residual
(actual value minus fitted trend). Walks the entire sequence in connected
order with the same fixed tool everywhere — no near-peak cherry-picking.

**Key findings:**
- Perpendicular residual statistics: Mean = -0.0320, Std = 0.2031
- Corr(perp residual, local slope) = -0.0798 (orthogonal to grade as designed)
- Corr(perp residual, local curvature) = -0.8273 (very strong — the drill measures curvature)
- No near-peak bias: near-peak std=0.2073 vs near-trough std=0.1756 (similar)
- Max consecutive same-sign residuals = 5 (weak memory — each step is new info)

**Conclusion:** The perpendicular-to-grade drill measures the curvature field
uniformly along a connected path. It reconstructs hill geometry from local
bending alone without needing to know where peaks/troughs are in advance.

**What it drills:** Perpendicular to grade (curvature field).

### SHAPE TEST 11 — Gradient Descent Zero Finding

Uses gradient + curvature (perpendicular drill) to predict zeros via
Newton-like extrapolation: t_predicted = t - f(t)/f'(t), validated by checking
that f·f'' > 0 (the hill bends in a way that supports the zero existing).
Respects the 100-foot constraint by only trusting predictions within 50 units.

**Protocol:**
1. At each point along the connected walk, compute f(t), f'(t), f''(t)
2. Solve f + f'·dt + 0.5·f''·dt² = 0 for predicted zero locations
3. Validate: if f·f'' > 0 and |dt| < 50, the prediction is geometrically plausible
4. If gradient points uphill (f' > 0) with negative curvature (f'' < 0),
   estimate distance to peak: d ≈ |f'| / |f''|

**Key findings:**
- 48 predicted zeros from 20 starting points
- All 10 known zeros matched by at least one prediction
- Typical prediction error: < 0.5 units
- Distance-to-peak estimates: mean = 0.57, median = 0.36 (peaks are close together)
- 2 predicted peaks matched actual peaks in trajectory

**Conclusion:** Gradient-based zero finding with curvature validation can
locate zeros far from where you start using only local information. The
perpendicular drill (curvature measurement) is what makes predictions
trustworthy — without it, you'd be doing blind linear extrapolation.

**What it drills:** Gradient direction + perpendicular validation (zero finding).

## Drilling Directions Summary

| Test | Direction | Measures | Effort to Connect |
|------|-----------|----------|-------------------|
| 1-4 | Vertical | Local dynamics, chaos | Per-point |
| 5, 7 | Horizontal | Width/height relationships | Peak detection |
| 6 | Diagonal | Temporal position vs magnitude | Step counting |
| 8 | Diagonal (rigorous) | Depth-dependent W/H constraint | 20 peaks |
| 9 | Edge partition | Positional structure | T=33 |
| 10 | Perpendicular to grade | Curvature field | 1 connected walk |
| 11 | Gradient + perpendicular | Zero locations, peak distances | Connected walk |

Each direction uses the same fixed tool but orients it differently relative
to terrain. They measure different things (global relationship vs local
bending vs positional effects) and connect through logical consistency —
all must describe the same underlying hill.

## The 100-Foot Drill Constraint

All measurements respect a single constraint: the same fixed-length tool,
always perpendicular to local grade, applied uniformly across the entire
sequence. This means:

- No cherry-picking easy near-peak points
- Connected walks only (no teleporting between distant points)
- Predictions validated by curvature before being trusted
- The "optimal angle" hypothesis: perpendicular drilling may not just be a
  constraint but the geometrically optimal direction for mapping uncertainty

## Key Results Summary

| Finding | Value | Source Test |
|---------|-------|-------------|
| W/H depth dependence (Model B slope) | -0.70 | Test 8 |
| Variance explained by depth | 67% | Test 8 |
| Early/middle/late std dev | 2.29 / 2.42 / 0.94 | Test 8 |
| Detrended Corr(W²+H², N_smooth) | 0.72 | Test 8 |
| Pythagorean constraint correlation | 0.98 | Test 7 |
| Perpendicular residual ↔ curvature | -0.83 | Test 10 |
| Perpendicular residual ↔ slope | -0.08 (orthogonal) | Test 10 |
| Zero prediction error (typical) | < 0.5 units | Test 11 |
| Distance to peak (mean) | 0.57 units | Test 11 |
| Layered dimension discovery | peaks 20 → T=33 → peak 47 | Test 9 |

## SHAPE TEST 12 — Connected Multi-Start Walk

**Question:** Can we chain predictions together? If each validated zero becomes
the next starting point, how far can we map the hill before the chain breaks?

**Protocol:**
1. Start at a point on the trajectory
2. Use gradient + curvature to predict the next zero (quadratic extrapolation)
3. Validate: is the predicted T within 2 units of an actual zero?
4. If yes, move there and repeat. If no, the walk breaks.

**Results (T=1000, 645 zeros):**
- Walks attempted: 65
- Total predictions made: 58
- Validated steps: 55 (94.8% validation rate)
- Mean walk length: 0.85 steps
- Max walk length: 5 steps

**Interpretation:**
The hill is **locally predictable** (94.8% of predictions land near actual zeros)
but **globally fragmented** (walks break after ~1 step on average). This means:

- The 100ft drill works: local geometry (gradient + curvature) reliably predicts
  where the next zero is
- But you can't build long chains — the terrain changes too rapidly to continue
  without re-orienting
- Mapping the full hill requires many independent starting points, not one long walk

**Scale comparison:**

| Metric | T=200 (93 zeros) | T=1000 (645 zeros) |
|--------|------------------|---------------------|
| Validation rate | 90.0% | 94.8% |
| Mean walk length | 0.90 steps | 0.85 steps |
| Max walk length | 5 steps | 5 steps |

The protocol is scale-invariant: local predictability holds at both scales,
but global connectivity doesn't improve with more data.

## Scale-Up Findings (T=200 → T=1000)

**What changed:**
- Zeros: 93 → 645
- Peaks: 25 → 200

**Key corrections from small-sample bias:**

1. **Model A (W = H^k) is the real constraint**, not Model B (depth-dependent):
   - T=200: Neither model fit well (R² ≈ 0) — insufficient data
   - T=1000: Model A wins with R² = 0.81, W = H^1.42
   - This is a **fixed geometric constraint** that holds across scales

2. **The Pythagorean constraint was partially an artifact:**
   - T=200: Corr(W²+H², N_smooth) = 0.72 (looked like geometry)
   - T=1000: Detrended Corr = 0.795 (survives detrending — real relationship)
   - But it's not the simple Pythagorean tuple we thought — it's more complex

3. **W/H behavior is scale-dependent:**
   - T=200: W/H increases with depth (1.48 → 5.03)
   - T=1000: Non-monotonic (25.9 → 64.8 → 25.4)
   - Small sample was misleading about the trend

**What survived scale-up:**
- Connected walk validation rate: 90% → 94.8% (local predictability holds)
- Perpendicular drill orthogonality to slope: still ~0 correlation
- Curvature-based prediction: works at both scales

## Implementation

All shape tests are in `src/riemann/zeta_riemann_siegel.rs`:

- `count_trajectory(t_max)` — generates the base trajectory data
- `find_peaks(r_values)` — local extrema detection
- `two_sample_ks(a, b)` — Kolmogorov-Smirnov test for distribution comparison
- `shape_test_*` functions — individual tests returning formatted reports
- `all_shape_tests(t_max)` — runs all 12 tests in sequence

Run with: `cargo run --example debug_multiscale`
