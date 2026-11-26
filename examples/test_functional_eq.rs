use num_complex::Complex64;
use std::f64::consts::PI;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;

fn main() {
    println!("=== FUNCTIONAL EQUATION TEST ===");
    println!("ζ(s) = χ(s)·ζ(1-s) where χ(s) = 2^s·π^(s-1)·sin(πs/2)·Γ(1-s)");
    println!();
    
    // Test at several points using EM approximation
    let test_points = vec![
        Complex64::new(0.3, 5.0),
        Complex64::new(0.7, 8.0),
        Complex64::new(0.2, 12.0),
        Complex64::new(0.8, 15.0),
        Complex64::new(0.5, 20.0),  // On critical line (self-dual)
    ];
    
    for &s in &test_points {
        let zeta_s = zeta_euler_maclaurin(s, 2000);
        let zeta_1_minus_s = zeta_euler_maclaurin(Complex64::new(1.0 - s.re, -s.im), 2000);
        
        // Compute χ(s) = 2^s · π^(s-1) · sin(πs/2) · Γ(1-s)
        // Using Stirling's approximation for Gamma for now
        let chi_s = compute_chi(s);
        let rhs = chi_s * zeta_1_minus_s;
        
        let rel_error = if zeta_s.norm() > 1e-10 {
            (zeta_s - rhs).norm() / zeta_s.norm()
        } else {
            f64::MAX
        };
        
        println!("s = {:.1} + {:.1}i:", s.re, s.im);
        println!("  ζ(s)       = {:.6}", zeta_s);
        println!("  χ·ζ(1-s)   = {:.6}", rhs);
        println!("  Rel error  = {:.4e}", rel_error);
        println!();
    }
}

/// Compute χ(s) = 2^s · π^(s-1) · sin(πs/2) · Γ(1-s)
fn compute_chi(s: Complex64) -> Complex64 {
    let two_s = Complex64::new(0.0, s.im * PI.ln()).exp() * 2.0f64.powf(s.re);
    let pi_term = Complex64::new((s.re - 1.0) * PI.ln(), s.im * PI.ln()).exp();
    let sin_term = (Complex64::new(0.0, PI / 2.0) * s).sin();
    let gamma_term = stirling_gamma(Complex64::new(1.0 - s.re, -s.im));
    
    two_s * pi_term * sin_term * gamma_term
}

/// Stirling's approximation for Gamma function
fn stirling_gamma(z: Complex64) -> Complex64 {
    if z.re < 0.5 {
        // Reflection formula: Γ(z)Γ(1-z) = π/sin(πz)
        let one_minus_z = Complex64::new(1.0 - z.re, -z.im);
        let sin_pi_z = (Complex64::new(0.0, PI) * z).sin();
        return Complex64::new(PI, 0.0) / sin_pi_z / stirling_gamma(one_minus_z);
    }
    
    let z_minus_half = z - 0.5;
    let base = (z_minus_half / std::f64::consts::E).exp();
    let coeff = (2.0 * PI).sqrt() * z_minus_half.powc(z - 0.5);
    
    // First correction term
    let correction = 1.0 + 1.0 / (12.0 * z);
    
    base * coeff * correction
}
