use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  HYPOTHESIS TEST: UNIVERSAL INTER-ZERO TERRAIN GEOMETRY");
    println!("  Does the shape BETWEEN zeros show universal patterns?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let known_zeros = vec![
        14.134725, 21.022040, 25.010858, 30.424876,
        32.935062, 37.586178, 40.918719, 43.327073
    ];
    
    let n_samples = 101; // points per interval
    
    println!("Computing Z(t) terrain between consecutive zeros...");
    println!();
    
    // Store normalized terrain for each interval
    let mut terrains: Vec<(f64, f64, f64, Vec<f64>)> = Vec::new(); 
    // (t_left, t_right, width, normalized_y)
    
    for i in 0..known_zeros.len() - 1 {
        let t_left = known_zeros[i];
        let t_right = known_zeros[i + 1];
        let width = t_right - t_left;
        let midpoint = (t_left + t_right) / 2.0;
        
        let mut ts = Vec::new();
        let mut zs = Vec::new();
        
        for j in 0..n_samples {
            let t = t_left + width * (j as f64) / ((n_samples - 1) as f64);
            let z = basic_rs_sum(t);
            ts.push(t);
            zs.push(z);
        }
        
        // Normalize: x to [-0.5, 0.5], y by max|Z|
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let normalized_x: Vec<f64> = ts.iter()
            .map(|t| (t - midpoint) / width)  // centered, scaled to [-0.5, 0.5]
            .collect();
        let normalized_y: Vec<f64> = zs.iter()
            .map(|z| if max_z > 1e-15 { z / max_z } else { 0.0 })
            .collect();
        
        // Count local extrema
        let mut extrema_count = 0;
        for k in 1..zs.len() - 1 {
            if (zs[k] > zs[k-1] && zs[k] > zs[k+1]) || 
               (zs[k] < zs[k-1] && zs[k] < zs[k+1]) {
                extrema_count += 1;
            }
        }
        
        // Find position of maximum |Z| relative to interval
        let max_idx = zs.iter().enumerate()
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
            .unwrap().0;
        let max_pos = (max_idx as f64) / ((n_samples - 1) as f64); // 0 to 1
        
        terrains.push((t_left, t_right, width, normalized_y.clone()));
        
        println!("Interval [{:.3}, {:.3}] (width={:.3}):", t_left, t_right, width);
        println!("  max|Z|={:.4e}, extrema_count={}, max_at_fraction={:.2}", 
            max_z, extrema_count, max_pos);
        println!();
    }
    
    // Compare terrain shapes via correlation
    println!("═══════════════════════════════════════════════════════");
    println!("  TERRAIN SIMILARITY: PAIRWISE CORRELATION COEFFICIENTS");
    println!("  (Normalized inter-zero landscapes)");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let n_intervals = terrains.len();
    print!("        ");
    for i in 0..n_intervals {
        print!("{:>6.0}-{:<4.0}", terrains[i].0, terrains[i].1);
    }
    println!();
    
    for i in 0..n_intervals {
        print!("{:>5.0}-{:<4.0} ", terrains[i].0, terrains[i].1);
        for j in 0..n_intervals {
            if i == j {
                print!("{:>7.2}", 1.0);
            } else {
                let corr = correlation(&terrains[i].3, &terrains[j].3);
                print!("{:>7.2}", corr);
            }
        }
        println!();
    }
    
    // Analyze structural features
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  STRUCTURAL FEATURES OF INTER-ZERO TERRAIN");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // For each interval, compute:
    // 1. Position of maximum (fraction of interval)
    // 2. Asymmetry: (max_pos - 0.5) 
    // 3. Curvature at midpoint
    // 4. Number of extrema
    
    println!("Interval       Width   MaxPos   Asymmetry  Extrema");
    println!("{}", "-".repeat(55));
    
    let mut max_positions = Vec::new();
    
    for (i, &(t_left, t_right, width, _)) in terrains.iter().enumerate() {
        let midpoint = (t_left + t_right) / 2.0;
        let n_s = 101;
        
        let mut zs = Vec::new();
        for j in 0..n_s {
            let t = t_left + width * (j as f64) / ((n_s - 1) as f64);
            zs.push(basic_rs_sum(t));
        }
        
        // Max position
        let max_idx = zs.iter().enumerate()
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
            .unwrap().0;
        let max_pos = (max_idx as f64) / ((n_s - 1) as f64);
        max_positions.push(max_pos);
        
        // Asymmetry
        let asymmetry = max_pos - 0.5;
        
        // Count extrema
        let mut extrema_count = 0;
        for k in 1..zs.len() - 1 {
            if (zs[k] > zs[k-1] && zs[k] > zs[k+1]) || 
               (zs[k] < zs[k-1] && zs[k] < zs[k+1]) {
                extrema_count += 1;
            }
        }
        
        // Curvature at midpoint (second derivative approximation)
        let dt = width / ((n_s - 1) as f64);
        let mid_idx = n_s / 2;
        let curvature = (zs[mid_idx+1] - 2.0*zs[mid_idx] + zs[mid_idx-1]) / (dt*dt);
        
        println!("[{:>5.1},{:<5.1}] {:>7.3}   {:>6.3}    {:>+8.4}     {}", 
            t_left, t_right, width, max_pos, asymmetry, extrema_count);
    }
    
    // Check if max positions are uniformly distributed or show patterns
    println!();
    println!("Max position distribution (fraction of interval):");
    for &pos in &max_positions {
        let bar = ".".repeat((pos * 40.0) as usize);
        let marker = "*".to_string();
        println!("  {}{} ({:.3})", bar, marker, pos);
    }
    
    // Compute mean and std of max positions
    let mean_pos = max_positions.iter().sum::<f64>() / max_positions.len() as f64;
    let variance = max_positions.iter()
        .map(|p| (p - mean_pos).powi(2))
        .sum::<f64>() / max_positions.len() as f64;
    let std_pos = variance.sqrt();
    
    println!();
    println!("Mean max position: {:.3} (0.5 = symmetric)", mean_pos);
    println!("Std dev: {:.3}", std_pos);
    println!();
    
    if std_pos < 0.1 {
        println!("→ Maxima are CONSISTENTLY positioned → universal terrain shape");
    } else if std_pos < 0.2 {
        println!("→ Maxima show MODERATE variation → partial universality");
    } else {
        println!("→ Maxima are SCATTERED → no universal positioning");
    }
    
    // Final analysis: correlation with interval width
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  KEY FINDING: TERRAIN GEOMETRY vs INTERVAL WIDTH");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Does the terrain shape depend on interval width?
    // Compare narrow intervals (< 3.0) vs wide intervals (> 4.0)
    let mut narrow_corrs = Vec::new();
    let mut wide_corrs = Vec::new();
    
    for i in 0..n_intervals {
        for j in (i+1)..n_intervals {
            let w_i = terrains[i].2;
            let w_j = terrains[j].2;
            let corr = correlation(&terrains[i].3, &terrains[j].3);
            
            if w_i < 3.0 && w_j < 3.0 {
                narrow_corrs.push(corr);
            } else if w_i > 4.0 && w_j > 4.0 {
                wide_corrs.push(corr);
            }
        }
    }
    
    if !narrow_corrs.is_empty() {
        let mean_narrow = narrow_corrs.iter().sum::<f64>() / narrow_corrs.len() as f64;
        println!("Mean correlation (narrow intervals, width<3.0): {:.3} (n={})", 
            mean_narrow, narrow_corrs.len());
    }
    if !wide_corrs.is_empty() {
        let mean_wide = wide_corrs.iter().sum::<f64>() / wide_corrs.len() as f64;
        println!("Mean correlation (wide intervals, width>4.0): {:.3} (n={})", 
            mean_wide, wide_corrs.len());
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  SUMMARY");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("If inter-zero terrain shapes are similar after normalization:");
    println!("  → The 'shape of change' between zeros is universal");
    println!("  → Zeros are not isolated events but part of a repeating pattern");
    println!("  → The geometry of the transition system is independent of location");
    println!();
    println!("If they differ significantly:");
    println!("  → Each interval has unique character");
    println!("  → Local density or other factors shape the terrain");
    println!("  → The 'shape of change' depends on context");
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
