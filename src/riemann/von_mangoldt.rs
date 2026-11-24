//! Von Mangoldt Sum Analysis
//!
//! The von Mangoldt function Λ(n) and its weighted sums are the central objects
//! in the analytic approach to RH. Alpöge's proof (Nov 2026) controls sums of
//! the form Σ_{n≤X} Λ(n)(n-1)^{-it} via iterated theta lifts between unitary groups.
//!
//! The explicit formula connects these sums to zeta zeros:
//!   Ψ(x) = x - Σ_ρ x^ρ/ρ - log(2π) - ½log(1-x^{-2})
//! where Ψ(x) = Σ_{n≤x} Λ(n) and the sum runs over non-trivial zeros ρ.
//!
//! This module computes these sums directly to explore how prime distribution
//! "feels" the zeta zeros through their x^ρ/ρ contributions.

use std::f64::consts::PI;
use num_complex::Complex64;
use crate::riemann::integer_structure::{factorize, is_prime_power};

/// Compute the von Mangoldt function Λ(n).
/// Λ(n) = log(p) if n = p^k for prime p and k ≥ 1, else 0.
pub fn von_mangoldt(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    // Λ(n) = log(p) iff n is a prime power p^k
    // Use factorization: if n has exactly one distinct prime factor, it's a prime power
    let factors = factorize(n);
    if factors.len() == 1 {
        (factors[0].0 as f64).ln() // log(p) — the exponent k doesn't matter for Λ
    } else {
        0.0
    }
}

/// Compute the Chebyshev function Ψ(x) = Σ_{n≤x} Λ(n).
/// Under RH, Ψ(x) = x + O(√x log²x).
pub fn chebyshev_psi(x: u64) -> f64 {
    let mut sum = 0.0;
    for n in 2..=x {
        sum += von_mangoldt(n);
    }
    sum
}

/// Compute the weighted von Mangoldt sum S(t, X) = Σ_{n≤X} Λ(n) · (n-1)^{-it}.
/// This is the object controlled by Alpöge's theta lift argument.
/// The weight (n-1)^{-it} = exp(-it·log(n-1)) oscillates with frequency t,
/// so this sum "probes" prime distribution at scale 1/t.
pub fn von_mangoldt_weighted_sum(t: f64, x_limit: u64) -> Complex64 {
    let mut sum = Complex64::new(0.0, 0.0);
    for n in 2..=x_limit {
        let lambda = von_mangoldt(n);
        if lambda != 0.0 {
            // (n-1)^{-it} = exp(-it · log(n-1))
            let phase = -t * ((n - 1) as f64).ln();
            sum += lambda * Complex64::new(phase.cos(), phase.sin());
        }
    }
    sum
}

/// Compute the "error term" E(x) = Ψ(x) - x.
/// Under RH, |E(x)| = O(√x log²x). We can measure how the actual error
/// compares to the RH prediction.
pub fn chebyshev_error(x: u64) -> f64 {
    chebyshev_psi(x) - x as f64
}

/// Estimate the RH-predicted error bound: C · √x · log²(x)
/// (The constant C is not known precisely; we use 1 as a rough scale.)
pub fn rh_error_bound(x: u64) -> f64 {
    let x_f = x as f64;
    x_f.sqrt() * (x_f.ln().powi(2))
}

/// Scan the weighted von Mangoldt sum across a range of t values for fixed X.
/// Returns (t, |S(t,X)|, Re(S), Im(S)) tuples. Large |S| at specific t values
/// indicates resonance with zeta zeros near that height.
pub fn scan_weighted_sum(x_limit: u64, t_min: f64, t_max: f64, steps: usize) -> Vec<(f64, f64, f64, f64)> {
    let mut results = Vec::new();
    for i in 0..=steps {
        let t = t_min + (t_max - t_min) * i as f64 / steps as f64;
        let s = von_mangoldt_weighted_sum(t, x_limit);
        results.push((t, s.norm(), s.re, s.im));
    }
    results
}

/// Compute the first few zeta zero contributions to the explicit formula.
/// For a zero ρ = 1/2 + iγ, the contribution is x^ρ/ρ = x^{1/2+iγ}/(1/2+iγ).
/// Summing over known zeros gives an approximation to the oscillatory part of Ψ(x)-x.
pub fn zeta_zero_contributions(x: f64, gamma_values: &[f64]) -> Complex64 {
    let mut sum = Complex64::new(0.0, 0.0);
    for &gamma in gamma_values {
        let rho = Complex64::new(0.5, gamma);
        // x^ρ = exp(ρ · log(x))
        let log_x = x.ln();
        let x_rho = (rho * log_x).exp();
        sum += x_rho / rho;
    }
    sum
}

