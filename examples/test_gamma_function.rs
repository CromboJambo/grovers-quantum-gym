use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  GAMMA_FUNCTION VALIDATION");
    println!("═══════════════════════════════════════════════════════");
    
    // Test Γ(n) = (n-1)! for positive integers
    println!();
    println!("  Gamma(n) should equal (n-1)!:");
    let values = [
        (1.0, 1.0),   // Γ(1) = 0! = 1
        (2.0, 1.0),   // Γ(2) = 1! = 1
        (3.0, 2.0),   // Γ(3) = 2! = 2
        (4.0, 6.0),   // Γ(4) = 3! = 6
        (5.0, 24.0),  // Γ(5) = 4! = 24
    ];
    
    for &(a, expected) in &values {
        let val = gamma_function(a);
        println!("    Gamma({:.0}) = {:.10}  (expected: {:.10}, error: {:.2e})", 
                 a, val, expected, (val - expected).abs());
    }
    
    // Test Γ(0.5) = sqrt(pi)
    println!();
    let val = gamma_function(0.5);
    let sqrt_pi = std::f64::consts::PI.sqrt();
    println!("  Gamma(0.5) = {:.10}  (expected: {:.10}, error: {:.2e})", 
             val, sqrt_pi, (val - sqrt_pi).abs());
    
    // Test Γ(1.5) = 0.5·sqrt(pi)
    println!();
    let val = gamma_function(1.5);
    let expected = 0.5 * sqrt_pi;
    println!("  Gamma(1.5) = {:.10}  (expected: {:.10}, error: {:.2e})", 
             val, expected, (val - expected).abs());
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
