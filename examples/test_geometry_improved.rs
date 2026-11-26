use shor_grover_exploration::riemann::*;
use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  GEOMETRIC ANALYSIS WITH IMPROVED Z(t)");
    println!("═══════════════════════════════════════════════════════");
    
    // Find zeros in [10, 250] using sign changes
    let mut prev_t = 10.0;
    let mut prev_z = zeta_z_function(10.0);
    let mut found_zeros = Vec::new();
    
    for i in 1..4800 {
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
    
    println!("  Found {} zeros in [10, 250]", found_zeros.len());
    
    // Compute shape metrics for each zero interval
    let mut narrow_results = Vec::new();
    let mut wide_results = Vec::new();
    
    for i in 1..found_zeros.len().saturating_sub(1) {
        let t_center = (found_zeros[i-1] + found_zeros[i+1]) / 2.0;
        let width = found_zeros[i+1] - found_zeros[i-1];
        
        // Only analyze intervals with reasonable width
        if width < 0.5 || width > 10.0 {
            continue;
        }
        
        // Sample Z(t) across the interval
        let n_samples = 50;
        let mut samples = Vec::new();
        for j in 0..n_samples {
            let t = found_zeros[i-1] + width * (j as f64 / (n_samples - 1) as f64);
            let z = zeta_z_function(t);
            samples.push(z);
        }
        
        // Compute metrics
        let mean: f64 = samples.iter().sum::<f64>() / n_samples as f64;
        let variance: f64 = samples.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / n_samples as f64;
        let std_dev = variance.sqrt();
        
        // Count inflection points (sign changes in second derivative)
        let mut inflections = 0;
        for j in 2..samples.len().saturating_sub(1) {
            let d2_prev = samples[j] - 2.0 * samples[j-1] + samples[j-2];
            let d2_curr = samples[j+1] - 2.0 * samples[j] + samples[j-1];
            if d2_prev * d2_curr < 0.0 {
                inflections += 1;
            }
        }
        
        let result = (width, std_dev, inflections);
        if width < 3.0 {
            narrow_results.push(result);
        } else if width >= 5.0 {
            wide_results.push(result);
        }
    }
    
    println!();
    println!("  NARROW INTERVALS (width < 3.0): {} found", narrow_results.len());
    if !narrow_results.is_empty() {
        let mean_width: f64 = narrow_results.iter().map(|r| r.0).sum::<f64>() / narrow_results.len() as f64;
        let mean_std: f64 = narrow_results.iter().map(|r| r.1).sum::<f64>() / narrow_results.len() as f64;
        let mean_inflections: f64 = narrow_results.iter().map(|r| r.2 as f64).sum::<f64>() / narrow_results.len() as f64;
        println!("    Mean width: {:.4}", mean_width);
        println!("    Mean std_dev: {:.4}", mean_std);
        println!("    Mean inflections: {:.2}", mean_inflections);
    }
    
    println!();
    println!("  WIDE INTERVALS (width >= 5.0): {} found", wide_results.len());
    if !wide_results.is_empty() {
        let mean_width: f64 = wide_results.iter().map(|r| r.0).sum::<f64>() / wide_results.len() as f64;
        let mean_std: f64 = wide_results.iter().map(|r| r.1).sum::<f64>() / wide_results.len() as f64;
        let mean_inflections: f64 = wide_results.iter().map(|r| r.2 as f64).sum::<f64>() / wide_results.len() as f64;
        println!("    Mean width: {:.4}", mean_width);
        println!("    Mean std_dev: {:.4}", mean_std);
        println!("    Mean inflections: {:.2}", mean_inflections);
    }
    
    // Correlation between width and complexity
    let all_results: Vec<_> = narrow_results.iter().chain(wide_results.iter()).collect();
    if all_results.len() >= 3 {
        let n = all_results.len() as f64;
        let mean_w: f64 = all_results.iter().map(|r| r.0).sum::<f64>() / n;
        let mean_s: f64 = all_results.iter().map(|r| r.1).sum::<f64>() / n;
        let cov_ws: f64 = all_results.iter().map(|r| (r.0 - mean_w) * (r.1 - mean_s)).sum::<f64>() / n;
        let std_w: f64 = all_results.iter().map(|r| (r.0 - mean_w).powi(2)).sum::<f64>() / n;
        let std_s: f64 = all_results.iter().map(|r| (r.1 - mean_s).powi(2)).sum::<f64>() / n;
        if std_w > 0.0 && std_s > 0.0 {
            let corr = cov_ws / (std_w.sqrt() * std_s.sqrt());
            println!();
            println!("  CORRELATION (width vs std_dev): r = {:.4}", corr);
        }
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