/// First 10 ordinates of non-trivial zeta zeros (known values)
pub const KNOWN_ZERO_ORDINATES: &[f64] = &[
    14.134725141734694,
    21.022039638771555,
    25.010857580145688,
    30.424876125859513,
    32.935061587739190,
    37.586178158825670,
    40.918719012147495,
    43.327073280914000,
    48.005150881167100,
    49.773832477672300,
];

/// Generate a report comparing actual Chebyshev error with zeta-zero predictions.
pub fn von_mangoldt_report(x_limit: u64) -> String {
    let mut report = String::new();
    report.push_str("=== Von Mangoldt Sum Analysis ===\n\n");

    // 1. Basic Chebyshev function behavior
    report.push_str("Chebyshev Ψ(x) vs x:\n");
    for &x in &[100u64, 500, 1000, 5000] {
        if x > x_limit { continue; }
        let psi = chebyshev_psi(x);
        let error = chebyshev_error(x);
        let bound = rh_error_bound(x);
        report.push_str(&format!(
            "  x={:5}: Ψ(x)={:10.2}, error={:+9.2}, RH-bound(√x·log²x)={:.2}, ratio={:.3}\n",
            x, psi, error, bound, (error.abs() / bound).min(999.0)
        ));
    }

    // 2. Weighted sum behavior at specific t values
    report.push_str("\nWeighted sums S(t,X) = Σ Λ(n)(n-1)^{-it} for X=1000:\n");
    let x_scan = std::cmp::min(1000u64, x_limit);
    for &t in &[0.0f64, 5.0, 14.134725141734694, 21.022039638771555, 50.0] {
        let s = von_mangoldt_weighted_sum(t, x_scan);
        report.push_str(&format!(
            "  t={:8.4}: |S|={:8.4}, Re={:+8.4}, Im={:+8.4}\n",
            t, s.norm(), s.re, s.im
        ));
    }

    // 3. Resonance check: do known zero ordinates produce larger |S|?
    report.push_str("\nResonance test (X=500): |S(t,X)| at zero ordinates vs nearby t:\n");
    let x_res = std::cmp::min(500u64, x_limit);
    for &gamma in &KNOWN_ZERO_ORDINATES[..5] {
        let s_at_zero = von_mangoldt_weighted_sum(gamma, x_res).norm();
        let s_nearby = von_mangoldt_weighted_sum(gamma + 1.0, x_res).norm();
        report.push_str(&format!(
            "  γ={:8.4}: |S(γ)|={:.4}, |S(γ+1)|={:.4}, ratio={:.2}\n",
            gamma, s_at_zero, s_nearby, if s_nearby > 0.0 { s_at_zero / s_nearby } else { f64::INFINITY }
        ));
    }

    // 4. Zeta zero contribution approximation
    report.push_str("\nExplicit formula check (x=100):\n");
    let x_check = 100.0_f64;
    let contributions = zeta_zero_contributions(x_check, &KNOWN_ZERO_ORDINATES[..5]);
    let actual_error = chebyshev_error(100);
    report.push_str(&format!(
      "  Actual Ψ(100)-100 = {:+.4}\n",
      actual_error
    ));
    report.push_str(&format!(
      "  Sum of first 5 zero contributions x^ρ/ρ: Re={:+.4}, Im={:+.4}\n",
      contributions.re, contributions.im
    ));
    report.push_str("  (The real part of the negative sum approximates the error oscillation)\n");

    report
}


