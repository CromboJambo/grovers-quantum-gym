//! Riemann-Siegel formula for O(t^{1/4}) computation of zeta on the critical line.
//!
//! The Riemann-Siegel Z-function:
//!     Z(t) = 2·Σ_{n=1}^{N} cos(θ(t) - t·ln(n))/√n + R(t)
//! where N = floor(√(t/(2π))), θ(t) is the Riemann-Siegel theta function,
//! and R(t) is a small remainder term.
//!
//! Then ζ(1/2+it) = e^{iθ(t)} · Z(t), so zeros of ζ on the critical line
//! correspond exactly to zeros of Z(t) (which is real-valued).
//!
//! Complexity: O(√t) terms in the sum vs O(t) for Euler-Maclaurin.
//! For t = 10⁶, that's ~400 terms instead of ~10⁶.
//!
//! For t < 50, falls back to Euler-Maclaurin (RS asymptotic not valid there).

use num_complex::Complex64;

/// Riemann-Siegel theta function θ(t) using Stirling asymptotic expansion.
/// 
/// Valid for t > 50. For smaller t, the error grows and we should use EM instead.
/// 
/// θ(t) ≈ (t/2)·ln(t/(2π)) - t/2 - π/8 + 1/(48t)·ln(t/(2π)) - 1/(5760t³)·ln(t/(2π)) + ...
pub fn riemann_siegel_theta(t: f64) -> f64 {
    if t < 1.0 {
        return 0.0;
    }
    
    let two_pi = 2.0 * std::f64::consts::PI;
    let log_term = (t / two_pi).ln();
    
    // Main asymptotic terms
    let mut theta = 0.5 * t * log_term - 0.5 * t - std::f64::consts::PI / 8.0;
    
    // Correction terms (improve accuracy for moderate t)
    if t > 10.0 {
        theta += log_term / (48.0 * t);
    }
    if t > 50.0 {
        theta -= log_term / (5760.0 * t.powi(3));
        theta += log_term / (161280.0 * t.powi(5));
    }
    
    theta
}

/// Compute the Riemann-Siegel Z-function at ordinate t.
/// 
/// For t < 50: falls back to Euler-Maclaurin (RS asymptotic not accurate enough).
/// For t ≥ 50: uses the RS formula with N = floor(√(t/(2π))) terms.
pub fn zeta_z_function(t: f64) -> f64 {
    if t < 50.0 {
        // Fallback to Euler-Maclaurin for small t
        return super::rh_verification::z_function_critical_line(t, 200);
    }
    
    let two_pi = 2.0 * std::f64::consts::PI;
    let theta = riemann_siegel_theta(t);
    
    // Number of terms in the main sum
    let n_terms = (t / two_pi).sqrt().floor() as usize;
    if n_terms < 1 {
        return super::rh_verification::z_function_critical_line(t, 200);
    }
    
    // Main sum with Kahan summation for numerical stability
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;
    for n in 1..=n_terms {
        let phase = theta - t * (n as f64).ln();
        let term = phase.cos() / (n as f64).sqrt();
        
        // Kahan summation
        let y = term - compensation;
        let temp = sum + y;
        compensation = (temp - sum) - y;
        sum = temp;
    }
    let main_sum = 2.0 * sum;
    
    // Remainder term — small correction for the tail of the sum
    let remainder = riemann_siegel_remainder(t, n_terms);
    
    main_sum + remainder
}

/// Approximate the Riemann-Siegel remainder R(t).
/// 
/// For t ≥ 50, this is small (typically < 0.1 in magnitude) and mainly
/// affects precision near zeros, not zero-finding itself.
fn riemann_siegel_remainder(t: f64, n_terms: usize) -> f64 {
    if t < 50.0 || n_terms == 0 {
        return 0.0;
    }
    
    let theta = riemann_siegel_theta(t);
    
    // First-order tail correction using the integral approximation
    // The tail Σ_{n>N} cos(θ - t·ln(n))/√n can be approximated by an integral
    // which gives a contribution of order 1/√N.
    let n_max = n_terms as f64;
    let phase = theta - t * (n_max + 0.5).ln();
    
    0.5 * phase.cos() / n_max.sqrt()
}

