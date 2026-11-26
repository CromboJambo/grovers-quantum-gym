use shor_grover_exploration::riemann::zeta_riemann_siegel::*;
use num_complex::Complex64;

fn main() {
    println!("=== BASIC RS SUM (NO R-FUNCTION) AT KNOWN ZEROS ===");
    println!();
    
    // Compute the basic RS sum without R-function
    let known_zeros = vec![14.134725, 21.022040, 25.010858, 49.773832, 52.970321];
    
    for &t in &known_zeros {
        let theta = riemann_siegel_theta(t);
        let n_terms = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize + 1;
        
        // Basic RS sum: Z(t) ≈ 2 * sum cos(theta - t*ln(n)) / sqrt(n)
        let mut sum = 0.0;
        for n in 1..=n_terms {
            sum += (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
        }
        let z_basic = 2.0 * sum;
        
        // Full RS with R-function (our current implementation)
        let z_full = zeta_z_function(t);
        
        println!("t={:>10.6}: N_terms={:>3}, Z_basic={:>12.6e}, Z_full={:>12.6e}", 
            t, n_terms, z_basic, z_full);
    }
    
    println!();
    println!("=== BASIC RS SIGN CHANGES IN [10, 30] ===");
    
    let mut prev_t = 10.0;
    let mut prev_z = basic_rs_sum(prev_t);
    let step = 0.005;
    let mut sign_changes = Vec::new();
    
    let mut t = 10.0 + step;
    while t < 30.0 {
        let z = basic_rs_sum(t);
        
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            sign_changes.push((prev_t + t) / 2.0);
        }
        
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Sign changes found: {}", sign_changes.len());
    for &z in &sign_changes {
        println!("  t = {:.8}", z);
    }
}

fn basic_rs_sum(t: f64) -> f64 {
    let theta = riemann_siegel_theta(t);
    let n_terms = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize + 1;
    
    let mut sum = 0.0;
    for n in 1..=n_terms {
        sum += (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
    }
    2.0 * sum
}
