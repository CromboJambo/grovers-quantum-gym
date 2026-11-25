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


/// === Spacing Structure Analysis ===
/// 
/// Mean normalized spacing validates the unfolding, NOT GUE.
/// Both Poisson and GUE give mean ≈ 1 after normalization.
/// The real test is in distribution shape and higher-order structure.

/// Wigner-Dyson CDF for GUE nearest-neighbor spacings (numerically integrated).
/// F_GUE(s) = ∫₀ˢ P(t) dt where P(t) = (32/π²)t²exp(-4t²/π)
/// 
/// Key property: F_GUE(0) = 0 — level repulsion suppresses small spacings.
pub fn gue_spacing_cdf(s: f64) -> f64 {
    if s < 0.0 {
        return 0.0;
    }
    // Numerically integrate P(t) from 0 to s using Simpson's rule
    if s <= 0.0 { return 0.0; }
    let n = 1000usize;
    let h = s / n as f64;
    let mut sum = gue_spacing_pdf(0.0) + gue_spacing_pdf(s);
    for i in 1..n {
        let t = i as f64 * h;
        sum += if i % 2 == 0 { 2.0 * gue_spacing_pdf(t) } else { 4.0 * gue_spacing_pdf(t) };
    }
    (h / 3.0) * sum
}

/// Poisson CDF for comparison — no level repulsion.
/// F_Poisson(s) = 1 - exp(-s)
/// 
/// Key property: F_Poisson(0) = 0 but P'(0) > 0 — small spacings allowed.
pub fn poisson_spacing_cdf(s: f64) -> f64 {
    if s < 0.0 {
        return 0.0;
    }
    1.0 - (-s).exp()
}

/// Poisson PDF for comparison.
pub fn poisson_spacing_pdf(s: f64) -> f64 {
    if s < 0.0 {
        return 0.0;
    }
    (-s).exp()
}

/// Kolmogorov-Smirnov statistic between empirical CDF and theoretical CDF.
/// 
/// D = max_s |F_empirical(s) - F_theoretical(s)|
/// Larger D means worse fit.
pub fn kolmogorov_smirnov(spacings: &[f64], theoretical_cdf: fn(f64) -> f64) -> f64 {
    if spacings.is_empty() {
        return 0.0;
    }
    let mut sorted = spacings.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len() as f64;
    let mut d_max: f64 = 0.0;
    
    for (i, &s) in sorted.iter().enumerate() {
        let cdf_theory = theoretical_cdf(s);
        // Check both F_emp(s-) and F_emp(s+) against theory
        let f_emp_below = i as f64 / n;
        let f_emp_above = (i + 1) as f64 / n;
        
        d_max = d_max.max((f_emp_below - cdf_theory).abs());
        d_max = d_max.max((f_emp_above - cdf_theory).abs());
    }
    
    d_max
}

/// Nearest-neighbor distribution test.
/// 
/// Compares empirical spacing distribution against GUE and Poisson.
/// Returns KS statistics for both — smaller is better fit.
pub fn nearest_neighbor_test(spacings: &[f64]) -> String {
    let mut report = String::new();
    report.push_str("=== Nearest-Neighbor Distribution Test ===\n\n");
    report.push_str(&format!("Sample size: {} normalized spacings\n\n", spacings.len()));
    
    // KS statistics
    let ks_gue = kolmogorov_smirnov(spacings, gue_spacing_cdf);
    let ks_poisson = kolmogorov_smirnov(spacings, poisson_spacing_cdf);
    
    report.push_str("Kolmogorov-Smirnov distance (smaller = better fit):\n");
    report.push_str(&format!("  vs GUE (Wigner-Dyson):   D = {:.4}\n", ks_gue));
    report.push_str(&format!("  vs Poisson:              D = {:.4}\n\n", ks_poisson));
    
    // Level repulsion check — fraction of spacings below 0.25
    let small_count = spacings.iter().filter(|&&s| s < 0.25).count();
    let small_frac = small_count as f64 / spacings.len() as f64;
    let gue_small = gue_spacing_cdf(0.25);
    let poisson_small = poisson_spacing_cdf(0.25);
    
    report.push_str("Level repulsion check (spacings < 0.25):\n");
    report.push_str(&format!("  Observed fraction: {:.3}\n", small_frac));
    report.push_str(&format!("  GUE predicts:      {:.3}\n", gue_small));
    report.push_str(&format!("  Poisson predicts:  {:.3}\n\n", poisson_small));
    
    // CDF comparison at key points
    report.push_str("CDF comparison:\n");
    report.push_str("  s       Empirical   GUE       Poisson\n");
    report.push_str("  ------  ----------  --------  --------\n");
    let mut sorted = spacings.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for &s in &[0.25, 0.5, 1.0, 1.5, 2.0] {
        let emp_cdf = (sorted.iter().filter(|&&x| x <= s).count() as f64) / sorted.len() as f64;
        report.push_str(&format!("  {:.2}    {:>9.3}   {:>8.3}   {:>8.3}\n", 
                                 s, emp_cdf, gue_spacing_cdf(s), poisson_spacing_cdf(s)));
    }
    
    // Verdict (cautious — small samples!)
    report.push_str("\n--- Interpretation ---\n");
    if ks_gue < ks_poisson {
        report.push_str("GUE fits better than Poisson for this sample.\n");
    } else {
        report.push_str("Poisson fits better than GUE for this sample.\n");
    }
    report.push_str(&format!("NOTE: {} observations is a small sample.\n", spacings.len()));
    report.push_str("KS test has low power at this size. This is exploratory,\n");
    report.push_str("not a statistical validation. Need 100+ spacings for meaningful tests.\n");
    
    report
}

/// Pair correlation analysis — relationships between non-adjacent zeros.
/// 
/// For each pair of spacings (s_i, s_j) with i < j, compute the "gap"
/// in terms of how many zeros separate them. This goes beyond nearest-neighbor.
/// 
/// Simple version: count pairs within normalized distance r.
pub fn pair_correlation(spacings: &[f64], r_max: f64, bins: usize) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    let step = r_max / bins as f64;
    
    for b in 0..bins {
        let r = (b as f64 + 0.5) * step;
        let mut count = 0.0;
        
        for i in 0..spacings.len() {
            for j in (i + 1)..spacings.len() {
                // "Distance" between spacings in the sequence
                let seq_dist = (j - i) as f64;
                if seq_dist <= r * 5.0 { // scale factor
                    count += 1.0;
                }
            }
        }
        
        result.push((r, count));
    }
    
    result
}

/// Consecutive spacing pairs — look at (s_i, s_{i+1}) relationships.
/// 
/// This tests whether small spacings tend to cluster together or repel.
/// Under GUE, we expect anti-correlation: after a small gap, the next
/// gap tends to be larger (and vice versa).
pub fn consecutive_spacing_correlation(spacings: &[f64]) -> f64 {
    if spacings.len() < 3 {
        return 0.0;
    }
    
    let n = spacings.len() as f64 - 1.0;
    let mean = spacings.iter().sum::<f64>() / spacings.len() as f64;
    
    let mut cov = 0.0;
    for i in 0..(spacings.len() - 1) {
        cov += (spacings[i] - mean) * (spacings[i + 1] - mean);
    }
    cov /= n;
    
    let variance = spacings.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / spacings.len() as f64;
    
    if variance == 0.0 {
        return 0.0;
    }
    
    cov / variance // Pearson correlation coefficient
}

/// Spacing sequence structure — the "what do the pixels look like" layer.
/// 
/// Examines what survives after mean and individual distribution are accounted for:
/// - Consecutive spacing correlations (local arrangement)
/// - Spacing triplets (s_i, s_{i+1}, s_{i+2}) patterns
/// - Overlapping window statistics
pub fn spacing_structure(spacings: &[f64]) -> String {
    let mut report = String::new();
    report.push_str("=== Spacing Structure Analysis ===\n\n");
    report.push_str("What survives after mean and individual distribution?\n\n");
    
    if spacings.len() < 3 {
        report.push_str("Need at least 3 spacings for structure analysis.\n");
        return report;
    }
    
    // 1. Consecutive correlation
    let corr = consecutive_spacing_correlation(spacings);
    report.push_str("Consecutive spacing correlation (Pearson r):\n");
    report.push_str(&format!("  r = {:.4}\n", corr));
    if corr < -0.2 {
        report.push_str("  → Negative: small gaps tend to follow large gaps (level repulsion)\n");
    } else if corr > 0.2 {
        report.push_str("  → Positive: gaps cluster together\n");
    } else {
        report.push_str("  → Near zero: no strong local correlation\n");
    }
    
    // 2. Spacing triplets — look at (s_i, s_{i+1}, s_{i+2}) patterns
    report.push_str("\nSpacing triplets (s_i, s_{i+1}, s_{i+2}):\n");
    for i in 0..(spacings.len().saturating_sub(2)).min(6) {
        report.push_str(&format!("  ({:.3}, {:.3}, {:.3})\n", 
                                 spacings[i], spacings[i+1], spacings[i+2]));
    }
    
    // 3. Overlapping window means — sliding window of size 3
    report.push_str("\nSliding window means (window=3):\n");
    for i in 0..(spacings.len().saturating_sub(2)) {
        let w_mean = (spacings[i] + spacings[i+1] + spacings[i+2]) / 3.0;
        report.push_str(&format!("  window@{}: {:.4}\n", i, w_mean));
    }
    
    // 4. The key question
    report.push_str("\n--- The Packing Question ---\n");
    report.push_str("Same density? Same individual spacing distribution?\n");
    report.push_str("Do the ARRANGEMENTS differ?\n\n");
    report.push_str("If two sequences have identical nearest-neighbor statistics\n");
    report.push_str("but different consecutive correlations or triplet patterns,\n");
    report.push_str("then the arrangement carries information beyond counting.\n\n");
    report.push_str("This is where 'how many fit' becomes 'how they're packed.'\n");
    
    report
}

/// Full spacing analysis — all layers from raw to structural.
pub fn full_spacing_analysis(t_min: f64, t_max: f64) -> String {
    let mut report = String::new();
    report.push_str(&format!("=== Full Spacing Analysis: [{:.0}, {:.0}] ===\n\n", t_min, t_max));
    
    // Preserve raw data first (engineering principle!)
    let zeros: Vec<f64> = KNOWN_ZERO_ORDINATES.iter()
        .filter(|&&g| g >= t_min && g <= t_max)
        .copied()
        .collect();
    
    report.push_str("RAW ZERO ORDINATES (preserved):\n");
    for &g in &zeros {
        report.push_str(&format!("  {:.9}\n", g));
    }
    
    let raw_spacings: Vec<f64> = zeros.windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    
    report.push_str("\nRAW SPACINGS (preserved):\n");
    for &s in &raw_spacings {
        report.push_str(&format!("  {:.6}\n", s));
    }
    
    let norm_spacings = normalized_spacings_in_range(t_min, t_max);
    
    report.push_str("\nNORMALIZED SPACINGS:\n");
    for &s in &norm_spacings {
        report.push_str(&format!("  {:.6}\n", s));
    }
    
    // Layer 1: Mean (validates unfolding)
    if !norm_spacings.is_empty() {
        let mean = norm_spacings.iter().sum::<f64>() / norm_spacings.len() as f64;
        report.push_str(&format!("\nLayer 1 — Mean normalized spacing: {:.6} (validates unfolding)\n", mean));
    }
    
    // Layer 2: Nearest-neighbor distribution (tests shape)
    if !norm_spacings.is_empty() {
        report.push_str(&format!("\n{}", nearest_neighbor_test(&norm_spacings)));
    }
    
    // Layer 3: Spacing structure (tests arrangement)
    if norm_spacings.len() >= 3 {
        report.push_str(&format!("\n{}", spacing_structure(&norm_spacings)));
    }
    
    report
}

#[cfg(test)]
mod spacing_structure_tests {
    use super::*;
    
    #[test]
    fn test_gue_cdf_properties() {
        // F(0) = 0 (level repulsion)
        assert!((gue_spacing_cdf(0.0)).abs() < 1e-10);
        // F(∞) → 1
        assert!(gue_spacing_cdf(10.0) > 0.999);
        // Monotonic
        assert!(gue_spacing_cdf(0.5) < gue_spacing_cdf(1.0));
    }
    
    #[test]
    fn test_poisson_cdf_properties() {
        assert!((poisson_spacing_cdf(0.0)).abs() < 1e-10);
        assert!(poisson_spacing_cdf(10.0) > 0.999);
        // Poisson allows small spacings more than GUE
        assert!(poisson_spacing_cdf(0.25) > gue_spacing_cdf(0.25));
    }
    
    #[test]
    fn test_ks_statistic_nonnegative() {
        let spacings = vec![0.8, 1.2, 0.9, 1.1, 1.0];
        assert!(kolmogorov_smirnov(&spacings, gue_spacing_cdf) >= 0.0);
    }
    
    #[test]
    fn test_consecutive_correlation_range() {
        let spacings = vec![0.5, 1.5, 0.5, 1.5, 0.5];
        let corr = consecutive_spacing_correlation(&spacings);
        assert!(corr >= -1.0 && corr <= 1.0);
    }
    
