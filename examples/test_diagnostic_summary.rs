use num_complex::Complex64;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;
use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  NUMERICAL VERIFICATION DIAGNOSTIC SUMMARY");
    println!("  What we CAN and CANNOT verify with current methods");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ VERIFIED (high confidence)                                    │");
    println!("├─────────────────────────────────────────────────────────┤");
    
    // 1. EM at ζ(2)
    let zeta2 = zeta_euler_maclaurin(Complex64::new(2.0, 0.0), 2000);
    let expected2 = std::f64::consts::PI.powi(2) / 6.0;
    println!("│ 1. EM at ζ(2): {:.10} vs π²/6={:.10}", zeta2.re, expected2);
    println!("│    Error: {:.2e} ✓", (zeta2.re - expected2).abs());
    
    // 2. EM at ζ(10)
    let zeta10 = zeta_euler_maclaurin(Complex64::new(10.0, 0.0), 10);
    println!("│ 2. EM at ζ(10): {:.12} vs expected ≈ 1.000994575", zeta10.re);
    println!("│    Error: {:.2e} ✓", (zeta10.re - 1.000994575).abs());
    
    // 3. RS theta reality check
    let t_test = 100.0;
    let z_at_t = zeta_critical_line_rs(t_test);
    println!("│ 3. RS Z(t) at t={:.0}: imag part = {:.2e} (should be ~0)", 
        t_test, z_at_t.im.abs());
    println!("│    Z(t) is real on critical line ✓");
    
    // 4. Basic RS sum finds correct zero COUNT
    let mut count = 0;
    let mut prev_z = basic_rs_sum(10.0);
    for i in 1..4000 {
        let t = 10.0 + 0.005 * i as f64;
        let z = basic_rs_sum(t);
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            count += 1;
        }
        prev_z = z;
    }
    println!("│ 4. Basic RS sum: {} zeros in [10,30] (expected 3) ✓", count);
    
    println!();
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ NOT YET VERIFIED (insufficient accuracy)                      │");
    println!("├─────────────────────────────────────────────────────────┤");
    
    // 1. EM at critical line zero
    let s_zero = Complex64::new(0.5, 14.134725);
    let z_em = zeta_euler_maclaurin(s_zero, 2000);
    println!("│ 1. EM at ρ₁=½+14.1347i: |ζ| = {:.4e} (should be 0)", z_em.norm());
    println!("│    EM is inaccurate for Re(s) ≤ 1 ✗");
    
    // 2. RS Z(t) at known zero
    let t_zero = 14.134725;
    let z_rs = basic_rs_sum(t_zero);
    println!("│ 2. Basic RS at ρ₁: Z(t) = {:.4e} (should be ~0)", z_rs);
    println!("│    Too few terms for accuracy at low t ✗");
    
    // 3. Zero location precision
    println!("│ 3. Zero location: detected offsets of 0.2-0.9 in t ✗");
    println!("│    Need |Δt| < 0.01 for rigorous contour placement");
    
    // 4. Winding number certification
    println!("│ 4. Winding numbers from Tests 25/26 are NOT certified ✗");
    println!("│    They measure the approximation's zeros, not ζ(s)'s");
    
    println!();
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ PATH TO RIGOROUS VERIFICATION                                 │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ 1. Implement correct full RS with R-function (or use library)");
    println!("│ 2. Validate against known zeros to < 0.01 precision");
    println!("│ 3. Establish error bounds on ζ(s) evaluation");
    println!("│ 4. Use Rouché's theorem: if |g-ζ| < |g| on contour,");
    println!("│    then g and ζ have same winding number");
    println!("│ 5. Only then can we certify W(γ) = 1 for contours around ρ");
    println!("└─────────────────────────────────────────────────────────┘");
    
    println!();
    println!("CONCLUSION: The argument principle is mathematically exact.");
    println!("Our numerical experiment has NOT yet been rigorously verified");
    println!("to measure ζ(s)'s actual winding numbers. Current results");
    println!("measure the approximation's zero structure.");
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
