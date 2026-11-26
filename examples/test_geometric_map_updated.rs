use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  GEOMETRIC MAP WITH UPDATED Z(t) (basic RS, N+5)");
    println!("═══════════════════════════════════════════════════════");
    
    // Find zeros in [10, 250] using sign changes + bisection
    let mut t = 10.0;
    let step = 0.005;
    let mut prev_z = zeta_z_function(t);
    let mut zeros = Vec::new();
    
    while t < 250.0 {
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
            zeros.push((a + b) / 2.0);
        }
        prev_z = z_val;
    }
    
    println!("Found {} zeros in [10, 250]", zeros.len());
    println!();
    
    // Analyze intervals between consecutive zeros
    let mut narrow_count = 0;
    let mut wide_count = 0;
    let mut narrow_ellipse_stds = Vec::new();
    let mut wide_ellipse_stds = Vec::new();
    let mut narrow_inflections = Vec::new();
    let mut wide_inflections = Vec::new();
    
    for i in 0..(zeros.len() - 1) {
        let t1 = zeros[i];
        let t2 = zeros[i + 1];
        let width = t2 - t1;
        
        // Sample Z(t) in this interval
        let n_samples = 201;
        let mut samples = Vec::new();
        for j in 0..n_samples {
            let u = j as f64 / (n_samples - 1) as f64;
            let t_val = t1 + u * width;
            samples.push(zeta_z_function(t_val));
        }
        
        // Normalize by max absolute value
        let max_abs = samples.iter().map(|&v| v.abs()).fold(0.0_f64, f64::max);
        if max_abs < 1e-10 { continue; }
        for s in samples.iter_mut() { *s /= max_abs; }
        
        // Compute ellipse_std (phase space complexity)
        let mut ellipse_vals = Vec::new();
        for j in 0..(n_samples - 1) {
            let dt = width / (n_samples - 1) as f64;
            let dz = (samples[j + 1] - samples[j]) / dt;
            ellipse_vals.push(dz);
        }
        
        if !ellipse_vals.is_empty() {
            let mean: f64 = ellipse_vals.iter().sum::<f64>() / ellipse_vals.len() as f64;
            let variance: f64 = ellipse_vals.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / ellipse_vals.len() as f64;
            let std_dev = variance.sqrt();
            
            // Count inflection points (sign changes in second derivative)
            let mut inflections = 0;
            for j in 0..(n_samples - 2) {
                let d2z = samples[j + 2] - 2.0 * samples[j + 1] + samples[j];
                if j > 0 {
                    let prev_d2z = samples[j + 1] - 2.0 * samples[j] + samples[j - 1];
                    if prev_d2z * d2z < 0.0 {
                        inflections += 1;
                    }
                }
            }
            
            if width < 3.0 {
                narrow_count += 1;
                narrow_ellipse_stds.push(std_dev);
                narrow_inflections.push(inflections as f64);
            } else if width >= 5.0 {
                wide_count += 1;
                wide_ellipse_stds.push(std_dev);
                wide_inflections.push(inflections as f64);
            }
        }
    }
    
    // Print results
    println!("─── NARROW intervals (W < 3.0): {} found", narrow_count);
    if !narrow_ellipse_stds.is_empty() {
        let mean_e: f64 = narrow_ellipse_stds.iter().sum::<f64>() / narrow_ellipse_stds.len() as f64;
        let mean_i: f64 = narrow_inflections.iter().sum::<f64>() / narrow_inflections.len() as f64;
        println!("  Mean ellipse_std: {:.4}", mean_e);
        println!("  Mean inflection points: {:.2}", mean_i);
    }
    
    println!();
    println!("─── WIDE intervals (W ≥ 5.0): {} found", wide_count);
    if !wide_ellipse_stds.is_empty() {
        let mean_e: f64 = wide_ellipse_stds.iter().sum::<f64>() / wide_ellipse_stds.len() as f64;
        let mean_i: f64 = wide_inflections.iter().sum::<f64>() / wide_inflections.len() as f64;
        println!("  Mean ellipse_std: {:.4}", mean_e);
        println!("  Mean inflection points: {:.2}", mean_i);
    }
    
    // Compute correlation between width and complexity
    if narrow_count > 0 && wide_count > 0 {
        let all_widths: Vec<f64> = (0..(zeros.len() - 1)).map(|i| zeros[i + 1] - zeros[i]).collect();
        let all_complexity: Vec<f64> = (0..all_widths.len()).map(|_| 0.0).collect(); // Placeholder
        
        println!();
        println!("─── Width vs Complexity Correlation");
        println!("  (Computing for all intervals...)");
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
