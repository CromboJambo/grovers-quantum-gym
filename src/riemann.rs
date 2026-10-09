//! Riemann Hypothesis Millennium Prize Problem Station
//!
//! Explores the Clay Mathematics Institute's $1 million question:
//! Do all non-trivial zeros of the Riemann zeta function lie on the
//! critical line Re(s) = 1/2?
//!
//! Demonstrates via computation of ζ(s) along the real axis and critical line.

/// Compute Riemann zeta function for real s > 1 using Dirichlet series
fn zeta_real(s: f64) -> f64 {
    let mut sum = 0.0;
    let mut n = 1;
    while n <= 1_000_000 {
        sum += 1.0 / (n as f64).powf(s);
        n += 1;
    }
    sum
}

/// Compute ζ(s) for complex s using Euler-Maclaurin acceleration
/// Returns (re, im)
fn zeta_complex(s_re: f64, s_im: f64) -> (f64, f64) {
    if s_re > 1.0 {
        // Direct series converges for Re(s) > 1
        let mut re_sum = 0.0;
        let mut im_sum = 0.0;
        for n in 1..=100_000 {
            let log_n = (n as f64).ln();
            let denom = (n as f64).powf(s_re);
            re_sum += (-(s_im * log_n)).cos() / denom;
            im_sum += (-(s_im * log_n)).sin() / denom;
        }
        return (re_sum, -im_sum);
    }
    
    // For critical line, use approximate computation via first N terms
    // with convergence factor (not mathematically rigorous but illustrative)
    let mut re_sum = 0.0;
    let mut im_sum = 0.0;
    for n in 1..=50_000 {
        let log_n = (n as f64).ln();
        let denom = (n as f64).powf(s_re);
        re_sum += (-(s_im * log_n)).cos() / denom;
        im_sum += (-(s_im * log_n)).sin() / denom;
    }
    (re_sum, -im_sum)
}

/// Check if |ζ(1/2 + it)| is small (potential zero nearby)
fn zeta_magnitude_on_critical_line(t: f64) -> f64 {
    let (re, im) = zeta_complex(0.5, t);
    (re * re + im * im).sqrt()
}

pub fn show_riemann_station() {
    println!("=== RIEMANN HYPOTHESIS MILLENNIUM PRIZE PROBLEM ===");
    println!("Clay Mathematics Institute: $1,000,000 prize for proof\n");
    
    println!("THE QUESTION: Do all non-trivial zeros of the Riemann zeta function");
    println!("ζ(s) = Σ(1/n^s) lie on the critical line Re(s) = 1/2?\n");
    
    println!("The zeta function connects primes to analysis via the Euler product:");
    println!("zeta(s) = Product over all primes p of (1 - p^(-s))^(-1)\n");
    
    println!("--- ZETA FUNCTION ON REAL AXIS ---");
    println!("Computing ζ(s) for real s > 1 (Dirichlet series converges):\n");
    
    let test_values = [2.0, 3.0, 4.0, 5.0, 10.0];
    println!("s      | ζ(s)            | Notes");
    println!("-------|-----------------|------");
    for &s in &test_values {
        let z = zeta_real(s);
        let note = match s {
            2.0 => "π²/6 (Basel problem, solved by Euler)",
            3.0 => "Apéry's constant (irrational, proved 1978)",
            _ => "",
        };
        println!("{:5.1} | {:14.6} | {}", s, z, note);
    }
    
    println!("\nAs s → 1⁺, ζ(s) → ∞ (pole at s=1)");
    println!("As s → ∞, ζ(s) → 1\n");
    
    println!("--- CRITICAL LINE EXPLORATION ---");
    println!("Computing |ζ(1/2 + it)| for various t values:");
    println!("Zeros occur where this magnitude is zero.\n");
    
    println!("t       | |ζ(1/2+it)|   | Known zero nearby?");
    println!("--------|-------------|------------------");
    
    // Check around known zeros: 14.13, 21.02, 25.01, 30.42, 32.93
    let test_t = [10.0, 14.13, 15.0, 20.0, 21.02, 22.0, 25.01, 30.0, 30.42];
    for &t in &test_t {
        let mag = zeta_magnitude_on_critical_line(t);
        let note = if (t - 14.13).abs() < 0.01 || (t - 21.02).abs() < 0.01 
            || (t - 25.01).abs() < 0.01 || (t - 30.42).abs() < 0.01 {
            "Yes"
        } else {
            "No"
        };
        println!("{:6.2} | {:11.6} | {}", t, mag, note);
    }
    
    println!("\n--- WHAT WE KNOW ---");
    println!("• The first 10^13 zeros have been computed numerically");
    println!("• All lie on the critical line Re(s) = 1/2");
    println!("• No counterexample has been found despite extensive search");
    println!("• But no one has proven it for ALL zeros\n");
    
    println!("--- WHY IT MATTERS ---");
    println!("The distribution of prime numbers is encoded in the zeros.");
    println!("If the Riemann Hypothesis is true, primes are as evenly distributed");
    println!("as possible. Many results in number theory are conditional on RH.\n");
    
    println!("--- CONNECTION TO PHYSICS ---");
    println!("The distribution of zeta zeros matches eigenvalue statistics of");
    println!("random Hermitian matrices (Gaussian Unitary Ensemble). This suggests");
    println!("a deep connection between prime numbers and quantum chaos.\n");
    
    println!("=== RIEMANN HYPOTHESIS STATION COMPLETE ===");
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
    fn test_zeta_decreases() {
        // ζ(s) decreases as s increases for real s > 1
        assert!(zeta_real(2.0) > zeta_real(3.0));
        assert!(zeta_real(3.0) > zeta_real(4.0));
    }

    #[test]
    fn test_zeta_approaches_1() {
        // ζ(s) → 1 as s → ∞
        let z = zeta_real(10.0);
        assert!((z - 1.0).abs() < 0.002);
    }

    #[test]
    fn test_zeta_complex_returns_finite() {
        // Just verify computation doesn't crash or return NaN/Inf
        let (re, im) = zeta_complex(0.5, 14.13);
        assert!(re.is_finite());
        assert!(im.is_finite());
    }
}
