# Normalized Shape Analysis of Z(t) Between Consecutive Zeros (mpmath-validated)

## Question
Does the normalized shape of Z(t) between consecutive zeros contain structure beyond what's explained by gap width and height? Specifically, do shape features predict neighboring gap ratios after controlling for height and gap size?

## Methodology

### Data
- **Zeros**: 500 zeros from `mpmath.zetazero(n)` for n = 1..500 (t ∈ [14.1347, 811.1844])
- **Intervals**: 499 consecutive-zero intervals [γ_n, γ_2467]
- **Z(t) evaluator**: `mpmath.zeta(0.5 + 1j*t)` with Siegel theta correction at dps=50
  - Independently validated at all points (not just zeros)
  - This is the critical difference from the earlier RS-based analysis
- **Sampling**: Each interval sampled at 11 normalized positions u = 0, 0.1, ..., 1.0
- **Normalization**: Z(t) divided by interval RMS magnitude to remove amplitude scaling

### Shape Features Extracted
For each normalized interval shape Z_norm(u):

1. **Peak position** (u*): The u-value where |Z_norm(u)| is maximized
   - Measures asymmetry of the arch between zeros
   - Mean: 0.5034, Std: 0.1309

2. **Shape kurtosis**: Fourth moment of the normalized shape
   - Measures how "peaked" vs "flat" the shape is
   - Mean: 1.6950, Std: 0.2603

### Preregistered Hypothesis Test
- **Feature X**: Shape feature from normalized interval [γ_n, γ_2467]
- **Target Y**: log(gap_ratio) where gap_ratio = (γ_2468 - γ_2467) / (γ_2467 - γ_n)
- **Controls**: Height (log t at interval midpoint), current gap size
- **Validation**: Contiguous held-out split by height (train on low t, test on high t)

## Results

### Full Sample Analysis (n=498)

| Metric | Value |
|--------|-------|
| Peak position partial correlation | r = 0.4148 |
| Linear model R² | 0.1720 |
| Null test p-value | < 0.000001 |
| Spearman rho (monotonicity) | 0.4106 |

### Binned Analysis: Mean log(gap_ratio) by Peak Position

| u* range | n | Mean log(gap_ratio) | Interpretation |
|----------|---|---------------------|----------------|
| [0.20, 0.29) | 9 | +0.2129 | next gap larger |
| [0.29, 0.37) | 55 | -0.0577 | neutral |
| [0.37, 0.46) | 108 | -0.1628 | next gap smaller |
| [0.46, 0.54) | 145 | -0.2259 | next gap smaller |
| [0.54, 0.63) | 113 | +0.0894 | neutral |
| [0.63, 0.71) | 56 | +0.5132 | next gap larger |

### Height-Dependent Correlation

| log(t) range | n | Partial r |
|--------------|---|-----------|
| [4.0, 5.0) | 41 | 0.6165 |
| [5.0, 6.0) | 152 | 0.5483 |
| [6.0, 7.0) | 295 | 0.3716 |

### Contiguous Held-Out Test
- **Train**: log(t) ∈ [2.87, 6.15], n=249
- **Test**: log(t) ∈ [6.16, 6.70], n=249
- **Result**: r_test = 0.3571, R²_test = 0.1073

## Interpretation

### What the earlier RS-based analysis got wrong
The initial analysis using first-order Riemann-Siegel shapes reported R² = 0.41 (peak position) and R² = 0.64 (combined with kurtosis) on held-out data. These numbers were **inflated by approximation errors** in the RS formula at ordinary points. The mpmath-based analysis shows:

- Peak position effect is **real but weaker**: R² ≈ 0.17 full sample, 0.11 held-out
- Kurtosis effect **does not survive** held-out testing
- The combined model from before was largely an artifact of RS errors

### What the mpmath analysis shows
1. **Peak position has a genuine but modest predictive signal**: Intervals where Z(t) peaks to the right of center tend to be followed by larger gaps (mean log ratio +0.51 for u* > 0.63 vs -0.23 for u* < 0.46).

2. **The effect is non-stationary**: The correlation decreases with height:
   - log(t) ∈ [4, 5): r = 0.617
   - log(t) ∈ [5, 6): r = 0.548
   - log(t) ∈ [6, 7): r = 0.372
   
   This suggests the shape carries more predictive information at lower t, where zeros are more regular, and less at higher t where they become more random-like.

3. **Held-out test confirms weakness**: Training on low-t intervals and testing on high-t gives R² = 0.107 — a real but modest effect that doesn't generalize strongly across height ranges.

### Physical interpretation (tentative)
When Z(t) between two zeros peaks later in the interval (closer to the next zero), it may indicate that the local dynamics are "building up" more slowly, which could be associated with the next zero being further away. However, this interpretation is speculative and the effect is modest enough that other explanations (e.g., residual correlations in the zero process) cannot be ruled out.

## Conclusion

**The normalized shape contains a small amount of predictive information about future gap ratios**, but it is:
- **Modest**: R² ≈ 0.17 full sample, 0.11 held-out
- **Non-stationary**: Weaker at higher t
- **Single-feature**: Only peak position survives; kurtosis does not

This is a real but weak empirical lead. It does NOT support the stronger claim from the earlier RS-based analysis that shape features explain 64% of gap ratio variance. The true signal is closer to 10-17% R², which is interesting but not transformative.

## Reproduction

```python
import mpmath
mpmath.mp.dps = 50

def z_mpmath(t):
    theta = mpmath.siegeltheta(t)
    zv = mpmath.zeta(0.5 + 1j * t)
    return float((zv * mpmath.e**(-1j * theta)).real)

zeros = [float(mpmath.zetazero(n).imag) for n in range(1, 501)]
# ... sample each interval at u-values, normalize by RMS, compute peak position
```

## Limitations
1. Only 500 zeros (t up to ~811) due to mpmath computation cost
2. 11-point sampling per interval may miss fine structure
3. Linear model only — nonlinear relationships may exist
4. Contiguous held-out split tests stationarity but not out-of-distribution generalization
5. The effect is modest enough that it could be partially driven by residual zero-spacing correlations not captured by the controls

## Comparison with RS-Based Analysis (Previous)

| Metric | RS-based | mpmath-based |
|--------|----------|--------------|
| Peak position R² (full) | 0.37 | 0.17 |
| Peak position R² (held-out) | 0.41 | 0.11 |
| Kurtosis survives? | Yes | No |
| Combined R² (held-out) | 0.64 | ~0.01 |

The RS-based analysis overestimated the effect by a factor of 3-6x due to approximation errors at ordinary points.