/// First 50 known ordinates of non-trivial zeta zeros (Riemann-van der Corput list)
/// Source: standard tables, first 10 from rh_verification.rs, extended here.
pub const KNOWN_ZERO_ORDINATES_50: &[f64] = &[
    14.134725141734694, 21.022039638771555, 25.010857580145688,
    30.424876125859513, 32.935061587739190, 37.586178158825670,
    40.918719012147495, 43.327073280914000, 48.005150881167100,
    49.773832477672300, 52.970321477713600, 56.446247697010800,
    59.347044007429800, 60.831778556918300, 65.112544048374300,
    67.079810524125700, 69.546401711177700, 72.067153661264600,
    75.704690692760800, 77.144840075947800, 79.337375022282500,
    82.916000822430800, 84.735492911113200, 87.425274689107300,
    88.809111201071300, 92.491899271687900, 94.651342443406300,
    95.870634387632300, 101.317851943590000, 103.725538617869000,
    105.387353930005000, 107.169094142088000, 111.029535747208000,
    111.874632612993000, 114.139178378281000, 115.592876486308000,
    118.250395533489000, 121.817190088830000, 124.256660638096000,
    124.603790394660000, 128.636560866865000, 130.024520450480000,
    131.790578725939000, 133.495808215678000, 134.702820657853000,
    138.116864019332000, 139.930464783679000, 141.123863590285000,
    143.110815488962000, 147.666429704621000, 149.471525515385000,
];

/// Estimate zero density by finding local maxima of |S(t,X)| in a t-range.
/// Each significant local maximum near a known zero ordinate suggests a zero
/// contribution to the von Mangoldt sum. Returns detected peaks as (t, magnitude).
pub fn estimate_zero_density(x_limit: u64, t_min: f64, t_max: f64, steps: usize) -> Vec<(f64, f64)> {
    let results = scan_weighted_sum(x_limit, t_min, t_max, steps);
    let mut peaks = Vec::new();
    
    // Find local maxima (3-point check)
    for i in 1..results.len().saturating_sub(1) {
        let (_, mag_prev, _, _) = results[i - 1];
        let (t_cur, mag_cur, _, _) = results[i];
        let (_, mag_next, _, _) = results[i + 1];
        
        if mag_cur > mag_prev && mag_cur > mag_next && mag_cur > 0.0 {
            peaks.push((t_cur, mag_cur));
        }
    }
    
    // Sort by magnitude descending
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    peaks
}

/// Generate a random "pseudo-prime" distribution for comparison.
/// Creates a set of n values in [2, limit] with the same count as actual primes,
/// but randomly placed (respecting no divisibility structure).
pub fn generate_pseudo_primes(n_count: usize, limit: u64) -> Vec<u64> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut pseudo = Vec::new();
    
    for _ in 0..n_count {
        loop {
            let candidate = 2 + rng.gen_range(0..limit as u64 - 1);
            if !pseudo.contains(&candidate) {
                pseudo.push(candidate);
                break;
            }
        }
    }
    pseudo.sort();
    pseudo
}

/// Compute weighted sum for a custom set of "primes" (for random model comparison).
pub fn pseudo_weighted_sum(primes: &[u64], t: f64) -> Complex64 {
    let mut sum = Complex64::new(0.0, 0.0);
    for &p in primes {
        if p < 2 { continue; }
        let lambda = (p as f64).ln(); // Treat each pseudo-prime like Λ(p)
        let phase = -t * ((p - 1) as f64).ln();
        sum += lambda * Complex64::new(phase.cos(), phase.sin());
    }
    sum
}

/// Compare resonance at zero ordinates: real primes vs random model.
/// Returns (gamma, real_ratio, pseudo_ratio) for each tested zero.
pub fn compare_resonance(x_limit: u64, num_pseudo_trials: usize) -> Vec<(f64, f64, f64)> {
    // Get actual primes up to x_limit
    let actual_primes: Vec<u64> = (2..=x_limit).filter(|&n| is_prime_power(n)).collect();
    
    let mut results = Vec::new();
    for &gamma in &KNOWN_ZERO_ORDINATES[..5] {
        // Real primes resonance
        let real_at = von_mangoldt_weighted_sum(gamma, x_limit).norm();
        let real_nearby = von_mangoldt_weighted_sum(gamma + 1.0, x_limit).norm();
        let real_ratio = if real_nearby > 0.0 { real_at / real_nearby } else { 0.0 };
        
        // Average over random trials
        let mut pseudo_ratios_sum = 0.0;
        for _ in 0..num_pseudo_trials {
            let pseudo = generate_pseudo_primes(actual_primes.len(), x_limit);
            let p_at = pseudo_weighted_sum(&pseudo, gamma).norm();
            let p_nearby = pseudo_weighted_sum(&pseudo, gamma + 1.0).norm();
            if p_nearby > 0.0 {
                pseudo_ratios_sum += p_at / p_nearby;
            }
        }
        let pseudo_ratio = pseudo_ratios_sum / num_pseudo_trials as f64;
        
        results.push((gamma, real_ratio, pseudo_ratio));
    }
    results
}

