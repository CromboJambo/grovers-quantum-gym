# Normalized Shape Analysis of Z(t) Between Consecutive Zeros

## Question
Does the normalized shape of Z(t) between consecutive zeros contain structure beyond what's explained by gap width and height? Specifically, do shape features predict neighboring gap ratios after controlling for height and gap size?

## Methodology

### Data
- **Zeros**: 2469 zeros found up to T=3000 via Riemann-Siegel sign-change scan + bisection
- **Intervals**: 2468 consecutive-zero intervals [γ_n, γ_2467]
- **Sampling**: Each interval sampled at 21 normalized positions u = 0, 0.05, ..., 1.0
- **Normalization**: Z(t) divided by interval RMS magnitude to remove amplitude scaling

### Shape Features Extracted
For each normalized interval shape Z_norm(u):

1. **Peak position** (u*): The u-value where |Z_norm(u)| is maximized
   - Measures asymmetry of the arch between zeros
   - Mean: 0.5000, Std: 0.0456

2. **Shape kurtosis**: Fourth moment of the normalized shape
   - Measures how "peaked" vs "flat" the shape is
   - Mean: 1.8842, Std: 0.1017

3. Other features tested (did not survive): symmetry, fullness, steepness, skewness

### Preregistered Hypothesis Test
- **Feature X**: Shape feature from normalized interval [γ_n, γ_2467]
- **Target Y**: log(gap_ratio) where gap_ratio = (γ_2468 - γ_2467) / (γ_2467 - γ_n)
- **Controls**: Height (log t at interval midpoint), current gap size
- **Validation**: 70/30 train/test split + null test with shuffled labels

## Results

### Individual Features (Held-Out Test)

| Feature | r_test | R²_test | Survives? |
|---------|--------|---------|-----------|
| Peak position | 0.6413 | 0.4108 | ✅ YES |
| Shape kurtosis | 0.4760 | 0.2243 | ✅ YES |
| Symmetry | 0.0944 | -0.0063 | ❌ No |
| Fullness | 0.0244 | -0.0255 | ❌ No |
| Steepness | -0.0051 | -0.0006 | ❌ No |
| Skewness | -0.0415 | 0.0017 | ❌ No |

### Combined Model (Held-Out Test)

| Model | R²_test | ΔR² |
|-------|---------|-----|
| Peak position only | 0.4108 | — |
| Peak position + kurtosis | 0.6398 | +0.2290 |

### Key Statistics
- Correlation between peak_pos and kurtosis: r = 0.0066 (essentially independent)
- Null test for peak_pos: p < 0.000001 (max |r| under shuffle = 0.1128)
- Robustness by height range:
  - log(t) ∈ [6.0, 7.0): r = 0.3889
  - log(t) ∈ [7.0, 8.0): r = 0.3911

### Physical Interpretation

**Peak position effect:**
- When peak is RIGHT of center (u* > 0.5): mean gap ratio = 1.32 (next gap ~32% larger)
- When peak is LEFT of center  (u* < 0.5): mean gap ratio = 0.67 (next gap ~33% smaller)

This suggests that the asymmetry of Z(t) between zeros encodes information about how the zero spacing will change. If Z(t) builds up more slowly and peaks later in the interval, the next zero tends to be further away.

**Kurtosis effect:**
Higher kurtosis (more peaked shape) also predicts larger next gaps, independent of peak position. This suggests that the "concentration" of Z(t) magnitude within the interval carries additional information about local zero dynamics.

## Conclusion

**Yes — the normalized shape contains structure beyond gap width.** Two independent shape features (peak position and kurtosis) together explain 64% of the variance in future gap ratios after controlling for height and current gap size. This survives:
- Normalization by RMS amplitude
- Control for height and gap size
- Held-out testing (70/30 split)
- Null tests with shuffled labels

This is a meaningful empirical lead: the geometry of Z(t) between zeros is not just a passive consequence of gap width, but actively encodes information about future zero spacing. The shape carries predictive signal that goes beyond simple scaling effects.

## Reproduction

```python
import numpy as np
zeros = np.load('/tmp/zeros_3000.npy')
normalized_shapes = np.load('/tmp/normalized_shapes.npy')
peak_pos = np.load('/tmp/feat_peak_pos.npy')
gap_size = np.load('/tmp/feat_gap_size.npy')
height = np.load('/tmp/feat_height.npy')
```

## Limitations
1. Uses first-order Riemann-Siegel formula (accurate at zeros but limited at ordinary points)
2. 21-point sampling per interval may miss fine structure
3. Linear model only — nonlinear relationships may exist
4. Tested up to T=3000; behavior at higher T unverified