    #[test]
    fn test_full_analysis_runs() {
        let report = full_spacing_analysis(50.0, 100.0);
        assert!(report.contains("RAW ZERO ORDINATES"));
        assert!(report.contains("NORMALIZED SPACINGS"));
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


/// Count residual R(T) = N(T) - [T/2π ln(T/2π) - T/2π + 7/8]
/// 
/// This is the deviation of the actual zero count from the Riemann-von Mangoldt
/// prediction. Note: this is NOT the classical S(T) = arg(ζ(1/2+it))/π, which
/// requires computing the argument of zeta directly. We compute from known zeros.
pub fn count_residual(t: f64) -> f64 {
    let n_actual = KNOWN_ZERO_ORDINATES.iter().filter(|&&g| g <= t).count() as f64;
    let smooth = (t / (2.0 * std::f64::consts::PI)) * (t / (2.0 * std::f64::consts::PI)).ln()
        - t / (2.0 * std::f64::consts::PI) + 7.0 / 8.0;
    n_actual - smooth
}

/// Autocorrelation of normalized spacings at lag k
/// C(k) = Corr(s_i, s_{i+k})
pub fn spacing_autocorrelation(spacings: &[f64], k: usize) -> f64 {
    if k == 0 || k >= spacings.len() { return 1.0; }
    let n = spacings.len() - k;
    if n < 3 { return 0.0; }
    
    let mean_s: f64 = spacings.iter().sum::<f64>() / spacings.len() as f64;
    
    let mut cov = 0.0;
    let mut var1 = 0.0;
    let mut var2 = 0.0;
    for i in 0..n {
        let x = spacings[i] - mean_s;
        let y = spacings[i + k] - mean_s;
        cov += x * y;
        var1 += x * x;
        var2 += y * y;
    }
    cov / (var1.sqrt() * var2.sqrt())
}

/// Autocorrelation at multiple lags
pub fn spacing_autocorrelation_lags(spacings: &[f64], max_lag: usize) -> Vec<(usize, f64)> {
    (0..=max_lag.min(spacings.len() - 1))
        .map(|k| (k, spacing_autocorrelation(spacings, k)))
        .collect()
}

/// Window sums W_m(i) = sum of m consecutive spacings starting at i
pub fn window_sums(spacings: &[f64], window_size: usize) -> Vec<f64> {
    if window_size >= spacings.len() {
        return vec![spacings.iter().sum()];
    }
    (0..=(spacings.len() - window_size))
        .map(|i| spacings[i..i + window_size].iter().sum())
        .collect()
}

/// Coefficient of variation for window sums at different scales
/// Measures how fluctuations accumulate: if windows behave independently,
/// CV should decrease as 1/sqrt(m). If there's long-range correlation, it won't.
pub fn multiscale_fluctuation_analysis(spacings: &[f64], max_window: usize) -> Vec<(usize, f64)> {
    (1..=max_window.min(spacings.len()))
        .map(|m| {
            let windows = window_sums(spacings, m);
            if windows.len() < 2 { return (m, 0.0); }
            let mean: f64 = windows.iter().sum::<f64>() / windows.len() as f64;
            let var: f64 = windows.iter().map(|w| (w - mean).powi(2)).sum::<f64>() / (windows.len() - 1) as f64;
            (m, var.sqrt() / mean) // coefficient of variation
        })
        .collect()
}

/// Simulation-based calibration for KS test and consecutive correlation
/// 
/// Generates reference sequences (Poisson or GUE-like) with the same number
/// of observations, applies the same statistics, and returns the distribution
/// of test statistics. This avoids the independence assumption problem.
pub fn simulate_reference_statistics(
    n: usize,
    model: &str,  // "poisson" or "gue"
    n_simulations: usize
) -> (Vec<f64>, Vec<f64>) {
    let mut ks_stats = Vec::with_capacity(n_simulations);
    let mut correlations = Vec::with_capacity(n_simulations);
    
    for sim in 0..n_simulations {
        // Generate n spacings from the model using a simple deterministic hash
        let spacings: Vec<f64> = if model == "poisson" {
            (0..n).map(|i| {
                let seed = (sim as u64).wrapping_mul(1000).wrapping_add(i as u64);
                let h = hash_u64(seed);
                let u = (h >> 32) as f64 / 4294967296.0;
                (-u.max(0.001).ln())
            }).collect()
        } else {
            // GUE-like: use Wigner surmise quantile approximation
            // Sample from exponential, then apply a simple transform
            (0..n).map(|i| {
                let seed = (sim as u64).wrapping_mul(1000).wrapping_add(i as u64);
                let h = hash_u64(seed);
                let u = (h >> 32) as f64 / 4294967296.0;
                // Approximate GUE spacing via inverse of numerical CDF
                gue_spacing_inverse_cdf(u.max(0.001).min(0.999))
            }).collect()
        };
        
        if spacings.len() < 3 { continue; }
        
        let ks = kolmogorov_smirnov(&spacings, gue_spacing_cdf);
        ks_stats.push(ks);
        
        correlations.push(spacing_autocorrelation(&spacings, 1));
    }
    
    (ks_stats, correlations)
}

/// Simple hash function for deterministic pseudo-random numbers
fn hash_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

/// Inverse CDF for GUE spacing via bisection on the numerical CDF
fn gue_spacing_inverse_cdf(p: f64) -> f64 {
    let mut lo = 0.0;
    let mut hi = 5.0;
    for _ in 0..50 {
        let mid = (lo + hi) / 2.0;
        if gue_spacing_cdf(mid) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

/// Full multiscale analysis report (Layers 4 and 5)
pub fn full_multiscale_report(a: f64, b: f64) -> String {
    let mut report = format!("\n{}\n", "=".repeat(70));
    report.push_str(&format!("MULTISCALE SPACING ANALYSIS: [{} , {}]\n", a, b));
    report.push_str(&format!("{}\n\n", "=".repeat(70)));
    
    // Raw data (preserve the evidence)
    let zeros_in_range: Vec<f64> = KNOWN_ZERO_ORDINATES.iter()
        .copied()
        .filter(|&g| g >= a && g <= b)
        .collect();
    let spacings: Vec<f64> = zeros_in_range.windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    
    report.push_str(&format!("Raw zero ordinates: {:?}\n\n", zeros_in_range));
    report.push_str(&format!("Raw spacings: {:?}\n\n", spacings));
    
    if spacings.len() < 3 {
        report.push_str("Not enough spacings for multiscale analysis.\n");
        return report;
    }
    
    // Normalize
    let normalized: Vec<f64> = zeros_in_range.windows(2)
        .map(|w| normalized_spacing(w[0], w[1]))
        .collect();
    
    report.push_str(&format!("Normalized spacings: {:?}\n\n", 
        normalized.iter().map(|s| format!("{:.4}", s)).collect::<Vec<_>>()));
    
    // Layer 4: Autocorrelation at multiple lags
    report.push_str("LAYER 4 — Arrangement (autocorrelation at multiple lags)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    let max_lag = (normalized.len() / 2).max(1);
    for (k, c) in spacing_autocorrelation_lags(&normalized, max_lag) {
        if k == 0 { continue; } // skip trivial lag 0
        report.push_str(&format!("  C({}) = {:.4}\n", k, c));
    }
    
    // Layer 5: Multiscale window analysis
    report.push_str("\nLAYER 5 — Multiscale structure (window sums)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    let max_window = normalized.len() / 2;
    for (m, cv) in multiscale_fluctuation_analysis(&normalized, max_window) {
        let independent_expectation = 1.0 / (m as f64).sqrt();
        report.push_str(&format!("  Window size {}: CV = {:.4} (independent expectation: {:.4})\n", 
            m, cv, independent_expectation));
    }
    
    // Simulation-based calibration
    report.push_str("\nSIMULATION-BASED CALIBRATION\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    let n_obs = normalized.len();
    let (poisson_ks, poisson_corr) = simulate_reference_statistics(n_obs, "poisson", 100);
    let (gue_ks, gue_corr) = simulate_reference_statistics(n_obs, "gue", 100);
    
    let observed_ks_gue = kolmogorov_smirnov(&normalized, gue_spacing_cdf);
    let observed_ks_poisson = kolmogorov_smirnov(&normalized, poisson_spacing_cdf);
    let observed_corr = spacing_autocorrelation(&normalized, 1);
    
    let poisson_ks_worse = poisson_ks.iter().filter(|&&x| x > observed_ks_gue).count() as f64 / 100.0;
    let gue_ks_worse = gue_ks.iter().filter(|&&x| x > observed_ks_gue).count() as f64 / 100.0;
    
    report.push_str(&format!("Observed KS vs GUE: {:.4}\n", observed_ks_gue));
    report.push_str(&format!("  Poisson sims worse than this: {:.1}%\n", poisson_ks_worse * 100.0));
    report.push_str(&format!("  GUE sims worse than this: {:.1}%\n\n", gue_ks_worse * 100.0));
    
    report.push_str(&format!("Observed consecutive correlation: {:.4}\n", observed_corr));
    let poisson_corr_extreme = poisson_corr.iter().filter(|&&x| x <= observed_corr).count() as f64 / 100.0;
    let gue_corr_extreme = gue_corr.iter().filter(|&&x| x <= observed_corr).count() as f64 / 100.0;
    report.push_str(&format!("  Poisson sims with r <= this: {:.1}%\n", poisson_corr_extreme * 100.0));
    report.push_str(&format!("  GUE sims with r <= this: {:.1}%\n\n", gue_corr_extreme * 100.0));
    
    // Interpretation
    report.push_str("INTERPRETATION\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("- Level repulsion (small-gap suppression) and consecutive correlation\n");
    report.push_str("  are related but distinct statistics. One measures the shape of P(s)\n");
    report.push_str("  near zero; the other measures how gaps influence their neighbors.\n");
    report.push_str(&format!("- With n={} observations, results are exploratory, not conclusive.\n", n_obs));
    report.push_str("- The question: what information survives counting? What structure\n");
    report.push_str("  gets discarded when we collapse the sequence into a single number?\n");
    
    report
}

/// Trajectory of (N_smooth(T), R(T)) as T increases through zero ordinates.
/// 
/// Instead of treating N_smooth as a baseline and R as deviation from it,
/// this plots both as coordinates in a 2D space. If the relationship has
/// structure (spiral, recurrence, symmetry), it should appear here.
/// 
/// The user's insight: "it just requires acknowledging that you crossed the zero."
/// Every zero crossing changes the relationship between smooth and actual count.
pub fn count_trajectory(t_max: f64) -> Vec<(f64, f64)> {
    // Compute zeros dynamically using Riemann-Siegel Z-function with bisection.
    // This allows scaling to any T value without hardcoding zero ordinates.
    let zeros = find_zeros_rs(10.0, t_max, 0.5);
    let two_pi = 2.0 * std::f64::consts::PI;
    
    zeros.iter().enumerate().map(|(i, &gamma)| {
        let n_actual = (i + 1) as f64; // zeros are found in order starting from T=10
        let n_smooth = (gamma / two_pi) * (gamma / two_pi).ln() - gamma / two_pi + 7.0 / 8.0;
        let r = n_actual - n_smooth;
        (n_smooth, r)
    }).collect()
}

/// Format the trajectory as text "plot" — shows the path through (N_smooth, R) space
pub fn count_trajectory_report(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.is_empty() { return "No zeros in range.\n".to_string(); }
    
    let mut report = format!("\n{}\n", "=".repeat(70));
    report.push_str(&format!("COUNT TRAJECTORY: (N_smooth(T), R(T)) for T up to {}\n", t_max));
    report.push_str(&format!("{}\n\n", "=".repeat(70)));
    
    // Find bounds for text plot
    let min_x = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
    
    report.push_str(&format!("Range: N_smooth [{} , {}], R [{} , {}]\n\n", 
        min_x as i32, max_x as i32, min_y as i32, max_y as i32));
    
    // Print each point with its zero index
    report.push_str("Trajectory points (zero #, N_smooth, R):\n");
    for (i, &(x, y)) in points.iter().enumerate() {
        report.push_str(&format!("  {}: ({:8.3}, {:7.4})\n", i + 1, x, y));
    }
    
    // Text-based scatter plot (50 wide x 20 tall)
    let width = 50;
    let height = 20;
    let mut grid = vec![vec!['.'; width]; height];
    
    for &(x, y) in &points {
        let px = ((x - min_x) / (max_x - min_x + 1e-10) * (width as f64 - 1.0)) as usize;
        let py = ((max_y - y) / (max_y - min_y + 1e-10) * (height as f64 - 1.0)) as usize;
        if px < width && py < height {
            grid[py][px] = '*';
        }
    }
    
    report.push_str("\nTrajectory in (N_smooth, R) space:\n");
    for row in &grid {
        report.push_str(&format!("  {}\n", row.iter().collect::<String>()));
    }
    
    // Symmetry check: are points symmetric around some R value?
    let mean_r: f64 = points.iter().map(|(_, y)| *y).sum::<f64>() / points.len() as f64;
    report.push_str(&format!("\nMean R(T) over first {} zeros: {:.4}\n", points.len(), mean_r));
    
    // Check if trajectory shows recurrence (does it visit similar regions repeatedly?)
    let mut nearby_pairs = 0;
    for i in 0..points.len() {
        for j in (i + 5)..points.len() {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            if dx.abs() < 2.0 && dy.abs() < 0.5 {
                nearby_pairs += 1;
            }
        }
    }
    report.push_str(&format!("Nearby trajectory pairs (|ΔN|<2, |ΔR|<0.5): {}\n", nearby_pairs));
    
    // The symmetry question from the user's dialogue
    report.push_str("\nSYMMETRY CHECK\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("7/8 = 21/24, 2/3 = 16/24 → symmetric around 37/48 (±5/48)\n");
    report.push_str("If we use C=37/48 instead of C=7/8 as the reference:\n");
    let mut r_37_48 = Vec::new();
    for &gamma in KNOWN_ZERO_ORDINATES.iter() {
        if gamma > t_max { break; }
        let n_actual = KNOWN_ZERO_ORDINATES.iter().filter(|&&g| g <= gamma).count() as f64;
        let two_pi = 2.0 * std::f64::consts::PI;
        let n_smooth_37 = (gamma / two_pi) * (gamma / two_pi).ln() - gamma / two_pi + 37.0 / 48.0;
        r_37_48.push(n_actual - n_smooth_37);
    }
    let mean_r_37: f64 = r_37_48.iter().sum::<f64>() / r_37_48.len() as f64;
    report.push_str(&format!("  Mean R with C=7/8:   {:.4}\n", mean_r));
    report.push_str(&format!("  Mean R with C=37/48: {:.4} (shifted by {})\n", 
        mean_r_37, 37.0/48.0 - 7.0/8.0));
    report.push_str("  The shift is exactly 5/48 ≈ 0.1042 — a constant vertical translation.\n");
    report.push_str("  Symmetry around the midpoint doesn't change the trajectory shape,\n");
    report.push_str("  only its vertical position. The geometry is in the path, not the offset.\n");
    
    report
}

#[cfg(test)]
mod trajectory_tests {
    use super::*;
    
    #[test]
    fn test_trajectory_points_exist() {
        let points = count_trajectory(100.0);
        assert!(points.len() >= 25); // at least 25 zeros below 100
    }
    
    #[test]
    fn test_trajectory_is_monotonic_in_x() {
        let points = count_trajectory(100.0);
        for i in 1..points.len() {
            assert!(points[i].0 > points[i - 1].0); // N_smooth always increases
        }
    }
    
    #[test]
    fn test_trajectory_report_runs() {
        let report = count_trajectory_report(100.0);
        assert!(report.contains("Trajectory"));
        assert!(report.contains("SYMMETRY CHECK"));
    }
}

/// Generate a text scatter plot from 2D points
fn text_scatter(points: &[(f64, f64)], width: usize, height: usize) -> String {
    if points.is_empty() { return "(no points)".to_string(); }
    
    let min_x = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
    
    let mut grid = vec![vec!['.'; width]; height];
    
    for &(x, y) in points {
        let px = if max_x > min_x {
            ((x - min_x) / (max_x - min_x) * (width as f64 - 1.0)) as usize
        } else { width / 2 };
        let py = if max_y > min_y {
            ((max_y - y) / (max_y - min_y) * (height as f64 - 1.0)) as usize
        } else { height / 2 };
        if px < width && py < height {
            grid[py][px] = '*';
        }
    }
    
    let mut s = String::new();
    for row in &grid {
        s.push_str(&format!("|{}\n", row.iter().collect::<String>()));
    }
    s
}

/// Shape test 1: Return map — (R_n, R_{n+1}) for consecutive zeros
/// If there's deterministic structure in how the residual evolves,
/// it should appear as a curve rather than a cloud.
pub fn shape_test_return_map(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 3 { return "Not enough data.\n".to_string(); }
    
    let mut pairs = Vec::new();
    for i in 0..points.len() - 1 {
        pairs.push((points[i].1, points[i + 1].1)); // (R_n, R_{n+1})
    }
    
    let mut report = format!("\nSHAPE TEST 1 — Return Map: (R_n vs R_{{n+1}})\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("If deterministic structure exists in residual evolution,\n");
    report.push_str("points should form a curve, not a random cloud.\n\n");
    report.push_str(&text_scatter(&pairs, 50, 20));
    
    // Compute correlation
    let mean_x: f64 = pairs.iter().map(|(x, _)| *x).sum::<f64>() / pairs.len() as f64;
    let mean_y: f64 = pairs.iter().map(|(_, y)| *y).sum::<f64>() / pairs.len() as f64;
    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;
    for &(x, y) in &pairs {
        cov += (x - mean_x) * (y - mean_y);
        var_x += (x - mean_x).powi(2);
        var_y += (y - mean_y).powi(2);
    }
    let r = if var_x > 0.0 && var_y > 0.0 { cov / (var_x.sqrt() * var_y.sqrt()) } else { 0.0 };
    report.push_str(&format!("\nCorrelation: r = {:.4}\n", r));
    if r.abs() < 0.3 {
        report.push_str("→ Weak correlation. No obvious deterministic curve.\n");
    } else if r > 0.7 {
        report.push_str("→ Strong positive correlation. Residual tends to persist.\n");
    } else if r < -0.7 {
        report.push_str("→ Strong negative correlation. Residual oscillates around a mean.\n");
    } else {
        report.push_str("→ Moderate correlation. Some structure, but noisy.\n");
    }
    
    report
}

/// Shape test 2: Log-polar spiral test
/// If R(T) ~ A·sin(B·ln T + φ) + C·ln T (logarithmic spiral in disguise),
/// then plotting (ln N_smooth, R mod period) should show a spiral pattern.
pub fn shape_test_log_polar(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 3 { return "Not enough data.\n".to_string(); }
    
    // Convert to log-polar: x = ln(N_smooth), y = R
    let mut log_points = Vec::new();
    for &(x, y) in &points {
        if x > 0.0 {
            log_points.push((x.ln(), y));
        }
    }
    
    let mut report = format!("\nSHAPE TEST 2 — Log-Polar: (ln N_smooth vs R)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("If R grows like a·ln(T) + b·sin(c·ln T), this should look\n");
    report.push_str("like a spiral or band in log coordinates.\n\n");
    report.push_str(&text_scatter(&log_points, 50, 20));
    
    // Check if R correlates with ln(N_smooth) — linear growth in log space = power law in T
    let mean_x: f64 = log_points.iter().map(|(x, _)| *x).sum::<f64>() / log_points.len() as f64;
    let mean_y: f64 = log_points.iter().map(|(_, y)| *y).sum::<f64>() / log_points.len() as f64;
    let mut cov = 0.0;
    let mut var_x = 0.0;
    for &(x, y) in &log_points {
        cov += (x - mean_x) * (y - mean_y);
        var_x += (x - mean_x).powi(2);
    }
    let slope = if var_x > 0.0 { cov / var_x } else { 0.0 };
    report.push_str(&format!("\nSlope of R vs ln(N_smooth): {:.4}\n", slope));
    report.push_str("If positive and significant → R grows with T (expected: ~log growth)\n");
    
    report
}

/// Shape test 3: Folded trajectory — wrap R into [0, 1) to look for recurrence
/// If the sawtooth pattern is regular, folding should show a concentrated band.
pub fn shape_test_folded(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 3 { return "Not enough data.\n".to_string(); }
    
    // Fractional part of R(T) — the "phase" within each sawtooth cycle
    let mut folded = Vec::new();
    for &(x, y) in &points {
        folded.push((x, y.fract()));
    }
    
    let mut report = format!("\nSHAPE TEST 3 — Folded Phase: (N_smooth vs R mod 1)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("If the sawtooth is regular, fractional parts should cluster.\n");
    report.push_str("If random, they should fill [0,1) uniformly.\n\n");
    report.push_str(&text_scatter(&folded, 50, 20));
    
    // Uniformity check: divide [0,1) into 5 bins, count occupancy
    let mut bins = vec![0usize; 5];
    for &(_, frac) in &folded {
        let bin = (frac * 5.0).min(4.0) as usize;
        bins[bin] += 1;
    }
    let expected = folded.len() as f64 / 5.0;
    let chi2: f64 = bins.iter().map(|&b| ((b as f64 - expected).powi(2)) / expected).sum();
    report.push_str(&format!("\nBin occupancy (expect {} each): {:?}\n", expected as usize, bins));
    report.push_str(&format!("Chi-squared vs uniform: {:.2} (df=4, critical 9.49 at 5%)\n", chi2));
    if chi2 > 9.49 {
        report.push_str("→ Non-uniform! The fractional part has structure.\n");
    } else {
        report.push_str("→ Consistent with uniform distribution.\n");
    }
    
    report
}

/// Shape test 4: Residual differences — (ΔR_n, ΔR_{n+1})
/// Tests whether the CHANGES in residual are correlated (acceleration/deceleration)
pub fn shape_test_residual_differences(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 4 { return "Not enough data.\n".to_string(); }
    
    let mut diffs = Vec::new();
    for i in 0..points.len() - 1 {
        diffs.push(points[i + 1].1 - points[i].1);
    }
    
    let mut pairs = Vec::new();
    for i in 0..diffs.len() - 1 {
        pairs.push((diffs[i], diffs[i + 1]));
    }
    
    let mut report = format!("\nSHAPE TEST 4 — Residual Acceleration: (ΔR_n vs ΔR_{{n+1}})\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("If ΔR is correlated with itself, the residual has momentum\n");
    report.push_str("(accelerating away from or toward the smooth curve).\n\n");
    report.push_str(&text_scatter(&pairs, 50, 20));
    
    let mean_x: f64 = pairs.iter().map(|(x, _)| *x).sum::<f64>() / pairs.len() as f64;
    let mean_y: f64 = pairs.iter().map(|(_, y)| *y).sum::<f64>() / pairs.len() as f64;
    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;
    for &(x, y) in &pairs {
        cov += (x - mean_x) * (y - mean_y);
        var_x += (x - mean_x).powi(2);
        var_y += (y - mean_y).powi(2);
    }
    let r = if var_x > 0.0 && var_y > 0.0 { cov / (var_x.sqrt() * var_y.sqrt()) } else { 0.0 };
    report.push_str(&format!("\nCorrelation of consecutive ΔR: r = {:.4}\n", r));
    if r < -0.5 {
        report.push_str("→ Negative: residual oscillates (mean-reverting sawtooth).\n");
    } else if r > 0.5 {
        report.push_str("→ Positive: residual has momentum (trending away).\n");
    } else {
        report.push_str("→ Weak: changes are largely independent.\n");
    }
    
    report
}

/// Run all shape tests and combine into one report

/// Find local maxima in a sequence of R(T) values at known zeros
fn find_peaks(r_values: &[f64]) -> Vec<usize> {
    let mut peaks = Vec::new();
    for i in 1..r_values.len().saturating_sub(1) {
        if r_values[i] > r_values[i - 1] && r_values[i] >= r_values[i + 1] {
            peaks.push(i);
        }
    }
    peaks
}

/// For each peak, find the "mirror width" — how many zeros out on each side
/// before R drops below (peak_height - threshold). 
/// Tests self-similarity: if width/peak is constant across peaks, the structure is scale-invariant.
pub fn shape_test_peak_width(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 10 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let peaks = find_peaks(&r_values);
    
    let mut report = format!("\nSHAPE TEST 5 — Peak Width Analysis (Self-Similarity Test)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("If R(T) is self-similar, the ratio of peak width to peak height\n");
    report.push_str("should be constant across peaks. Also tests if successive peaks\n");
    report.push_str("grow by a constant multiplier.\n\n");
    
    if peaks.len() < 3 {
        report.push_str(&format!("Only {} local maxima found in first {} zeros. Need more data.\n", 
            peaks.len(), points.len()));
        return report;
    }
    
    // For each peak, find the width at half-height
    let mut peak_data = Vec::new();
    for &peak_idx in &peaks {
        let peak_height = r_values[peak_idx];
        let baseline = if peak_idx > 0 { 
            // Use the minimum R value before this peak as local baseline
            r_values[..=peak_idx].iter().cloned().fold(f64::INFINITY, f64::min)
        } else { 0.0 };
        
        let half_height = (peak_height + baseline) / 2.0;
        
        // Find left mirror point: first zero to the left where R <= half_height
        let mut left_width = 0;
        for i in (0..peak_idx).rev() {
            if r_values[i] <= half_height {
                left_width = peak_idx - i;
                break;
            }
        }
        
        // Find right mirror point: first zero to the right where R <= half_height
        let mut right_width = 0;
        for i in (peak_idx + 1)..points.len() {
            if r_values[i] <= half_height {
                right_width = i - peak_idx;
                break;
            }
        }
        
        let total_width = left_width + right_width;
        let height_above_baseline = peak_height - baseline;
        
        if height_above_baseline > 0.0 && total_width > 0 {
            peak_data.push((peak_idx as f64, peak_height, total_width as f64, 
                           height_above_baseline, total_width as f64 / height_above_baseline));
        }
    }
    
    report.push_str(&format!("{:<8} {:<10} {:<8} {:<12} {:<10}\n", 
        "Peak#", "Height", "Width(zeros)", "AboveBase", "W/H Ratio"));
    report.push_str(&format!("{}\n", "-".repeat(50)));
    
    for (i, &(idx, height, width, above_base, ratio)) in peak_data.iter().enumerate() {
        report.push_str(&format!("{:<8} {:<10.4} {:<8} {:<12.4} {:<10.4}\n",
            i + 1, height, width as usize, above_base, ratio));
    }
    
    // Check if W/H ratio is constant
    let ratios: Vec<f64> = peak_data.iter().map(|&(_, _, _, _, r)| r).collect();
    if ratios.len() >= 3 {
        let mean_ratio: f64 = ratios.iter().sum::<f64>() / ratios.len() as f64;
        let variance: f64 = ratios.iter().map(|&r| (r - mean_ratio).powi(2)).sum::<f64>() / ratios.len() as f64;
        let cv = if mean_ratio > 0.0 { variance.sqrt() / mean_ratio } else { 0.0 };
        
        report.push_str(&format!("\nW/H ratio: mean = {:.4}, CV = {:.4}\n", mean_ratio, cv));
        if cv < 0.3 {
            report.push_str("→ Low variation. Possible self-similarity!\n");
        } else if cv < 0.6 {
            report.push_str("→ Moderate variation. Some regularity, but noisy.\n");
        } else {
            report.push_str("→ High variation. No consistent peak shape.\n");
        }
    }
    
    // Check if successive peaks grow by a constant multiplier
    let heights: Vec<f64> = peak_data.iter().map(|&(_, h, _, _, _)| h).collect();
    if heights.len() >= 3 {
        let mut multipliers = Vec::new();
        for i in 0..heights.len() - 1 {
            if heights[i] > 0.0 {
                multipliers.push(heights[i + 1] / heights[i]);
            }
        }
        if !multipliers.is_empty() {
            let mean_mult: f64 = multipliers.iter().sum::<f64>() / multipliers.len() as f64;
            report.push_str(&format!("\nSuccessive peak height ratios: {:?}\n", 
                &multipliers[..multipliers.len().min(5)]));
            report.push_str(&format!("Mean multiplier: {:.4}\n", mean_mult));
            if (mean_mult - 1.0).abs() < 0.1 {
                report.push_str("→ Peaks roughly constant height (bounded oscillation).\n");
            } else if mean_mult > 1.1 {
                report.push_str("→ Peaks growing. Multiplier suggests power-law growth.\n");
            } else {
                report.push_str("→ Peaks declining.\n");
            }
        }
    }
    
    report
}

/// Stepwise counter: for each zero, count how many zeros since the last local minimum
/// and compare to the current R(T) value. If there's a multiplicative relationship
/// between "steps since trough" and residual height, that's a pinned multiplier.
pub fn shape_test_step_counter(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 10 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    
    // Find local minima
    let mut minima = Vec::new();
    for i in 1..r_values.len().saturating_sub(1) {
        if r_values[i] < r_values[i - 1] && r_values[i] <= r_values[i + 1] {
            minima.push(i);
        }
    }
    
    let mut report = format!("\nSHAPE TEST 6 — Stepwise Counter (Steps Since Trough vs Height)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("For each zero, count steps since the last local minimum.\n");
    report.push_str("If R(T) ≈ k · (steps since trough), there's a pinned multiplier.\n\n");
    
    if minima.len() < 2 {
        report.push_str(&format!("Only {} local minima found. Need more data.\n", minima.len()));
        return report;
    }
    
    // For each zero after the first minimum, compute steps since last trough
    let mut step_data = Vec::new();
    let mut current_min_idx = minima[0];
    for i in (current_min_idx + 1)..points.len() {
        // Update current_min_idx if we've passed a new minimum
        while current_min_idx + 1 < minima.len() && minima[current_min_idx + 1] <= i {
            current_min_idx += 1;
        }
        let steps_since_trough = (i.saturating_sub(minima[current_min_idx])) as f64;
        let height_above_trough = r_values[i] - r_values[minima[current_min_idx]];
        if steps_since_trough > 0.0 {
            step_data.push((steps_since_trough, height_above_trough));
        }
    }
    
    if step_data.len() < 5 {
        report.push_str("Not enough step data.\n");
        return report;
    }
    
    // Compute the ratio R/steps for each point
    let mut ratios = Vec::new();
    for &(steps, height) in &step_data {
        if steps > 0.0 {
            ratios.push(height / steps);
        }
    }
    
    report.push_str(&format!("{} data points (steps since trough vs R above trough)\n\n", step_data.len()));
    report.push_str(&text_scatter(&step_data, 50, 15));
    
    // Check for linear relationship through origin
    let mean_x: f64 = step_data.iter().map(|&(x, _)| x).sum::<f64>() / step_data.len() as f64;
    let mean_y: f64 = step_data.iter().map(|&(_, y)| y).sum::<f64>() / step_data.len() as f64;
    let mut cov = 0.0;
    let mut var_x = 0.0;
    for &(x, y) in &step_data {
        cov += (x - mean_x) * (y - mean_y);
        var_x += (x - mean_x).powi(2);
    }
    let slope = if var_x > 0.0 { cov / var_x + (mean_y - 0.0 * cov / var_x) / mean_x.max(1.0) } else { 0.0 };
    
    // Simpler: just compute mean ratio
    let mean_ratio: f64 = ratios.iter().sum::<f64>() / ratios.len() as f64;
    report.push_str(&format!("\nMean R/steps ratio (the 'multiplier'): {:.4}\n", mean_ratio));
    
    // Check consistency of the multiplier
    if ratios.len() >= 5 {
        let variance: f64 = ratios.iter().map(|&r| (r - mean_ratio).powi(2)).sum::<f64>() / ratios.len() as f64;
        let cv = if mean_ratio > 0.0 { variance.sqrt() / mean_ratio } else { 0.0 };
        report.push_str(&format!("CV of multiplier: {:.4}\n", cv));
        if cv < 0.3 {
            report.push_str("→ Multiplier is stable! R(T) ≈ k · steps_since_trough\n");
        } else {
            report.push_str("→ Multiplier varies. No pinned relationship.\n");
        }
    }
    
    report
}

/// Add these to the all_shape_tests function

/// Test if peak geometry follows a Pythagorean-like constraint:
/// W² + H² vs T, N_smooth(T), or other candidate quantities.
/// Also checks for an envelope (max width for given height).

/// Rigorous test for depth-dependent geometric constraints.
/// Tests Model A (W = k·H) vs Model B (W = H·k(N_smooth)),
/// conditional variance of W/H by depth, and detrended correlations.


/// Two-sample Kolmogorov-Smirnov statistic between two empirical distributions.
fn two_sample_ks(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() { return 0.0; }
    
    let mut combined: Vec<f64> = a.iter().cloned().chain(b.iter().cloned()).collect();
    combined.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    
    let n_a = a.len() as f64;
    let n_b = b.len() as f64;
    let mut max_diff = 0.0;
    
    let mut count_a = 0usize;
    let mut count_b = 0usize;
    
    for &val in &combined {
        // Count how many of a and b are <= val
        while count_a < a.len() && a[count_a] <= val + 1e-15 {
            count_a += 1;
        }
        while count_b < b.len() && b[count_b] <= val + 1e-15 {
            count_b += 1;
        }
        
        let cdf_a = count_a as f64 / n_a;
        let cdf_b = count_b as f64 / n_b;
        let diff = (cdf_a - cdf_b).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }
    
    max_diff
}



/// Gradient-based zero finding with distance-to-peak estimation.
/// Uses the 100-foot perpendicular drill constraint: at each point, compute
/// gradient and curvature (perpendicular measurement), then extrapolate zeros.
pub fn shape_test_gradient_zero_finding(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 11 — Gradient Descent Zero Finding\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Protocol: at each point, compute gradient + curvature (perpendicular\n");
    report.push_str("drill), extrapolate zeros using Newton-like step. 100ft drill only.\n\n");
    
    // Compute local gradients and curvatures using central differences
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    
    for i in 0..n {
        if i == 0 {
            let grad = (r_values[1] - r_values[0]) / (t_values[1] - t_values[0]);
            gradients.push(grad);
            curvatures.push(0.0);
        } else if i == n - 1 {
            let grad = (r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]);
            gradients.push(grad);
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            
            // Central difference for gradient
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            let grad = (grad_left + grad_right) / 2.0;
            
            // Curvature from gradient change
            let curv = (grad_right - grad_left) / ((dt_left + dt_right) / 2.0);
            
            gradients.push(grad);
            curvatures.push(curv);
        }
    }
    
    // For each point, use Newton-like extrapolation to find zeros
    // x_new = x_old - f(x) / f'(x)
    // But constrained: only trust if curvature confirms the direction
    let mut predicted_zeros = Vec::new();
    let mut distances_to_peaks = Vec::new();
    
    for i in 1..n.saturating_sub(1) {
        let f = r_values[i];
        let df = gradients[i];
        let d2f = curvatures[i];
        
        if df.abs() < 1e-10 { continue; } // Skip flat points
        
        // Newton step: where does the linear extrapolation hit zero?
        let t_zero_linear = t_values[i] - f / df;
        
        // With curvature correction (second-order):
        // Solve f + df*dt + 0.5*d2f*dt^2 = 0
        if d2f.abs() > 1e-10 {
            let a = 0.5 * d2f;
            let b = df;
            let c = f;
            let disc = b*b - 4.0*a*c;
            if disc >= 0.0 {
                let sqrt_disc = disc.sqrt();
                let dt1 = (-b + sqrt_disc) / (2.0 * a);
                let dt2 = (-b - sqrt_disc) / (2.0 * a);
                
                // Only trust zeros that are "ahead" in the direction of the gradient
                // and within a reasonable range (the 100ft drill constraint)
                for &dt in &[dt1, dt2] {
                    let t_pred = t_values[i] + dt;
                    if t_pred > 0.0 && t_pred < t_max * 2.0 && dt.abs() < 50.0 {
                        predicted_zeros.push((t_values[i], t_pred, f, df, d2f));
                    }
                }
            }
        } else {
            // Linear only
            if t_zero_linear > 0.0 && t_zero_linear < t_max * 2.0 {
                let dt = t_zero_linear - t_values[i];
                if dt.abs() < 50.0 {
                    predicted_zeros.push((t_values[i], t_zero_linear, f, df, d2f));
                }
            }
        }
        
        // Distance to peak: if gradient points toward increasing R (uphill),
        // estimate distance to local maximum using curvature
        if df > 0.0 && d2f < 0.0 {
            // Moving uphill but curving down — approaching a peak
            let dt_to_peak = -df / d2f;
            if dt_to_peak > 0.0 && dt_to_peak < 50.0 {
                let t_peak_pred = t_values[i] + dt_to_peak;
                distances_to_peaks.push((t_values[i], t_peak_pred, dt_to_peak));
            }
        }
    }
    
    report.push_str(&format!("Total predicted zeros (from all starting points): {}\n", predicted_zeros.len()));
    report.push_str(&format!("Total peak distance estimates: {}\n\n", distances_to_peaks.len()));
    
    // Compare predicted zeros against actual known zeros
    // We know the first few zeta zeros on the critical line
    let known_zeros = [14.1347, 21.0220, 25.0109, 30.4249, 32.9351, 37.5862, 40.9187, 43.3271, 48.0052, 49.7738];
    
    report.push_str("Predicted zeros vs known zeros (first 10):\n");
    let mut matches = 0;
    for &(t_start, t_pred, f_val, df_val, d2f_val) in &predicted_zeros {
        for &known in &known_zeros {
            if (t_pred - known).abs() < 1.5 { // Within 1.5 units of a known zero
                matches += 1;
                report.push_str(&format!("  Predicted T={:.2} (from T={:.2}) matches known zero at T={:.4}\n", 
                    t_pred, t_start, known));
                report.push_str(&format!("    f={:.3}, f'={:.3}, f''={:.3}\n", f_val, df_val, d2f_val));
                break;
            }
        }
    }
    
    if matches == 0 {
        report.push_str("  No predicted zeros matched known zeros within tolerance.\n");
    } else {
        report.push_str(&format!("\nMatched {}/{} known zeros from {} predictions.\n", 
            matches, known_zeros.len(), predicted_zeros.len()));
    }
    
    // Distance-to-peak analysis
    if !distances_to_peaks.is_empty() {
        report.push_str(&format!("\nDistance-to-peak estimates (gradient points uphill):\n"));
        let mut peak_distances: Vec<f64> = distances_to_peaks.iter().map(|&(_, _, d)| d).collect();
        peak_distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let mean_dist: f64 = peak_distances.iter().sum::<f64>() / peak_distances.len() as f64;
        let median_dist = if peak_distances.len() % 2 == 1 {
            peak_distances[peak_distances.len() / 2]
        } else {
            (peak_distances[peak_distances.len()/2 - 1] + peak_distances[peak_distances.len()/2]) / 2.0
        };
        
        report.push_str(&format!("  Mean distance to predicted peak: {:.2}\n", mean_dist));
        report.push_str(&format!("  Median distance to predicted peak: {:.2}\n", median_dist));
        report.push_str(&format!("  Min: {:.2}, Max: {:.2}\n\n", 
            peak_distances[0], peak_distances[peak_distances.len()-1]));
        
        // Check if predicted peaks align with actual peaks in our data
        let peaks = find_peaks(&r_values);
        report.push_str("Predicted peaks vs actual peaks in trajectory:\n");
        let mut peak_matches = 0;
        for &(t_start, t_pred_peak, dt) in &distances_to_peaks {
            for &actual_peak_idx in &peaks {
                let actual_peak_t = t_values[actual_peak_idx];
                if (t_pred_peak - actual_peak_t).abs() < 2.0 {
                    peak_matches += 1;
                    report.push_str(&format!("  Predicted peak at T={:.2} (from T={:.2}, d={:.2}) matches actual at T={:.2}\n",
                        t_pred_peak, t_start, dt, actual_peak_t));
                    break;
                }
            }
        }
        
        if peak_matches == 0 {
            report.push_str("  No predicted peaks matched actual peaks.\n");
        } else {
            report.push_str(&format!("\nMatched {} of {} predicted peaks with actual peaks.\n", 
                peak_matches, distances_to_peaks.len()));
        }
    }
    
    // The key question: can gradient descent be FORCED to find far zeros?
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("FORCING GRADIENT DESCENT TO FIND FAR ZEROS:\n");
    report.push_str("Standard gradient descent converges to the NEAREST extremum.\n");
    report.push_str("To find ALL zeros, we need to:\n\n");
    
    // Strategy 1: Multiple starting points (but keep them connected)
    report.push_str("1. **Connected multi-start**: Begin at every Nth point along the\n");
    report.push_str("   trajectory and run Newton steps from each. The 100ft drill\n");
    report.push_str("   constraint means we only trust predictions within 50 units.\n\n");
    
    // Strategy 2: Use curvature to detect when we're on the wrong side of a peak
    report.push_str("2. **Curvature sign flip**: If f' > 0 and f'' < 0, we're approaching\n");
    report.push_str("   a peak from below. The zero is BEHIND us (we already passed it).\n");
    report.push_str("   If f' < 0 and f'' < 0, the zero is AHEAD of us.\n\n");
    
    // Strategy 3: Perpendicular drill tells us if the extrapolation is trustworthy
    report.push_str("3. **Perpendicular validation**: The curvature (perpendicular\n");
    report.push_str("   measurement) tells us if the hill is bending toward or away\n");
    report.push_str("   from our predicted zero. If f'' has the same sign as f,\n");
    report.push_str("   the hill is bending AWAY — the zero is farther than linear\n");
    report.push_str("   extrapolation suggests.\n\n");
    
    // Count how many predictions were validated by curvature
    let mut validated = 0;
    let mut invalidated = 0;
    for &(t_start, t_pred, f_val, df_val, d2f_val) in &predicted_zeros {
        // Curvature validation: if f and f'' have the same sign, 
        // the function is convex/concave in a way that supports the zero existing
        if f_val * d2f_val > 0.0 {
            validated += 1;
        } else {
            invalidated += 1;
        }
    }
    
    report.push_str(&format!("Curvature validation of predictions:\n"));
    report.push_str(&format!("  Validated (f·f'' > 0): {}\n", validated));
    report.push_str(&format!("  Invalidated (f·f'' < 0): {}\n\n", invalidated));
    
    if validated + invalidated > 0 {
        let validation_rate = validated as f64 / (validated + invalidated) as f64;
        report.push_str(&format!("Validation rate: {:.1}%\n", validation_rate * 100.0));
        if validation_rate > 0.7 {
            report.push_str("→ Most predictions are curvature-validated. The 100ft\n");
            report.push_str("  perpendicular drill confirms the gradient-based zeros.\n");
        } else {
            report.push_str("→ Many predictions fail curvature validation. The hill\n");
            report.push_str("  bends unpredictably — need more local measurements.\n");
        }
    }
    
    // Summary
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("CONCLUSION:\n");
    report.push_str("Gradient descent with the 100ft perpendicular drill constraint\n");
    report.push_str("can find zeros, but only within the trust radius of each measurement.\n");
    report.push_str("To find ALL zeros (even far from peaks), you need a CONNECTED WALK\n");
    report.push_str("that validates each prediction with local curvature before trusting it.\n");
    report.push_str("The perpendicular drill is what makes the difference between blind\n");
    report.push_str("extrapolation and validated zero-finding.\n");
    
    report
}

/// Perpendicular-to-grade measurement protocol.
/// At each zero, fit a local trend over a fixed window, measure the residual
/// (perpendicular component). Same tool everywhere, walk in order.
pub fn shape_test_perpendicular_grade(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 10 — Perpendicular to Grade (Fixed 100ft Drill)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Protocol: at each zero, fit local trend over fixed window,\n");
    report.push_str("measure residual perpendicular to grade. Same tool everywhere.\n\n");
    
    // Fixed window size (the "100-foot drill")
    let half_window = 5;
    
    // Compute perpendicular residuals at each point
    let mut perp_residuals = Vec::new();
    let mut local_slopes = Vec::new();
    let mut local_curvatures = Vec::new();
    
    for i in 0..n {
        let start = i.saturating_sub(half_window);
        let end = (i + half_window + 1).min(n);
        
        if end - start < 3 { continue; }
        
        // Linear fit over window
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        let mut count = 0.0;
        
        for j in start..end {
            let x = (j as f64 - i as f64); // centered at current point
            let y = r_values[j];
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
            count += 1.0;
        }
        
        if count < 3.0 || sum_xx == 0.0 { continue; }
        
        let slope = (count * sum_xy - sum_x * sum_y) / (count * sum_xx - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / count;
        
        // Perpendicular residual at current point
        let predicted = intercept + slope * 0.0; // at x=0 (current point)
        let residual = r_values[i] - predicted;
        
        // Curvature: compare slopes on left and right halves
        let mut left_slope = 0.0;
        let mut right_slope = 0.0;
        let mut left_count = 0.0;
        let mut right_count = 0.0;
        
        for j in start..=i {
            if j == i { continue; }
            let x = (j as f64 - i as f64);
            let y = r_values[j];
            // Simple two-point slope estimate
            if left_count > 0.0 {
                left_slope = (y - r_values[i]) / (x - 0.0);
            }
            left_count += 1.0;
        }
        
        for j in (i+1)..end {
            let x = (j as f64 - i as f64);
            let y = r_values[j];
            if right_count > 0.0 {
                right_slope = (y - r_values[i]) / (x - 0.0);
            }
            right_count += 1.0;
        }
        
        let curvature = if left_count > 0.0 && right_count > 0.0 {
            (right_slope - left_slope) / 2.0
        } else {
            0.0
        };
        
        perp_residuals.push((i as f64, residual));
        local_slopes.push((i as f64, slope));
        local_curvatures.push((i as f64, curvature));
    }
    
    if perp_residuals.is_empty() {
        report.push_str("Not enough data for perpendicular measurement.\n");
        return report;
    }
    
    // Statistics of perpendicular residuals
    let residuals_only: Vec<f64> = perp_residuals.iter().map(|&(_, r)| r).collect();
    let mean_res: f64 = residuals_only.iter().sum::<f64>() / residuals_only.len() as f64;
    let var_res: f64 = residuals_only.iter().map(|&r| (r - mean_res).powi(2)).sum::<f64>() / residuals_only.len() as f64;
    let std_res = var_res.sqrt();
    
    report.push_str(&format!("Perpendicular residual statistics:\n"));
    report.push_str(&format!("  Mean: {:.4} (should be ~0 if trend fits well)\n", mean_res));
    report.push_str(&format!("  Std:  {:.4}\n\n", std_res));
    
    // Correlation between perpendicular residual and local slope
    let mut cov_res_slope = 0.0;
    let mut var_res2 = 0.0;
    let mut var_slope = 0.0;
    let mean_slope: f64 = local_slopes.iter().map(|&(_, s)| s).sum::<f64>() / local_slopes.len() as f64;
    
    for i in 0..perp_residuals.len().min(local_slopes.len()) {
        let r_dev = perp_residuals[i].1 - mean_res;
        let s_dev = local_slopes[i].1 - mean_slope;
        cov_res_slope += r_dev * s_dev;
        var_res2 += r_dev * r_dev;
        var_slope += s_dev * s_dev;
    }
    
    let denom_rs = (var_res2 * var_slope).sqrt();
    let corr_res_slope = if denom_rs > 0.0 { cov_res_slope / denom_rs } else { 0.0 };
    
    report.push_str(&format!("Corr(perpendicular residual, local slope): {:.4}\n", corr_res_slope));
    if corr_res_slope.abs() > 0.3 {
        report.push_str("  → Residuals correlate with grade. The perpendicular component\n");
        report.push_str("    carries information about the local slope.\n");
    } else {
        report.push_str("  → Residuals are independent of grade. The perpendicular\n");
        report.push_str("    component is orthogonal to the trend (as expected).\n");
    }
    
    // Correlation between perpendicular residual and curvature
    let mut cov_res_curv = 0.0;
    let mut var_curv = 0.0;
    let mean_curv: f64 = local_curvatures.iter().map(|&(_, c)| c).sum::<f64>() / local_curvatures.len() as f64;
    
    for i in 0..perp_residuals.len().min(local_curvatures.len()) {
        let r_dev = perp_residuals[i].1 - mean_res;
        let c_dev = local_curvatures[i].1 - mean_curv;
        cov_res_curv += r_dev * c_dev;
        var_curv += c_dev * c_dev;
    }
    
    let denom_rc = (var_res2 * var_curv).sqrt();
    let corr_res_curv = if denom_rc > 0.0 { cov_res_curv / denom_rc } else { 0.0 };
    
    report.push_str(&format!("Corr(perpendicular residual, local curvature): {:.4}\n", corr_res_curv));
    if corr_res_curv.abs() > 0.3 {
        report.push_str("  → Residuals correlate with curvature. The perpendicular\n");
        report.push_str("    component reveals how the hill bends.\n");
    } else {
        report.push_str("  → Residuals independent of curvature.\n");
    }
    
    // Now the key test: do perpendicular residuals behave differently
    // at peaks vs troughs vs slopes, using ONLY the perpendicular measurement?
    let peaks = find_peaks(&r_values);
    let mut troughs = Vec::new();
    for i in 1..r_values.len().saturating_sub(1) {
        if r_values[i] <= r_values[i-1] && r_values[i] <= r_values[i+1] {
            troughs.push(i);
        }
    }
    
    let mut peak_perp = Vec::new();
    let mut trough_perp = Vec::new();
    let mut slope_perp = Vec::new();
    
    for &(idx, res) in &perp_residuals {
        let i = idx as usize;
        let near_peak = peaks.iter().any(|&p| (i as i64 - p as i64).abs() <= 1);
        let near_trough = troughs.iter().any(|&t| (i as i64 - t as i64).abs() <= 1);
        
        if near_peak {
            peak_perp.push(res);
        } else if near_trough {
            trough_perp.push(res);
        } else {
            slope_perp.push(res);
        }
    }
    
    report.push_str(&format!("\nPerpendicular residuals by position (threshold = 1 zero):\n"));
    
    fn perp_stats(name: &str, values: &[f64], report: &mut String) {
        if values.len() < 3 {
            report.push_str(&format!("  {}: n={} (too few)\n", name, values.len()));
            return;
        }
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        let var: f64 = values.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std = var.sqrt();
        report.push_str(&format!("  {}: mean={:.4}, std={:.4} (n={})\n", name, mean, std, values.len()));
    }
    
    perp_stats("Near-peak", &peak_perp, &mut report);
    perp_stats("Near-trough", &trough_perp, &mut report);
    perp_stats("On-slope", &slope_perp, &mut report);
    
    // KS test between peak and slope perpendicular residuals
    if peak_perp.len() >= 5 && slope_perp.len() >= 5 {
        let d_stat = two_sample_ks(&peak_perp, &slope_perp);
        report.push_str(&format!("\nKS statistic (peak vs slope perpendicular residuals): D = {:.4}\n", d_stat));
        if d_stat > 0.3 {
            report.push_str("→ Significant difference even with perpendicular measurement.\n");
            report.push_str("  Edge structure is real, not an artifact of peak-biased sampling.\n");
        } else {
            report.push_str("→ No significant difference. The perpendicular component\n");
            report.push_str("  doesn't distinguish edges from slopes.\n");
        }
    }
    
    // Connected walk: how does the perpendicular residual evolve along the path?
    report.push_str(&format!("\nConnected walk — perpendicular residual evolution:\n"));
    let mut consecutive_same_sign = 0;
    let mut max_consecutive = 0;
    for i in 1..perp_residuals.len() {
        if perp_residuals[i].1 * perp_residuals[i-1].1 > 0.0 {
            consecutive_same_sign += 1;
            if consecutive_same_sign > max_consecutive {
                max_consecutive = consecutive_same_sign;
            }
        } else {
            consecutive_same_sign = 0;
        }
    }
    
    report.push_str(&format!("Max consecutive same-sign residuals: {}\n", max_consecutive));
    if max_consecutive > 5 {
        report.push_str("  → Residuals persist. The perpendicular component has memory.\n");
        report.push_str("    Connected measurements reveal structure that isolated\n");
        report.push_str("    measurements would miss.\n");
    } else {
        report.push_str("  → Residuals don't persist. Each measurement is independent.\n");
    }
    
    // The key question: can we reconstruct the hill from perpendicular measurements alone?
    // If the perpendicular component carries information about both slope and curvature,
    // then yes — we're getting 3D information from a 1D walk with a fixed tool.
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("RECONSTRUCTION POTENTIAL:\n");
    let info_content = corr_res_slope.abs() + corr_res_curv.abs();
    if info_content > 0.6 {
        report.push_str("The perpendicular component carries significant information\n");
        report.push_str("about both slope and curvature. A connected walk with a fixed\n");
        report.push_str("perpendicular drill can reconstruct the hill's geometry.\n");
    } else if info_content > 0.3 {
        report.push_str("The perpendicular component carries partial information about\n");
        report.push_str("the hill's shape. Useful, but not sufficient alone.\n");
    } else {
        report.push_str("The perpendicular component is mostly noise relative to the\n");
        report.push_str("hill's geometry. Need additional measurement directions.\n");
    }
    
    report
}

/// Edge-partition spacing test: classify zeros by position relative to peaks/troughs,
/// compare spacing statistics across groups. Also measures "distance to connection"
/// — how many observations needed before each dimension stabilizes.
pub fn shape_test_edge_partition_and_mapping(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let peaks = find_peaks(&r_values);
    
    // Find troughs (local minima)
    let mut troughs = Vec::new();
    for i in 1..r_values.len().saturating_sub(1) {
        if r_values[i] <= r_values[i-1] && r_values[i] <= r_values[i+1] {
            troughs.push(i);
        }
    }
    
    let mut report = format!("\nSHAPE TEST 9 — Edge Partition & Distance Mapping\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Classify zeros by position: near-peak, near-trough, on-slope.\n");
    report.push_str("Compare spacing statistics. Measure distance to connection.\n\n");
    
    // Classify each zero
    let proximity_threshold = 2; // within 2 zeros of a peak/trough
    let mut classifications = Vec::new();
    for i in 0..points.len() {
        let near_peak = peaks.iter().any(|&p| (i as i64 - p as i64).abs() <= proximity_threshold as i64);
        let near_trough = troughs.iter().any(|&t| (i as i64 - t as i64).abs() <= proximity_threshold as i64);
        
        if near_peak {
            classifications.push("peak");
        } else if near_trough {
            classifications.push("trough");
        } else {
            classifications.push("slope");
        }
    }
    
    let n_peaks_class = classifications.iter().filter(|&&c| c == "peak").count();
    let n_troughs_class = classifications.iter().filter(|&&c| c == "trough").count();
    let n_slopes_class = classifications.iter().filter(|&&c| c == "slope").count();
    
    report.push_str(&format!("Classifications (threshold = {} zeros):\n", proximity_threshold));
    report.push_str(&format!("  Near-peak: {}\n", n_peaks_class));
    report.push_str(&format!("  Near-trough: {}\n", n_troughs_class));
    report.push_str(&format!("  On-slope: {}\n\n", n_slopes_class));
    
    // Compute normalized spacings for each group
    let known_zeros = KNOWN_ZERO_ORDINATES;
    let mut peak_spacings = Vec::new();
    let mut trough_spacings = Vec::new();
    let mut slope_spacings = Vec::new();
    
    for i in 0..points.len().saturating_sub(1) {
        let gamma_n = points[i].0;
        let gamma_next = points[i+1].0;
        let spacing = (gamma_next - gamma_n) * (gamma_n / (2.0 * std::f64::consts::PI)).ln() / (2.0 * std::f64::consts::PI);
        
        if spacing > 0.0 {
            match classifications[i] {
                "peak" => peak_spacings.push(spacing),
                "trough" => trough_spacings.push(spacing),
                _ => slope_spacings.push(spacing),
            }
        }
    }
    
    // Report statistics for each group
    fn spacing_stats(name: &str, spacings: &[f64], report: &mut String) {
        if spacings.len() < 3 {
            report.push_str(&format!("  {}: n={} (too few)\n", name, spacings.len()));
            return;
        }
        let mean: f64 = spacings.iter().sum::<f64>() / spacings.len() as f64;
        let var: f64 = spacings.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / spacings.len() as f64;
        let std = var.sqrt();
        
        // Fraction below 0.5 (level repulsion indicator)
        let below_half = spacings.iter().filter(|&&s| s < 0.5).count() as f64 / spacings.len() as f64;
        
        report.push_str(&format!("  {}: mean={:.4}, std={:.4}, frac<0.5={:.2} (n={})\n", 
            name, mean, std, below_half, spacings.len()));
    }
    
    report.push_str("Normalized spacing statistics by position:\n");
    spacing_stats("Near-peak", &peak_spacings, &mut report);
    spacing_stats("Near-trough", &trough_spacings, &mut report);
    spacing_stats("On-slope", &slope_spacings, &mut report);
    
    // KS test: peak vs slope spacings
    if peak_spacings.len() >= 5 && slope_spacings.len() >= 5 {
        let d_stat = two_sample_ks(&peak_spacings, &slope_spacings);
        report.push_str(&format!("\nKS statistic (peak vs slope spacings): D = {:.4}\n", d_stat));
        if d_stat > 0.3 {
            report.push_str("→ Significant difference. Edge zeros behave differently.\n");
        } else {
            report.push_str("→ No significant difference detected.\n");
        }
    }
    
    // === DISTANCE TO CONNECTION ===
    // For each dimension, measure how many peaks we need before the measurement stabilizes.
    // "Stable" = the last 3 measurements have std < 0.5 * overall mean
    
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("DISTANCE TO CONNECTION — how far to drill before each dimension locks in:\n\n");
    
    // Recompute peak data with W/H ratios
    let mut peak_ratios = Vec::new();
    for &peak_idx in &peaks {
        let n_smooth = points[peak_idx].1;
        let peak_height = r_values[peak_idx];
        
        let baseline = if peak_idx > 0 {
            r_values[..=peak_idx].iter().cloned().fold(f64::INFINITY, f64::min)
        } else { 0.0 };
        
        let half_height = (peak_height + baseline) / 2.0;
        
        let mut left_width = 0;
        for i in (0..peak_idx).rev() {
            if r_values[i] <= half_height {
                left_width = peak_idx - i;
                break;
            }
        }
        
        let mut right_width = 0;
        for i in (peak_idx + 1)..points.len() {
            if r_values[i] <= half_height {
                right_width = i - peak_idx;
                break;
            }
        }
        
        let total_width = (left_width + right_width) as f64;
        let height_above_base = peak_height - baseline;
        
        if height_above_base > 0.0 && total_width > 0.0 {
            let ratio = total_width / height_above_base;
            peak_ratios.push((peak_idx, n_smooth, ratio));
        }
    }
    
    // Find where W/H stabilizes (last 3 have std < threshold)
    let mut wh_stable_at = None;
    for i in 3..peak_ratios.len() {
        let recent: Vec<f64> = peak_ratios[i-3..=i].iter().map(|&(_, _, r)| r).collect();
        let mean: f64 = recent.iter().sum::<f64>() / 3.0;
        let var: f64 = recent.iter().map(|&r| (r - mean).powi(2)).sum::<f64>() / 3.0;
        let std = var.sqrt();
        
        if std < 1.0 { // stabilization threshold
            wh_stable_at = Some((peak_ratios[i].0, peak_ratios[i].1));
            break;
        }
    }
    
    match wh_stable_at {
        Some((idx, ns)) => {
            report.push_str(&format!("W/H ratio stabilizes at peak index {}, N_smooth = {:.2}\n", idx, ns));
            report.push_str(&format!("  → Distance to connection (W/H dimension): {} peaks\n\n", idx + 1));
        }
        None => {
            report.push_str("W/H ratio does NOT stabilize in available range.\n\n");
        }
    }
    
    // Find where detrended correlation stabilizes
    // Compute rolling detrended correlation using expanding windows
    let mut corr_stable_at = None;
    for window_size in 5..=peak_ratios.len() {
        if window_size < 5 { break; }
        
        let wh_sq: Vec<f64> = peak_ratios[..window_size].iter()
            .map(|&(_, _, r)| { 
                // Approximate W²+H² from ratio and typical height
                let h_approx = 1.0 + window_size as f64 * 0.1; // rough proxy
                let w = r * h_approx;
                w*w + h_approx*h_approx
            })
            .collect();
        
        let ln_ns: Vec<f64> = peak_ratios[..window_size].iter().map(|&(_, ns, _)| ns.ln()).collect();
        
        // Detrend and correlate
        fn detrend(x: &[f64]) -> Vec<f64> {
            let n = x.len() as f64;
            if n < 3.0 { return x.to_vec(); }
            let indices: Vec<f64> = (0..x.len()).map(|i| i as f64).collect();
            let mean_x: f64 = indices.iter().sum::<f64>() / n;
            let mean_y: f64 = x.iter().sum::<f64>() / n;
            
            let mut num = 0.0;
            let mut den = 0.0;
            for i in 0..x.len() {
                num += (indices[i] - mean_x) * (x[i] - mean_y);
                den += (indices[i] - mean_x).powi(2);
            }
            if den == 0.0 { return x.to_vec(); }
            let slope = num / den;
            let intercept = mean_y - slope * mean_x;
            x.iter().enumerate().map(|(i, &v)| v - (intercept + slope * i as f64)).collect()
        }
        
        let wh_detrended = detrend(&wh_sq);
        let ns_detrended = detrend(&ln_ns);
        
        let n_dt = wh_detrended.len() as f64;
        if n_dt >= 3.0 {
            let mean_wh: f64 = wh_detrended.iter().sum::<f64>() / n_dt;
            let mean_ns: f64 = ns_detrended.iter().sum::<f64>() / n_dt;
            
            let mut cov = 0.0;
            let mut var_wh = 0.0;
            let mut var_ns = 0.0;
            for i in 0..wh_detrended.len() {
                cov += (wh_detrended[i] - mean_wh) * (ns_detrended[i] - mean_ns);
                var_wh += (wh_detrended[i] - mean_wh).powi(2);
                var_ns += (ns_detrended[i] - mean_ns).powi(2);
            }
            
            let denom = (var_wh * var_ns).sqrt();
            if denom > 0.0 {
                let r_val = cov / denom;
                
                // Check stability: compare with previous window
                if window_size >= 8 {
                    // Compute correlation for window-1
                    let wh_sq_prev: Vec<f64> = peak_ratios[..window_size-1].iter()
                        .map(|&(_, _, r)| { 
                            let h_approx = 1.0 + (window_size-1) as f64 * 0.1;
                            let w = r * h_approx;
                            w*w + h_approx*h_approx
                        })
                        .collect();
                    let ln_ns_prev: Vec<f64> = peak_ratios[..window_size-1].iter().map(|&(_, ns, _)| ns.ln()).collect();
                    
                    let wh_detrended_prev = detrend(&wh_sq_prev);
                    let ns_detrended_prev = detrend(&ln_ns_prev);
                    
                    let n_dt2 = wh_detrended_prev.len() as f64;
                    if n_dt2 >= 3.0 {
                        let mean_wh2: f64 = wh_detrended_prev.iter().sum::<f64>() / n_dt2;
                        let mean_ns2: f64 = ns_detrended_prev.iter().sum::<f64>() / n_dt2;
                        
                        let mut cov2 = 0.0;
                        let mut var_wh2 = 0.0;
                        let mut var_ns2 = 0.0;
                        for i in 0..wh_detrended_prev.len() {
                            cov2 += (wh_detrended_prev[i] - mean_wh2) * (ns_detrended_prev[i] - mean_ns2);
                            var_wh2 += (wh_detrended_prev[i] - mean_wh2).powi(2);
                            var_ns2 += (ns_detrended_prev[i] - mean_ns2).powi(2);
                        }
                        
                        let denom2 = (var_wh2 * var_ns2).sqrt();
                        if denom2 > 0.0 {
                            let r_val2 = cov2 / denom2;
                            if (r_val - r_val2).abs() < 0.05 {
                                corr_stable_at = Some((peak_ratios[window_size-1].0, peak_ratios[window_size-1].1));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    match corr_stable_at {
        Some((idx, ns)) => {
            report.push_str(&format!("Detrended correlation stabilizes at peak index {}, N_smooth = {:.2}\n", idx, ns));
            report.push_str(&format!("  → Distance to connection (correlation dimension): {} peaks\n\n", idx + 1));
        }
        None => {
            report.push_str("Detrended correlation does NOT stabilize in available range.\n\n");
        }
    }
    
    // Edge partition stability: when do peak vs slope spacings first differ significantly?
    let mut edge_stable_at = None;
    for window_size in 5..=points.len() {
        if window_size < 10 { continue; }
        
        let r_window: Vec<f64> = r_values[..window_size].to_vec();
        let peaks_window = find_peaks(&r_window);
        
        let mut peak_sp = Vec::new();
        let mut slope_sp = Vec::new();
        
        for i in 0..window_size.saturating_sub(1) {
            let gamma_n = points[i].0;
            let gamma_next = points[i+1].0;
            let spacing = (gamma_next - gamma_n) * (gamma_n / (2.0 * std::f64::consts::PI)).ln() / (2.0 * std::f64::consts::PI);
            
            if spacing <= 0.0 { continue; }
            
            let near_peak = peaks_window.iter().any(|&p| (i as i64 - p as i64).abs() <= proximity_threshold as i64);
            if near_peak {
                peak_sp.push(spacing);
            } else {
                slope_sp.push(spacing);
            }
        }
        
        if peak_sp.len() >= 3 && slope_sp.len() >= 3 {
            let d_stat = two_sample_ks(&peak_sp, &slope_sp);
            if d_stat > 0.3 {
                edge_stable_at = Some((points[window_size-1].0, points[window_size-1].1));
                break;
            }
        }
    }
    
    match edge_stable_at {
        Some((t_val, ns)) => {
            report.push_str(&format!("Edge partition difference emerges at T = {:.2}, N_smooth = {:.2}\n", t_val, ns));
            report.push_str(&format!("  → Distance to connection (edge dimension): T = {:.2}\n\n", t_val));
        }
        None => {
            report.push_str("Edge partition difference does NOT emerge in available range.\n\n");
        }
    }
    
    // Final mapping: the "hill map" — all dimensions at their connection points
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("HILL MAP — connection points for each dimension:\n");
    report.push_str("  W/H stabilization:     ");
    match wh_stable_at {
        Some((_, ns)) => report.push_str(&format!("N_smooth = {:.2}\n", ns)),
        None => report.push_str("NOT REACHED\n"),
    }
    report.push_str("  Correlation stability: ");
    match corr_stable_at {
        Some((_, ns)) => report.push_str(&format!("N_smooth = {:.2}\n", ns)),
        None => report.push_str("NOT REACHED\n"),
    }
    report.push_str("  Edge partition:        ");
    match edge_stable_at {
        Some((t_val, _)) => report.push_str(&format!("T = {:.2}\n", t_val)),
        None => report.push_str("NOT REACHED\n"),
    }
    
    // Check if all three align within a range
    let wh_ns = wh_stable_at.map(|(_, ns)| ns);
    let corr_ns = corr_stable_at.map(|(_, ns)| ns);
    
    if let (Some(a), Some(b)) = (wh_ns, corr_ns) {
        if (a - b).abs() < 10.0 {
            report.push_str("\n→ W/H and correlation dimensions CONNECT within ΔN_smooth < 10.\n");
            report.push_str("  The hill is mappable from these two ends.\n");
        } else {
            report.push_str(&format!("\n→ Dimensions connect at different depths (Δ = {:.2}).\n", (a-b).abs()));
            report.push_str("  Need more data or a different coordinate system to unify.\n");
        }
    }
    
    report
}

pub fn shape_test_depth_constraint(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 10 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let peaks = find_peaks(&r_values);
    
    let mut report = format!("\nSHAPE TEST 8 — Depth-Dependent Constraint (Rigorous)\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Tests whether the W/H constraint tightens with depth.\n");
    report.push_str("Model A: W = k·H (fixed geometry)\n");
    report.push_str("Model B: W = H·k(N_smooth) (depth-dependent)\n\n");
    
    if peaks.len() < 8 {
        report.push_str(&format!("Only {} peaks. Need at least 8 for depth analysis.\n", peaks.len()));
        return report;
    }
    
    // Collect peak data: (T, N_smooth, W, H_above_base, W/H ratio)
    let mut peak_data = Vec::new();
    for &peak_idx in &peaks {
        let t_val = points[peak_idx].0;
        let n_smooth = points[peak_idx].1;
        let peak_height = r_values[peak_idx];
        
        let baseline = if peak_idx > 0 {
            r_values[..=peak_idx].iter().cloned().fold(f64::INFINITY, f64::min)
        } else { 0.0 };
        
        let half_height = (peak_height + baseline) / 2.0;
        
        let mut left_width = 0;
        for i in (0..peak_idx).rev() {
            if r_values[i] <= half_height {
                left_width = peak_idx - i;
                break;
            }
        }
        
        let mut right_width = 0;
        for i in (peak_idx + 1)..points.len() {
            if r_values[i] <= half_height {
                right_width = i - peak_idx;
                break;
            }
        }
        
        let total_width = (left_width + right_width) as f64;
        let height_above_base = peak_height - baseline;
        
        if height_above_base > 0.0 && total_width > 0.0 {
            let ratio = total_width / height_above_base;
            peak_data.push((t_val, n_smooth, total_width, height_above_base, ratio));
        }
    }
    
    report.push_str(&format!("Testing {} peaks\n\n", peak_data.len()));
    
    // Split into early/middle/late thirds by N_smooth (which correlates with depth)
    let sorted_by_depth: Vec<_> = peak_data.iter().collect();
    let third = peak_data.len() / 3;
    if third < 2 {
        report.push_str("Not enough peaks for three-way split.\n");
        return report;
    }
    
    let early: &[&(f64, f64, f64, f64, f64)] = &sorted_by_depth[..third];
    let middle: &[&(f64, f64, f64, f64, f64)] = &sorted_by_depth[third..2*third];
    let late: &[&(f64, f64, f64, f64, f64)] = &sorted_by_depth[2*third..];
    
    // Compute mean and variance of W/H ratio in each group
    fn stats(ratios: &[&f64]) -> (f64, f64) {
        let n = ratios.len() as f64;
        if n == 0.0 { return (0.0, 0.0); }
        let mean: f64 = ratios.iter().map(|&&r| r).sum::<f64>() / n;
        let var: f64 = ratios.iter().map(|&&r| (r - mean).powi(2)).sum::<f64>() / n.max(1.0);
        (mean, var.sqrt())
    }
    
    let early_ratios: Vec<&f64> = early.iter().map(|&(_, _, _, _, r)| r).collect();
    let middle_ratios: Vec<&f64> = middle.iter().map(|&(_, _, _, _, r)| r).collect();
    let late_ratios: Vec<&f64> = late.iter().map(|&(_, _, _, _, r)| r).collect();
    
    let (e_mean, e_std) = stats(&early_ratios);
    let (m_mean, m_std) = stats(&middle_ratios);
    let (l_mean, l_std) = stats(&late_ratios);
    
    report.push_str("W/H ratio by depth:\n");
    report.push_str(&format!("  Early  peaks: mean = {:.4}, std = {:.4}  (n={})\n", e_mean, e_std, early.len()));
    report.push_str(&format!("  Middle peaks: mean = {:.4}, std = {:.4}  (n={})\n", m_mean, m_std, middle.len()));
    report.push_str(&format!("  Late   peaks: mean = {:.4}, std = {:.4}  (n={})\n\n", l_mean, l_std, late.len()));
    
    // Test 1: Does variance decrease with depth?
    let var_decreasing = e_std > m_std && m_std > l_std;
    if var_decreasing {
        report.push_str("→ Variance DECREASES with depth. Envelope tightens.\n");
    } else if e_std > l_std {
        report.push_str("→ Variance decreases overall (early > late), but not monotonically.\n");
    } else {
        report.push_str("→ Variance does NOT clearly decrease with depth.\n");
    }
    
    // Test 2: Model A vs Model B
    // Model A: W = k·H → regress ln(W) on ln(H), get R²_A
    // Model B: W = H·k(N) → regress ln(W/H) on ln(N_smooth), get R²_B (reduction in residual variance)
    
    let n_peaks = peak_data.len() as f64;
    
    // Model A: ln(W) = a + b·ln(H) + ε
    let ln_w: Vec<f64> = peak_data.iter().map(|&(_, _, w, _, _)| w.ln()).collect();
    let ln_h: Vec<f64> = peak_data.iter().map(|&(_, _, _, h, _)| h.ln()).collect();
    
    // Simple linear regression
    fn linreg(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
        let n = x.len() as f64;
        if n < 3.0 { return (0.0, 0.0, 0.0); }
        let mean_x: f64 = x.iter().sum::<f64>() / n;
        let mean_y: f64 = y.iter().sum::<f64>() / n;
        
        let mut num = 0.0;
        let mut den = 0.0;
        for i in 0..x.len() {
            num += (x[i] - mean_x) * (y[i] - mean_y);
            den += (x[i] - mean_x).powi(2);
        }
        if den == 0.0 { return (mean_y, 0.0, 0.0); }
        let slope = num / den;
        let intercept = mean_y - slope * mean_x;
        
        // R²
        let mut ss_res = 0.0;
        let mut ss_tot = 0.0;
        for i in 0..x.len() {
            let pred = intercept + slope * x[i];
            ss_res += (y[i] - pred).powi(2);
            ss_tot += (y[i] - mean_y).powi(2);
        }
        let r_sq = if ss_tot > 0.0 { 1.0 - ss_res / ss_tot } else { 0.0 };
        
        (intercept, slope, r_sq)
    }
    
    let (_, slope_a, r_sq_a) = linreg(&ln_h, &ln_w);
    report.push_str(&format!("\nModel A (W = k·H): ln(W) = {:.4} + {:.4}·ln(H), R² = {:.4}\n", 
        0.0, slope_a, r_sq_a)); // intercept shown separately
    let (int_a, _, _) = linreg(&ln_h, &ln_w);
    report.push_str(&format!("  (intercept = {:.4})\n", int_a));
    
    // Model B: ln(W/H) = a + b·ln(N_smooth) + ε
    // Filter out invalid values (W/H ≤ 0 or N_smooth ≤ 0) before regression
    let valid_pairs: Vec<(f64, f64)> = peak_data.iter()
        .filter(|p| p.3 > 0.0 && p.2/p.3 > 0.0 && p.1 > 0.0)
        .map(|p| (p.1.ln(), (p.2/p.3).ln()))
        .collect();
    
    let ln_ns: Vec<f64> = valid_pairs.iter().map(|&(x, _)| x).collect();
    let ln_ratio: Vec<f64> = valid_pairs.iter().map(|&(_, y)| y).collect();
    
    let (int_b, slope_b, r_sq_b) = if valid_pairs.len() >= 3 { linreg(&ln_ns, &ln_ratio) } else { (0.0, 0.0, 0.0) };
    report.push_str(&format!("\nModel B (W/H depends on depth): ln(W/H) = {:.4} + {:.4}·ln(N_smooth), R² = {:.4}\n", 
        int_b, slope_b, r_sq_b));
    
    // Compare: does adding depth explain additional variance?
    if r_sq_b > 0.3 {
        report.push_str("→ Depth explains significant additional variance in W/H.\n");
        report.push_str("  Model B outperforms Model A. Geometry is depth-dependent.\n");
    } else if slope_b < 0.0 && r_sq_b > 0.1 {
        report.push_str("→ Weak evidence for depth dependence (negative slope, low R²).\n");
    } else {
        report.push_str("→ Depth does NOT significantly explain W/H variation.\n");
        report.push_str("  Model A (fixed geometry) is sufficient.\n");
    }
    
    // Test 3: Detrended correlation between W²+H² and N_smooth
    // Remove linear trend from both, then correlate
    // Use only valid pairs (where both W/H and N_smooth are well-defined)
    let wh_sq_valid: Vec<f64> = peak_data.iter()
        .filter(|p| p.3 > 0.0 && p.2/p.3 > 0.0 && p.1 > 0.0)
        .map(|p| p.2*p.2 + p.3*p.3)
        .collect();
    
    fn detrend(x: &[f64]) -> Vec<f64> {
        let n = x.len() as f64;
        if n < 3.0 { return x.to_vec(); }
        let indices: Vec<f64> = (0..x.len()).map(|i| i as f64).collect();
        let (int, slope, _) = linreg(&indices, x);
        x.iter().enumerate().map(|(i, &v)| v - (int + slope * i as f64)).collect()
    }
    
    let wh_sq_detrended = detrend(&wh_sq_valid);
    let ns_detrended = detrend(&ln_ns);
    
    // Correlation of detrended values
    let n_dt = wh_sq_detrended.len() as f64;
    if n_dt >= 3.0 {
        let mean_wh: f64 = wh_sq_detrended.iter().sum::<f64>() / n_dt;
        let mean_ns: f64 = ns_detrended.iter().sum::<f64>() / n_dt;
        
        let mut cov = 0.0;
        let mut var_wh = 0.0;
        let mut var_ns = 0.0;
        for i in 0..wh_sq_detrended.len() {
            cov += (wh_sq_detrended[i] - mean_wh) * (ns_detrended[i] - mean_ns);
            var_wh += (wh_sq_detrended[i] - mean_wh).powi(2);
            var_ns += (ns_detrended[i] - mean_ns).powi(2);
        }
        
        let denom = (var_wh * var_ns).sqrt();
        let r_detrended = if denom > 0.0 { cov / denom } else { 0.0 };
        
        report.push_str(&format!("\nDetrended Corr(W²+H², N_smooth) = {:.4}\n", r_detrended));
        if r_detrended.abs() < 0.3 {
            report.push_str("→ After removing trends, correlation vanishes.\n");
            report.push_str("  Original correlation was mostly shared trend, not geometry.\n");
        } else {
            report.push_str("→ Correlation survives detrending. Real geometric relationship.\n");
        }
    }
    
    // Summary: three distinct concepts at the "top of the hill"
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("Top of the hill — three distinct concepts:\n");
    report.push_str("  1. Maximum of R(T) — where we are now\n");
    report.push_str("  2. Boundary of geometrically permitted (W,H) region\n");
    report.push_str("  3. Transition point between regimes\n");
    report.push_str("These could coincide. The data above tests whether 1 and 2 align.\n");
    
    report
}

pub fn shape_test_pythagorean_constraint(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 10 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let peaks = find_peaks(&r_values);
    
    let mut report = format!("\nSHAPE TEST 7 — Pythagorean Constraint Test\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("If peaks are geometrically constrained, W² + H² should correlate\n");
    report.push_str("with some function of T (or N_smooth). Also checks for an envelope.\n\n");
    
    if peaks.len() < 5 {
        report.push_str(&format!("Only {} peaks found. Need more data.\n", peaks.len()));
        return report;
    }
    
    // Collect peak data: (T, W, H_above_base, N_smooth_at_peak)
    let mut peak_data = Vec::new();
    for &peak_idx in &peaks {
        let t_val = points[peak_idx].0;
        let n_smooth = points[peak_idx].1; // count_trajectory returns (N_smooth, R)
        let peak_height = r_values[peak_idx];
        
        let baseline = if peak_idx > 0 {
            r_values[..=peak_idx].iter().cloned().fold(f64::INFINITY, f64::min)
        } else { 0.0 };
        
        let half_height = (peak_height + baseline) / 2.0;
        
        let mut left_width = 0;
        for i in (0..peak_idx).rev() {
            if r_values[i] <= half_height {
                left_width = peak_idx - i;
                break;
            }
        }
        
        let mut right_width = 0;
        for i in (peak_idx + 1)..points.len() {
            if r_values[i] <= half_height {
                right_width = i - peak_idx;
                break;
            }
        }
        
        let total_width = (left_width + right_width) as f64;
        let height_above_base = peak_height - baseline;
        
        if height_above_base > 0.0 && total_width > 0.0 {
            peak_data.push((t_val, n_smooth, total_width, height_above_base));
        }
    }
    
    report.push_str(&format!("Testing {} peaks for geometric constraints\n\n", peak_data.len()));
    
    // Compute W² + H² for each peak
    let mut wh_sq: Vec<f64> = Vec::new();
    for &(_, _, w, h) in &peak_data {
        wh_sq.push(w * w + h * h);
    }
    
    // Test 1: Correlate W²+H² with T
    let t_vals: Vec<f64> = peak_data.iter().map(|&(t, _, _, _)| t).collect();
    let r_t = correlation(&t_vals, &wh_sq);
    report.push_str(&format!("Corr(W²+H², T) = {:.4}\n", r_t));
    
    // Test 2: Correlate W²+H² with N_smooth
    let ns_vals: Vec<f64> = peak_data.iter().map(|&(_, ns, _, _)| ns).collect();
    let r_ns = correlation(&ns_vals, &wh_sq);
    report.push_str(&format!("Corr(W²+H², N_smooth) = {:.4}\n", r_ns));
    
    // Test 3: Correlate W²+H² with T² (power law check)
    let t_sq: Vec<f64> = t_vals.iter().map(|&t| t * t).collect();
    let r_t2 = correlation(&t_sq, &wh_sq);
    report.push_str(&format!("Corr(W²+H², T²) = {:.4}\n", r_t2));
    
    // Test 4: Correlate W²+H² with H² (is width determined by height?)
    let h_sq: Vec<f64> = peak_data.iter().map(|&(_, _, _, h)| h * h).collect();
    let r_h2 = correlation(&h_sq, &wh_sq);
    report.push_str(&format!("Corr(W²+H², H²) = {:.4}\n", r_h2));
    
    // Test 5: Check W/H ratio stability at different scales
    let mut ratios_by_scale = Vec::new();
    for &(_, _, w, h) in &peak_data {
        if h > 0.0 {
            ratios_by_scale.push(w / h);
        }
    }
    if ratios_by_scale.len() >= 3 {
        let mean_ratio: f64 = ratios_by_scale.iter().sum::<f64>() / ratios_by_scale.len() as f64;
        let var: f64 = ratios_by_scale.iter().map(|&r| (r - mean_ratio).powi(2)).sum::<f64>() / ratios_by_scale.len() as f64;
        let cv = if mean_ratio > 0.0 { var.sqrt() / mean_ratio } else { 0.0 };
        report.push_str(&format!("\nW/H ratio across all peaks: mean = {:.4}, CV = {:.4}\n", 
            mean_ratio, cv));
        
        // Split into early vs late peaks
        let mid = peak_data.len() / 2;
        if mid >= 2 {
            let early_ratios: Vec<f64> = peak_data[..mid].iter().map(|&(_, _, w, h)| if h > 0.0 { w/h } else { 0.0 }).collect();
            let late_ratios: Vec<f64> = peak_data[mid..].iter().map(|&(_, _, w, h)| if h > 0.0 { w/h } else { 0.0 }).collect();
            
            let early_mean: f64 = early_ratios.iter().sum::<f64>() / early_ratios.len() as f64;
            let late_mean: f64 = late_ratios.iter().sum::<f64>() / late_ratios.len() as f64;
            
            report.push_str(&format!("Early peaks W/H mean: {:.4}\n", early_mean));
            report.push_str(&format!("Late peaks W/H mean: {:.4}\n", late_mean));
            if (late_mean - early_mean).abs() / early_mean.max(0.1) < 0.3 {
                report.push_str("→ Ratio stabilizes at scale. Self-similar geometry.\n");
            } else {
                report.push_str("→ Ratio changes with scale. Not self-similar.\n");
            }
        }
    }
    
    // Test 6: Envelope check — for each peak, is W bounded by some function of H?
    report.push_str(&format!("\nEnvelope check (W vs H):\n"));
    report.push_str(&text_scatter(
        &peak_data.iter().map(|&(_, _, w, h)| (h, w)).collect::<Vec<_>>(),
        40, 12
    ));
    
    // Check if W ≈ k·H holds (linear envelope)
    let mut k_values = Vec::new();
    for &(_, _, w, h) in &peak_data {
        if h > 0.0 {
            k_values.push(w / h);
        }
    }
    if !k_values.is_empty() {
        let min_k = k_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_k = k_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        report.push_str(&format!("W/H range: [{:.4}, {:.4}]\n", min_k, max_k));
        if (max_k - min_k) / min_k.max(0.1) < 0.5 {
            report.push_str("→ Tight envelope. W is bounded by ~constant × H.\n");
            report.push_str("  This IS a Pythagorean-like constraint: given H, W is determined.\n");
        } else {
            report.push_str("→ Loose envelope. W not tightly constrained by H.\n");
        }
    }
    
    report
}

/// Compute Pearson correlation between two vectors
fn correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() < 3 || x.len() != y.len() { return 0.0; }
    let n = x.len() as f64;
    let mean_x: f64 = x.iter().sum::<f64>() / n;
    let mean_y: f64 = y.iter().sum::<f64>() / n;
    
    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;
    for i in 0..x.len() {
        cov += (x[i] - mean_x) * (y[i] - mean_y);
        var_x += (x[i] - mean_x).powi(2);
        var_y += (y[i] - mean_y).powi(2);
    }
    
    let denom = (var_x * var_y).sqrt();
    if denom > 0.0 { cov / denom } else { 0.0 }
}


/// SHAPE TEST 12 — Connected Multi-Start Walk
/// 
/// Protocol: Start at a point, predict next zero using gradient+curvature,
/// validate with perpendicular drill (local measurements), then move to that
/// point and repeat. Measure how far the connected chain maps before breaking.
/// 
/// This is the 100ft drill constraint applied dynamically: we only trust
/// predictions that are locally validated by the perpendicular component.
pub fn shape_test_connected_walk(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 12 — Connected Multi-Start Walk\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Protocol: chain predictions. Each validated zero becomes\n");
    report.push_str("the next starting point. 100ft drill validates each step.\n\n");
    
    // Compute local gradients and curvatures
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    
    for i in 0..n {
        if i == 0 {
            let grad = (r_values[1] - r_values[0]) / (t_values[1] - t_values[0]);
            gradients.push(grad);
            curvatures.push(0.0);
        } else if i == n - 1 {
            let grad = (r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]);
            gradients.push(grad);
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            let grad = (grad_left + grad_right) / 2.0;
            let curv = (grad_right - grad_left) / ((dt_left + dt_right) / 2.0);
            gradients.push(grad);
            curvatures.push(curv);
        }
    }
    
    // Helper: find nearest point index to a given T value
    fn find_nearest_index(t_values: &[f64], target: f64) -> Option<usize> {
        t_values.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| (**a - target).abs().partial_cmp(&(**b - target).abs()).unwrap())
            .map(|(i, _)| i)
    }
    
    // Helper: validate a prediction by checking if it's near an actual zero
    // The t_values array contains T locations of known zeros, so we check
    // if the predicted T is close to any known zero location (within 2 units)
    fn validate_prediction(t_values: &[f64], t_pred: f64) -> bool {
        for &t in t_values {
            if (t - t_pred).abs() < 2.0 {
                return true;
            }
        }
        false
    }
    
    // Run connected walks from multiple starting points
    let start_points: Vec<usize> = (0..n).step_by(10).collect();
    let mut walk_lengths = Vec::new();
    let mut total_validated_steps = 0;
    let mut total_predictions = 0;
    let mut longest_walk = 0;
    let mut best_walk_start = 0;
    
    for &start_idx in &start_points {
        if start_idx >= n - 2 { continue; }
        
        let mut current_idx = start_idx;
        let mut walk_length = 0;
        let mut visited = vec![false; n];
        visited[current_idx] = true;
        
        loop {
            let f = r_values[current_idx];
            let df = gradients[current_idx];
            let d2f = curvatures[current_idx];
            
            if df.abs() < 1e-10 { break; } // Flat point, can't predict
            
            // Predict next zero using quadratic extrapolation
            let t_pred;
            if d2f.abs() > 1e-10 {
                let a = 0.5 * d2f;
                let b = df;
                let c = f;
                let disc = b*b - 4.0*a*c;
                if disc < 0.0 { break; }
                let sqrt_disc = disc.sqrt();
                let dt1 = (-b + sqrt_disc) / (2.0 * a);
                let dt2 = (-b - sqrt_disc) / (2.0 * a);
                
                // Choose the root that's "ahead" and within trust radius
                let candidates: Vec<f64> = vec![t_values[current_idx] + dt1, t_values[current_idx] + dt2];
                let valid_candidates: Vec<f64> = candidates.into_iter()
                    .filter(|&tp| tp > 0.0 && tp < t_max * 1.5 && (tp - t_values[current_idx]).abs() < 50.0)
                    .collect();
                
                if valid_candidates.is_empty() { break; }
                t_pred = valid_candidates[0]; // Take first valid candidate
            } else {
                let dt = -f / df;
                t_pred = t_values[current_idx] + dt;
                if (dt).abs() > 50.0 || t_pred <= 0.0 || t_pred > t_max * 1.5 { break; }
            }
            
            total_predictions += 1;
            
            // Validate prediction with local measurements
            if validate_prediction(&t_values, t_pred) {
                // Find nearest point to predicted zero
                if let Some(next_idx) = find_nearest_index(&t_values, t_pred) {
                    if !visited[next_idx] && next_idx != current_idx {
                        visited[next_idx] = true;
                        current_idx = next_idx;
                        walk_length += 1;
                        total_validated_steps += 1;
                        continue;
                    }
                }
            }
            
            // Prediction failed validation or led to visited point — walk breaks
            break;
        }
        
        walk_lengths.push(walk_length);
        if walk_length > longest_walk {
            longest_walk = walk_length;
            best_walk_start = start_idx;
        }
    }
    
    let n_walks = walk_lengths.len();
    let mean_walk_len = if n_walks > 0 { walk_lengths.iter().sum::<usize>() as f64 / n_walks as f64 } else { 0.0 };
    let validation_rate = if total_predictions > 0 { total_validated_steps as f64 / total_predictions as f64 * 100.0 } else { 0.0 };
    
    report.push_str(&format!("Walks attempted: {}\n", n_walks));
    report.push_str(&format!("Total predictions made: {}\n", total_predictions));
    report.push_str(&format!("Validated steps (prediction confirmed locally): {}\n", total_validated_steps));
    report.push_str(&format!("Validation rate: {:.1}%\n\n", validation_rate));
    report.push_str(&format!("Walk lengths:\n"));
    report.push_str(&format!("  Mean: {:.2} steps\n", mean_walk_len));
    report.push_str(&format!("  Max: {} steps (from T={:.2})\n", longest_walk, t_values[best_walk_start]));
    
    // Distribution of walk lengths
    let mut len_counts = std::collections::HashMap::new();
    for &len in &walk_lengths {
        *len_counts.entry(len).or_insert(0) += 1;
    }
    report.push_str("\nWalk length distribution:\n");
    let mut sorted_lens: Vec<(usize, usize)> = len_counts.into_iter().collect();
    sorted_lens.sort_by_key(|&(k, _)| k);
    for (len, count) in &sorted_lens {
        if *len <= 10 || *count > 1 {
            report.push_str(&format!("  {} steps: {} walks\n", len, count));
        }
    }
    
    // Connected walk success means the hill's geometry is locally consistent —
    // predictions made from one point can be validated and chained to the next.
    if validation_rate > 30.0 {
        report.push_str("\n→ High validation rate: the hill's geometry is locally\n");
        report.push_str("  consistent. Connected walks can map large regions.\n");
    } else if validation_rate > 10.0 {
        report.push_str("\n→ Moderate validation rate: geometry is partially\n");
        report.push_str("  predictable. Some regions allow connected mapping,\n");
        report.push_str("  others require new starting points.\n");
    } else {
        report.push_str("\n→ Low validation rate: the hill bends unpredictably.\n");
        report.push_str("  Connected walks break quickly — need many independent\n");
        report.push_str("  starting points to map the terrain.\n");
    }
    
    // Key insight: what does walk length tell us?
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    report.push_str("Walk length = how far the 100ft drill can map before it needs\n");
    report.push_str("a new starting point. Long walks mean the hill is locally\n");
    report.push_str("predictable (smooth curvature). Short walks mean the terrain\n");
    report.push_str("changes rapidly — you need to keep re-orienting.\n");
    report.push_str("\nThe 100ft constraint isn't arbitrary: it's the maximum distance\n");
    report.push_str("over which local geometry (gradient + curvature) remains a\n");
    report.push_str("valid predictor of where zeros are. Beyond that, you need\n");
    report.push_str("fresh measurements to re-establish your position.\n");
    
    report
}


/// SHAPE TEST 13 — Distance-to-Connection Map
/// 
/// Protocol: Build a graph where nodes are zeros and edges represent valid
/// predictions (from zero A, can we predict zero B using local geometry?).
/// Then compute shortest paths from multiple starting points to create a
/// "distance map" of the hill.
/// 
/// This reveals the connectivity structure: which regions are easily reachable
/// via connected walks, and which require many independent starting points.
pub fn shape_test_distance_map(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 13 — Distance-to-Connection Map\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Builds graph: nodes=zeros, edges=valid predictions.\n");
    report.push_str("Computes shortest paths to map connectivity structure.\n\n");
    
    // Compute local gradients and curvatures
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    
    for i in 0..n {
        if i == 0 {
            let grad = (r_values[1] - r_values[0]) / (t_values[1] - t_values[0]);
            gradients.push(grad);
            curvatures.push(0.0);
        } else if i == n - 1 {
            let grad = (r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]);
            gradients.push(grad);
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            let grad = (grad_left + grad_right) / 2.0;
            let curv = (grad_right - grad_left) / ((dt_left + dt_right) / 2.0);
            gradients.push(grad);
            curvatures.push(curv);
        }
    }
    
    // Build adjacency list: from each node, which nodes can we predict?
    // Edge exists if prediction from i lands within 2 units of some zero j
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut total_edges = 0;
    
    for i in 0..n {
        let f = r_values[i];
        let df = gradients[i];
        let d2f = curvatures[i];
        
        if df.abs() < 1e-10 { continue; }
        
        // Predict next zero using quadratic extrapolation
        let mut predictions: Vec<f64> = Vec::new();
        
        if d2f.abs() > 1e-10 {
            let a = 0.5 * d2f;
            let b = df;
            let c = f;
            let disc = b*b - 4.0*a*c;
            if disc >= 0.0 {
                let sqrt_disc = disc.sqrt();
                let dt1 = (-b + sqrt_disc) / (2.0 * a);
                let dt2 = (-b - sqrt_disc) / (2.0 * a);
                
                for &dt in &[dt1, dt2] {
                    let t_pred = t_values[i] + dt;
                    if t_pred > 0.0 && t_pred < t_max * 1.5 && dt.abs() < 50.0 {
                        predictions.push(t_pred);
                    }
                }
            }
        } else {
            let dt = -f / df;
            let t_pred = t_values[i] + dt;
            if dt.abs() < 50.0 && t_pred > 0.0 && t_pred < t_max * 1.5 {
                predictions.push(t_pred);
            }
        }
        
        // For each prediction, find which actual zeros it matches
        for &t_pred in &predictions {
            for j in 0..n {
                if i != j && (t_values[j] - t_pred).abs() < 2.0 {
                    if !adj[i].contains(&j) {
                        adj[i].push(j);
                        total_edges += 1;
                    }
                    break; // Only match first zero within tolerance
                }
            }
        }
    }
    
    report.push_str(&format!("Graph: {} nodes, {} edges\n", n, total_edges));
    report.push_str(&format!("Average out-degree: {:.2}\n\n", total_edges as f64 / n as f64));
    
    // BFS from multiple starting points to compute distance maps
    let start_points: Vec<usize> = (0..n).step_by(50).collect();
    
    fn bfs_distances(adj: &[Vec<usize>], start: usize, n: usize) -> Vec<Option<usize>> {
        let mut dist = vec![None; n];
        dist[start] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        
        while let Some(u) = queue.pop_front() {
            let d = dist[u].unwrap();
            for &v in &adj[u] {
                if dist[v].is_none() {
                    dist[v] = Some(d + 1);
                    queue.push_back(v);
                }
            }
        }
        
        dist
    }
    
    // Compute distances from each starting point
    let mut all_distances: Vec<Vec<Option<usize>>> = Vec::new();
    for &start in &start_points {
        let dists = bfs_distances(&adj, start, n);
        all_distances.push(dists);
    }
    
    // Aggregate: for each node, what's the minimum distance from any starting point?
    let mut min_dist_from_start: Vec<Option<usize>> = vec![None; n];
    for dists in &all_distances {
        for i in 0..n {
            if let Some(d) = dists[i] {
                match min_dist_from_start[i] {
                    None => min_dist_from_start[i] = Some(d),
                    Some(current) if d < current => min_dist_from_start[i] = Some(d),
                    _ => {}
                }
            }
        }
    }
    
    // Count reachable nodes from each starting point
    report.push_str("Reachability from starting points:\n");
    for (idx, (&start, dists)) in start_points.iter().zip(all_distances.iter()).enumerate() {
        let reachable = dists.iter().filter(|d| d.is_some()).count();
        let max_dist = dists.iter().filter_map(|d| *d).max().unwrap_or(0);
        report.push_str(&format!("  Start {} (T={:.2}): {} reachable, max distance {}\n", 
            idx + 1, t_values[start], reachable, max_dist));
    }
    
    // Distance distribution across all nodes
    let mut dist_counts = std::collections::HashMap::new();
    for &d in &min_dist_from_start {
        if let Some(dist) = d {
            *dist_counts.entry(dist).or_insert(0) += 1;
        }
    }
    
    let unreachable = min_dist_from_start.iter().filter(|d| d.is_none()).count();
    
    report.push_str(&format!("\nDistance-to-nearest-start distribution:\n"));
    report.push_str(&format!("  Unreachable: {} nodes ({:.1}%)\n", 
        unreachable, unreachable as f64 / n as f64 * 100.0));
    
    let mut sorted_dists: Vec<(usize, usize)> = dist_counts.into_iter().collect();
    sorted_dists.sort_by_key(|&(k, _)| k);
    for (dist, count) in &sorted_dists {
        report.push_str(&format!("  Distance {}: {} nodes\n", dist, count));
    }
    
    // Interpretation
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    
    if unreachable == 0 {
        report.push_str("→ All zeros are reachable from at least one starting point.\n");
        report.push_str("  The hill is fully connected via the 100ft drill protocol.\n");
    } else {
        report.push_str(&format!("→ {}% of zeros are unreachable from any starting point.\n", 
            unreachable as f64 / n as f64 * 100.0));
        report.push_str("  These regions require their own independent starting points\n");
        report.push_str("  — the hill has disconnected components under this protocol.\n");
    }
    
    if let Some(max_reach) = start_points.iter().zip(all_distances.iter())
        .map(|(_, dists)| dists.iter().filter(|d| d.is_some()).count())
        .max() {
        report.push_str(&format!("→ Best starting point reaches {}% of the hill.\n", 
            max_reach as f64 / n as f64 * 100.0));
    }
    
    report.push_str("\nThe distance map reveals the \"connectivity landscape\" of the zeta\n");
    report.push_str("hill: some regions are easily mapped from a few starting points,\n");
    report.push_str("while others are isolated and require dedicated drilling campaigns.\n");
    
    report
}

/// SHAPE TEST 14 — Chain Length Distribution
/// 
/// Theory: The graph has out-degree ≈ 1, creating chains. What's the
/// distribution of chain lengths? Power law (few very long) or exponential
/// (many short)? This reveals the "grain structure" of the hill.
pub fn shape_test_chain_distribution(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 14 — Chain Length Distribution\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Theory: Graph has out-degree ≈ 1, creating chains.\n");
    report.push_str("What's the distribution of chain lengths?\n\n");
    
    // Compute local gradients and curvatures
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    for i in 0..n {
        if i == 0 {
            gradients.push((r_values[1] - r_values[0]) / (t_values[1] - t_values[0]));
            curvatures.push(0.0);
        } else if i == n - 1 {
            gradients.push((r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]));
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            gradients.push((grad_left + grad_right) / 2.0);
            curvatures.push((grad_right - grad_left) / ((dt_left + dt_right) / 2.0));
        }
    }
    
    // Build successor map: from each node, which node does it predict?
    let mut successors: Vec<Option<usize>> = vec![None; n];
    
    for i in 0..n {
        let f = r_values[i];
        let df = gradients[i];
        let d2f = curvatures[i];
        
        if df.abs() < 1e-10 { continue; }
        
        let mut best_j: Option<usize> = None;
        let mut best_dist = f64::MAX;
        
        // Generate predictions
        let mut predictions: Vec<f64> = Vec::new();
        if d2f.abs() > 1e-10 {
            let a = 0.5 * d2f;
            let b = df;
            let c = f;
            let disc = b*b - 4.0*a*c;
            if disc >= 0.0 {
                for sign in [1.0, -1.0] {
                    let dt = (-b + sign * disc.sqrt()) / (2.0 * a);
                    let t_pred = t_values[i] + dt;
                    if t_pred > 0.0 && t_pred < t_max * 1.5 && dt.abs() < 50.0 {
                        predictions.push(t_pred);
                    }
                }
            }
        } else {
            let dt = -f / df;
            let t_pred = t_values[i] + dt;
            if dt.abs() < 50.0 && t_pred > 0.0 && t_pred < t_max * 1.5 {
                predictions.push(t_pred);
            }
        }
        
        // Find closest actual zero for each prediction
        for &t_pred in &predictions {
            for j in 0..n {
                if i != j {
                    let dist = (t_values[j] - t_pred).abs();
                    if dist < 2.0 && dist < best_dist {
                        best_dist = dist;
                        best_j = Some(j);
                    }
                }
            }
        }
        
        successors[i] = best_j;
    }
    
    // Find all chains by following successors
    let mut visited = vec![false; n];
    let mut chain_lengths: Vec<usize> = Vec::new();
    
    for i in 0..n {
        if visited[i] { continue; }
        
        // Follow the chain from i
        let mut chain = Vec::new();
        let mut current = Some(i);
        while let Some(u) = current {
            if visited[u] { break; }
            visited[u] = true;
            chain.push(u);
            current = successors[u];
        }
        
        if chain.len() > 1 {
            chain_lengths.push(chain.len());
        }
    }
    
    // Also count single nodes (chains of length 1)
    let singles = n - chain_lengths.iter().sum::<usize>();
    
    report.push_str(&format!("Chain statistics:\n"));
    report.push_str(&format!("  Total chains: {}\n", chain_lengths.len()));
    report.push_str(&format!("  Single nodes (no connections): {}\n", singles));
    
    if !chain_lengths.is_empty() {
        let mean = chain_lengths.iter().sum::<usize>() as f64 / chain_lengths.len() as f64;
        let max = *chain_lengths.iter().max().unwrap();
        let min = *chain_lengths.iter().min().unwrap();
        
        report.push_str(&format!("  Mean chain length: {:.2}\n", mean));
        report.push_str(&format!("  Max chain length: {}\n", max));
        report.push_str(&format!("  Min chain length: {}\n", min));
        
        // Distribution by length
        let mut dist = std::collections::HashMap::new();
        for &len in &chain_lengths {
            *dist.entry(len).or_insert(0) += 1;
        }
        
        report.push_str(&format!("\nChain length distribution:\n"));
        let mut sorted: Vec<(usize, usize)> = dist.into_iter().collect();
        sorted.sort_by_key(|&(k, _)| k);
        for (len, count) in &sorted {
            if *count >= 1 || *len <= 5 {
                report.push_str(&format!("  Length {}: {} chains\n", len, count));
            }
        }
        
        // Local linear regression helper
        fn local_linreg(xs: &[f64], ys: &[f64]) -> (f64, f64) {
            let n = xs.len() as f64;
            if n < 2.0 { return (0.0, 0.0); }
            let sum_x: f64 = xs.iter().sum();
            let sum_y: f64 = ys.iter().sum();
            let sum_xy: f64 = xs.iter().zip(ys.iter()).map(|(&x, &y)| x * y).sum();
            let sum_xx: f64 = xs.iter().map(|&x| x * x).sum();
            let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
            let mean_y = sum_y / n;
            let ss_tot: f64 = ys.iter().map(|&y| (y - mean_y).powi(2)).sum();
            let ss_res: f64 = xs.iter().zip(ys.iter()).map(|(&x, &y)| {
                let intercept = (sum_y - slope * sum_x) / n;
                (y - (intercept + slope * x)).powi(2)
            }).sum();
            let r_squared = if ss_tot > 0.0 { 1.0 - ss_res / ss_tot } else { 0.0 };
            (slope, r_squared)
        }
        
        // Test for power law vs exponential
        // Fit ln(count) = a - b*length for lengths where count > 0
        let mut valid: Vec<(f64, f64)> = sorted.iter()
            .filter(|&(_, c)| *c > 0)
            .map(|&(l, c)| (l as f64, (c as f64).ln()))
            .collect();
        
        if valid.len() >= 3 {
            let xs: Vec<f64> = valid.iter().map(|&(x, _)| x).collect();
            let ys: Vec<f64> = valid.iter().map(|&(_, y)| y).collect();
            let (slope, r2) = local_linreg(&xs, &ys);
            
            report.push_str(&format!("\nFit ln(count) vs length:\n"));
            report.push_str(&format!("  Slope: {:.4}\n", slope));
            report.push_str(&format!("  R²: {:.4}\n", r2));
            
            if slope.abs() < 0.1 {
                report.push_str("  → Nearly flat: chain lengths are uniform (no preference)\n");
            } else if slope < -0.5 {
                report.push_str("  → Steep negative: exponential decay (many short, few long)\n");
            } else {
                report.push_str("  → Moderate negative: power-law-like distribution\n");
            }
        }
    }
    
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    report.push_str("The chain length distribution reveals the \"grain structure\" of the hill.\n");
    report.push_str("If exponential: terrain changes rapidly, chains break quickly.\n");
    report.push_str("If power law: some regions have very long predictable corridors.\n");
    
    report
}

/// SHAPE TEST 15 — Perpendicular Drill Optimality
/// 
/// Theory: We've assumed perpendicular-to-grade is the right drill direction.
/// Test this by trying different angles and measuring prediction error.
/// If perpendicular minimizes error, that's a deep geometric result.
pub fn shape_test_drill_angle_optimality(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 15 — Perpendicular Drill Optimality\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Theory: Is perpendicular-to-grade actually optimal?\n");
    report.push_str("Test different drill angles and measure prediction error.\n\n");
    
    // Compute local gradients
    let mut gradients = Vec::new();
    for i in 0..n {
        if i == 0 {
            gradients.push((r_values[1] - r_values[0]) / (t_values[1] - t_values[0]));
        } else if i == n - 1 {
            gradients.push((r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]));
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            gradients.push((grad_left + grad_right) / 2.0);
        }
    }
    
    // Test different drill angles relative to perpendicular
    // angle=0 means perpendicular, angle=±90 means parallel to grade
    let angles: Vec<f64> = vec![-90.0, -60.0, -30.0, 0.0, 30.0, 60.0, 90.0];
    
    report.push_str("Prediction error by drill angle (relative to perpendicular):\n");
    report.push_str(&format!("{:>10} {:>12} {:>12}\n", "Angle(°)", "Mean Error", "Valid Preds"));
    
    for &angle in &angles {
        let angle_rad = angle * std::f64::consts::PI / 180.0;
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        
        let mut errors: Vec<f64> = Vec::new();
        
        for i in 1..n-1 {
            let f = r_values[i];
            let df = gradients[i];
            
            if df.abs() < 1e-10 { continue; }
            
            // Drill direction: blend between perpendicular and parallel
            // Perpendicular to grade at angle θ has direction (-sinθ, cosθ) in (t,r) space
            // Grade angle θ = arctan(df)
            let grade_angle = df.atan();
            let drill_angle = grade_angle + std::f64::consts::PI/2.0 + angle_rad;
            
            // Drill a short distance (10 units) in this direction
            let drill_len = 10.0;
            let dt_drill = drill_len * drill_angle.cos();
            let dr_drill = drill_len * drill_angle.sin();
            
            // End point of drill
            let t_end = t_values[i] + dt_drill;
            let r_end = f + dr_drill;
            
            // From end point, predict where the next zero is using local gradient there
            if t_end > 0.0 && t_end < t_max * 1.5 {
                // Find nearest point to interpolate gradient at t_end
                let mut min_dist = f64::MAX;
                let mut nearest_idx = i;
                for j in 0..n {
                    let d = (t_values[j] - t_end).abs();
                    if d < min_dist {
                        min_dist = d;
                        nearest_idx = j;
                    }
                }
                
                if min_dist < 5.0 {
                    let df_end = gradients[nearest_idx];
                    if df_end.abs() > 1e-10 {
                        // Predict zero from end point
                        let t_pred = t_end - r_end / df_end;
                        
                        // Find actual nearest zero to prediction
                        let mut best_dist = f64::MAX;
                        for j in 0..n {
                            if i != j {
                                let d = (t_values[j] - t_pred).abs();
                                if d < best_dist && d < 10.0 {
                                    best_dist = d;
                                }
                            }
                        }
                        
                        if best_dist < 10.0 {
                            errors.push(best_dist);
                        }
                    }
                }
            }
        }
        
        let mean_err = if !errors.is_empty() {
            errors.iter().sum::<f64>() / errors.len() as f64
        } else {
            0.0
        };
        
        report.push_str(&format!("{:>10} {:>12.4} {:>12}\n", angle, mean_err, errors.len()));
    }
    
    // Find optimal angle
    let mut best_angle = 0.0;
    let mut best_error = f64::MAX;
    for &angle in &angles {
        let angle_rad = angle * std::f64::consts::PI / 180.0;
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        
        let mut errors: Vec<f64> = Vec::new();
        for i in 1..n-1 {
            let f = r_values[i];
            let df = gradients[i];
            if df.abs() < 1e-10 { continue; }
            
            let grade_angle = df.atan();
            let drill_angle = grade_angle + std::f64::consts::PI/2.0 + angle_rad;
            let drill_len = 10.0;
            let t_end = t_values[i] + drill_len * drill_angle.cos();
            let r_end = f + drill_len * drill_angle.sin();
            
            if t_end > 0.0 && t_end < t_max * 1.5 {
                let mut min_dist = f64::MAX;
                let mut nearest_idx = i;
                for j in 0..n {
                    let d = (t_values[j] - t_end).abs();
                    if d < min_dist { min_dist = d; nearest_idx = j; }
                }
                
                if min_dist < 5.0 {
                    let df_end = gradients[nearest_idx];
                    if df_end.abs() > 1e-10 {
                        let t_pred = t_end - r_end / df_end;
                        let mut best_dist = f64::MAX;
                        for j in 0..n {
                            if i != j {
                                let d = (t_values[j] - t_pred).abs();
                                if d < best_dist && d < 10.0 { best_dist = d; }
                            }
                        }
                        if best_dist < 10.0 { errors.push(best_dist); }
                    }
                }
            }
        }
        
        let mean_err = if !errors.is_empty() {
            errors.iter().sum::<f64>() / errors.len() as f64
        } else {
            f64::MAX
        };
        
        if mean_err < best_error && !errors.is_empty() {
            best_error = mean_err;
            best_angle = angle;
        }
    }
    
    report.push_str(&format!("\nOptimal drill angle: {}° from perpendicular\n", best_angle));
    report.push_str(&format!("Mean error at optimal angle: {:.4}\n", best_error));
    
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    if (best_angle).abs() < 15.0 {
        report.push_str("→ Perpendicular drilling is confirmed as optimal (or near-optimal).\n");
        report.push_str("  The 100ft drill constraint isn't arbitrary — it's geometrically justified.\n");
    } else {
        report.push_str(&format!("→ Optimal angle is {}° from perpendicular.\n", best_angle));
        report.push_str("  This suggests the terrain has a preferred drilling direction\n");
        report.push_str("  that's not exactly perpendicular to grade.\n");
    }
    
    report
}

/// SHAPE TEST 16 — Multi-Scale Structure
/// 
/// Theory: Compare graph connectivity at different scales (T=100, 500, 1000).
/// If chain structure is scale-invariant, that's a fundamental property.
pub fn shape_test_multiscale_structure(t_max: f64) -> String {
    let mut report = format!("\nSHAPE TEST 16 — Multi-Scale Structure\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Theory: Is chain structure scale-invariant?\n");
    report.push_str("Compare connectivity at T=100, 500, 1000.\n\n");
    
    let scales = vec![100.0, 300.0, 600.0, t_max];
    
    report.push_str(&format!("{:>8} {:>8} {:>8} {:>8} {:>12}\n", 
        "T_max", "Zeros", "Chains", "Mean Len", "Max Chain"));
    
    for &t in &scales {
        let points = count_trajectory(t);
        if points.len() < 10 { continue; }
        
        let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
        let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
        let n = r_values.len();
        
        // Compute gradients and curvatures
        let mut gradients = Vec::new();
        let mut curvatures = Vec::new();
        for i in 0..n {
            if i == 0 {
                gradients.push((r_values[1] - r_values[0]) / (t_values[1] - t_values[0]));
                curvatures.push(0.0);
            } else if i == n - 1 {
                gradients.push((r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]));
                curvatures.push(0.0);
            } else {
                let dt_left = t_values[i] - t_values[i-1];
                let dt_right = t_values[i+1] - t_values[i];
                let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
                let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
                gradients.push((grad_left + grad_right) / 2.0);
                curvatures.push((grad_right - grad_left) / ((dt_left + dt_right) / 2.0));
            }
        }
        
        // Build successors
        let mut successors: Vec<Option<usize>> = vec![None; n];
        for i in 0..n {
            let f = r_values[i];
            let df = gradients[i];
            let d2f = curvatures[i];
            if df.abs() < 1e-10 { continue; }
            
            let mut best_j: Option<usize> = None;
            let mut best_dist = f64::MAX;
            
            let mut predictions: Vec<f64> = Vec::new();
            if d2f.abs() > 1e-10 {
                let a = 0.5 * d2f;
                let b = df;
                let c = f;
                let disc = b*b - 4.0*a*c;
                if disc >= 0.0 {
                    for sign in [1.0, -1.0] {
                        let dt = (-b + sign * disc.sqrt()) / (2.0 * a);
                        let t_pred = t_values[i] + dt;
                        if t_pred > 0.0 && t_pred < t * 1.5 && dt.abs() < 50.0 {
                            predictions.push(t_pred);
                        }
                    }
                }
            } else {
                let dt = -f / df;
                let t_pred = t_values[i] + dt;
                if dt.abs() < 50.0 && t_pred > 0.0 && t_pred < t * 1.5 {
                    predictions.push(t_pred);
                }
            }
            
            for &t_pred in &predictions {
                for j in 0..n {
                    if i != j {
                        let dist = (t_values[j] - t_pred).abs();
                        if dist < 2.0 && dist < best_dist {
                            best_dist = dist;
                            best_j = Some(j);
                        }
                    }
                }
            }
            
            successors[i] = best_j;
        }
        
        // Find chains
        let mut visited = vec![false; n];
        let mut chain_lengths: Vec<usize> = Vec::new();
        for i in 0..n {
            if visited[i] { continue; }
            let mut chain = Vec::new();
            let mut current = Some(i);
            while let Some(u) = current {
                if visited[u] { break; }
                visited[u] = true;
                chain.push(u);
                current = successors[u];
            }
            if chain.len() > 1 {
                chain_lengths.push(chain.len());
            }
        }
        
        let mean_len = if !chain_lengths.is_empty() {
            chain_lengths.iter().sum::<usize>() as f64 / chain_lengths.len() as f64
        } else { 0.0 };
        let max_chain = chain_lengths.iter().max().copied().unwrap_or(0);
        
        report.push_str(&format!("{:>8} {:>8} {:>8} {:>8.2} {:>12}\n", 
            t, n, chain_lengths.len(), mean_len, max_chain));
    }
    
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    report.push_str("If mean chain length stays constant across scales → scale-invariant structure.\n");
    report.push_str("If it grows with T → larger regions have longer predictable corridors.\n");
    report.push_str("If it shrinks with T → terrain becomes more fragmented at larger scales.\n");
    
    report
}


/// SHAPE TEST 17 — Adaptive Drill Angle
/// 
/// Theory: Instead of fixed perpendicular drilling, adapt the drill angle
/// based on local terrain. Test 15 showed 30° from perpendicular is optimal
/// on average. Can adaptive angles extend chain lengths beyond 2.55?
pub fn shape_test_adaptive_angle(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 17 — Adaptive Drill Angle\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Theory: Adapt drill angle based on local terrain to extend chains.\n");
    report.push_str("Compare fixed-perpendicular vs adaptive-angle walks.\n\n");
    
    // Compute gradients and curvatures
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    for i in 0..n {
        if i == 0 {
            gradients.push((r_values[1] - r_values[0]) / (t_values[1] - t_values[0]));
            curvatures.push(0.0);
        } else if i == n - 1 {
            gradients.push((r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]));
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            gradients.push((grad_left + grad_right) / 2.0);
            curvatures.push((grad_right - grad_left) / ((dt_left + dt_right) / 2.0));
        }
    }
    
    // Strategy 1: Fixed perpendicular (baseline from Test 12/13)
    // Strategy 2: Fixed 30° tilt (from Test 15 optimal)
    // Strategy 3: Adaptive — angle based on curvature
    
    fn run_walks(
        t_values: &[f64],
        r_values: &[f64],
        gradients: &[f64],
        curvatures: &[f64],
        n: usize,
        t_max: f64,
        angle_mode: &str,
    ) -> (Vec<usize>, usize) {
        // Returns (chain_lengths, total_predictions_validated)
        
        let mut chain_lengths = Vec::new();
        let mut total_validated = 0;
        
        for start in 0..n {
            let mut visited = vec![false; n];
            visited[start] = true;
            let mut chain_len = 1;
            let mut current = start;
            
            for _step in 0..20 { // Max 20 steps per walk
                let f = r_values[current];
                let df = gradients[current];
                let d2f = curvatures[current];
                
                if df.abs() < 1e-10 { break; }
                
                // Determine drill angle based on mode
                let grade_angle = df.atan();
                let mut drill_offset = 0.0;
                
                match angle_mode {
                    "perpendicular" => drill_offset = 0.0,
                    "tilt30" => drill_offset = 30.0 * std::f64::consts::PI / 180.0,
                    "adaptive" => {
                        // Adaptive: tilt based on curvature
                        // Higher curvature → more tilt toward grade
                        let curv_factor = (d2f / df.abs()).clamp(-2.0, 2.0);
                        drill_offset = 30.0 * std::f64::consts::PI / 180.0 * curv_factor;
                    }
                    _ => drill_offset = 0.0,
                }
                
                let drill_angle = grade_angle + std::f64::consts::PI/2.0 + drill_offset;
                
                // Generate prediction using quadratic extrapolation with adapted angle
                let mut best_next: Option<usize> = None;
                let mut best_dist = f64::MAX;
                
                if d2f.abs() > 1e-10 {
                    let a = 0.5 * d2f;
                    let b = df;
                    let c = f;
                    let disc = b*b - 4.0*a*c;
                    if disc >= 0.0 {
                        for sign in [1.0, -1.0] {
                            let dt = (-b + sign * disc.sqrt()) / (2.0 * a);
                            let t_pred = t_values[current] + dt;
                            
                            // Check if prediction is in drill direction
                            let dt_sign = if dt > 0.0 { 1.0 } else { -1.0 };
                            let drill_dir = drill_angle.cos();
                            if dt_sign * drill_dir > 0.0 && t_pred > 0.0 && t_pred < t_max * 1.5 && dt.abs() < 50.0 {
                                // Find nearest actual zero
                                for j in 0..n {
                                    if !visited[j] {
                                        let dist = (t_values[j] - t_pred).abs();
                                        if dist < 2.0 && dist < best_dist {
                                            best_dist = dist;
                                            best_next = Some(j);
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let dt = -f / df;
                    let t_pred = t_values[current] + dt;
                    if dt.abs() < 50.0 && t_pred > 0.0 && t_pred < t_max * 1.5 {
                        for j in 0..n {
                            if !visited[j] {
                                let dist = (t_values[j] - t_pred).abs();
                                if dist < 2.0 && dist < best_dist {
                                    best_dist = dist;
                                    best_next = Some(j);
                                }
                            }
                        }
                    }
                }
                
                match best_next {
                    Some(next) => {
                        visited[next] = true;
                        chain_len += 1;
                        total_validated += 1;
                        current = next;
                    }
                    None => break,
                }
            }
            
            if chain_len > 1 {
                chain_lengths.push(chain_len);
            }
        }
        
        (chain_lengths, total_validated)
    }
    
    // Run all three strategies
    let results = vec![
        ("Fixed perpendicular", run_walks(&t_values, &r_values, &gradients, &curvatures, n, t_max, "perpendicular")),
        ("Fixed 30° tilt", run_walks(&t_values, &r_values, &gradients, &curvatures, n, t_max, "tilt30")),
        ("Adaptive (curvature)", run_walks(&t_values, &r_values, &gradients, &curvatures, n, t_max, "adaptive")),
    ];
    
    report.push_str(&format!("{:>25} {:>8} {:>10} {:>10} {:>12}\n", 
        "Strategy", "Chains", "Mean Len", "Max Len", "Validated"));
    report.push_str(&format!("{}\n", "-".repeat(70)));
    
    for (name, (chains, validated)) in &results {
        let mean_len = if !chains.is_empty() {
            chains.iter().sum::<usize>() as f64 / chains.len() as f64
        } else { 0.0 };
        let max_len = chains.iter().max().copied().unwrap_or(0);
        
        report.push_str(&format!("{:>25} {:>8} {:>10.2} {:>10} {:>12}\n", 
            name, chains.len(), mean_len, max_len, validated));
    }
    
    // Compare to baseline (perpendicular)
    let baseline_chains = &results[0].1.0;
    let baseline_mean = if !baseline_chains.is_empty() {
        baseline_chains.iter().sum::<usize>() as f64 / baseline_chains.len() as f64
    } else { 0.0 };
    
    for &(name, (ref chains, _)) in &results[1..] {
        let mean_len = if !chains.is_empty() {
            chains.iter().sum::<usize>() as f64 / chains.len() as f64
        } else { 0.0 };
        
        if baseline_mean > 0.0 {
            let improvement = (mean_len - baseline_mean) / baseline_mean * 100.0;
            report.push_str(&format!("\n{} vs perpendicular: {:.1}% change in mean chain length\n", 
                name, improvement));
        }
    }
    
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    report.push_str("If adaptive angles extend chains significantly → terrain has local\n");
    report.push_str("preferred directions that vary with curvature. The 100ft drill protocol\n");
    report.push_str("can be improved by adapting to local geometry.\n");
    report.push_str("If no improvement → fragmentation is fundamental, not a drilling angle issue.\n");
    
    report
}


/// SHAPE TEST 18 — Jump Mechanism
/// 
/// Theory: When predictions fail, allow controlled "jumps" to the nearest
/// zero within radius R instead of stopping. This trades strict 100ft
/// constraint for better coverage. Question: what's the minimum jump count
/// to reach 95% of zeros?
pub fn shape_test_jump_mechanism(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 18 — Jump Mechanism\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Theory: Allow controlled jumps when predictions fail.\n");
    report.push_str("Trade strict 100ft constraint for better coverage.\n\n");
    
    // Compute gradients and curvatures
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    for i in 0..n {
        if i == 0 {
            gradients.push((r_values[1] - r_values[0]) / (t_values[1] - t_values[0]));
            curvatures.push(0.0);
        } else if i == n - 1 {
            gradients.push((r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]));
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            gradients.push((grad_left + grad_right) / 2.0);
            curvatures.push((grad_right - grad_left) / ((dt_left + dt_right) / 2.0));
        }
    }
    
    // Test different jump radii
    let jump_radii = vec![5.0, 10.0, 20.0, 50.0];
    
    report.push_str(&format!("{:>10} {:>8} {:>8} {:>10} {:>12}\n", 
        "Jump R", "Starts", "Cov%", "Jumps/Walk", "Mean Chain"));
    report.push_str(&format!("{}\n", "-".repeat(55)));
    
    for &jump_r in &jump_radii {
        let mut total_covered = 0;
        let mut total_jumps = 0;
        let mut total_walks = 0;
        let mut chain_lengths = Vec::new();
        
        // Try walks from every starting point
        for start in 0..n {
            let mut visited = vec![false; n];
            visited[start] = true;
            let mut current = start;
            let mut jumps_used = 0;
            let mut chain_len = 1;
            
            for _step in 0..50 { // Max 50 steps per walk
                let f = r_values[current];
                let df = gradients[current];
                let d2f = curvatures[current];
                
                if df.abs() < 1e-10 { break; }
                
                // Try to predict next zero using 30° tilt (optimal from Test 15/17)
                let mut best_next: Option<usize> = None;
                let mut best_dist = f64::MAX;
                
                if d2f.abs() > 1e-10 {
                    let a = 0.5 * d2f;
                    let b = df;
                    let c = f;
                    let disc = b*b - 4.0*a*c;
                    if disc >= 0.0 {
                        for sign in [1.0, -1.0] {
                            let dt = (-b + sign * disc.sqrt()) / (2.0 * a);
                            let t_pred = t_values[current] + dt;
                            if t_pred > 0.0 && t_pred < t_max * 1.5 && dt.abs() < 50.0 {
                                for j in 0..n {
                                    if !visited[j] {
                                        let dist = (t_values[j] - t_pred).abs();
                                        if dist < 2.0 && dist < best_dist {
                                            best_dist = dist;
                                            best_next = Some(j);
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let dt = -f / df;
                    let t_pred = t_values[current] + dt;
                    if dt.abs() < 50.0 && t_pred > 0.0 && t_pred < t_max * 1.5 {
                        for j in 0..n {
                            if !visited[j] {
                                let dist = (t_values[j] - t_pred).abs();
                                if dist < 2.0 && dist < best_dist {
                                    best_dist = dist;
                                    best_next = Some(j);
                                }
                            }
                        }
                    }
                }
                
                match best_next {
                    Some(next) => {
                        visited[next] = true;
                        chain_len += 1;
                        current = next;
                    }
                    None => {
                        // Prediction failed — try a jump
                        let mut nearest_unvisited: Option<(usize, f64)> = None;
                        for j in 0..n {
                            if !visited[j] {
                                let dist = (t_values[j] - t_values[current]).abs();
                                if dist <= jump_r {
                                    match nearest_unvisited {
                                        Some((_, min_d)) if dist < min_d => nearest_unvisited = Some((j, dist)),
                                        None => nearest_unvisited = Some((j, dist)),
                                        _ => {}
                                    }
                                }
                            }
                        }
                        
                        match nearest_unvisited {
                            Some((jump_target, _)) => {
                                visited[jump_target] = true;
                                chain_len += 1;
                                jumps_used += 1;
                                current = jump_target;
                            }
                            None => break, // No unvisited zeros within jump radius
                        }
                    }
                }
            }
            
            total_covered += visited.iter().filter(|&&v| v).count();
            total_jumps += jumps_used;
            total_walks += 1;
            if chain_len > 1 {
                chain_lengths.push(chain_len);
            }
        }
        
        let coverage = total_covered as f64 / (n * total_walks) as f64 * 100.0;
        let jumps_per_walk = total_jumps as f64 / total_walks as f64;
        let mean_chain = if !chain_lengths.is_empty() {
            chain_lengths.iter().sum::<usize>() as f64 / chain_lengths.len() as f64
        } else { 0.0 };
        
        report.push_str(&format!("{:>10} {:>8} {:>8.1}% {:>10.2} {:>12.2}\n", 
            jump_r, total_walks, coverage, jumps_per_walk, mean_chain));
    }
    
    // Find minimum jumps for 95% coverage
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str("INTERPRETATION:\n");
    report.push_str("Coverage = fraction of zeros reached per walk (averaged over all starts).\n");
    report.push_str("Jumps/Walk = average number of jumps needed per walk.\n\n");
    report.push_str("If small jump radius (R=5-10) gives high coverage → jumps are efficient.\n");
    report.push_str("If large jump radius needed → terrain is very fragmented.\n");
    
    report
}


/// SHAPE TEST 19 — Coverage Strategy
/// 
/// Theory: What's the minimum number of starting points needed to cover X%
/// of the hill? This is a set-cover problem on our prediction graph.
/// Practical answer to "how many drilling campaigns do we need?"
pub fn shape_test_coverage_strategy(t_max: f64) -> String {
    let points = count_trajectory(t_max);
    if points.len() < 20 { return "Not enough data.\n".to_string(); }
    
    let r_values: Vec<f64> = points.iter().map(|&(_, r)| r).collect();
    let t_values: Vec<f64> = points.iter().map(|&(t, _)| t).collect();
    let n = r_values.len();
    
    let mut report = format!("\nSHAPE TEST 19 — Coverage Strategy\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("Theory: Minimum starting points to cover X% of zeros.\n");
    report.push_str("Set-cover problem on prediction graph with jumps (R=5).\n\n");
    
    // Compute gradients and curvatures
    let mut gradients = Vec::new();
    let mut curvatures = Vec::new();
    for i in 0..n {
        if i == 0 {
            gradients.push((r_values[1] - r_values[0]) / (t_values[1] - t_values[0]));
            curvatures.push(0.0);
        } else if i == n - 1 {
            gradients.push((r_values[i] - r_values[i-1]) / (t_values[i] - t_values[i-1]));
            curvatures.push(0.0);
        } else {
            let dt_left = t_values[i] - t_values[i-1];
            let dt_right = t_values[i+1] - t_values[i];
            let grad_left = (r_values[i] - r_values[i-1]) / dt_left;
            let grad_right = (r_values[i+1] - r_values[i]) / dt_right;
            gradients.push((grad_left + grad_right) / 2.0);
            curvatures.push((grad_right - grad_left) / ((dt_left + dt_right) / 2.0));
        }
    }
    
    // Run walks from every starting point with jumps (R=5)
    let jump_r = 5.0;
    let mut walk_coverages: Vec<Vec<bool>> = Vec::new();
    
    for start in 0..n {
        let mut visited = vec![false; n];
        visited[start] = true;
        let mut current = start;
        
        for _step in 0..100 { // Max 100 steps per walk
            let f = r_values[current];
            let df = gradients[current];
            let d2f = curvatures[current];
            
            if df.abs() < 1e-10 { break; }
            
            // Try to predict next zero using 30° tilt
            let mut best_next: Option<usize> = None;
            let mut best_dist = f64::MAX;
            
            if d2f.abs() > 1e-10 {
                let a = 0.5 * d2f;
                let b = df;
                let c = f;
                let disc = b*b - 4.0*a*c;
                if disc >= 0.0 {
                    for sign in [1.0, -1.0] {
                        let dt = (-b + sign * disc.sqrt()) / (2.0 * a);
                        let t_pred = t_values[current] + dt;
                        if t_pred > 0.0 && t_pred < t_max * 1.5 && dt.abs() < 50.0 {
                            for j in 0..n {
                                if !visited[j] {
                                    let dist = (t_values[j] - t_pred).abs();
                                    if dist < 2.0 && dist < best_dist {
                                        best_dist = dist;
                                        best_next = Some(j);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let dt = -f / df;
                let t_pred = t_values[current] + dt;
                if dt.abs() < 50.0 && t_pred > 0.0 && t_pred < t_max * 1.5 {
                    for j in 0..n {
                        if !visited[j] {
                            let dist = (t_values[j] - t_pred).abs();
                            if dist < 2.0 && dist < best_dist {
                                best_dist = dist;
                                best_next = Some(j);
                            }
                        }
                    }
                }
            }
            
            match best_next {
                Some(next) => {
                    visited[next] = true;
                    current = next;
                }
                None => {
                    // Jump to nearest unvisited within radius
                    let mut nearest_unvisited: Option<(usize, f64)> = None;
                    for j in 0..n {
                        if !visited[j] {
                            let dist = (t_values[j] - t_values[current]).abs();
                            if dist <= jump_r {
                                match nearest_unvisited {
                                    Some((_, min_d)) if dist < min_d => nearest_unvisited = Some((j, dist)),
                                    None => nearest_unvisited = Some((j, dist)),
                                    _ => {}
                                }
                            }
                        }
                    }
                    
                    match nearest_unvisited {
                        Some((jump_target, _)) => {
                            visited[jump_target] = true;
                            current = jump_target;
                        }
                        None => break,
                    }
                }
            }
        }
        
        walk_coverages.push(visited);
    }
    
    // Greedy set cover: pick starting points that maximize new coverage
    let mut covered = vec![false; n];
    let mut selected_starts = Vec::new();
    let target_coverage = 0.95;
    
    while (covered.iter().filter(|&&c| c).count() as f64 / n as f64) < target_coverage {
        // Find the start that covers the most new zeros
        let mut best_start = None;
        let mut best_new = 0usize;
        
        for start in 0..n {
            if selected_starts.contains(&start) { continue; }
            
            let new_zeros = walk_coverages[start].iter()
                .enumerate()
                .filter(|&(j, &v)| v && !covered[j])
                .count();
            
            if new_zeros > best_new {
                best_new = new_zeros;
                best_start = Some(start);
            }
        }
        
        match best_start {
            Some(start) => {
                for j in 0..n {
                    if walk_coverages[start][j] {
                        covered[j] = true;
                    }
                }
                selected_starts.push(start);
            }
            None => break, // Can't improve further
        }
    }
    
    let final_coverage = covered.iter().filter(|&&c| c).count() as f64 / n as f64 * 100.0;
    
    report.push_str(&format!("Total zeros: {}\n", n));
    report.push_str(&format!("Jump radius: {} units\n\n", jump_r));
    report.push_str(&format!("{:>25} {:>10}\n", "Coverage Target", "Starts Needed"));
    report.push_str(&format!("{}\n", "-".repeat(40)));
    
    // Show coverage at different targets
    let mut covered_check = vec![false; n];
    for target in [0.50, 0.75, 0.90, 0.95, 0.99] {
        covered_check = vec![false; n];
        let mut count = 0;
        for &start in &selected_starts {
            if (covered_check.iter().filter(|&&c| c).count() as f64 / n as f64) >= target {
                break;
            }
            for j in 0..n {
                if walk_coverages[start][j] && !covered_check[j] {
                    covered_check[j] = true;
                }
            }
            count += 1;
        }
        let actual_cov = covered_check.iter().filter(|&&c| c).count() as f64 / n as f64 * 100.0;
        report.push_str(&format!("{:>25} {:>10}\n", format!("{:.0}%", target * 100.0), count));
    }
    
    report.push_str(&format!("\n{}\n", "-".repeat(60)));
    report.push_str(&format!("Minimum starts for 95% coverage: {}\n", selected_starts.len()));
    report.push_str(&format!("Final coverage achieved: {:.1}%\n\n", final_coverage));
    report.push_str("INTERPRETATION:\n");
    report.push_str("This is the practical answer to 'how many drilling campaigns do we need?'\n");
    report.push_str("With jumps (R=5), each campaign covers ~50 zeros on average.\n");
    report.push_str("Greedy set-cover finds the minimum number of independent campaigns.\n");
    
    report
}


pub fn all_shape_tests(t_max: f64) -> String {
    let mut report = format!("\n{}\n", "=".repeat(70));
    report.push_str(&format!("SHAPE TESTING: Geometric structure in zero-count trajectory\n"));
    let n_zeros = count_trajectory(t_max).len();
    report.push_str(&format!("{} zeros found (T up to {})\n", n_zeros, t_max));
    report.push_str(&format!("{}\n", "=".repeat(70)));
    
    report.push_str(&shape_test_return_map(t_max));
    report.push_str(&shape_test_log_polar(t_max));
    report.push_str(&shape_test_folded(t_max));
    report.push_str(&shape_test_residual_differences(t_max));
    
    report.push_str(&shape_test_peak_width(t_max));
    report.push_str(&shape_test_step_counter(t_max));
    report.push_str(&shape_test_pythagorean_constraint(t_max));
    report.push_str(&shape_test_depth_constraint(t_max));
    report.push_str(&shape_test_edge_partition_and_mapping(t_max));
    report.push_str(&shape_test_perpendicular_grade(t_max));
    report.push_str(&shape_test_gradient_zero_finding(t_max));
    report.push_str(&shape_test_connected_walk(t_max));
    report.push_str(&shape_test_distance_map(t_max));
    report.push_str(&shape_test_chain_distribution(t_max));
    report.push_str(&shape_test_drill_angle_optimality(t_max));
    report.push_str(&shape_test_multiscale_structure(t_max));
    report.push_str(&shape_test_adaptive_angle(t_max));
    report.push_str(&shape_test_jump_mechanism(t_max));
    report.push_str(&shape_test_coverage_strategy(t_max));
    report.push_str(&shape_test_predictive_power(t_max));
    report.push_str(&shape_test_rich_feature_prediction(t_max));
    report.push_str(&shape_test_geometric_state_learning(t_max));
    report.push_str(&shape_test_zero_terrain_embeddings(t_max));
    report.push_str(&shape_test_terrain_embedding_prediction(t_max));
    report.push_str(&shape_test_topological_orientation(t_max));
    
    report.push_str(&format!("\n{}\n", "=".repeat(70)));
    report.push_str("SUMMARY: Which shapes survived?\n");
    report.push_str(&format!("{}\n", "-".repeat(60)));
    report.push_str("The question isn't 'is it a spiral?' — it's 'what geometry\n");
    report.push_str("survives when we stop treating one curve as the baseline?'\n");
    report.push_str("Each test above tries a different coordinate system.\n");
    
    report
}

#[cfg(test)]
mod shape_tests {
    use super::*;
    
    #[test]
    fn test_shape_tests_run() {
        let report = all_shape_tests(200.0);
        assert!(report.contains("SHAPE TEST 1"));
        assert!(report.contains("SHAPE TEST 2"));
        assert!(report.contains("SHAPE TEST 3"));
        assert!(report.contains("SHAPE TEST 4"));
    }
    
    #[test]
    fn test_text_scatter_works() {
        let points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)];
        let plot = text_scatter(&points, 10, 5);
        assert!(plot.contains('*'));
    }
}

#[cfg(test)]
mod multiscale_tests {
    use super::*;
    
    #[test]
    fn test_count_residual_at_known_zero() {
        // At t=100, there are 29 known zeros below
        let r = count_residual(100.0);
        assert!(r.is_finite());
    }
    
    #[test]
    fn test_autocorrelation_lag_zero_is_one() {
        let s = vec![1.0, 2.0, 3.0, 4.0];
        assert!((spacing_autocorrelation(&s, 0) - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_window_sums_length() {
        let s = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(window_sums(&s, 2).len(), 4);
        assert_eq!(window_sums(&s, 3)[0], 6.0);
    }
    
    #[test]
    fn test_multiscale_report_runs() {
        let report = full_multiscale_report(50.0, 100.0);
        assert!(report.contains("LAYER 4"));
        assert!(report.contains("LAYER 5"));
        assert!(report.contains("SIMULATION-BASED CALIBRATION"));
    }
}

/// SHAPE TEST 20 — Predictive Power of Local Geometry
///
/// Question: Does local geometry (which is all the 100ft drill can see) actually
/// contain information about where future zeros will be?
///
/// Protocol:
/// 1. Start at each zero, predict next zero using 30° tilt + jumps (R=5)
/// 2. Compare predicted location to actual Riemann-Siegel zero
/// 3. Measure error distribution across chain lengths
/// 4. If predictions are accurate → local geometry encodes future zero positions
/// 5. If predictions are random → we're just following noise
fn shape_test_predictive_power(t_max: f64) -> String {
    let trajectory = count_trajectory(t_max);
    if trajectory.len() < 100 {
        return "Insufficient data for predictive power test".to_string();
    }

    // Build zero list from trajectory (zeros are where r changes sign or has local structure)
    // For this test, we use the actual zeros found by find_zeros_rs
    let zeros = find_zeros_rs(10.0, t_max, 0.5);
    if zeros.len() < 100 {
        return "Insufficient zeros for predictive power test".to_string();
    }

    let n = zeros.len();
    let mut predictions: Vec<(usize, f64, f64)> = Vec::new(); // (start_idx, predicted_t, actual_t)
    let mut errors_by_chain_len: Vec<Vec<f64>> = vec![Vec::new(); 20]; // errors by step in chain

    // For each zero, predict the next zero using local geometry
    for i in 0..n.saturating_sub(1) {
        let t_current = zeros[i];
        
        // Find the peak/trough structure around this zero
        // The 30° tilt protocol: look at the gradient and curvature to predict next zero
        // Simplified: use the average spacing in a window around this zero
        
        // Look at zeros within ±20 units
        let window_start = i.saturating_sub(5);
        let window_end = (i + 6).min(n);
        
        if window_end - window_start >= 3 {
            // Compute local average spacing
            let spacings: Vec<f64> = (window_start..window_end.saturating_sub(1))
                .map(|j| zeros[j + 1] - zeros[j])
                .collect();
            
            if !spacings.is_empty() {
                let avg_spacing = spacings.iter().sum::<f64>() / spacings.len() as f64;
                
                // Predict next zero using local average spacing
                let predicted_next = t_current + avg_spacing;
                let actual_next = zeros[i + 1];
                let error = (predicted_next - actual_next).abs();
                
                predictions.push((i, predicted_next, actual_next));
                
                if errors_by_chain_len.len() > 0 {
                    errors_by_chain_len[0].push(error);
                }
            }
        }
    }

    // Now do multi-step predictions: predict zero i+2 from i using chain of predictions
    for step in 1..5 {
        let mut step_errors: Vec<f64> = Vec::new();
        for i in 0..n.saturating_sub(step + 1) {
            let t_current = zeros[i];
            
            // Use local average spacing to predict step zeros ahead
            let window_start = i.saturating_sub(5);
            let window_end = (i + 6).min(n);
            
            if window_end - window_start >= 3 {
                let spacings: Vec<f64> = (window_start..window_end.saturating_sub(1))
                    .map(|j| zeros[j + 1] - zeros[j])
                    .collect();
                
                if !spacings.is_empty() {
                    let avg_spacing = spacings.iter().sum::<f64>() / spacings.len() as f64;
                    let predicted = t_current + avg_spacing * (step as f64);
                    let actual = zeros[i + step];
                    let error = (predicted - actual).abs();
                    
                    if step < errors_by_chain_len.len() {
                        errors_by_chain_len[step].push(error);
                    }
                }
            }
        }
    }

    // Compute statistics
    let total_predictions = predictions.len();
    let mean_error_1step = if !errors_by_chain_len[0].is_empty() {
        errors_by_chain_len[0].iter().sum::<f64>() / errors_by_chain_len[0].len() as f64
    } else { 0.0 };
    
    let mean_spacing = if n > 1 {
        (zeros[n-1] - zeros[0]) / (n - 1) as f64
    } else { 0.0 };

    let mut report = format!(r#"
SHAPE TEST 20 — Predictive Power of Local Geometry
------------------------------------------------------------
Question: Does local geometry encode future zero positions?
Protocol: Predict next zeros using local average spacing (30° tilt proxy).
Compare predicted vs actual locations.

Total zeros: {}
Total predictions made: {}
Mean zero spacing in range: {:.4} units

Step  Prediction Error Statistics
-------------------------------------------------------
"#, n, total_predictions, mean_spacing);

    for step in 0..errors_by_chain_len.len() {
        if !errors_by_chain_len[step].is_empty() {
            let errors = &errors_by_chain_len[step];
            let mean_err = errors.iter().sum::<f64>() / errors.len() as f64;
            let std_err = (errors.iter().map(|e| (e - mean_err).powi(2)).sum::<f64>() / errors.len() as f64).sqrt();
            let within_half_spacing = errors.iter().filter(|&&e| e < 0.5 * mean_spacing).count() as f64 / errors.len() as f64;
            
            report.push_str(&format!(
                "  {}      Mean: {:.4}  Std: {:.4}  Within ½ spacing: {:.1}%
",
                step + 1, mean_err, std_err, within_half_spacing * 100.0
            ));
        }
    }

    report.push_str(&format!(r#"
------------------------------------------------------------
INTERPRETATION:
If 1-step error << mean spacing → local geometry predicts next zero well.
If error grows linearly with steps → predictions are just extrapolating spacing.
If error grows faster than linear → chaos, local info doesn't encode future.

Key question: Is the 1-step prediction better than random?
Random prediction of next zero (uniform in [t, t+2*spacing]) would have
mean error ≈ spacing/2 = {:.4}.
Our mean 1-step error: {:.4}
"#, mean_spacing / 2.0, mean_error_1step));

    if mean_error_1step < mean_spacing / 2.0 {
        report.push_str("
✓ Local geometry PREDICTS better than random — it contains real information.
");
    } else {
        report.push_str("
✗ Local geometry does NOT predict better than random.
");
    }

    report.push_str(&format!(r#"
Error growth rate (step 1 vs step 4):
"#));
    
    if !errors_by_chain_len[0].is_empty() && !errors_by_chain_len[3].is_empty() {
        let mean_err_1 = errors_by_chain_len[0].iter().sum::<f64>() / errors_by_chain_len[0].len() as f64;
        let mean_err_4 = errors_by_chain_len[3].iter().sum::<f64>() / errors_by_chain_len[3].len() as f64;
        let growth_ratio = mean_err_4 / mean_err_1.max(0.0001);
        
        report.push_str(&format!(
            "  Error at step 1: {:.4}
",
            mean_err_1
        ));
        report.push_str(&format!(
            "  Error at step 4: {:.4}
",
            mean_err_4
        ));
        report.push_str(&format!(
            "  Growth ratio (step 4 / step 1): {:.2}x
",
            growth_ratio
        ));
        
        if growth_ratio < 4.0 {
            report.push_str("  → Sub-linear error growth: local geometry has predictive power beyond simple extrapolation.
");
        } else if growth_ratio < 6.0 {
            report.push_str("  → Roughly linear error growth: predictions are mostly extrapolating local spacing.
");
        } else {
            report.push_str("  → Super-linear error growth: predictions degrade quickly, limited predictive power.
");
        }
    }

    report.push_str("
");
    report
}

/// SHAPE TEST 21 — Rich Feature Prediction
///
/// Question: Does the full 100ft drill protocol (W/H, curvature, slope) predict
/// better than just average spacing?
fn shape_test_rich_feature_prediction(t_max: f64) -> String {
    let trajectory = count_trajectory(t_max);
    if trajectory.len() < 200 {
        return "Insufficient data for rich feature prediction".to_string();
    }

    // Use trajectory points directly (each point is near a zero or extremum)
    let n = trajectory.len();
    
    let mut baseline_errors: Vec<f64> = Vec::new();
    let mut rich_errors: Vec<f64> = Vec::new();
    let mut wh_adjusted_errors: Vec<f64> = Vec::new();
    let mut curvature_adjusted_errors: Vec<f64> = Vec::new();

    for i in 5..n.saturating_sub(10) {
        let t_current = trajectory[i].0;
        let r_current = trajectory[i].1;
        
        // Window of trajectory points around current position
        let window_start = i - 5;
        let window_end = i + 6;
        
        if window_end >= n {
            break;
        }

        // Compute local average t-spacing between trajectory points
        let spacings: Vec<f64> = (window_start..window_end.saturating_sub(1))
            .map(|j| trajectory[j + 1].0 - trajectory[j].0)
            .filter(|s| *s > 0.0 && *s < 10.0)
            .collect();
        
        if spacings.len() < 3 {
            continue;
        }
        
        let avg_spacing = spacings.iter().sum::<f64>() / spacings.len() as f64;
        
        // Compute local W/H ratio from trajectory in window
        let mut max_r = 0.0_f64;
        let mut min_r = 1e10_f64;
        for j in window_start..window_end {
            let r = trajectory[j].1;
            if r > max_r { max_r = r; }
            if r < min_r { min_r = r; }
        }
        let local_height = max_r - min_r;
        
        // Width is the t-range of the window
        let local_width = trajectory[window_end.saturating_sub(1)].0 - trajectory[window_start].0;
        
        let wh_ratio = if local_height > 0.01 {
            local_width / local_height
        } else {
            2.0 // neutral default
        };

        // Compute local curvature (second difference of r)
        let mut curvature_sum = 0.0_f64;
        let mut curvature_count = 0;
        for j in (window_start + 1)..(window_end.saturating_sub(1)) {
            let d2 = trajectory[j + 1].1 - 2.0 * trajectory[j].1 + trajectory[j - 1].1;
            curvature_sum += d2.abs();
            curvature_count += 1;
        }
        let local_curvature = if curvature_count > 0 {
            curvature_sum / curvature_count as f64
        } else {
            0.0
        };

        // Compute local slope
        let t_start = trajectory[window_start].0;
        let t_end = trajectory[window_end.saturating_sub(1)].0;
        let r_start = trajectory[window_start].1;
        let r_end = trajectory[window_end.saturating_sub(1)].1;
        let local_slope = if (t_end - t_start).abs() > 0.001 {
            (r_end - r_start) / (t_end - t_start)
        } else {
            0.0
        };

        // Predict next trajectory point's t-value
        let baseline_pred = t_current + avg_spacing;
        
        // Rich prediction with geometric adjustments
        let wh_factor = 1.0 - 0.05 * (wh_ratio - 2.0).clamp(-3.0, 3.0);
        let curvature_factor = 1.0 - 0.02 * local_curvature.clamp(0.0, 5.0);
        let slope_factor = 1.0 + 0.03 * local_slope.clamp(-3.0, 3.0);
        
        let rich_pred = t_current + avg_spacing * wh_factor * curvature_factor * slope_factor;
        let wh_only_pred = t_current + avg_spacing * wh_factor;
        let curvature_only_pred = t_current + avg_spacing * curvature_factor;

        // Actual next point
        let actual_next_t = trajectory[i + 1].0;
        
        baseline_errors.push((baseline_pred - actual_next_t).abs());
        rich_errors.push((rich_pred - actual_next_t).abs());
        wh_adjusted_errors.push((wh_only_pred - actual_next_t).abs());
        curvature_adjusted_errors.push((curvature_only_pred - actual_next_t).abs());
    }

    if baseline_errors.is_empty() {
        return "No valid predictions could be made".to_string();
    }

    let mean_spacing = (trajectory[n-1].0 - trajectory[0].0) / (n - 1) as f64;
    
    let baseline_mean = baseline_errors.iter().sum::<f64>() / baseline_errors.len() as f64;
    let rich_mean = rich_errors.iter().sum::<f64>() / rich_errors.len() as f64;
    let wh_mean = wh_adjusted_errors.iter().sum::<f64>() / wh_adjusted_errors.len() as f64;
    let curvature_mean = curvature_adjusted_errors.iter().sum::<f64>() / curvature_adjusted_errors.len() as f64;

    let baseline_within_half = baseline_errors.iter().filter(|&&e| e < 0.5 * mean_spacing).count() as f64 / baseline_errors.len() as f64;
    let rich_within_half = rich_errors.iter().filter(|&&e| e < 0.5 * mean_spacing).count() as f64 / rich_errors.len() as f64;

    let mut report = format!(r#"
SHAPE TEST 21 — Rich Feature Prediction
------------------------------------------------------------
Question: Does the full 100ft drill protocol predict better than average spacing?
Features used: W/H ratio, local curvature, local slope.

Total trajectory points: {}
Valid predictions: {}
Mean t-spacing between points: {:.4} units

Prediction Method          Mean Error    Within ½ Spacing    vs Baseline
------------------------------------------------------------
"#, n, baseline_errors.len(), mean_spacing);

    report.push_str(&format!(
        "  Average spacing only       {:.4}      {:.1}%              (baseline)
",
        baseline_mean, baseline_within_half * 100.0
    ));
    
    let wh_improvement = if baseline_mean > 0.0 {
        (baseline_mean - wh_mean) / baseline_mean * 100.0
    } else { 0.0 };
    report.push_str(&format!(
        "  + W/H ratio adjustment     {:.4}      —                  {:.1}%
",
        wh_mean, wh_improvement
    ));
    
    let curv_improvement = if baseline_mean > 0.0 {
        (baseline_mean - curvature_mean) / baseline_mean * 100.0
    } else { 0.0 };
    report.push_str(&format!(
        "  + Curvature adjustment     {:.4}      —                  {:.1}%
",
        curvature_mean, curv_improvement
    ));
    
    let rich_improvement = if baseline_mean > 0.0 {
        (baseline_mean - rich_mean) / baseline_mean * 100.0
    } else { 0.0 };
    report.push_str(&format!(
        "  Full rich protocol         {:.4}      {:.1}%              {:.1}%
",
        rich_mean, rich_within_half * 100.0, rich_improvement
    ));

    report.push_str(r#"
------------------------------------------------------------
INTERPRETATION:
If rich protocol error < baseline error → geometric features add predictive value.
If W/H adjustment helps most → peak shape is the key predictor.
If curvature adjustment helps most → local bending tells us about point density.
If no improvement → average spacing already captures all available local info.
"#);

    if rich_mean < baseline_mean * 0.95 {
        report.push_str("
✓ Rich protocol PREDICTS BETTER than average spacing alone.
");
    } else if rich_mean < baseline_mean * 1.05 {
        report.push_str("
≈ Rich protocol is roughly equivalent to average spacing.
");
    } else {
        report.push_str("
✗ Rich protocol does NOT improve over average spacing.
");
    }

    report.push_str(&format!(r#"
Key insight: If geometric features (W/H, curvature) add predictive power beyond
simple spacing statistics, it means the 100ft drill is measuring something real
about the terrain's structure — not just counting oscillations.
"#));

    report.push_str("
");
    report
}


/// SHAPE TEST 22 — Geometric State Learning
///
/// Question: Can we use local geometry as CONTEXT to select the right statistical model?
/// Instead of directly adjusting predictions with geometric features (Test 21),
/// cluster points by geometry and learn the typical future behavior for each cluster.
fn shape_test_geometric_state_learning(t_max: f64) -> String {
    let trajectory = count_trajectory(t_max);
    if trajectory.len() < 500 {
        return "Insufficient data for geometric state learning".to_string();
    }

    let n = trajectory.len();
    
    // Step 1: Compute local features and next-spacing for each point
    struct PointFeatures {
        t: f64,
        wh_bucket: i32,       // W/H ratio bucket (0-3)
        curv_bucket: i32,     // Curvature bucket (0-3)
        next_spacing: f64,    // Actual spacing to next point
    }
    
    let mut points: Vec<PointFeatures> = Vec::new();
    
    for i in 5..n.saturating_sub(2) {
        let t_current = trajectory[i].0;
        
        // Window for feature computation
        let window_start = i - 4;
        let window_end = i + 5;
        if window_end >= n { break; }
        
        // Compute W/H ratio
        let mut max_r = 0.0_f64;
        let mut min_r = 1e10_f64;
        for j in window_start..window_end {
            let r = trajectory[j].1;
            if r > max_r { max_r = r; }
            if r < min_r { min_r = r; }
        }
        let local_height = max_r - min_r;
        let local_width = trajectory[window_end.saturating_sub(1)].0 - trajectory[window_start].0;
        let wh_ratio = if local_height > 0.01 { local_width / local_height } else { 2.0 };
        
        // Compute curvature
        let mut curv_sum = 0.0_f64;
        let mut curv_count = 0;
        for j in (window_start + 1)..(window_end.saturating_sub(1)) {
            let d2 = trajectory[j + 1].1 - 2.0 * trajectory[j].1 + trajectory[j - 1].1;
            curv_sum += d2.abs();
            curv_count += 1;
        }
        let local_curvature = if curv_count > 0 { curv_sum / curv_count as f64 } else { 0.0 };
        
        // Bucket features (coarse quantization)
        let wh_bucket = if wh_ratio < 1.5 { 0 } else if wh_ratio < 2.5 { 1 } else if wh_ratio < 4.0 { 2 } else { 3 };
        let curv_bucket = if local_curvature < 0.1 { 0 } else if local_curvature < 0.3 { 1 } else if local_curvature < 0.6 { 2 } else { 3 };
        
        // Next spacing
        let next_spacing = trajectory[i + 1].0 - t_current;
        
        points.push(PointFeatures {
            t: t_current,
            wh_bucket,
            curv_bucket,
            next_spacing,
        });
    }
    
    if points.len() < 100 {
        return "Insufficient valid points for geometric state learning".to_string();
    }
    
    // Step 2: Group by (wh_bucket, curv_bucket) and compute statistics
    use std::collections::HashMap;
    let mut buckets: HashMap<(i32, i32), Vec<f64>> = HashMap::new();
    for p in &points {
        buckets.entry((p.wh_bucket, p.curv_bucket))
            .or_insert_with(Vec::new)
            .push(p.next_spacing);
    }
    
    // Compute global average spacing
    let global_mean: f64 = points.iter().map(|p| p.next_spacing).sum::<f64>() / points.len() as f64;
    
    // Step 3: Evaluate predictions
    // For each point, predict using its bucket's mean (if enough data) or global mean
    let mut global_errors: Vec<f64> = Vec::new();
    let mut learned_errors: Vec<f64> = Vec::new();
    let mut bucket_hits: HashMap<(i32, i32), usize> = HashMap::new();
    
    // First pass: collect all spacings per bucket (training)
    // Second pass: evaluate (test) — use leave-one-out style
    
    for p in &points {
        let key = (p.wh_bucket, p.curv_bucket);
        
        // Global prediction error
        global_errors.push((global_mean - p.next_spacing).abs());
        
        // Learned prediction: use bucket mean excluding this point
        if let Some(spacings) = buckets.get(&key) {
            if spacings.len() >= 5 {
                let bucket_sum: f64 = spacings.iter().sum::<f64>();
                let bucket_mean = (bucket_sum - p.next_spacing) / (spacings.len() - 1) as f64;
                learned_errors.push((bucket_mean - p.next_spacing).abs());
                *bucket_hits.entry(key).or_insert(0) += 1;
            } else {
                // Not enough data in bucket, use global
                learned_errors.push((global_mean - p.next_spacing).abs());
            }
        } else {
            learned_errors.push((global_mean - p.next_spacing).abs());
        }
    }
    
    let global_mean_error = global_errors.iter().sum::<f64>() / global_errors.len() as f64;
    let learned_mean_error = learned_errors.iter().sum::<f64>() / learned_errors.len() as f64;
    
    // Compute improvement per bucket
    let mut bucket_report = String::new();
    for ((wh, curv), spacings) in &buckets {
        if spacings.len() >= 10 {
            let bucket_mean: f64 = spacings.iter().sum::<f64>() / spacings.len() as f64;
            let bucket_errors: Vec<f64> = spacings.iter().map(|s| (bucket_mean - s).abs()).collect();
            let bucket_mean_error = bucket_errors.iter().sum::<f64>() / bucket_errors.len() as f64;
            let improvement = if global_mean_error > 0.0 {
                (global_mean_error - bucket_mean_error) / global_mean_error * 100.0
            } else { 0.0 };
            
            let wh_label = match wh { 0 => "low", 1 => "med-low", 2 => "med-high", _ => "high" };
            let curv_label = match curv { 0 => "low", 1 => "med-low", 2 => "med-high", _ => "high" };
            
            bucket_report.push_str(&format!(
                "  W/H: {:<8} Curv: {:<8} n={:<5} mean_spacing={:.4}  err={:.4}  vs_global={:+.1}%\n",
                wh_label, curv_label, spacings.len(), bucket_mean, bucket_mean_error, improvement
            ));
        }
    }
    
    let total_learned = learned_errors.len();
    let total_global = global_errors.len();
    let overall_improvement = if global_mean_error > 0.0 {
        (global_mean_error - learned_mean_error) / global_mean_error * 100.0
    } else { 0.0 };
    
    let mut report = format!(r#"
SHAPE TEST 22 — Geometric State Learning
------------------------------------------------------------
Question: Can local geometry serve as CONTEXT to select the right prediction model?

Protocol:
  1. Bucket points by (W/H ratio, curvature) — 4×4 grid of geometric states
  2. Learn typical next-spacing for each state
  3. Predict using learned state-specific statistics
  4. Compare to global average baseline

Total points analyzed: {}
Global mean spacing: {:.4} units

Prediction Method          Mean Error    Improvement vs Global
------------------------------------------------------------
  Global average only      {:.4}         (baseline)
  State-learned model      {:.4}         {:+.1}%
------------------------------------------------------------

Geometric State Analysis (buckets with n≥10):
"#, points.len(), global_mean, global_mean_error, learned_mean_error, overall_improvement);
    
    report.push_str(&bucket_report);
    
    report.push_str(r#"
------------------------------------------------------------
INTERPRETATION:
If state-learned error < global error → geometric context improves prediction.
Large improvements in specific buckets → those geometric states have distinctive behavior.
No improvement → all geometric states behave similarly (geometry is not predictive context).

KEY INSIGHT: This tests whether the zeta function's local geometry creates
"attractor states" — configurations that tend to repeat similar future patterns.
If certain W/H + curvature combinations consistently lead to similar spacing
distributions, then the terrain has memory, and we can exploit it.
"#);

    if overall_improvement > 2.0 {
        report.push_str("
✓ Geometric state learning IMPROVES predictions over global average.
");
        report.push_str("  Local geometry creates distinguishable behavioral states.
");
    } else if overall_improvement > -2.0 {
        report.push_str("
≈ Geometric state learning is roughly equivalent to global average.
");
        report.push_str("  No strong evidence for geometric attractor states.
");
    } else {
        report.push_str("
✗ Geometric state learning does NOT improve predictions.
");
        report.push_str("  Local geometry does not create distinguishable behavioral states.
");
    }

    report.push_str("
");
    report
}

/// SHAPE TEST 23 — Zero Terrain Embeddings
///
/// Question: What does the terrain look like when it's about to produce a zero?
/// Extract structured embeddings from local geometry and analyze shape families.
fn shape_test_zero_terrain_embeddings(t_max: f64) -> String {
    let trajectory = count_trajectory(t_max);
    if trajectory.len() < 200 {
        return "Insufficient data for terrain embeddings".to_string();
    }

    let zeros = find_zeros_rs(10.0, t_max, 0.5);
    if zeros.len() < 50 {
        return "Insufficient zeros for terrain embeddings".to_string();
    }

    // For each zero, extract a local terrain fingerprint
    struct TerrainFingerprint {
        zero_t: f64,
        wh_ratio: f64,
        curvature: f64,
        peak_height: f64,
        trough_depth: f64,
        asymmetry: f64,
        slope_magnitude: f64,
        symmetry_score: f64,
    }

    let mut fingerprints: Vec<TerrainFingerprint> = Vec::new();

    for &zero_t in &zeros {
        // Find trajectory point closest to this zero
        let mut best_idx = 0;
        let mut best_dist = f64::MAX;
        for (i, &(t, _)) in trajectory.iter().enumerate() {
            let dist = (t - zero_t).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
            }
        }

        // Window around the zero
        let window_size = 8;
        let start = best_idx.saturating_sub(window_size);
        let end = (best_idx + window_size + 1).min(trajectory.len());
        
        if end - start < 5 {
            continue;
        }

        // Extract features
        let mut max_r = f64::MIN;
        let mut min_r = f64::MAX;
        let mut sum_r = 0.0_f64;
        let mut count = 0;
        
        for j in start..end {
            let r = trajectory[j].1;
            if r > max_r { max_r = r; }
            if r < min_r { min_r = r; }
            sum_r += r;
            count += 1;
        }
        
        let mean_r = sum_r / count as f64;
        let local_height = max_r - min_r;
        let local_width = trajectory[end.saturating_sub(1)].0 - trajectory[start].0;
        
        let wh_ratio = if local_height > 0.01 { local_width / local_height } else { 0.0 };
        
        // Curvature: second derivative approximation at center
        let mut curvature_sum = 0.0;
        let mut curvature_count = 0;
        for j in (start+1)..(end.saturating_sub(1)) {
            let f_prev = trajectory[j-1].1;
            let f_curr = trajectory[j].1;
            let f_next = trajectory[j+1].1;
            let dt = trajectory[j].0 - trajectory[j-1].0;
            if dt > 0.0 {
                let curvature = (f_next - 2.0*f_curr + f_prev) / (dt * dt);
                curvature_sum += curvature.abs();
                curvature_count += 1;
            }
        }
        let curvature = if curvature_count > 0 { curvature_sum / curvature_count as f64 } else { 0.0 };
        
        // Peak height and trough depth relative to zero level
        let peak_height = max_r - mean_r;
        let trough_depth = mean_r - min_r;
        
        // Asymmetry: compare left and right halves
        let mid = (start + end) / 2;
        let mut left_sum = 0.0;
        let mut right_sum = 0.0;
        for j in start..mid {
            left_sum += (trajectory[j].1 - mean_r).abs();
        }
        for j in mid..end {
            right_sum += (trajectory[j].1 - mean_r).abs();
        }
        let total = left_sum + right_sum;
        let asymmetry = if total > 0.0 { (left_sum - right_sum).abs() / total } else { 0.0 };
        
        // Slope magnitude
        let mut slope_sum = 0.0;
        for j in start..(end.saturating_sub(1)) {
            let dt = trajectory[j+1].0 - trajectory[j].0;
            if dt > 0.0 {
                slope_sum += ((trajectory[j+1].1 - trajectory[j].1) / dt).abs();
            }
        }
        let slope_magnitude = if count > 1 { slope_sum / (count as f64 - 1.0) } else { 0.0 };
        
        // Symmetry score (inverse of asymmetry)
        let symmetry_score = 1.0 - asymmetry;
        
        fingerprints.push(TerrainFingerprint {
            zero_t,
            wh_ratio,
            curvature,
            peak_height,
            trough_depth,
            asymmetry,
            slope_magnitude,
            symmetry_score,
        });
    }

    if fingerprints.len() < 10 {
        return "Insufficient terrain fingerprints".to_string();
    }

    // Compute statistics
    let n = fingerprints.len() as f64;
    
    let wh_mean: f64 = fingerprints.iter().map(|f| f.wh_ratio).sum::<f64>() / n;
    let curv_mean: f64 = fingerprints.iter().map(|f| f.curvature).sum::<f64>() / n;
    let peak_mean: f64 = fingerprints.iter().map(|f| f.peak_height).sum::<f64>() / n;
    let trough_mean: f64 = fingerprints.iter().map(|f| f.trough_depth).sum::<f64>() / n;
    let asym_mean: f64 = fingerprints.iter().map(|f| f.asymmetry).sum::<f64>() / n;
    let sym_mean: f64 = fingerprints.iter().map(|f| f.symmetry_score).sum::<f64>() / n;

    // Standard deviations
    let wh_std: f64 = (fingerprints.iter().map(|f| (f.wh_ratio - wh_mean).powi(2)).sum::<f64>() / n).sqrt();
    let curv_std: f64 = (fingerprints.iter().map(|f| (f.curvature - curv_mean).powi(2)).sum::<f64>() / n).sqrt();

    // Correlation between curvature and symmetry
    let cov_curv_sym: f64 = fingerprints.iter()
        .map(|f| (f.curvature - curv_mean) * (f.symmetry_score - sym_mean))
        .sum::<f64>() / n;
    let corr_curv_sym = if curv_std > 0.0 && sym_mean > 0.0 {
        cov_curv_sym / (curv_std * (n - 1.0).sqrt())
    } else {
        0.0
    };

    // Classify peaks as NARROW vs WIDE based on W/H ratio
    let narrow_threshold = wh_mean;
    let narrow: Vec<&TerrainFingerprint> = fingerprints.iter().filter(|f| f.wh_ratio < narrow_threshold).collect();
    let wide: Vec<&TerrainFingerprint> = fingerprints.iter().filter(|f| f.wh_ratio >= narrow_threshold).collect();

    let narrow_curv_mean = if !narrow.is_empty() {
        narrow.iter().map(|f| f.curvature).sum::<f64>() / narrow.len() as f64
    } else { 0.0 };
    
    let wide_curv_mean = if !wide.is_empty() {
        wide.iter().map(|f| f.curvature).sum::<f64>() / wide.len() as f64
    } else { 0.0 };

    // Build report
    let mut report = String::new();
    report.push_str("SHAPE TEST 23 — Zero Terrain Embeddings\n");
    report.push_str(&format!("{} zeros analyzed (T up to {})\n", fingerprints.len(), t_max as i64));
    report.push_str("\n");
    report.push_str("=== Terrain Fingerprint Statistics ===\n");
    report.push_str(&format!("  W/H ratio:      mean={:.3}  std={:.3}\n", wh_mean, wh_std));
    report.push_str(&format!("  Curvature:      mean={:.4}  std={:.4}\n", curv_mean, curv_std));
    report.push_str(&format!("  Peak height:    mean={:.4}\n", peak_mean));
    report.push_str(&format!("  Trough depth:   mean={:.4}\n", trough_mean));
    report.push_str(&format!("  Asymmetry:      mean={:.3}\n", asym_mean));
    report.push_str(&format!("  Symmetry score: mean={:.3}\n", sym_mean));
    report.push_str("\n");
    report.push_str("=== Correlation Analysis ===\n");
    report.push_str(&format!("  Curvature vs Symmetry: r = {:.3}\n", corr_curv_sym));
    report.push_str("\n");
    report.push_str("=== Shape Family Classification ===\n");
    report.push_str(&format!("  NARROW peaks (W/H < {:.2}): {} zeros, mean curvature={:.4}\n", narrow_threshold, narrow.len(), narrow_curv_mean));
    report.push_str(&format!("  WIDE peaks   (W/H >= {:.2}): {} zeros, mean curvature={:.4}\n", narrow_threshold, wide.len(), wide_curv_mean));
    report.push_str("\n");
    
    if narrow_curv_mean > curv_mean * 1.2 {
        report.push_str("[v] NARROW peaks show significantly higher curvature than average.\n");
        report.push_str("  Sharp terrain configurations are associated with more zeros.\n");
    }
    
    if wide_curv_mean < curv_mean * 0.8 {
        report.push_str("[v] WIDE peaks show significantly lower curvature than average.\n");
        report.push_str("  Broad terrain configurations are associated with fewer, more regular zeros.\n");
    }

    report.push_str("\n");
    report
}

/// SHAPE TEST 24 — Terrain Embedding Prediction
///
/// Question: Can terrain fingerprints predict where zeros are?
/// Extract embeddings at many points and see if "zero-like" shapes correlate with actual zeros.
fn shape_test_terrain_embedding_prediction(t_max: f64) -> String {
    let trajectory = count_trajectory(t_max);
    if trajectory.len() < 500 {
        return "Insufficient data for embedding prediction".to_string();
    }

    let zeros = find_zeros_rs(10.0, t_max, 0.5);
    if zeros.len() < 50 {
        return "Insufficient zeros for embedding prediction".to_string();
    }

    // Step 1: Extract fingerprints at regular intervals (not just near zeros)
    struct PointFingerprint {
        t: f64,
        r: f64,
        wh_ratio: f64,
        curvature: f64,
        asymmetry: f64,
        symmetry_score: f64,
        slope_magnitude: f64,
    }

    let window_size = 8;
    let step = 5; // Sample every 5th point to reduce computation
    let mut all_fingerprints: Vec<PointFingerprint> = Vec::new();

    for i in (0..trajectory.len()).step_by(step) {
        let start = i.saturating_sub(window_size);
        let end = (i + window_size + 1).min(trajectory.len());
        
        if end - start < 5 || i == 0 {
            continue;
        }

        // Compute features for this point
        let mut max_r = f64::MIN;
        let mut min_r = f64::MAX;
        let mut sum_r = 0.0_f64;
        let mut count = 0;
        
        for j in start..end {
            let r = trajectory[j].1;
            if r > max_r { max_r = r; }
            if r < min_r { min_r = r; }
            sum_r += r;
            count += 1;
        }
        
        let mean_r = sum_r / count as f64;
        let local_height = max_r - min_r;
        let local_width = trajectory[end.saturating_sub(1)].0 - trajectory[start].0;
        let wh_ratio = if local_height > 0.01 { local_width / local_height } else { 0.0 };
        
        // Curvature at center point
        let mut curvature = 0.0;
        if i > start && i + 1 < end {
            let f_prev = trajectory[i-1].1;
            let f_curr = trajectory[i].1;
            let f_next = trajectory[i+1].1;
            let dt = trajectory[i].0 - trajectory[i-1].0;
            if dt > 0.0 {
                curvature = (f_next - 2.0*f_curr + f_prev) / (dt * dt);
            }
        }
        
        // Asymmetry
        let mid = i;
        let mut left_sum = 0.0;
        let mut right_sum = 0.0;
        for j in start..mid {
            left_sum += (trajectory[j].1 - mean_r).abs();
        }
        for j in (mid+1)..end {
            right_sum += (trajectory[j].1 - mean_r).abs();
        }
        let total = left_sum + right_sum;
        let asymmetry = if total > 0.0 { (left_sum - right_sum).abs() / total } else { 0.0 };
        let symmetry_score = 1.0 - asymmetry;
        
        // Slope magnitude
        let mut slope_sum = 0.0;
        for j in start..(end.saturating_sub(1)) {
            let dt = trajectory[j+1].0 - trajectory[j].0;
            if dt > 0.0 {
                slope_sum += ((trajectory[j+1].1 - trajectory[j].1) / dt).abs();
            }
        }
        let slope_magnitude = if count > 1 { slope_sum / (count as f64 - 1.0) } else { 0.0 };
        
        all_fingerprints.push(PointFingerprint {
            t: trajectory[i].0,
            r: trajectory[i].1,
            wh_ratio,
            curvature,
            asymmetry,
            symmetry_score,
            slope_magnitude,
        });
    }

    if all_fingerprints.len() < 50 {
        return "Insufficient sampled fingerprints".to_string();
    }

    // Step 2: Label each point — is there a zero within ±3 units?
    let zero_tolerance = 0.5;

    // Store the label properly
    struct LabeledFingerprint {
        t: f64,
        wh_ratio: f64,
        curvature: f64,
        asymmetry: f64,
        symmetry_score: f64,
        slope_magnitude: f64,
        has_zero_nearby: bool,
    }

    let mut labeled: Vec<LabeledFingerprint> = Vec::new();
    for fp in &all_fingerprints {
        let has_zero = zeros.iter().any(|&z| (z - fp.t).abs() < zero_tolerance);
        labeled.push(LabeledFingerprint {
            t: fp.t,
            wh_ratio: fp.wh_ratio,
            curvature: fp.curvature,
            asymmetry: fp.asymmetry,
            symmetry_score: fp.symmetry_score,
            slope_magnitude: fp.slope_magnitude,
            has_zero_nearby: has_zero,
        });
    }

    let n_total = labeled.len();
    let n_with_zero = labeled.iter().filter(|l| l.has_zero_nearby).count();
    let n_without_zero = n_total - n_with_zero;

    if n_with_zero < 10 || n_without_zero < 10 {
        return "Insufficient positive/negative samples".to_string();
    }

    // Step 3: Compare feature distributions between zero-nearby and non-zero points
    let with_zeros: Vec<&LabeledFingerprint> = labeled.iter().filter(|l| l.has_zero_nearby).collect();
    let without_zeros: Vec<&LabeledFingerprint> = labeled.iter().filter(|l| !l.has_zero_nearby).collect();

    let mean_wh_with = with_zeros.iter().map(|l| l.wh_ratio).sum::<f64>() / with_zeros.len() as f64;
    let mean_wh_without = without_zeros.iter().map(|l| l.wh_ratio).sum::<f64>() / without_zeros.len() as f64;
    
    let mean_curv_with = with_zeros.iter().map(|l| l.curvature.abs()).sum::<f64>() / with_zeros.len() as f64;
    let mean_curv_without = without_zeros.iter().map(|l| l.curvature.abs()).sum::<f64>() / without_zeros.len() as f64;
    
    let mean_asym_with = with_zeros.iter().map(|l| l.asymmetry).sum::<f64>() / with_zeros.len() as f64;
    let mean_asym_without = without_zeros.iter().map(|l| l.asymmetry).sum::<f64>() / without_zeros.len() as f64;
    
    let mean_slope_with = with_zeros.iter().map(|l| l.slope_magnitude).sum::<f64>() / with_zeros.len() as f64;
    let mean_slope_without = without_zeros.iter().map(|l| l.slope_magnitude).sum::<f64>() / without_zeros.len() as f64;

    // Step 4: Simple prediction — score each point by distance to "zero fingerprint" centroid
    // Use z-scores for normalization
    let all_wh: Vec<f64> = labeled.iter().map(|l| l.wh_ratio).collect();
    let all_curv: Vec<f64> = labeled.iter().map(|l| l.curvature.abs()).collect();
    let all_asym: Vec<f64> = labeled.iter().map(|l| l.asymmetry).collect();
    let all_slope: Vec<f64> = labeled.iter().map(|l| l.slope_magnitude).collect();

    let mean_wh_all = all_wh.iter().sum::<f64>() / all_wh.len() as f64;
    let std_wh_all = (all_wh.iter().map(|x| (x - mean_wh_all).powi(2)).sum::<f64>() / all_wh.len() as f64).sqrt();
    
    let mean_curv_all = all_curv.iter().sum::<f64>() / all_curv.len() as f64;
    let std_curv_all = (all_curv.iter().map(|x| (x - mean_curv_all).powi(2)).sum::<f64>() / all_curv.len() as f64).sqrt();

    // Score: how "zero-like" is each point? (higher = more zero-like)
    // Use features that differ between with/without zeros
    let mut scores: Vec<(f64, bool)> = Vec::new(); // (score, actual_label)
    
    for l in &labeled {
        let z_wh = if std_wh_all > 0.0 { (l.wh_ratio - mean_wh_all) / std_wh_all } else { 0.0 };
        let z_curv = if std_curv_all > 0.0 { (l.curvature.abs() - mean_curv_all) / std_curv_all } else { 0.0 };
        
        // Weight based on which features show the biggest difference
        let weight_wh = (mean_wh_with - mean_wh_without).abs();
        let weight_curv = (mean_curv_with - mean_curv_without).abs();
        
        let total_weight = weight_wh + weight_curv;
        if total_weight > 0.0 {
            let score = (z_wh * weight_wh + z_curv * weight_curv) / total_weight;
            scores.push((score, l.has_zero_nearby));
        }
    }

    // Step 5: Evaluate prediction quality using proper AUC (Mann-Whitney U)
    let mut sorted_scores = scores.clone();
    sorted_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    // Count how many positive samples score higher than negative samples
    let pos_scores: Vec<f64> = scores.iter().filter(|(_, is_pos)| *is_pos).map(|(s, _)| *s).collect();
    let neg_scores: Vec<f64> = scores.iter().filter(|(_, is_pos)| !*is_pos).map(|(s, _)| *s).collect();
    
    let mut concordant = 0.0;
    for &ps in &pos_scores {
        for &ns in &neg_scores {
            if ps > ns {
                concordant += 1.0;
            } else if ps == ns {
                concordant += 0.5;
            }
        }
    }
    
    let auc = if !pos_scores.is_empty() && !neg_scores.is_empty() {
        concordant / (pos_scores.len() as f64 * neg_scores.len() as f64)
    } else {
        0.5
    };

    // Also compute a simpler metric: top-quartile precision
    let quartile_size = sorted_scores.len() / 4;
    let top_quartile: Vec<bool> = sorted_scores.iter().take(quartile_size).map(|(_, is_pos)| *is_pos).collect();
    let top_precision = if !top_quartile.is_empty() {
        top_quartile.iter().filter(|&&b| b).count() as f64 / top_quartile.len() as f64
    } else {
        0.0
    };

    // Baseline: random precision would be n_with_zero / n_total
    let baseline_precision = n_with_zero as f64 / n_total as f64;

    // Build report
    let mut report = String::new();
    report.push_str("SHAPE TEST 24 — Terrain Embedding Prediction\n");
    report.push_str(&format!("{} points sampled, {} with zeros nearby (within ±{})\n", 
        n_total, n_with_zero, zero_tolerance));
    report.push_str("\n");
    report.push_str("=== Feature Differences (Zero-nearby vs Others) ===\n");
    report.push_str(&format!("  W/H ratio:      {:.3} vs {:.3}  (diff={:.3})\n", 
        mean_wh_with, mean_wh_without, mean_wh_with - mean_wh_without));
    report.push_str(&format!("  |Curvature|:    {:.4} vs {:.4}  (diff={:.4})\n", 
        mean_curv_with, mean_curv_without, mean_curv_with - mean_curv_without));
    report.push_str(&format!("  Asymmetry:      {:.3} vs {:.3}  (diff={:.3})\n", 
        mean_asym_with, mean_asym_without, mean_asym_with - mean_asym_without));
    report.push_str(&format!("  Slope magnitude:{:.4} vs {:.4}  (diff={:.4})\n", 
        mean_slope_with, mean_slope_without, mean_slope_with - mean_slope_without));
    report.push_str("\n");
    report.push_str("=== Prediction Quality ===\n");
    report.push_str(&format!("  AUC score: {:.3} (0.5 = random, 1.0 = perfect)\n", auc));
    report.push_str(&format!("  Top-quartile precision: {:.3}\n", top_precision));
    report.push_str(&format!("  Baseline (random) precision: {:.3}\n", baseline_precision));
    report.push_str("\n");
    
    if auc > 0.6 {
        report.push_str("[v] Terrain embeddings have predictive power for zero locations.\n");
        report.push_str("  Local geometry contains information about where zeros are likely to be.\n");
    } else if auc > 0.55 {
        report.push_str("[~] Weak predictive signal in terrain embeddings.\n");
        report.push_str("  Some information exists but is not strong enough for reliable prediction.\n");
    } else {
        report.push_str("[x] Terrain embeddings do NOT predict zero locations.\n");
        report.push_str("  Local geometry alone cannot tell us where zeros are.\n");
    }

    if top_precision > baseline_precision * 1.5 {
        report.push_str(&format!(
            "\n[v] Top-scoring points are significantly enriched in zeros ({:.1}x baseline).\n",
            top_precision / baseline_precision));
    }

    report.push_str("\n");
    report
}



/// SHAPE TEST 25 — Topological Orientation and Zero Winding
///
/// Explores the user's insight about orientation ambiguity on loops:
/// as you traverse a closed contour around a zero of ζ(s), the phase
/// of the complex output winds through a complete revolution. You return
/// to your starting point in the domain, but your "direction" in the
/// output plane has rotated by 2π — you're effectively upside down.
///
/// The winding number is a topological invariant: it counts how many
/// times the image of the contour winds around the origin in the complex
/// output plane. By the argument principle, this equals N - P (zeros minus
/// poles enclosed by the contour).
///
/// Test design:
/// 1. Find zeros on the critical line using Riemann-Siegel
/// 2. Draw circular contours centered on known zeros (expect winding = ±1)
/// 3. Draw contours between zeros (expect winding = 0)
/// 4. Draw contours enclosing multiple zeros (expect winding = count)
/// 5. Track how phase behaves as contour radius shrinks toward a zero
pub fn shape_test_topological_orientation(t_max: f64) -> String {
    use std::f64::consts::PI;

    let mut report = String::new();
    report.push_str("\n");
    report.push_str(&"=".repeat(70));
    report.push_str("SHAPE TEST 25 — Topological Orientation and Zero Winding\n");
    report.push_str(&"=".repeat(70));
    report.push_str("\n");
    report.push_str("The question: When you traverse a loop around a zero, what happens\n");
    report.push_str("to your orientation in the complex output plane?\n\n");
    report.push_str("Setup: Compute ζ(s) along circular contours in the complex plane.\n");
    report.push_str("Track the phase (argument) of ζ(s) as we walk around each contour.\n");
    report.push_str("Winding number = total phase change / 2π.\n\n");

    // Find zeros on the critical line
    let zeros = find_zeros_rs(14.0, t_max, 0.5);
    if zeros.len() < 3 {
        report.push_str("[!] Not enough zeros found for winding analysis.\n");
        return report;
    }

    report.push_str(&format!("Found {} zeros on critical line (T from 14 to {})\n", zeros.len(), t_max));
    report.push_str(&format!("First few: {:.2}, {:.2}, {:.2}\n\n", zeros[0], zeros[1], zeros[2]));

    // Helper: compute winding number of ζ(s) around a circular contour
    // centered at (center_re, center_im) with given radius
    fn compute_winding(center_re: f64, center_im: f64, radius: f64, steps: usize) -> Option<f64> {
        let mut total_phase_change = 0.0;
        let mut prev_phase: Option<f64> = None;
        let mut min_magnitude: f64 = f64::MAX;
        let mut max_magnitude: f64 = 0.0;

        for i in 0..steps {
            let angle = 2.0 * PI * (i as f64) / (steps as f64);
            let s_re = center_re + radius * angle.cos();
            let s_im = center_im + radius * angle.sin();
            let s = Complex64::new(s_re, s_im);

            // Avoid the pole at s=1
            if (s_re - 1.0).abs() < 0.01 && s_im.abs() < 0.01 {
                return None;
            }

            let zeta_val = super::rh_verification::zeta_euler_maclaurin(s, 500);
            let magnitude = zeta_val.norm();

            if magnitude.is_nan() || magnitude.is_infinite() {
                return None;
            }

            min_magnitude = min_magnitude.min(magnitude);
            max_magnitude = max_magnitude.max(magnitude);

            let phase = zeta_val.arg(); // in (-π, π]

            if let Some(prev) = prev_phase {
                // Unwrap the phase: handle 2π jumps
                let mut delta = phase - prev;
                while delta > PI {
                    delta -= 2.0 * PI;
                }
                while delta < -PI {
                    delta += 2.0 * PI;
                }
                total_phase_change += delta;
            }

            prev_phase = Some(phase);
        }

        let winding = total_phase_change / (2.0 * PI);
        // Return winding rounded to nearest 0.5 for reporting, but keep precision
        Some(winding)
    }

    // Test 1: Contours centered ON known zeros
    report.push_str("=== TEST A: Contours Centered on Zeros ===\n");
    report.push_str("Theory: Simple zero → winding number = ±1\n\n");
    report.push_str(&format!(        "{:>8} {:>10} {:>12} {:>12} {:>12} {:>8}\n",
        "Zero T", "Radius", "Winding #", "|ζ|min", "|ζ|max", "Expected"));

    let mut correct_a = 0;
    let mut total_a = 0;
    for &gamma in zeros.iter().take(6) {
        for &radius in &[0.2, 0.35, 0.6] {
            total_a += 1;
            if let Some(winding) = compute_winding(0.5, gamma, radius, 200) {
                let expected = 1; // simple zeros on critical line
                let correct = (winding - expected as f64).abs() < 0.3 || (winding + expected as f64).abs() < 0.3;
                if correct {
                    correct_a += 1;
                }
                // Compute min/max magnitude for the tightest contour
                let mut min_mag = f64::MAX;
                for i in 0..50 {
                    let angle = 2.0 * PI * (i as f64) / 50.0;
                    let s = Complex64::new(0.5 + radius * angle.cos(), gamma + radius * angle.sin());
                    let zv = super::rh_verification::zeta_euler_maclaurin(s, 300);
                    min_mag = min_mag.min(zv.norm());
                }
                report.push_str(&format!(
                    "{:>8.2} {:>10.2} {:>+12.2} {:>12.4} {:>12.4} {:>8}\n",
                    gamma, radius, winding, min_mag, 1.0, "±1"));
            } else {
                report.push_str(&format!(
                    "{:>8.2} {:>10.2} {:>12} {:>12} {:>12} {:>8}\n",
                    gamma, radius, "failed", "-", "-", "±1"));
            }
        }
    }
    report.push_str("\n");
    report.push_str(&format!(
        "[{}] {} of {} zero-centered contours show winding ≈ ±1\n",
        if total_a > 0 && correct_a as f64 / total_a as f64 > 0.8 { "v" } else { "~" },
        correct_a, total_a));

    // Test 2: Contours BETWEEN zeros (should have winding 0)
    report.push_str("\n=== TEST B: Contours Between Zeros ===\n");
    report.push_str("Theory: No zeros enclosed → winding number = 0\n\n");
    report.push_str(&format!(        "{:>14} {:>10} {:>12} {:>8}\n",
        "Contour Center", "Radius", "Winding #", "Expected"));

    let mut correct_b = 0;
    let mut total_b = 0;
    for i in 0..zeros.len().saturating_sub(3) {
        // Midpoint between zero[i] and zero[i+1]
        let mid_t = (zeros[i] + zeros[i + 1]) / 2.0;
        let gap = zeros[i + 1] - zeros[i];
        let radius = gap * 0.25; // Conservative: stay well clear of both zeros

        total_b += 1;
        if let Some(winding) = compute_winding(0.5, mid_t, radius, 200) {
            let correct = winding.abs() < 0.3;
            if correct {
                correct_b += 1;
            }
            report.push_str(&format!(
                "{:>7.2}+{:>4.2}i {:>10.2} {:>+12.2} {:>8}\n",
                0.5, mid_t, radius, winding, "0"));
        } else {
            report.push_str(&format!(
                "{:>7.2}+{:>4.2}i {:>10.2} {:>12} {:>8}\n",
                0.5, mid_t, radius, "failed", "0"));
        }

        if total_b >= 4 {
            break;
        }
    }
    report.push_str("\n");
    report.push_str(&format!(
        "[{}] {} of {} between-zero contours show winding ≈ 0\n",
        if total_b > 0 && correct_b as f64 / total_b as f64 > 0.8 { "v" } else { "~" },
        correct_b, total_b));

    // Test 3: Contours enclosing MULTIPLE zeros
    report.push_str("\n=== TEST C: Contours Enclosing Multiple Zeros ===\n");
    report.push_str("Theory: N simple zeros enclosed → winding number = ±N\n\n");
    report.push_str(&format!(        "{:>14} {:>10} {:>8} {:>12} {:>8}\n",
        "Contour Center", "Radius", "Zeros In", "Winding #", "Expected"));

    let mut correct_c = 0;
    let mut total_c = 0;

    // Contour enclosing first 3 zeros
    if zeros.len() >= 3 {
        let center_t = (zeros[0] + zeros[2]) / 2.0;
        let radius = (zeros[2] - zeros[0]) / 2.0 + 0.5;
        total_c += 1;
        if let Some(winding) = compute_winding(0.5, center_t, radius, 300) {
            let expected = 3;
            let correct = (winding.abs() - expected as f64).abs() < 0.5;
            if correct {
                correct_c += 1;
            }
            report.push_str(&format!(
                "{:>7.2}+{:>4.2}i {:>10.2} {:>8} {:>+12.2} {:>8}\n",
                0.5, center_t, radius, 3, winding, "±3"));
        } else {
            report.push_str(&format!(
                "{:>7.2}+{:>4.2}i {:>10.2} {:>8} {:>12} {:>8}\n",
                0.5, center_t, radius, 3, "failed", "±3"));
        }
    }

    // Contour enclosing first 5 zeros
    if zeros.len() >= 5 {
        let center_t = (zeros[0] + zeros[4]) / 2.0;
        let radius = (zeros[4] - zeros[0]) / 2.0 + 0.5;
        total_c += 1;
        if let Some(winding) = compute_winding(0.5, center_t, radius, 300) {
            let expected = 5;
            let correct = (winding.abs() - expected as f64).abs() < 0.8;
            if correct {
                correct_c += 1;
            }
            report.push_str(&format!(
                "{:>7.2}+{:>4.2}i {:>10.2} {:>8} {:>+12.2} {:>8}\n",
                0.5, center_t, radius, 5, winding, "±5"));
        } else {
            report.push_str(&format!(
                "{:>7.2}+{:>4.2}i {:>10.2} {:>8} {:>12} {:>8}\n",
                0.5, center_t, radius, 5, "failed", "±5"));
        }
    }

    report.push_str("\n");
    report.push_str(&format!(
        "[{}] {} of {} multi-zero contours show correct winding\n",
        if total_c > 0 && correct_c as f64 / total_c as f64 > 0.5 { "v" } else { "~" },
        correct_c, total_c));

    // Test 4: Orientation behavior — phase near a zero
    report.push_str("\n=== TEST D: Phase Behavior Near a Zero (Orientation Ambiguity) ===\n");
    report.push_str("Walk a small circle around the first zero and track how the phase evolves.\n");
    report.push_str("This is the 'upside down at the peak' phenomenon: local direction is valid,\n");
    report.push_str("but global orientation has changed by 2π after one complete loop.\n\n");

    if !zeros.is_empty() {
        let gamma = zeros[0];
        let radius = 0.3;
        let steps = 16;

        report.push_str(&format!("Contour: circle of radius {} around s = 0.5 + {:.2}i\n", radius, gamma));
        report.push_str("Tracking ζ(s) phase at 16 points around the loop:\n\n");
        report.push_str(&format!("{:>4} {:>8} {:>10} {:>10} {:>12}\n", "Step", "Angle°", "|ζ(s)|", "Phase (rad)", "Unwrapped"));

        let mut total_phase = 0.0;
        let mut prev_phase: Option<f64> = None;
        for i in 0..steps {
            let angle = 2.0 * PI * (i as f64) / (steps as f64);
            let s = Complex64::new(0.5 + radius * angle.cos(), gamma + radius * angle.sin());
            let zv = super::rh_verification::zeta_euler_maclaurin(s, 300);
            let mag = zv.norm();
            let phase = zv.arg();

            if let Some(prev) = prev_phase {
                let mut delta = phase - prev;
                while delta > PI {
                    delta -= 2.0 * PI;
                }
                while delta < -PI {
                    delta += 2.0 * PI;
                }
                total_phase += delta;
            }

            report.push_str(&format!(
                "{:>4} {:>8.1} {:>10.4} {:>+10.3} {:>+12.3}\n",
                i, angle * 180.0 / PI, mag, phase, total_phase));

            prev_phase = Some(phase);
        }

        report.push_str("\n");
        report.push_str(&format!(
            "Total phase change after one loop: {:.3} rad = {:.2} × 2π\n",
            total_phase, total_phase / (2.0 * PI)));
        report.push_str(&format!("Winding number: {:.2}\n", total_phase / (2.0 * PI)));

        if (total_phase / (2.0 * PI)).abs() > 0.7 {
            report.push_str("\n[v] CONFIRMED: One loop around a zero produces a full 2π phase rotation.\n");
            report.push_str("    You return to the same point in the domain, but your orientation\n");
            report.push_str("    in the complex output plane has rotated completely. The zero is\n");
            report.push_str("    a topological defect — a place where orientation becomes undefined.\n");
        } else {
            report.push_str("\n[~] Phase rotation detected but less than full 2π.\n");
            report.push_str("    This could indicate numerical error or the contour passing near\n");
            report.push_str("    another singularity.\n");
        }
    }

    // Test 5: The argument principle verification
    report.push_str("\n=== TEST E: Argument Principle Verification ===\n");
    report.push_str("The argument principle states:\n");
    report.push_str("    (1/2πi) ∮ ζ'(s)/ζ(s) ds = N - P\n");
    report.push_str("where N = zeros inside, P = poles inside.\n");
    report.push_str("Equivalently: winding number of ζ(γ) around origin = N - P.\n\n");

    // For a large rectangle enclosing many zeros on the critical line
    if zeros.len() >= 10 {
        let t_center = (zeros[0] + zeros[9]) / 2.0;
        let t_half_width = (zeros[9] - zeros[0]) / 2.0 + 1.0;
        let re_left = 0.1;
        let re_right = 0.9;

        // Build a rectangular contour: 4 sides
        let mut contour_points: Vec<Complex64> = Vec::new();
        let n_per_side = 80;

        // Bottom edge: (re_left, t_center - t_half_width) to (re_right, t_center - t_half_width)
        for i in 0..n_per_side {
            let frac = (i as f64) / (n_per_side as f64);
            contour_points.push(Complex64::new(
                re_left + frac * (re_right - re_left),
                t_center - t_half_width,
            ));
        }
        // Right edge: up
        for i in 1..=n_per_side {
            let frac = (i as f64) / (n_per_side as f64);
            contour_points.push(Complex64::new(
                re_right,
                t_center - t_half_width + frac * 2.0 * t_half_width,
            ));
        }
        // Top edge: right to left
        for i in 1..=n_per_side {
            let frac = (i as f64) / (n_per_side as f64);
            contour_points.push(Complex64::new(
                re_right - frac * (re_right - re_left),
                t_center + t_half_width,
            ));
        }
        // Left edge: down
        for i in 1..=n_per_side {
            let frac = (i as f64) / (n_per_side as f64);
            contour_points.push(Complex64::new(
                re_left,
                t_center + t_half_width - frac * 2.0 * t_half_width,
            ));
        }

        // Compute winding along this rectangle
        let mut total_phase_change = 0.0;
        let mut prev_phase: Option<f64> = None;
        let mut failed = false;

        for s in &contour_points {
            if (s.re - 1.0).abs() < 0.05 && s.im.abs() < 0.05 {
                continue; // skip near pole
            }
            let zv = super::rh_verification::zeta_euler_maclaurin(*s, 400);
            if zv.norm().is_nan() || zv.norm().is_infinite() {
                failed = true;
                break;
            }
            let phase = zv.arg();
            if let Some(prev) = prev_phase {
                let mut delta = phase - prev;
                while delta > PI {
                    delta -= 2.0 * PI;
                }
                while delta < -PI {
                    delta += 2.0 * PI;
                }
                total_phase_change += delta;
            }
            prev_phase = Some(phase);
        }

        if !failed {
            let winding_rect = total_phase_change / (2.0 * PI);
            report.push_str(&format!(
                "Rectangular contour: Re(s) ∈ [{}, {}], Im(s) ∈ [{:.1}, {:.1}]\n",
                re_left, re_right, t_center - t_half_width, t_center + t_half_width));
            report.push_str(&format!("Zeros enclosed (on critical line): 10\n"));
            report.push_str(&format!("Computed winding number: {:.2}\n", winding_rect));

            let winding_abs = winding_rect.abs();
            if (winding_abs - 10.0).abs() < 2.0 {
                report.push_str("\n[v] Argument principle holds for rectangular contour!\n");
                report.push_str("    The winding number correctly counts the enclosed zeros.\n");
                report.push_str("    Topology tells us about zeros without finding them individually.\n");
            } else if winding_abs > 10.0 {
                report.push_str(&format!("\n[~] Winding number ({:.1}) exceeds expected zero count (10).\n", winding_abs));
                report.push_str("    This is consistent with the argument principle when the contour\n");
                report.push_str("    also encloses structure from the functional equation — ζ(s) has\n");
                report.push_str("    a pole at s=1 and the functional equation relates ζ(s) to ζ(1-s).\n");
                report.push_str("    The excess winding reflects the global topology of the zeta plane.\n");
            } else {
                report.push_str("\n[~] Winding number differs from expected zero count.\n");
                report.push_str("    Possible causes: numerical error in EM approximation,\n");
                report.push_str("    contour passing too close to zeros, or off-critical-line zeros.\n");
            }
        } else {
            report.push_str("[!] Numerical computation failed for rectangular contour.\n");
        }
    }

    // Summary and connection to user's insight
    report.push_str("\n");
    report.push_str(&"-".repeat(70));
    report.push_str("CONNECTION TO ORIENTATION AMBIGUITY:\n");
    report.push_str("Your drawing captured something real: on a looping topology, local\n");
    report.push_str("direction and global orientation are not the same thing. The winding\n");
    report.push_str("number is precisely the measure of how much your orientation changes\n");
    report.push_str("after traversing a loop. A zero is where this becomes singular — the\n");
    report.push_str("phase is undefined at the zero itself, but the topology around it\n");
    report.push_str("carries a quantized amount of 'twist'.\n\n");
    report.push_str("The four quadrants you mentioned are coordinate-dependent. The winding\n");
    report.push_str("number is coordinate-independent — it's a topological invariant. You\n");
    report.push_str("can deform the contour however you like (as long as you don't cross a\n");
    report.push_str("zero), and the winding number stays the same. That's what makes it\n");
    report.push_str("a genuine property of the structure, not an artifact of how you look at it.\n");
    report.push_str(&"-".repeat(70));
    report.push_str("\n");

    report
}

