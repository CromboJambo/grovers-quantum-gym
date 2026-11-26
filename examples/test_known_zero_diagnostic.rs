use shor_grover_exploration::riemann::zeta_riemann_siegel::*;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;
use num_complex::Complex64;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  KNOWN ZERO DIAGNOSTIC: t = 14.134725...");
    println!("  Three-layer verification with explicit residuals");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let t0 = 14.134725; // Known first nontrivial zero ordinate
    
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ LAYER 1: θ(t) — THE COORDINATE TRANSFORMATION              │");
    println!("├─────────────────────────────────────────────────────────┤");
    
    let theta = riemann_siegel_theta(t0);
    println!("θ({:.6}) = {:.10}", t0, theta);
    println!();
    println!("Formula: θ(t) = Im[log Γ(1/4 + it/2)] - (t/2)·log(π)");
    println!("Status: COMPUTED ACCURATELY (gamma function is well-conditioned)");
    println!("Certification: ✓ Can be computed to arbitrary precision");
    println!();
    
    // Verify theta via the asymptotic expansion
    let t = t0;
    let log_term = (t / (2.0 * std::f64::consts::PI)).ln();
    let theta_asymptotic = 0.5 * t * log_term - 0.5 * t 
        - std::f64::consts::FRAC_PI_8 
        + 1.0 / (48.0 * t) 
        + 7.0 / (5760.0 * t.powi(3)) 
        + 31.0 / (80640.0 * t.powi(5));
    
    println!("Asymptotic check: θ_asymptotic = {:.10}", theta_asymptotic);
    println!("Difference from computed: {:.2e}", (theta - theta_asymptotic).abs());
    println!();
    
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ LAYER 2: Z(t) — THE TRANSFORMED SIGNAL                    │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!();
    
    // Method A: Basic RS sum
    let z_basic = basic_rs_sum(t0);
    println!("Method A (Basic RS sum, N=optimal+5):");
    println!("  Z(t) = {:.10e}", z_basic);
    println!("  Status: APPROXIMATE (no error bound established)");
    println!();
    
    // Method B: Current zeta_z_function (RS with more terms)
    let z_full = zeta_z_function(t0);
    println!("Method B (Current RS implementation):");
    println!("  Z(t) = {:.10e}", z_full);
    println!("  Status: APPROXIMATE (no error bound established)");
    println!();
    
    // Method C: EM on critical line (known to be problematic)
    let s = Complex64::new(0.5, t0);
    let mut prev_em = 0.0;
    println!("Method C (Euler-Maclaurin at s=1/2+it):");
    for &n in &[7, 10, 20, 50, 100] {
        let z_em = zeta_euler_maclaurin(s, n);
        let z_rotated = Complex64::new(theta.cos(), theta.sin()) * z_em;
        println!("  N={:>3}: Z_rotated = {:>12.6e} + {:>12.6e}i", 
            n, z_rotated.re, z_rotated.im);
    }
    println!("  Status: NOT CONVERGING (|Z| grows with N)");
    println!();
    
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ LAYER 3: NUMERICAL CERTIFICATION STATUS                  │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!();
    
    // Test 1: Reality
    let z_rotated = Complex64::new(theta.cos(), theta.sin()) 
        * Complex64::new(z_basic, 0.0); // Z_basic is already real by construction
    println!("Test 1 — REALITY: Im(Z(t)) within certified error?");
    println!("  Z_basic is real BY CONSTRUCTION (cosine sum)");
    println!("  But magnitude is uncertain → NOT CERTIFIED");
    println!();
    
    // Test 2: Zero convergence
    println!("Test 2 — ZERO CONVERGENCE: Does Z(γ) contain zero?");
    println!("  Z_basic = {:.4e}", z_basic);
    println!("  Without error bound, cannot certify Z ∈ [-ε, ε]");
    println!("  NOT CERTIFIED");
    println!();
    
    // Test 3: Winding
    println!("Test 3 — WINDING: Is argument-principle count enclosed?");
    println!("  Requires certified Z(t) on a contour around t0");
    println!("  NOT CERTIFIED (depends on Tests 1 and 2)");
    println!();
    
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ DUAL REPRESENTATION: COMPLEX vs REAL TRAJECTORY          │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!();
    println!("Keeping both representations as you suggested:");
    println!();
    
    // Complex trajectory near the zero
    println!("Complex trajectory ζ(1/2+it) near t0 (via RS reconstruction):");
    println!("  (Reconstructed from Z_basic via zeta = e^(-i*theta)*Z)");
    println!();
    println!("  t           Re(zeta)       Im(zeta)      |zeta|     arg(zeta)");
    println!("  {}", "-".repeat(65));
    
    for dt in [-0.5, -0.2, -0.1, 0.0, 0.1, 0.2, 0.5] {
        let t = t0 + dt;
        let theta_t = riemann_siegel_theta(t);
        let z_t = basic_rs_sum(t);
        let zeta = Complex64::new(theta_t.cos(), -theta_t.sin()) * z_t;
        println!("  {:>10.3}   {:>12.6e}  {:>12.6e}  {:>10.6e}  {:>9.4}", 
            t, zeta.re, zeta.im, zeta.norm(), zeta.arg());
    }
    
    println!();
    println!("Real trajectory Z(t) near t0:");
    println!("  t           Z(t)");
    println!("  {}", "-".repeat(25));
    
    for dt in [-0.5, -0.2, -0.1, 0.0, 0.1, 0.2, 0.5] {
        let t = t0 + dt;
        let z_t = basic_rs_sum(t);
        println!("  {:>10.3}   {:>14.8e}", t, z_t);
    }
    
    println!();
    println!("RELATIONSHIP: Z(t) = |ζ(1/2+it)| when arg(ζ) = -θ(t)");
    println!("At the zero: both representations should vanish simultaneously.");
    println!("The complex trajectory spirals in; the real trajectory crosses zero.");
    println!();
    
    println!("═══════════════════════════════════════════════════════");
    println!("  RESEARCH PROGRAM (refined)");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("θ(t) is ALREADY KNOWN. The question is not to rediscover it.");
    println!("The question is: WHAT BECOMES VISIBLE when we use θ(t) as");
    println!("our coordinate system?");
    println!();
    println!("Specific testable hypotheses:");
    println!("  1. Does Z(t) exhibit predictable structure between zeros?");
    println!("  2. Are the shapes of Z(t) near different zeros similar");
    println!("     (after appropriate scaling)?");
    println!("  3. Does the geometry of Z(t) (curvature, inflection points)");
    println!("     predict zero locations?");
    println!("  4. What is the relationship between the complex trajectory's");
    println!("     winding and the real trajectory's sign changes?");
    println!();
    println!("These are questions about the SHAPE OF CHANGE, not the data.");
}

fn basic_rs_sum(t: f64) -> f64 {
    let theta = riemann_siegel_theta(t);
    let n_terms = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize + 5;
    
    let mut sum = 0.0;
    for n in 1..=n_terms {
        sum += (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
    }
    2.0 * sum
}
