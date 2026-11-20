//! Zeta Analysis Module
//! Euler product verification, Möbius function, zeta zeros, approximation errors.

use crate::riemann::integer_structure::{is_prime, mobius};

/// Compute ζ(s) for real s > 1 using Dirichlet series
pub fn zeta_real(s: f64) -> f64 {
    let mut sum = 0.0;
    let mut n = 1;
    while n <= 1_000_000 {
        sum += 1.0 / (n as f64).powf(s);
        n += 1;
    }
    sum
}

/// Compute Euler product Π_p (1 - p^(-s))^(-1) for real s > 1
pub fn euler_product_real(s: f64, max_prime: usize) -> f64 {
    let mut product = 1.0;
    let mut n = 2;
    while n <= max_prime {
        if is_prime(n as u64) {
            let p_inv_s = (n as f64).powf(-s);
            product *= 1.0 / (1.0 - p_inv_s);
        }
        n += 1;
    }
    product
}

/// Verify Euler product identity: ζ(s) = Π_p (1 - p^(-s))^(-1)
/// Returns (zeta_series, euler_product, relative_error)
pub fn verify_euler_product(s: f64, max_prime: usize) -> (f64, f64, f64) {
    let zeta = zeta_real(s);
    let product = euler_product_real(s, max_prime);
    let error = ((zeta - product) / zeta).abs();
    (zeta, product, error)
}

/// Compute 1/ζ(s) via Möbius series Σ μ(n)/n^s for real s > 1
pub fn mobius_series_inverted_zeta(s: f64, max_n: usize) -> f64 {
    let mut sum = 0.0;
    for n in 1..=max_n {
        let mu = mobius(n as u64);
        if mu != 0 {
            sum += (mu as f64) / (n as f64).powf(s);
        }
    }
    sum
}

/// Verify that Σ μ(n)/n^s = 1/ζ(s)
pub fn verify_mobius_identity(s: f64, max_n: usize) -> (f64, f64, f64) {
    let mobius_sum = mobius_series_inverted_zeta(s, max_n);
    let zeta = zeta_real(s);
    let expected = 1.0 / zeta;
    let error = ((mobius_sum - expected) / expected).abs();
    (mobius_sum, expected, error)
}

/// Logarithmic integral Li(x) = ∫_2^x dt/ln(t) using Simpson's rule
pub fn logarithmic_integral(x: f64) -> f64 {
    if x < 2.0 {
        return 0.0;
    }
    let n = 10000; // Simpson's rule subdivisions
    let h = (x - 2.0) / n as f64;
    let mut sum = 1.0 / 2.0_f64.ln() + 1.0 / x.ln();
    for i in 1..n {
        let t = 2.0 + i as f64 * h;
        let weight = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += weight / t.ln();
    }
    sum * h / 3.0
}

/// Prime counting function π(x)
pub fn prime_counting(x: u64) -> usize {
    let mut count = 0;
    for n in 2..=x {
        if is_prime(n) {
            count += 1;
        }
    }
    count
}

/// Compare π(x) with Li(x) at various points
pub fn pnt_comparison(limit: u64, sample_points: &[u64]) -> Vec<(u64, usize, f64, f64)> {
    let mut results = Vec::new();
    for &x in sample_points {
        if x > limit {
            continue;
        }
        let pi_x = prime_counting(x);
        let li_x = logarithmic_integral(x as f64);
        let ratio = pi_x as f64 / li_x;
        let error = ((pi_x as f64) - li_x).abs() / li_x;
        results.push((x, pi_x, ratio, error));
    }
    results
}

/// Estimate prime gap bound under RH: O(sqrt(x) * ln(x))
pub fn rh_gap_bound(x: f64) -> f64 {
    x.sqrt() * x.ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeta_at_2() {
        // ζ(2) = π²/6 ≈ 1.644934
        let z = zeta_real(2.0);
        assert!((z - std::f64::consts::PI * std::f64::consts::PI / 6.0).abs() < 0.001);
    }

    #[test]
    fn test_euler_product_converges() {
        // Euler product should approach ζ(s) as max_prime increases
        let (zeta, product, error) = verify_euler_product(2.0, 1000);
        assert!(error < 0.01); // Within 1% for primes up to 1000
    }

    #[test]
    fn test_mobius_identity() {
        // Σ μ(n)/n^s should approach 1/ζ(s)
        let (mobius_sum, expected, error) = verify_mobius_identity(2.0, 1000);
        assert!(error < 0.01);
    }

    #[test]
    fn test_prime_counting() {
        assert_eq!(prime_counting(10), 4); // 2,3,5,7
        assert_eq!(prime_counting(20), 8); // 2,3,5,7,11,13,17,19
    }

    #[test]
    fn test_pnt_ratio_approaches_1() {
        // π(x)/Li(x) should approach 1 as x increases
        let results = pnt_comparison(1000, &[100, 500, 1000]);
        assert!(results.len() == 3);
        // Later ratios should be closer to 1
        assert!(results[2].2 > results[0].2);
    }
}