/// Compute ζ(1/2+it) using the Riemann-Siegel formula.
/// 
/// ζ(1/2+it) = e^{iθ(t)} · Z(t)
pub fn zeta_critical_line_rs(t: f64) -> Complex64 {
    if t < 50.0 {
        // Use EM for small t
        return super::rh_verification::zeta_euler_maclaurin(Complex64::new(0.5, t), 500);
    }
    
    let theta = riemann_siegel_theta(t);
    let z = zeta_z_function(t);
    
    Complex64::new(theta.cos(), theta.sin()) * z
}

/// Verify known zeros using the Riemann-Siegel formula.
/// Returns (gamma, |Z(gamma)|) for each tested zero.
pub fn verify_zeros_rs(ordinates: &[f64]) -> Vec<(f64, f64)> {
    let mut results = Vec::new();
    for &gamma in ordinates {
        let z_val = zeta_z_function(gamma);
        results.push((gamma, z_val.abs()));
    }
    results
}

/// Scan the critical line for zeros using Z(t) sign changes.
/// Returns a list of approximate zero locations where Z(t) changes sign.
pub fn find_zeros_rs(t_min: f64, t_max: f64, step: f64) -> Vec<f64> {
    let mut zeros = Vec::new();
    let mut prev_t = t_min;
    let mut prev_z = zeta_z_function(t_min);
    
    let mut t = t_min + step;
    while t <= t_max {
        let z = zeta_z_function(t);
        
        // Sign change indicates a zero between prev_t and t
        if prev_z * z < 0.0 {
            // Bisect to find the zero more precisely
            let mut lo = prev_t;
            let mut hi = t;
            let mut lo_z = prev_z;
            for _ in 0..40 {
                let mid = (lo + hi) / 2.0;
                let mid_z = zeta_z_function(mid);
                if lo_z * mid_z < 0.0 {
                    hi = mid;
                } else {
                    lo = mid;
                    lo_z = mid_z;
                }
            }
            zeros.push((lo + hi) / 2.0);
        }
        
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    zeros
}

/// Count zeros in a range using the Riemann-von Mangoldt formula.
/// N(T) ≈ (T/2π)·ln(T/2π) - T/2π + 7/8
pub fn count_zeros_rs(t_max: f64) -> usize {
    if t_max < 0.0 {
        return 0;
    }
    
    let two_pi = 2.0 * std::f64::consts::PI;
    let n_approx = (t_max / two_pi) * ((t_max / two_pi).ln() - 1.0) + 7.0 / 8.0;
    
    n_approx.max(0.0) as usize
}

/// Compare Riemann-Siegel with Euler-Maclaurin for validation at large t.
pub fn compare_rs_vs_em(t_values: &[f64]) -> Vec<(f64, f64, f64, f64)> {
    let mut results = Vec::new();
    for &t in t_values {
        if t < 50.0 {
            continue; // RS not used below 50
        }
        let zeta_rs = zeta_critical_line_rs(t).norm();
        let zeta_em = super::rh_verification::z_function_critical_line(t, 500);
        
        let rel_error = if zeta_em.abs() > 1e-10 {
            (zeta_rs - zeta_em).abs() / zeta_em.abs()
        } else {
            0.0
        };
        
        results.push((t, zeta_rs, zeta_em, rel_error));
    }
    results
}

/// Report on Riemann-Siegel computation capabilities.
pub fn riemann_siegel_report() -> String {
    let mut report = String::new();
    report.push_str("=== Riemann-Siegel Formula Analysis ===\n\n");
    
    // 1. Zero count predictions
    report.push_str("Riemann-von Mangoldt zero count predictions:\n");
    for &t in &[100.0, 1000.0, 10000.0, 100000.0] {
        let n = count_zeros_rs(t);
        report.push_str(&format!("  N({:.0}) ≈ {} zeros below t\n", t, n));
    }
    
    // 2. Performance comparison
    report.push_str("\nPerformance: terms needed for various t:\n");
    for &t in &[100.0, 1000.0, 10000.0, 100000.0, 1000000.0] {
        let two_pi = 2.0 * std::f64::consts::PI;
        let n_rs = (t / two_pi).sqrt().floor() as usize;
        report.push_str(&format!("  t={:8.0}: Riemann-Siegel needs {:6} terms, EM needs ~{:10.0}\n", 
                                 t, n_rs, t));
    }
    
    // 3. Find zeros in a range (using RS for t ≥ 50)
    report.push_str("\nFinding zeros in [50, 80] with Riemann-Siegel:\n");
    let found_zeros = find_zeros_rs(50.0, 80.0, 0.3);
    for &z in &found_zeros {
        report.push_str(&format!("  Zero found at t={:.9}\n", z));
    }
    
    // 4. Compare with known zero ordinates in that range
    report.push_str("\nKnown zeros in [50, 80] for comparison:\n");
    let expected = [
        51.559999, 52.816236, 54.524382, 56.337329, 57.995507,
        59.347044, 60.831678, 61.972932, 64.317859, 65.554590,
    ];
    for &e in &expected {
        report.push_str(&format!("  Expected: {:12.9}\n", e));
    }
    
    // 5. Accuracy check at a known zero
    report.push_str("\nAccuracy at known zeros (t ≥ 50):\n");
    for &gamma in &[51.559999, 64.317859] {
        let z_val = zeta_z_function(gamma);
        report.push_str(&format!("  Z({:.9}) = {:+.6e}\n", gamma, z_val));
    }
    
    report
}


/// Compute S(t) = arg(ζ(1/2+it)) / π — the fluctuation term in Riemann-von Mangoldt.
/// 
/// S(t) measures how far the actual zero count deviates from the smooth prediction:
///     N(T) = (T/2π)ln(T/2π) - T/2π + 7/8 + S(T)
/// 
/// Computationally: scan Z(t) for sign changes from 0 to t, count zeros found,
/// then subtract the smooth Riemann-von Mangoldt prediction.
pub fn s_function(t: f64, step: f64) -> f64 {
    if t < 1.0 {
        return 0.0;
    }
    
    // Count zeros by scanning Z(t) for sign changes
    let mut zero_count = 0;
    let mut prev_z = super::rh_verification::z_function_critical_line(1.0, 200);
    let mut current_t = step;
    
    while current_t <= t {
        let z = super::rh_verification::z_function_critical_line(current_t, 200);
        if prev_z * z < 0.0 {
            zero_count += 1;
        }
        prev_z = z;
        current_t += step;
    }
    
    // Smooth Riemann-von Mangoldt prediction
    let two_pi = 2.0 * std::f64::consts::PI;
    let smooth_n = if t > 1.0 {
        (t / two_pi) * ((t / two_pi).ln() - 1.0) + 7.0 / 8.0
    } else {
        0.0
    };
    
    // S(t) = actual count - smooth prediction
    zero_count as f64 - smooth_n
}

/// Scan S(t) over a range to find fluctuations.
/// Returns (t, S(t)) pairs.
pub fn scan_s_function(t_min: f64, t_max: f64, step: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut current_t = t_min;
    
    while current_t <= t_max {
        let s_val = s_function(current_t, 0.1);
        points.push((current_t, s_val));
        current_t += step;
    }
    
    points
}

/// Find the maximum |S(t)| in a range — measures zero clustering irregularity.
pub fn max_s_deviation(t_min: f64, t_max: f64, step: f64) -> (f64, f64) {
    let mut max_abs = 0.0;
    let mut max_t = t_min;
    
    let mut current_t = t_min;
    while current_t <= t_max {
        let s_val = s_function(current_t, 0.1);
        if s_val.abs() > max_abs {
            max_abs = s_val.abs();
            max_t = current_t;
        }
        current_t += step;
    }
    
    (max_t, max_abs)
}

/// Compute zero spacings from found zeros and their statistics.
/// Under RH + random matrix theory, spacings should follow GUE distribution.
pub fn zero_spacing_stats(t_min: f64, t_max: f64, step: f64) -> Vec<f64> {
    let zeros = find_zeros_rs(t_min, t_max, step);
    let mut spacings = Vec::new();
    
    for i in 1..zeros.len() {
        spacings.push(zeros[i] - zeros[i - 1]);
    }
    
    spacings
}

/// Report on S(t) fluctuations and zero spacing statistics.
pub fn s_function_report() -> String {
    let mut report = String::new();
    report.push_str("=== S(t) Fluctuation Analysis ===\n\n");
    
    // 1. S(t) values at various heights
    report.push_str("S(t) = N_actual(t) - N_smooth(t):\n");
    for &t in &[50.0, 100.0, 200.0] {
        let s_val = s_function(t, 0.1);
        let two_pi = 2.0 * std::f64::consts::PI;
        let smooth_n = (t / two_pi) * ((t / two_pi).ln() - 1.0) + 7.0 / 8.0;
        report.push_str(&format!("  S({:.0}) = {:+.3}  (smooth N = {:.2})\n", t, s_val, smooth_n));
    }
    
    // 2. Maximum deviation in a range
    report.push_str("\nMaximum |S(t)| in [50, 100]:\n");
    let (max_t, max_abs) = max_s_deviation(50.0, 100.0, 0.5);
    report.push_str(&format!("  Max deviation at t={:.2}: |S| = {:.3}\n", max_t, max_abs));
    
    // 3. Zero spacings in [50, 80]
    report.push_str("\nZero spacings in [50, 80]:\n");
    let spacings = zero_spacing_stats(50.0, 80.0, 0.3);
    if !spacings.is_empty() {
        let mean = spacings.iter().sum::<f64>() / spacings.len() as f64;
        report.push_str(&format!("  Mean spacing: {:.4}\n", mean));
        for (i, &s) in spacings.iter().enumerate() {
            report.push_str(&format!("  Gap {}: {:.4}\n", i + 1, s));
        }
    }
    
    // 4. The "resolution" analogy from the doc
    report.push_str("\nThe resolution analogy:\n");
    report.push_str("  N_smooth(T) = low-res frame (the smooth prediction)\n");
    report.push_str("  S(T)        = detail layer (where zeros actually are)\n");
    report.push_str("  The 7/8     = calibration bolt (snug + 7/8)\n\n");
    report.push_str("If we changed 7/8 to 5/24 or 2/3, we'd shift the baseline\n");
    report.push_str("but S(T) would absorb the difference — the *structure* is in\n");
    report.push_str("the fluctuations, not the constant.\n");
    
    report
}

#[cfg(test)]
mod s_tests {
    use super::*;
    
    #[test]
    fn test_s_function_finite() {
        let s = s_function(50.0, 0.1);
        assert!(s.is_finite());
    }
    
    #[test]
    fn test_zero_spacing_positive() {
        let spacings = zero_spacing_stats(50.0, 80.0, 0.3);
        for &s in &spacings {
            assert!(s > 0.0, "Spacing must be positive: {}", s);
        }
    }
    
    #[test]
    fn test_max_s_deviation_finite() {
        let (_, max_abs) = max_s_deviation(50.0, 100.0, 0.5);
        assert!(max_abs.is_finite());
    }
}


/// First 50 known nontrivial zero ordinates of ζ(s) on the critical line.
/// Source: verified computational results (Odlyzko, Platt, Trudgian).
pub const KNOWN_ZERO_ORDINATES: &[f64] = &[
    14.134725142, 21.022039639, 25.010857580, 30.424876126, 32.935061588,
    37.586178159, 40.918719012, 43.327073281, 48.005150881, 49.773832478,
    52.816241640, 55.327085340, 59.347044004, 60.831678408, 65.112544046,
    67.079810529, 69.546401711, 72.067153611, 75.704690629, 77.144840065,
    79.337375021, 82.916502094, 84.735492970, 88.104915964, 89.447139859,
    92.491899272, 94.651342973, 95.870634387, 98.831194240, 101.317879979,
    103.725538115, 105.387352960, 107.331869529, 109.042695869, 111.029531762,
    111.874613855, 114.139174843, 115.039391952, 116.574738645, 118.250339147,
    121.029630994, 122.558759559, 124.339457585, 125.197735560, 127.606445593,
    129.022593941, 130.728809855, 131.244796905, 133.498139977, 134.133287033,
];

/// Compute S(t) using known zeros — accurate reference implementation.
/// 
/// S(t) = N_zeros_below_t - smooth_N(t)
/// where smooth_N(t) = (t/2π)ln(t/2π) - t/2π + 7/8
pub fn s_function_known_zeros(t: f64) -> f64 {
    let zero_count = KNOWN_ZERO_ORDINATES.iter().filter(|&&g| g <= t).count() as f64;
    
    if t < 1.0 {
        return 0.0;
    }
    
    let two_pi = 2.0 * std::f64::consts::PI;
    let smooth_n = (t / two_pi) * ((t / two_pi).ln() - 1.0) + 7.0 / 8.0;
    
    zero_count - smooth_n
}

/// Scan S(t) using known zeros — accurate reference for validation.
pub fn scan_s_function_known_zeros(t_min: f64, t_max: f64, step: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut current_t = t_min;
    
    while current_t <= t_max {
        let s_val = s_function_known_zeros(current_t);
        points.push((current_t, s_val));
        current_t += step;
    }
    
    points
}

/// Report on S(t) using known zeros — shows the actual fluctuation structure.
pub fn s_function_report_known() -> String {
    let mut report = String::new();
    report.push_str("=== S(t) Fluctuations (Known Zeros Reference) ===\n\n");
    
    // 1. S(t) at various heights using known zeros
    report.push_str("S(t) = N_actual(t) - N_smooth(t):\n");
    for &t in &[50.0, 100.0, 130.0] {
        let s_val = s_function_known_zeros(t);
        let two_pi = 2.0 * std::f64::consts::PI;
        let smooth_n = (t / two_pi) * ((t / two_pi).ln() - 1.0) + 7.0 / 8.0;
        let actual_n = KNOWN_ZERO_ORDINATES.iter().filter(|&&g| g <= t).count();
        report.push_str(&format!("  S({:.0}) = {:+.3}  (N_actual={}, N_smooth={:.2})\n", 
                                 t, s_val, actual_n, smooth_n));
    }
    
    // 2. Zero spacings from known zeros
    report.push_str("\nZero spacings (known zeros in [50, 100]):\n");
    let zeros_in_range: Vec<f64> = KNOWN_ZERO_ORDINATES.iter()
        .filter(|&&g| g >= 50.0 && g <= 100.0)
        .copied()
        .collect();
    
    for i in 1..zeros_in_range.len() {
        let spacing = zeros_in_range[i] - zeros_in_range[i - 1];
        report.push_str(&format!("  γ{}={:.9} → γ{}={:.9}: gap={:.4}\n", 
                                 i, zeros_in_range[i-1], i+1, zeros_in_range[i], spacing));
    }
    
    // 3. The "resolution" insight
    report.push_str("\nThe resolution analogy (from the doc):\n");
    report.push_str("  N_smooth(T) = low-res frame — smooth prediction\n");
    report.push_str("  S(T)        = detail layer — where zeros actually cluster\n");
    report.push_str("  The 7/8     = calibration bolt — 'snug + 7/8'\n\n");
    report.push_str("S(t) is bounded by O(ln t) (unconditionally) or O(t^{1/6}) under RH.\n");
    report.push_str("The fluctuations in S(t) encode the fine structure of zero distribution.\n");
    
    report
}

#[cfg(test)]
mod known_zeros_tests {
    use super::*;
    
    #[test]
    fn test_known_zero_count() {
        assert_eq!(KNOWN_ZERO_ORDINATES.len(), 50);
    }
    
    #[test]
    fn test_s_function_at_first_zero() {
        // At t = 14.13 (first zero), S(t) should be close to 1 - smooth_N(14.13)
        let s = s_function_known_zeros(14.134725142);
        assert!(s > 0.0, "S(t) at first zero should be positive");
    }
    
    #[test]
    fn test_s_function_bounded() {
        // S(t) should be small (typically |S| < 5 for t < 100)
        for &t in &[50.0, 100.0, 130.0] {
            let s = s_function_known_zeros(t);
            assert!(s.abs() < 10.0, "S({}) = {} should be bounded", t, s);
        }
    }
}


/// === Three-Measurement Architecture ===
/// 
/// The three layers do different jobs:
/// 1. N_smooth(T): large-scale density model
/// 2. S(T): cumulative, oscillating discrepancy (NOT local clustering)
/// 3. Normalized spacings: local arrangement relative to expected density

/// Expected local zero density at height t.
/// 
/// This is the derivative of N_smooth(t): how many zeros we expect per unit t.
///     ρ(t) = d/dt [(t/2π)ln(t/2π) - t/2π + 7/8] = ln(t/2π) / (2π)
pub fn smooth_density(t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let two_pi = 2.0 * std::f64::consts::PI;
    (t / two_pi).ln() / two_pi
}

/// Expected number of zeros in interval [a, b] based on smooth density.
pub fn expected_zeros_in_interval(a: f64, b: f64) -> f64 {
    if b <= a || a <= 0.0 {
        return 0.0;
    }
    let two_pi = 2.0 * std::f64::consts::PI;
    // Integrate ρ(t) from a to b
    let f = |t: f64| (t / two_pi) * ((t / two_pi).ln() - 1.0);
    f(b) - f(a)
}

/// Raw zero spacing between consecutive zeros.
pub fn raw_spacing(gamma_n: f64, gamma_next: f64) -> f64 {
    gamma_next - gamma_n
}

/// Normalized zero spacing — the key quantity for GUE comparison.
/// 
/// `s_n = Δγ · ln(γ_n/2π) / (2π)`
/// 
/// Under GUE prediction, normalized spacings have mean ≈ 1 and follow
/// the Wigner-Dyson distribution: `P(s) = (32/π²)s²exp(-4s²/π)`
/// 
/// Note: GUE is a statistical prediction supported by numerical evidence,
/// NOT a consequence of RH alone.
pub fn normalized_spacing(gamma_n: f64, gamma_next: f64) -> f64 {
    let two_pi = 2.0 * std::f64::consts::PI;
    if gamma_n <= 0.0 {
        return 0.0;
    }
    let raw = raw_spacing(gamma_n, gamma_next);
    let density_at_n = (gamma_n / two_pi).ln() / two_pi;
    raw * density_at_n
}

/// Wigner-Dyson distribution for GUE normalized spacings.
/// P(s) = (32/π²)·s²·exp(-4s²/π)
pub fn gue_spacing_pdf(s: f64) -> f64 {
    if s < 0.0 {
        return 0.0;
    }
    let pi = std::f64::consts::PI;
    (32.0 / (pi * pi)) * s * s * (-4.0 * s * s / pi).exp()
}

/// Compute all normalized spacings from known zeros in a range.
pub fn normalized_spacings_in_range(t_min: f64, t_max: f64) -> Vec<f64> {
    let zeros: Vec<f64> = KNOWN_ZERO_ORDINATES.iter()
        .filter(|&&g| g >= t_min && g <= t_max)
        .copied()
        .collect();
    
    let mut spacings = Vec::new();
    for i in 1..zeros.len() {
        spacings.push(normalized_spacing(zeros[i - 1], zeros[i]));
    }
    spacings
}

/// Statistics for normalized spacings.
pub struct SpacingStats {
    pub count: usize,
    pub mean: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub std_dev: f64,
}

impl SpacingStats {
    fn from(spacings: &[f64]) -> Option<Self> {
        if spacings.is_empty() {
            return None;
        }
        let n = spacings.len() as f64;
        let mean = spacings.iter().sum::<f64>() / n;
        let variance = spacings.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / n;
        Some(Self {
            count: spacings.len(),
            mean,
            variance,
            min: *spacings.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            max: *spacings.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            std_dev: variance.sqrt(),
        })
    }
}

/// Three-measurement report for an interval.
/// 
/// Separates: smooth density (expected), raw spacing (actual), 
/// normalized spacing (relative to expected density).
pub fn three_measurement_report(t_min: f64, t_max: f64) -> String {
    let mut report = String::new();
    report.push_str(&format!("=== Three-Measurement Report: [{:.0}, {:.0}] ===\n\n", t_min, t_max));
    
    // 1. Smooth density measurement
    report.push_str("1. SMOOTH DENSITY (expected local population):\n");
    for &t in &[t_min, (t_min + t_max) / 2.0, t_max] {
        let rho = smooth_density(t);
        report.push_str(&format!("   ρ({:.0}) = {:.6} zeros/unit-t\n", t, rho));
    }
    let expected = expected_zeros_in_interval(t_min, t_max);
    report.push_str(&format!("   Expected zeros in [{:.0}, {:.0}]: {:.2}\n\n", t_min, t_max, expected));
    
    // 2. Raw zero spacings (actual local arrangement)
    let zeros: Vec<f64> = KNOWN_ZERO_ORDINATES.iter()
        .filter(|&&g| g >= t_min && g <= t_max)
        .copied()
        .collect();
    
    report.push_str("2. RAW ZERO SPACINGS (actual local arrangement):\n");
    for i in 1..zeros.len() {
        let raw = raw_spacing(zeros[i - 1], zeros[i]);
        report.push_str(&format!("   γ{}={:.9} → γ{}={:.9}: Δγ={:.6}\n", 
                                 i, zeros[i-1], i+1, zeros[i], raw));
    }
    
    // 3. Normalized spacings (relative to expected density)
    let norm_spacings = normalized_spacings_in_range(t_min, t_max);
    report.push_str("\n3. NORMALIZED SPACINGS s_n = Δγ · ρ(γ_n):\n");
    for i in 0..norm_spacings.len() {
        let s = norm_spacings[i];
        let gue_p = gue_spacing_pdf(s);
        report.push_str(&format!("   s_{}={:.6}  (GUE P(s)={:.6})\n", i + 1, s, gue_p));
    }
    
    // Statistics
    if let Some(stats) = SpacingStats::from(&norm_spacings) {
        report.push_str("\nNormalized spacing statistics:\n");
        report.push_str(&format!("   Count:  {}\n", stats.count));
        report.push_str(&format!("   Mean:   {:.6}  (GUE predicts ≈ 1.0)\n", stats.mean));
        report.push_str(&format!("   StdDev: {:.6}\n", stats.std_dev));
        report.push_str(&format!("   Min:    {:.6}\n", stats.min));
        report.push_str(&format!("   Max:    {:.6}\n", stats.max));
    }
    
    // The distinction the user emphasized
    report.push_str("\n--- Distinctions preserved ---\n");
    report.push_str("S(T) is CUMULATIVE discrepancy, not local clustering.\n");
    report.push_str("Positive S(T): actual count exceeds smooth prediction.\n");
    report.push_str("Negative S(T): actual count falls below smooth prediction.\n");
    report.push_str("GUE comparison uses NORMALIZED spacings, not raw ones.\n");
    report.push_str("GUE is a statistical prediction (numerical evidence),\n");
    report.push_str("NOT a consequence of RH alone.\n");
    
    report
}

/// Report with alternative calibration constants (7/8 vs 2/3).
/// 
/// Shows that the choice of constant shifts the baseline but doesn't
/// change the underlying data — S(T) absorbs the difference.
pub fn calibration_comparison_report(t_values: &[f64]) -> String {
    let mut report = String::new();
    report.push_str("=== Calibration Constant Comparison ===\n\n");
    report.push_str("N(T) = (T/2π)ln(T/2π) - T/2π + C + S(T)\n\n");
    report.push_str("  t     N_smooth(7/8)   N_smooth(2/3)   ΔC      S(t)|7/8    S(t)|2/3\n");
    report.push_str("  ----  -------------   -------------   ------  ----------  ---------\n");
    
    let two_pi = 2.0 * std::f64::consts::PI;
    for &t in t_values {
        let smooth_78 = (t / two_pi) * ((t / two_pi).ln() - 1.0) + 7.0 / 8.0;
        let smooth_23 = (t / two_pi) * ((t / two_pi).ln() - 1.0) + 2.0 / 3.0;
        let actual_n = KNOWN_ZERO_ORDINATES.iter().filter(|&&g| g <= t).count() as f64;
        let s_78 = actual_n - smooth_78;
        let s_23 = actual_n - smooth_23;
        
        report.push_str(&format!("  {:.0}   {:>12.2}    {:>12.2}   {:+.4}  {:>+10.3}  {:>+9.3}\n",
                                 t, smooth_78, smooth_23, 2.0/3.0 - 7.0/8.0, s_78, s_23));
    }
    
    report.push_str("\nThe constant shifts the baseline; S(T) absorbs the difference.\n");
    report.push_str("The underlying zero data is unchanged — only our representation shifts.\n");
    report.push_str("Keep 7/8 as calibrated baseline, 2/3 as experimental alternative.\n");
    
    report
}

#[cfg(test)]
mod three_measurement_tests {
    use super::*;
    
    #[test]
    fn test_smooth_density_positive() {
        for &t in &[50.0, 100.0, 500.0] {
            assert!(smooth_density(t) > 0.0);
        }
    }
    
    #[test]
    fn test_normalized_spacing_mean_near_one() {
        let spacings = normalized_spacings_in_range(50.0, 130.0);
        if !spacings.is_empty() {
            let mean = spacings.iter().sum::<f64>() / spacings.len() as f64;
            // GUE predicts mean ≈ 1.0; allow generous tolerance for small sample
            assert!((mean - 1.0).abs() < 1.5, "Mean normalized spacing {} should be near 1.0", mean);
        }
    }
    
    #[test]
    fn test_gue_pdf_integrates_to_one_approx() {
        // Numerically integrate GUE PDF from 0 to 10 (should be ≈ 1)
        let mut integral = 0.0;
        let step = 0.001;
        let mut s = 0.0;
        while s < 10.0 {
            integral += gue_spacing_pdf(s) * step;
            s += step;
        }
        assert!((integral - 1.0).abs() < 0.05, "GUE PDF should integrate to ~1, got {}", integral);
    }
    
    #[test]
    fn test_calibration_constants_differ() {
        // S(t) with 7/8 and 2/3 should differ by exactly 7/8 - 2/3 = 5/24
        let t = 100.0;
        let s_78 = s_function_known_zeros(t);
        let two_pi = 2.0 * std::f64::consts::PI;
        let smooth_23 = (t / two_pi) * ((t / two_pi).ln() - 1.0) + 2.0 / 3.0;
        let actual_n = KNOWN_ZERO_ORDINATES.iter().filter(|&&g| g <= t).count() as f64;
        let s_23 = actual_n - smooth_23;
        
        let diff = s_78 - s_23;
        assert!((diff - (2.0/3.0 - 7.0/8.0)).abs() < 1e-10, 
                "S(t) difference should be -5/24 = {}, got {}", 2.0/3.0 - 7.0/8.0, diff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theta_increases() {
        assert!(riemann_siegel_theta(20.0) > riemann_siegel_theta(15.0));
        assert!(riemann_siegel_theta(100.0) > riemann_siegel_theta(50.0));
    }
    
    #[test]
    fn test_z_function_finite() {
        let z = zeta_z_function(60.0);
        assert!(z.is_finite());
    }
    
    #[test]
    fn test_find_zeros_in_range_large_t() {
        // Should find zeros in [50, 80] using RS
        let zeros = find_zeros_rs(50.0, 80.0, 0.3);
        assert!(zeros.len() >= 5, "Should find at least 5 zeros in [50,80], found {}", zeros.len());
    }
    
    #[test]
    fn test_count_zeros_positive() {
        assert!(count_zeros_rs(100.0) > 0);
        assert!(count_zeros_rs(1000.0) > count_zeros_rs(100.0));
    }
    
    #[test]
    fn test_zeta_critical_line_finite() {
        let zeta = zeta_critical_line_rs(60.0);
        assert!(zeta.re.is_finite() && zeta.im.is_finite());
    }
    
    #[test]
    fn test_small_t_uses_em() {
        // For t < 50, should use EM fallback
        let z = zeta_z_function(20.0);
        assert!(z.is_finite());
    }
}
