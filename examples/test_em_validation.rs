use num_complex::Complex64;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;

fn main() {
    println!("=== EM VALIDATION AT KNOWN POINTS ===");
    
    // ζ(2) = π²/6 ≈ 1.644934
    let zeta_2 = zeta_euler_maclaurin(Complex64::new(2.0, 0.0), 500);
    let expected_2 = std::f64::consts::PI.powi(2) / 6.0;
    println!("ζ(2)       = {:.10}", zeta_2.re);
    println!("π²/6       = {:.10}", expected_2);
    println!("Error      = {:.2e}", (zeta_2.re - expected_2).abs());
    println!();
    
    // ζ(3) ≈ 1.202057
    let zeta_3 = zeta_euler_maclaurin(Complex64::new(3.0, 0.0), 500);
    println!("ζ(3)       = {:.10}", zeta_3.re);
    println!("Expected   = 1.2020569032");
    println!("Error      = {:.2e}", (zeta_3.re - 1.2020569032).abs());
    println!();
    
    // ζ(0.75 + 10i) — Re(s) > 0.5, should be stable
    let zeta_test = zeta_euler_maclaurin(Complex64::new(0.75, 10.0), 2000);
    println!("ζ(0.75+10i) = {:.8}", zeta_test);
    
    // ζ(0.25 + 10i) — Re(s) < 0.5, problematic region
    let zeta_test2 = zeta_euler_maclaurin(Complex64::new(0.25, 10.0), 2000);
    println!("ζ(0.25+10i) = {:.8}", zeta_test2);
    
    // Check functional equation with CORRECT Stirling gamma
    let chi = compute_chi(Complex64::new(0.25, 10.0));
    let zeta_1_minus_s = zeta_euler_maclaurin(Complex64::new(0.75, -10.0), 2000);
    let fe_result = chi * zeta_1_minus_s;
    println!("χ·ζ(1-s)   = {:.8}", fe_result);
    println!("FE error   = {:.2e}", (zeta_test2 - fe_result).norm());
    println!();
    
    // Validate Stirling gamma at known points
    println!("=== STIRLING GAMMA VALIDATION ===");
    // Γ(1) = 1, Γ(2) = 1, Γ(3) = 2, Γ(4) = 6
    for &n in &[1.0, 2.0, 3.0, 4.0, 5.0] {
        let g = stirling_gamma(Complex64::new(n, 0.0));
        let expected = (1..n as usize).fold(1.0, |acc, x| acc * x as f64);
        println!("Γ({}) = {:.8}, expected {}, error = {:.2e}", 
            n, g.re, expected, (g.re - expected).abs());
    }
}

fn compute_chi(s: Complex64) -> Complex64 {
    let pi = std::f64::consts::PI;
    // χ(s) = 2^s · π^(s-1) · sin(πs/2) · Γ(1-s)
    let two_s = Complex64::new(0.0, s.im * 2.0f64.ln()).exp() * 2.0f64.powf(s.re);
    let pi_term = Complex64::new((s.re - 1.0) * pi.ln(), s.im * pi.ln()).exp();
    let sin_term = (Complex64::new(0.0, pi / 2.0) * s).sin();
    let gamma_term = stirling_gamma(Complex64::new(1.0 - s.re, -s.im));
    
    two_s * pi_term * sin_term * gamma_term
}

/// Correct Stirling approximation: Γ(z) ≈ √(2π) · z^(z-1/2) · e^(-z)
fn stirling_gamma(z: Complex64) -> Complex64 {
    if z.re < 0.5 {
        // Reflection formula: Γ(z)Γ(1-z) = π/sin(πz)
        let one_minus_z = Complex64::new(1.0 - z.re, -z.im);
        let sin_pi_z = (Complex64::new(0.0, std::f64::consts::PI) * z).sin();
        return Complex64::new(std::f64::consts::PI, 0.0) / sin_pi_z / stirling_gamma(one_minus_z);
    }
    
    // Γ(z) ≈ √(2π) · z^(z-1/2) · e^(-z) · (1 + 1/(12z))
    let sqrt_2pi = (2.0 * std::f64::consts::PI).sqrt();
    let z_pow = z.powc(z - 0.5);
    let exp_term = (-z).exp();
    let correction = 1.0 + 1.0 / (12.0 * z);
    
    sqrt_2pi * z_pow * exp_term * correction
}