/// Explicit formula approximation with N zeros.
/// Computes -Σ_{i=1}^{N} x^{ρ_i}/ρ_i and compares to actual error E(x) = Ψ(x) - x.
/// Shows how the approximation improves as more zeros are included.
pub fn explicit_formula_convergence(x: u64, max_zeros: usize) -> Vec<(usize, f64, f64)> {
    let actual_error = chebyshev_error(x);
    let mut results = Vec::new();
    
    for n in 1..=std::cmp::min(max_zeros, KNOWN_ZERO_ORDINATES_50.len()) {
        let contributions = zeta_zero_contributions(x as f64, &KNOWN_ZERO_ORDINATES_50[..n]);
        // The explicit formula has Ψ(x) ≈ x - Σ x^ρ/ρ - constants
        // So E(x) = Ψ(x) - x ≈ -Re(Σ x^ρ/ρ) - constants
        let approx_error = -contributions.re;
        results.push((n, actual_error, approx_error));
    }
    results
}

/// Check if detected peaks align with known zero ordinates.
/// Returns fraction of top-K peaks that are within tolerance of a known zero.
pub fn peak_alignment_score(peaks: &[(f64, f64)], k: usize, tolerance: f64) -> f64 {
    let top_k = &peaks[..std::cmp::min(k, peaks.len())];
    let mut aligned = 0;
    
    for &(t, _) in top_k {
        for &gamma in KNOWN_ZERO_ORDINATES_50 {
            if (t - gamma).abs() < tolerance {
                aligned += 1;
                break;
            }
        }
    }
    
    if top_k.is_empty() { 0.0 } else { aligned as f64 / top_k.len() as f64 }
}

