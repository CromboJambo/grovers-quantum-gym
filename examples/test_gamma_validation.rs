use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  GAMMA_UPPER VALIDATION (fixed coefficients)");
    println!("═══════════════════════════════════════════════════════");
    
    // Test Γ(1, x) = e^(-x) (exact for a=1)
    println!();
    println!("  Gamma(1, x) should equal exp(-x):");
    for &x in &[0.5_f64, 1.0, 2.0, 5.0] {
        let val = gamma_upper(1.0, x);
        let exact = (-x).exp();
        println!("    Gamma(1, {:.1}) = {:.10}  (exact: {:.10}, error: {:.2e})", 
                 x, val, exact, (val - exact).abs());
    }
    
    // Test Γ(2, x) = e^(-x)(1+x) (exact for a=2)
    println!();
    println!("  Gamma(2, x) should equal exp(-x)*(1+x):");
    for &x in &[0.5_f64, 1.0, 2.0, 5.0] {
        let val = gamma_upper(2.0, x);
        let exact = (-x).exp() * (1.0 + x);
        println!("    Gamma(2, {:.1}) = {:.10}  (exact: {:.10}, error: {:.2e})", 
                 x, val, exact, (val - exact).abs());
    }
    
    // Test Γ(0.5, 0) = sqrt(pi)
    println!();
    println!("  Gamma(0.5, 0) should equal sqrt(pi):");
    let val = gamma_upper(0.5, 0.0);
    let sqrt_pi = std::f64::consts::PI.sqrt();
    println!("    Gamma(0.5, 0) = {:.10}  (error: {:.2e})", 
             val, (val - sqrt_pi).abs());
    
    // Test at values relevant to R-function
    println!();
    println!("  Values relevant to R-function at t=50:");
    let x = 50.0 / (4.0 * std::f64::consts::PI);
    println!("    x = t/(4*pi) = {:.9}", x);
    for &k in &[0_f64, 1.0, 2.0, 3.0] {
        let val = gamma_upper(k + 0.25, x);
        println!("    Gamma({:.2}, {:.6}) = {:.10}", k + 0.25, x, val);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
