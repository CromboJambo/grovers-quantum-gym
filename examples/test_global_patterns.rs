use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  GLOBAL PATTERNS & SCALING ANALYSIS");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // First, find more zeros using our basic RS sum (sign changes)
    println!("Finding zeros via sign changes in Z(t)...");
    let mut found_zeros = Vec::new();
    let t_start = 10.0;
    let t_end = 150.0;
    let step = 0.01;
    
    let mut prev_t = t_start;
    let mut prev_z = basic_rs_sum(t_start);
    
    let mut t = t_start + step;
    while t <= t_end {
        let z = basic_rs_sum(t);
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            // Sign change — refine with bisection
            let zero_t = bisect_zero(prev_t, t, prev_z, z);
            found_zeros.push(zero_t);
        }
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Found {} zeros in [{}, {}]", found_zeros.len(), t_start, t_end);
    println!("First 15: {:?}", &found_zeros[..std::cmp::min(15, found_zeros.len())]);
    println!();
    
    // Test 1: Global patterns — correlation vs interval distance
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 1: GLOBAL PATTERNS — CORRELATION vs SEPARATION");
    println!("  Do non-adjacent intervals show special relationships?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Use first 20 found zeros to create intervals
    let n_intervals_to_use = std::cmp::min(19, found_zeros.len() - 1);
    let n_samples = 51;
    
    // Compute normalized terrains for all intervals
    let mut terrains: Vec<Vec<f64>> = Vec::new();
    for i in 0..n_intervals_to_use {
        let t_left = found_zeros[i];
        let t_right = found_zeros[i + 1];
        let width = t_right - t_left;
        let midpoint = (t_left + t_right) / 2.0;
        
        let mut zs = Vec::new();
        for j in 0..n_samples {
            let tt = t_left + width * (j as f64) / ((n_samples - 1) as f64);
            zs.push(basic_rs_sum(tt));
        }
        
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let normalized: Vec<f64> = zs.iter()
            .map(|z| if max_z > 1e-15 { z / max_z } else { 0.0 })
            .collect();
        
        terrains.push(normalized);
    }
    
    // Compute mean correlation for pairs separated by k intervals
    println!("Separation   Mean Corr   Std Dev   Count");
    println!("{}", "-".repeat(45));
    
    for sep in 1..=6 {
        let mut corrs = Vec::new();
        for i in 0..n_intervals_to_use {
            let j = i + sep;
            if j < n_intervals_to_use {
                let corr = correlation(&terrains[i], &terrains[j]);
                corrs.push(corr);
            }
        }
        
        if !corrs.is_empty() {
            let mean = corrs.iter().sum::<f64>() / corrs.len() as f64;
            let var = corrs.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / corrs.len() as f64;
            let std = var.sqrt();
            println!("{:>10}   {:>9.3}   {:>7.3}   {}", sep, mean, std, corrs.len());
        }
    }
    
    // Test 2: Scaling with t — does terrain change at higher t?
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 2: SCALING WITH t — TERRAIN CHARACTER AT HIGH t");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Compare low-t intervals (t < 50) with high-t intervals (t > 100)
    let mut low_t_corrs = Vec::new();
    let mut high_t_corrs = Vec::new();
    let mut cross_corrs = Vec::new();
    
    for i in 0..n_intervals_to_use {
        for j in (i+1)..n_intervals_to_use {
            let t_i_mid = (found_zeros[i] + found_zeros[i+1]) / 2.0;
            let t_j_mid = (found_zeros[j] + found_zeros[j+1]) / 2.0;
            let corr = correlation(&terrains[i], &terrains[j]);
            
            if t_i_mid < 50.0 && t_j_mid < 50.0 {
                low_t_corrs.push(corr);
            } else if t_i_mid > 100.0 && t_j_mid > 100.0 {
                high_t_corrs.push(corr);
            } else if (t_i_mid < 50.0 && t_j_mid > 100.0) || 
                      (t_i_mid > 100.0 && t_j_mid < 50.0) {
                cross_corrs.push(corr);
            }
        }
    }
    
    if !low_t_corrs.is_empty() {
        let mean = low_t_corrs.iter().sum::<f64>() / low_t_corrs.len() as f64;
        println!("Low-t intervals (t<50):     mean corr = {:>7.3} (n={})", mean, low_t_corrs.len());
    }
    if !high_t_corrs.is_empty() {
        let mean = high_t_corrs.iter().sum::<f64>() / high_t_corrs.len() as f64;
        println!("High-t intervals (t>100):   mean corr = {:>7.3} (n={})", mean, high_t_corrs.len());
    }
    if !cross_corrs.is_empty() {
        let mean = cross_corrs.iter().sum::<f64>() / cross_corrs.len() as f64;
        println!("Cross (low vs high t):      mean corr = {:>7.3} (n={})", mean, cross_corrs.len());
    }
    
    // Test 3: Zero spacing patterns — the drawing connection
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 3: ZERO SPACING PATTERNS (DRAWING CONNECTION)");
    println!("  Looking for global structure in zero distribution");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Compute spacings between consecutive zeros
    let mut spacings = Vec::new();
    for i in 0..found_zeros.len() - 1 {
        spacings.push(found_zeros[i+1] - found_zeros[i]);
    }
    
    // Mean and std of spacings
    let mean_spacing = spacings.iter().sum::<f64>() / spacings.len() as f64;
    let var_spacing = spacings.iter().map(|s| (s - mean_spacing).powi(2)).sum::<f64>() / spacings.len() as f64;
    let std_spacing = var_spacing.sqrt();
    
    println!("Zero spacing statistics:");
    println!("  Mean spacing: {:.4}", mean_spacing);
    println!("  Std dev:      {:.4}", std_spacing);
    println!("  Min:          {:.4}", spacings.iter().cloned().fold(f64::MAX, f64::min));
    println!("  Max:          {:.4}", spacings.iter().cloned().fold(0.0, f64::max));
    println!();
    
    // Look for patterns in spacing ratios (consecutive spacings)
    println!("Consecutive spacing ratios (s[i+1]/s[i]):");
    let mut ratios = Vec::new();
    for i in 0..spacings.len() - 1 {
        if spacings[i] > 0.01 {
            ratios.push(spacings[i+1] / spacings[i]);
        }
    }
    
    let mean_ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;
    println!("  Mean ratio: {:.4} (1.0 = constant spacing)", mean_ratio);
    
    // Count how many ratios are > 1 vs < 1
    let up = ratios.iter().filter(|&&r| r > 1.0).count();
    let down = ratios.iter().filter(|&&r| r < 1.0).count();
    println!("  Ratios > 1 (widening): {}", up);
    println!("  Ratios < 1 (narrowing): {}", down);
    println!();
    
    // Look for "clusters" — sequences of narrow spacings followed by wide ones
    println!("Looking for cluster patterns (narrow→wide transitions):");
    let threshold = mean_spacing;
    let mut clusters = Vec::new();
    let mut in_cluster = false;
    let mut cluster_start = 0;
    
    for i in 0..spacings.len() {
        if spacings[i] < threshold * 0.7 && !in_cluster {
            in_cluster = true;
            cluster_start = i;
        } else if spacings[i] > threshold * 1.3 && in_cluster {
            in_cluster = false;
            clusters.push((cluster_start, i));
        }
    }
    
    println!("Found {} potential clusters", clusters.len());
    for (start, end) in &clusters[..std::cmp::min(5, clusters.len())] {
        let t_start = found_zeros[*start];
        let t_end = found_zeros[*end + 1];
        println!("  Cluster: zeros {}-{} in t=[{:.2}, {:.2}]", 
            start, end + 1, t_start, t_end);
    }
    
    // Test 4: Spacing autocorrelation — do spacings repeat?
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 4: SPACING AUTOCORRELATION");
    println!("  Do zero spacings show periodic structure?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Normalize spacings
    let norm_spacings: Vec<f64> = spacings.iter()
        .map(|s| (s - mean_spacing) / std_spacing)
        .collect();
    
    for lag in 1..=10 {
        let mut corr_sum = 0.0;
        let mut count = 0;
        for i in 0..norm_spacings.len() - lag {
            corr_sum += norm_spacings[i] * norm_spacings[i + lag];
            count += 1;
        }
        if count > 0 {
            let autocorr = corr_sum / count as f64;
            println!("Lag {:>2}: autocorrelation = {:.4}", lag, autocorr);
        }
    }
    
    // Summary
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  INTERPRETATION");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("If spacing autocorrelation is significant at any lag:");
    println!("  → Zero spacings are not random");
    println!("  → There's a repeating pattern in the zero distribution");
    println!("  → This could correspond to 'global structure' in the drawing");
    println!();
    println!("If autocorrelation decays quickly to ~0:");
    println!("  → Spacings are locally correlated but not periodic");
    println!("  → Structure is fractal-like, not repeating");
    println!();
    println!("CAVEAT: Using basic RS sum without error bounds.");
    println!("Zero locations have uncertainty of ~0.01-0.1 in t.");
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

fn bisect_zero(t_a: f64, t_b: f64, z_a: f64, z_b: f64) -> f64 {
    let mut a = t_a;
    let mut b = t_b;
    let mut fa = z_a;
    
    for _ in 0..50 {
        let mid = (a + b) / 2.0;
        let fm = basic_rs_sum(mid);
        if fa * fm < 0.0 {
            b = mid;
        } else {
            a = mid;
            fa = fm;
        }
        if (b - a).abs() < 1e-8 {
            break;
        }
    }
    
    (a + b) / 2.0
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
