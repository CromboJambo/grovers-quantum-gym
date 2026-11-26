use shor_grover_exploration::riemann::zeta_riemann_siegel::*;
use num_complex::Complex64;

fn gamma_upper(a: f64, x: f64) -> f64 {
    // Compute upper incomplete gamma function Γ(a, x) using series for small x
    // and continued fraction for large x
    if x < 1.0 + a {
        // Series representation
        let mut sum = 1.0 / a;
        let mut term = 1.0 / a;
        for n in 1..100 {
            term *= x / (a + n as f64);
            sum += term;
            if term.abs() < 1e-15 * sum.abs() { break; }
        }
        // Γ(a,x) = Γ(a) - γ(a,x), where γ is lower incomplete gamma
        // But we need the upper one, so use: Γ(a,x) = x^a · e^{-x} · Σ_{n=0}^{∞} x^n / (a(a+1)...(a+n))
        let mut result = 0.0;
        let mut t = 1.0;
        for n in 0..200 {
            if n > 0 {
                t *= x / (a + (n - 1) as f64);
            }
            result += t;
            if t.abs() < 1e-15 * result.abs() && n > 10 { break; }
        }
        result * x.powf(a) * (-x).exp()
    } else {
        // Continued fraction (Lentz's algorithm)
        let mut f = 1.0 / x;
        let mut c = 1.0 / x;
        let mut d = 0.0;
        for n in 0..200 {
            let an = if n == 0 { 1.0 - a } else { -(n as f64 - 1.0) * (n as f64 - a) };
            let bn = x + 2.0 * n as f64;
            
            d = 1.0 / (bn + an * d);
            if d.abs() < 1e-30 { d = 1e-30; }
            c = bn + an / c;
            if c.abs() < 1e-30 { c = 1e-30; }
            
            let delta = c * d;
            f *= delta;
            
            if (delta - 1.0).abs() < 1e-15 && n > 10 { break; }
        }
        (-x).exp() * x.powf(a) / f
    }
}

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  R-FUNCTION COMPONENT TEST AT t = 14.134725");
    println!("═══════════════════════════════════════════════════════");
    
    let t = 14.134725;
    let x = t / (4.0 * std::f64::consts::PI);
    
    println!("  t = {:.9}", t);
    println!("  x = t/(4π) = {:.9}", x);
    println!();
    
    // Test gamma_upper at a few values
    println!("  Γ(1/4, x) = {:.10}", gamma_upper(0.25, x));
    println!("  Γ(5/4, x) = {:.10}", gamma_upper(1.25, x));
    println!();
    
    // Compute R(t) manually for k=0 and k=1
    let term0 = 0.5 * t.powf(-0.25) * gamma_upper(0.25, x);
    let deriv1 = -0.75 * t.powf(-1.75);
    let term1 = (-1.0 / 8.0) * deriv1 * gamma_upper(1.25, x);
    
    println!("  Term k=0: {:.10}", term0);
    println!("  Term k=1: {:.10}", term1);
    println!("  R(t) ≈ k=0+1: {:.10}", term0 + term1);
    
    // Compare with basic RS at this t
    let theta = riemann_siegel_theta(t);
    let n_terms = ((t / (2.0 * std::f64::consts::PI)).sqrt() as usize) + 5;
    
    let mut sum = 0.0_f64;
    for n in 1..=n_terms {
        let phase = theta - t * (n as f64).ln();
        sum += phase.cos() / (n as f64).sqrt();
    }
    let basic_z = 2.0 * sum;
    
    println!();
    println!("  Basic RS Z(t): {:.10}", basic_z);
    println!("  Full RS Z(t) with R: {:.10}", basic_z + term0 + term1);
    println!();
    println!("  Expected: Z(t) ≈ 0 at true zero");
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
