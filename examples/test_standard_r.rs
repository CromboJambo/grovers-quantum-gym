use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  STANDARD R-FUNCTION TEST (sourced formula)");
    println!("═══════════════════════════════════════════════════════");
    
    // Verified against LMFDB (https://www.lmfdb.org/zeros/Zip/first/)
    let known_zeros = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
        32.935061587739189,
        37.586178158825670,
        40.918719012147495,
        43.327073280914000,
        48.005150881657738,
        49.773832477672302,
        52.970325814187648,
        56.446247697178830,
        60.831859482717556,
        65.112544048287209,
    ];
    
    println!();
    println!("  Z(t) at known zeros (should be ~0):");
    for &t in &known_zeros {
        let z = zeta_z_function(t);
        let x = (t / (2.0 * std::f64::consts::PI)).sqrt();
        let n = x.floor() as usize;
        let p = x - n as f64;
        println!("  t = {:.9} (N={}, p={:.4}): Z(t) = {:.10}  (|Z| = {:.2e})", 
                 t, n, p, z, z.abs());
    }
    
    // Also test R-function values directly
    println!();
    println!("  R-function values:");
    for &t in &known_zeros {
        let r = riemann_siegel_r_function(t);
        println!("  R({:.9}) = {:.10}", t, r);
    }
    
    // Test zero-finding accuracy
    println!();
    println!("  Zero-finding test in [10, 50]:");
    let mut prev_t = 10.0;
    let mut prev_z = zeta_z_function(10.0);
    let mut found_zeros = Vec::new();
    
    for i in 1..800 {
        let t = 10.0 + i as f64 * 0.05;
        let z = zeta_z_function(t);
        
        if prev_z * z < 0.0 {
            // Bisect to find zero
            let mut a = prev_t;
            let mut b = t;
            for _ in 0..60 {
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
        }
        
        prev_t = t;
        prev_z = z;
    }
    
    let true_zeros = [14.1347, 21.0220, 25.0109, 30.4249, 32.9351, 37.5862, 40.9187, 43.3271, 48.0052, 49.7738];
    println!("  Found {} zeros in [10, 50]:", found_zeros.len());
    for (i, &z) in found_zeros.iter().enumerate() {
        if i < true_zeros.len() {
            let err = (z - true_zeros[i]).abs();
            println!("    Found: {:.9}  True: {:.9}  Error: {:.2e}", z, true_zeros[i], err);
        } else {
            println!("    Found: {:.9}", z);
        }
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
