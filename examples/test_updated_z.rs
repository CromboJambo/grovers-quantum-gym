use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  UPDATED Z(t) AT KNOWN ZEROS (basic RS, N+5, no R-func)");
    println!("═══════════════════════════════════════════════════════");
    
    let known_zeros = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
        32.935061587739189,
        37.586178158825670,
        40.918719012147495,
        43.327073280914000,
    ];
    
    println!();
    println!("  t (true)         Z(t)          |Z(t)|");
    println!("  ------------------------------ ------------ ----------");
    
    for &t in &known_zeros {
        let z_val = zeta_z_function(t);
        let abs_z = z_val.abs();
        println!("  {:.9}   {:>14.10}  {:>9.2e}", t, z_val, abs_z);
    }
    
    // Test sign changes to find zeros
    println!();
    println!("  Sign change scan in [10, 50] (step 0.005):");
    let mut t = 10.0;
    let step = 0.005;
    let mut prev_z = zeta_z_function(t);
    let mut zeros_found = Vec::new();
    
    while t < 50.0 {
        t += step;
        let z_val = zeta_z_function(t);
        if prev_z * z_val < 0.0 {
            // Sign change — bisect to find zero
            let mut a = t - step;
            let mut b = t;
            for _ in 0..50 {
                let mid = (a + b) / 2.0;
                if zeta_z_function(a) * zeta_z_function(mid) < 0.0 {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            zeros_found.push((a + b) / 2.0);
        }
        prev_z = z_val;
    }
    
    println!("  Found {} zeros in [10, 50]:", zeros_found.len());
    for &z in &zeros_found {
        println!("    {:.9}", z);
    }
    
    // Compare with known zeros
    println!();
    println!("  Comparison with known zeros:");
    let known_in_range = [14.134725141734693, 21.022039638771555, 25.010857580145688, 
                          30.424876125859513, 32.935061587739189];
    for &known in &known_in_range {
        let mut best_err = f64::MAX;
        let mut best_found = 0.0;
        for &found in &zeros_found {
            let err = (found - known).abs();
            if err < best_err {
                best_err = err;
                best_found = found;
            }
        }
        println!("    True: {:.9}  Found: {:.9}  Error: {:.2e}", known, best_found, best_err);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
