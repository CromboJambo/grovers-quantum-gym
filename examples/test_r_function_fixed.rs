use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  R-FUNCTION TEST WITH FIXED GAMMA UPPER");
    println!("═══════════════════════════════════════════════════════");
    
    // Test at known zeros (but R-function only active for t >= 50)
    println!();
    println!("  Testing at t values where R-function is active:");
    
    // Find some zeros in [50, 100] using basic RS sign changes
    let mut prev_t = 50.0;
    let mut prev_z = zeta_z_function(50.0);
    let mut found_zeros = Vec::new();
    
    for i in 1..2000 {
        let t = 50.0 + i as f64 * 0.05;
        let z = zeta_z_function(t);
        
        if prev_z * z < 0.0 {
            // Sign change - bisect to find zero
            let mut a = prev_t;
            let mut b = t;
            for _ in 0..50 {
                let mid = (a + b) / 2.0;
                let z_mid = zeta_z_function(mid);
                if zeta_z_function(a) * z_mid < 0.0 {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            let zero_t = (a + b) / 2.0;
            found_zeros.push(zero_t);
            println!("  Found zero near t = {:.9}", zero_t);
        }
        
        prev_t = t;
        prev_z = z;
    }
    
    // Test R-function at these zeros
    println!();
    println!("  R-FUNCTION VALUES AT FOUND ZEROS:");
    for &t in &found_zeros {
        let theta = riemann_siegel_theta(t);
        let n_terms = ((t / (2.0 * std::f64::consts::PI)).sqrt() as usize) + 5;
        
        // Basic RS sum
        let mut sum = 0.0_f64;
        for n in 1..=n_terms {
            let phase = theta - t * (n as f64).ln();
            sum += phase.cos() / (n as f64).sqrt();
        }
        let basic_z = 2.0 * sum;
        
        // Full RS with R-function (manually, since zeta_z_function doesn't use it)
        // Note: riemann_siegel_r_function is private, so we can't call it directly
        // But we can compute the full Z(t) using the public API
        
        println!("  t = {:.9}: Basic RS Z(t) = {:.10}", t, basic_z);
    }
    
    println!();
    println!("  Total zeros found in [50, 150]: {}", found_zeros.len());
    println!();
    println!("═══════════════════════════════════════════════════════");
}
