use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("=== SCANNING FOR Z(t) SIGN CHANGES ===");
    println!();
    
    // Scan t from 10 to 30 with fine step, look for sign changes
    let mut prev_t = 10.0;
    let mut prev_z = zeta_z_function(prev_t);
    let step = 0.005;
    let mut sign_changes = Vec::new();
    
    let mut t = 10.0 + step;
    while t < 30.0 {
        let z = zeta_z_function(t);
        
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            // Refine with bisection
            let mut a = prev_t;
            let mut b = t;
            for _ in 0..40 {
                let mid = (a + b) / 2.0;
                let z_mid = zeta_z_function(mid);
                if (zeta_z_function(a) * z_mid) < 0.0 {
                    b = mid;
                } else {
                    a = mid;
                }
            }
            sign_changes.push((a + b) / 2.0);
        }
        
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Sign changes (zeros) found in [10, 30]:");
    for &z in &sign_changes {
        println!("  t = {:.8}", z);
    }
    
    println!();
    println!("Known zeros in this range:");
    let known = vec![14.134725, 21.022040, 25.010858];
    for &k in &known {
        println!("  t = {:.8}", k);
    }
    
    println!();
    println!("=== COMPARISON ===");
    for &k in &known {
        // Find closest detected zero
        let closest = sign_changes.iter().map(|z| (*z, (z - k).abs())).min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        if let Some((detected, dist)) = closest {
            println!("Known {:>10.6} -> detected {:>10.6}, offset = {:>8.6}", k, detected, (detected - k));
        } else {
            println!("Known {:>10.6} -> NOT FOUND", k);
        }
    }
}
