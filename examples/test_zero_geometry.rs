use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  HYPOTHESIS TEST: UNIVERSAL LOCAL GEOMETRY NEAR ZEROS");
    println!("  Are Z(t) shapes similar after scaling?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Known nontrivial zeros (first 8)
    let known_zeros = vec![
        14.134725, 21.022040, 25.010858, 30.424876,
        32.935062, 37.586178, 40.918719, 43.327073
    ];
    
    println!("Known zeros: {:?}", known_zeros);
    println!();
    
    // Compute Z(t) in a window around each zero
    // Window: ±1.5 units (roughly half the spacing to neighbors)
    let half_window = 1.5;
    let n_samples = 61; // 61 points across the window
    
    println!("Computing Z(t) near each zero...");
    println!();
    
    // Store normalized shapes for comparison
    let mut all_shapes: Vec<(f64, Vec<f64>)> = Vec::new(); // (zero_location, shape)
    
    for &t0 in &known_zeros {
        let mut ts = Vec::new();
        let mut zs = Vec::new();
        
        for i in 0..n_samples {
            let t = t0 - half_window + 2.0 * half_window * (i as f64) / ((n_samples - 1) as f64);
            let z = basic_rs_sum(t);
            ts.push(t);
            zs.push(z);
        }
        
        // Find actual zero crossing in our data (sign change)
        let mut zero_found = false;
        for i in 0..zs.len() - 1 {
            if (zs[i] < 0.0 && zs[i+1] > 0.0) || (zs[i] > 0.0 && zs[i+1] < 0.0) {
                zero_found = true;
                break;
            }
        }
        
        // Normalize: x by half_window, y by max|Z| in window
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let normalized_x: Vec<f64> = ts.iter().map(|t| (t - t0) / half_window).collect();
        let normalized_y: Vec<f64> = zs.iter().map(|z| if max_z > 1e-15 { z / max_z } else { 0.0 }).collect();
        
        all_shapes.push((t0, normalized_y.clone()));
        
        println!("Zero at t={:.6}: zero_crossing={}, max|Z|={:.4e}", 
            t0, zero_found, max_z);
        println!("  Normalized shape (first/last 5 points):");
        for i in 0..5 {
            println!("    x={:>6.3}, y={:>8.4}", normalized_x[i], normalized_y[i]);
        }
        println!("    ...");
        for i in (n_samples-5)..n_samples {
            println!("    x={:>6.3}, y={:>8.4}", normalized_x[i], normalized_y[i]);
        }
        println!();
    }
    
    // Compare shapes: compute pairwise correlation
    println!("═══════════════════════════════════════════════════════");
    println!("  SHAPE SIMILARITY: PAIRWISE CORRELATION COEFFICIENTS");
    println!("  (1.0 = identical shape, 0.0 = uncorrelated)");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let n_zeros = all_shapes.len();
    print!("        ");
    for i in 0..n_zeros {
        print!("{:>7.1}", known_zeros[i]);
    }
    println!();
    
    for i in 0..n_zeros {
        print!("{:>6.1} ", known_zeros[i]);
        for j in 0..n_zeros {
            if i == j {
                print!("{:>7.2}", 1.0);
            } else {
                let corr = correlation(&all_shapes[i].1, &all_shapes[j].1);
                print!("{:>7.2}", corr);
            }
        }
        println!();
    }
    
    // Also test with DIFFERENT normalization: by local spacing to neighbors
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  ALTERNATIVE: NORMALIZE BY LOCAL ZERO SPACING");
    println!("  (Tests if shape width scales with local density)");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let mut spacing_shapes: Vec<(f64, Vec<f64>)> = Vec::new();
    
    for (idx, &t0) in known_zeros.iter().enumerate() {
        // Compute local spacing: average distance to nearest neighbors
        let mut spacings = Vec::new();
        for &other in &known_zeros {
            if other != t0 {
                spacings.push((t0 - other).abs());
            }
        }
        spacings.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let local_spacing = (spacings[0] + spacings[1]) / 2.0; // avg of 2 nearest
        
        // Use window proportional to local spacing
        let hw = local_spacing * 0.5;
        let mut ts = Vec::new();
        let mut zs = Vec::new();
        
        for i in 0..n_samples {
            let t = t0 - hw + 2.0 * hw * (i as f64) / ((n_samples - 1) as f64);
            let z = basic_rs_sum(t);
            ts.push(t);
            zs.push(z);
        }
        
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let normalized_x: Vec<f64> = ts.iter().map(|t| (t - t0) / hw).collect();
        let normalized_y: Vec<f64> = zs.iter().map(|z| if max_z > 1e-15 { z / max_z } else { 0.0 }).collect();
        
        spacing_shapes.push((t0, normalized_y.clone()));
        
        println!("Zero at t={:.6}: local_spacing={:.3}, window=±{:.3}", 
            t0, local_spacing, hw);
    }
    
    // Correlation with spacing-based normalization
    println!();
    print!("        ");
    for i in 0..n_zeros {
        print!("{:>7.1}", known_zeros[i]);
    }
    println!();
    
    for i in 0..n_zeros {
        print!("{:>6.1} ", known_zeros[i]);
        for j in 0..n_zeros {
            if i == j {
                print!("{:>7.2}", 1.0);
            } else {
                let corr = correlation(&spacing_shapes[i].1, &spacing_shapes[j].1);
                print!("{:>7.2}", corr);
            }
        }
        println!();
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  INTERPRETATION");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("High correlation (>0.9) across all pairs → universal local geometry.");
    println!("Low correlation (<0.5) → each zero has unique local structure.");
    println!("Intermediate (0.5-0.9) → partial universality with variations.");
    println!();
    println!("CAVEAT: Using basic RS sum without error bounds.");
    println!("Results indicate STRUCTURAL TRENDS, not certified geometry.");
}

fn basic_rs_sum(t: f64) -> f64 {
    let theta = riemann_siegel_theta(t);
    let n_terms = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize + 5;
    
    let mut sum = 0.0;
    for n in 1..=n_terms {
        sum += (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
    }
    2.0 * sum
}

fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;
    
    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    
    for i in 0..a.len() {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    
    if var_a < 1e-30 || var_b < 1e-30 {
        return 0.0;
    }
    
    cov / (var_a.sqrt() * var_b.sqrt())
}