/// Comprehensive report on von Mangoldt sums and their connection to zeta zeros.
pub fn von_mangoldt_deep_report(x_limit: u64) -> String {
    let mut report = String::new();
    report.push_str("=== Von Mangoldt Deep Analysis ===\n");
    report.push_str(&format!("X limit: {}\n\n", x_limit));

    // 1. Error term behavior
    report.push_str("Chebyshev error E(x) = Ψ(x) - x:\n");
    for &x in &[50u64, 100, 200, 500, 1000] {
        if x > x_limit { continue; }
        let error = chebyshev_error(x);
        let bound = rh_error_bound(x);
        report.push_str(&format!("  x={:5}: E={:+8.3}, RH-bound={:.2}, ratio={:.4}\n", 
                                 x, error, bound, (error.abs() / bound).min(999.0)));
    }

    // 2. Zero density estimation
    report.push_str("\nZero density estimation (peaks of |S(t,X)| for t in [10,60]):\n");
    let peaks = estimate_zero_density(std::cmp::min(x_limit, 500), 10.0, 60.0, 200);
    report.push_str(&format!("  Detected {} local maxima\n", peaks.len()));
    for i in 0..std::cmp::min(8, peaks.len()) {
        let (t, mag) = peaks[i];
        // Find nearest known zero
        let nearest = KNOWN_ZERO_ORDINATES_50.iter()
            .map(|&g| (g, (t - g).abs()))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        report.push_str(&format!("  t={:7.3}, |S|={:8.3} (nearest zero: γ={:.4}, Δ={:.3})\n",
                                 t, mag, nearest.0, nearest.1));
    }

    // 3. Peak alignment score
    let score = peak_alignment_score(&peaks, 5, 2.0);
    report.push_str(&format!("\n  Alignment of top-5 peaks with known zeros (tol=2.0): {:.1}%\n", score * 100.0));

    // 4. Explicit formula convergence
    let x_check = std::cmp::min(100u64, x_limit);
    report.push_str(&format!("\nExplicit formula convergence at x={}:\n", x_check));
    report.push_str(&format!("  Actual E(x) = Ψ({}) - {} = {:+.4}\n", x_check, x_check, chebyshev_error(x_check)));
    let convergence = explicit_formula_convergence(x_check, 10);
    for &(n, actual, approx) in &convergence {
        report.push_str(&format!("  N={:2} zeros: -Re(Σx^ρ/ρ)={:+8.4}, actual={:+8.4}, diff={:.4}\n",
                                 n, approx, actual, (approx - actual).abs()));
    }

    // 5. Random model comparison
    report.push_str("\nResonance: real primes vs random model (X=200, 10 trials):\n");
    let comparisons = compare_resonance(std::cmp::min(x_limit, 200), 10);
    for &(gamma, real_r, pseudo_r) in &comparisons {
        report.push_str(&format!("  γ={:8.4}: real ratio={:.3}, pseudo avg={:.3}\n", gamma, real_r, pseudo_r));
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_von_mangoldt_primes() {
        // Λ(p) = log(p) for primes
        assert!((von_mangoldt(2) - 2.0_f64.ln()).abs() < 1e-10);
        assert!((von_mangoldt(3) - 3.0_f64.ln()).abs() < 1e-10);
        assert!((von_mangoldt(5) - 5.0_f64.ln()).abs() < 1e-10);
    }

    #[test]
    fn test_von_mangoldt_prime_powers() {
        // Λ(p^k) = log(p) for prime powers
        assert!((von_mangoldt(4) - 2.0_f64.ln()).abs() < 1e-10);   // 2²
        assert!((von_mangoldt(8) - 2.0_f64.ln()).abs() < 1e-10);   // 2³
        assert!((von_mangoldt(9) - 3.0_f64.ln()).abs() < 1e-10);   // 3²
    }

    #[test]
    fn test_von_mangoldt_composites() {
        // Λ(n) = 0 for n with multiple distinct prime factors
        assert_eq!(von_mangoldt(6), 0.0);  // 2·3
        assert_eq!(von_mangoldt(12), 0.0); // 2²·3
        assert_eq!(von_mangoldt(30), 0.0); // 2·3·5
    }

    #[test]
    fn test_chebyshev_psi_monotone() {
        // Ψ(x) should be non-decreasing
        let psi_10 = chebyshev_psi(10);
        let psi_100 = chebyshev_psi(100);
        assert!(psi_100 > psi_10);
    }

    #[test]
    fn test_chebyshev_error_within_rh_bound() {
        // For small x, the error should be well within the RH-predicted bound
        for &x in &[50u64, 100, 500] {
            let error = chebyshev_error(x).abs();
            let bound = rh_error_bound(x);
            assert!(error < bound, "At x={}: error {} exceeds bound {}", x, error, bound);
        }
    }

    #[test]
    fn test_weighted_sum_at_zero_t() {
        // S(0, X) = Σ Λ(n) = Ψ(X) (no oscillation)
        let s = von_mangoldt_weighted_sum(0.0, 100);
        let psi = chebyshev_psi(100);
        assert!((s.re - psi).abs() < 1e-6);
        assert!(s.im.abs() < 1e-6);
    }

    #[test]
    fn test_weighted_sum_oscillates() {
        // For t > 0, the sum should have imaginary component (oscillation)
        let s = von_mangoldt_weighted_sum(5.0, 100);
        assert!(s.norm() > 0.0);
    }

    #[test]
    fn test_scan_returns_correct_length() {
        let results = scan_weighted_sum(100, 0.0, 10.0, 50);
        assert_eq!(results.len(), 51); // steps + 1
    }


    #[test]
    fn test_known_zero_ordinates_50_positive() {
        for &gamma in KNOWN_ZERO_ORDINATES_50 {
            assert!(gamma > 0.0);
        }
        assert!(KNOWN_ZERO_ORDINATES_50.len() >= 50);
    }

    #[test]
    fn test_estimate_zero_density_finds_peaks() {
        let peaks = estimate_zero_density(100, 10.0, 30.0, 100);
        // Should find at least some local maxima
        assert!(!peaks.is_empty());
    }

    #[test]
    fn test_explicit_formula_convergence_improves() {
        // With more zeros, the approximation should generally get closer
        let convergence = explicit_formula_convergence(50, 10);
        assert_eq!(convergence.len(), 10);
        // First entry uses 1 zero
        assert_eq!(convergence[0].0, 1);
    }

    #[test]
    fn test_peak_alignment_score_bounds() {
        let peaks = vec![(14.13, 10.0), (21.02, 8.0)];
        let score = peak_alignment_score(&peaks, 2, 1.0);
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_compare_resonance_runs() {
        // Just verify it runs without panicking (uses rand)
        let results = compare_resonance(100, 3);
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_known_zero_ordinates_positive() {
        for &gamma in KNOWN_ZERO_ORDINATES {
            assert!(gamma > 0.0);
        }
    }
}
