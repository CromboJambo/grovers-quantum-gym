use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("=== RS ZERO-FINDING VALIDATION (t ≥ 50) ===");
    println!();
    
    // Known zeros above 50 (from verified tables):
    let known_zeros = vec![
        49.7738324148899,
        52.9703214777136,
        56.4462476955779,
        59.3470440034354,
        62.8871662878908,
    ];
    
    println!("{:>12} {:>12} {:>12} {:>10}", "Known zero", "Z(t₀-ε)", "Z(t₀+ε)", "Sign flip?");
    println!("{}", "-".repeat(52));
    
    let eps = 0.001;
    for &t0 in &known_zeros {
        let z_left = zeta_z_function(t0 - eps);
        let z_right = zeta_z_function(t0 + eps);
        let sign_flip = (z_left < 0.0 && z_right > 0.0) || (z_left > 0.0 && z_right < 0.0);
        
        println!("{:>12.6} {:>12.6e} {:>12.6e} {:>10}", 
            t0, z_left, z_right,
            if sign_flip { "[v] YES" } else { "[x] no" });
    }
    
    println!();
    println!("=== Z(t) VALUES NEAR FIRST KNOWN ZERO ===");
    let t0 = 49.7738324148899;
    for dt in [-0.05, -0.02, -0.01, -0.005, 0.0, 0.005, 0.01, 0.02, 0.05] {
        let t = t0 + dt;
        let z = zeta_z_function(t);
        println!("t = {:>12.8}, Z(t) = {:>14.8e}", t, z);
    }
    
    println!();
    println!("=== FINDING ZEROS VIA SIGN CHANGES (t ∈ [49, 65]) ===");
    let mut zeros_found = Vec::new();
    let step = 0.01;
    let mut prev_t = 49.0;
    let mut prev_z = zeta_z_function(prev_t);
    
    let mut t = 49.0 + step;
    while t < 65.0 {
        let z = zeta_z_function(t);
        
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            // Bisection to refine
            let mut a = prev_t;
            let mut b = t;
            for _ in 0..50 {
                let mid = (a + b) / 2.0;
                let z_mid = zeta_z_function(mid);
                if (zeta_z_function(a) < 0.0 && z_mid > 0.0) || 
                   (zeta_z_function(a) > 0.0 && z_mid < 0.0) {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            zeros_found.push((a + b) / 2.0);
        }
        
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Zeros found in [49, 65]:");
    for &z in &zeros_found {
        println!("  t = {:.8}", z);
    }
}
