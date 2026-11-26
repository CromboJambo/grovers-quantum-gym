use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  WHAT DETERMINES CROSSING SYMMETRY?");
    println!("  Symmetric (elliptical) vs Asymmetric (spiky) crossings");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Find zeros
    let mut found_zeros = Vec::new();
    let t_start = 10.0;
    let t_end = 200.0;
    let step = 0.005;
    
    let mut prev_t = t_start;
    let mut prev_z = basic_rs_sum(t_start);
    
    let mut t = t_start + step;
    while t <= t_end {
        let z = basic_rs_sum(t);
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            let zero_t = bisect_zero(prev_t, t, prev_z, z);
            found_zeros.push(zero_t);
        }
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Found {} zeros in [{}, {}]", found_zeros.len(), t_start, t_end);
    println!();
    
    // For each interval, compute asymmetry measures
    let n_intervals = std::cmp::min(found_zeros.len() - 1, 50);
    let n_samples = 201;
    
    #[derive(Debug)]
    struct IntervalAnalysis {
        t_left: f64,
        t_right: f64,
        width: f64,
        prev_spacing: f64,  // spacing from previous zero to t_left
        next_spacing: f64,  // spacing from t_right to next zero
        max_pos: f64,       // position of max|Z| as fraction [0,1]
        skewness: f64,      // skewness of Z(t) within interval
        rise_fall_ratio: f64, // time to rise / time to fall
        ellipse_std: f64,   // from phase space test (lower = more elliptical)
        crossing_speed_left: f64,  // |Z'| at left zero
        crossing_speed_right: f64, // |Z'| at right zero
    }
    
    let mut analyses = Vec::new();
    
    for i in 1..n_intervals + 1 {
        let t_left = found_zeros[i];
        let t_right = found_zeros[i + 1];
        let width = t_right - t_left;
        
        // Spacings
        let prev_spacing = if i > 0 { t_left - found_zeros[i-1] } else { width };
        let next_spacing = if i + 2 < found_zeros.len() { 
            found_zeros[i+2] - t_right 
        } else { width };
        
        // Sample Z(t) in interval
        let mut zs = Vec::new();
        for j in 0..n_samples {
            let tt = t_left + width * (j as f64) / ((n_samples - 1) as f64);
            zs.push(basic_rs_sum(tt));
        }
        
        // Position of max|Z|
        let max_idx = zs.iter().enumerate()
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
            .unwrap().0;
        let max_pos = max_idx as f64 / (n_samples - 1) as f64;
        
        // Skewness of Z(t)
        let mean_z = zs.iter().sum::<f64>() / n_samples as f64;
        let var_z = zs.iter().map(|z| (z - mean_z).powi(2)).sum::<f64>() / n_samples as f64;
        let std_z = var_z.sqrt();
        let skewness = if std_z > 1e-15 {
            zs.iter().map(|z| ((z - mean_z) / std_z).powi(3)).sum::<f64>() / n_samples as f64
        } else { 0.0 };
        
        // Rise/fall ratio: time from left zero to max vs max to right zero
        let rise_fall_ratio = max_pos / (1.0 - max_pos);
        
        // Ellipse test (phase space)
        let mut dzs = Vec::new();
        for j in 0..n_samples {
            let tt = t_left + width * (j as f64) / ((n_samples - 1) as f64);
            let dt = 0.001;
            let z_plus = basic_rs_sum(tt + dt);
            let z_minus = basic_rs_sum(tt - dt);
            dzs.push((z_plus - z_minus) / (2.0 * dt));
        }
        
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let max_dz = dzs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        
        let mut ellipse_values = Vec::new();
        for j in 0..n_samples {
            let x = if max_z > 1e-15 { zs[j] / max_z } else { 0.0 };
            let y = if max_dz > 1e-15 { dzs[j] / max_dz } else { 0.0 };
            ellipse_values.push(x*x + y*y);
        }
        
        let mean_ellipse = ellipse_values.iter().sum::<f64>() / ellipse_values.len() as f64;
        let var_ellipse = ellipse_values.iter()
            .map(|v| (v - mean_ellipse).powi(2))
            .sum::<f64>() / ellipse_values.len() as f64;
        let ellipse_std = var_ellipse.sqrt();
        
        // Crossing speeds at endpoints
        let dt = 0.001;
        let speed_left = (basic_rs_sum(t_left + dt) - basic_rs_sum(t_left - dt)) / (2.0 * dt).abs();
        let speed_right = (basic_rs_sum(t_right + dt) - basic_rs_sum(t_right - dt)) / (2.0 * dt).abs();
        
        analyses.push(IntervalAnalysis {
            t_left, t_right, width,
            prev_spacing, next_spacing,
            max_pos, skewness, rise_fall_ratio,
            ellipse_std,
            crossing_speed_left: speed_left,
            crossing_speed_right: speed_right,
        });
    }
    
    // Print detailed analysis for first 15 intervals
    println!("{:<6} {:>8} {:>8} {:>7} {:>7} {:>8} {:>9} {:>10} {:>9}", 
        "Int#", "t_left", "t_right", "width", "prev_s", "next_s", "max_pos", "skewness", "ellipse");
    println!("{}", "-".repeat(85));
    
    for (i, a) in analyses.iter().take(15).enumerate() {
        println!("{:<6} {:>8.3} {:>8.3} {:>7.3} {:>7.3} {:>8.3} {:>9.4} {:>10.4} {:>9.4}", 
            i + 1, a.t_left, a.t_right, a.width, a.prev_spacing, a.next_spacing,
            a.max_pos, a.skewness, a.ellipse_std);
    }
    
    // TEST A: Correlate asymmetry with spacing ratio
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST A: ASYMMETRY vs SPACING RATIO (next/prev)");
    println!("  If next_spacing >> prev_spacing, is the crossing asymmetric?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let spacing_ratios: Vec<f64> = analyses.iter()
        .map(|a| a.next_spacing / a.prev_spacing)
        .collect();
    
    // Correlate |max_pos - 0.5| (asymmetry of peak position) with |log(spacing_ratio)|
    let asymmetries: Vec<f64> = analyses.iter()
        .map(|a| (a.max_pos - 0.5).abs())
        .collect();
    
    let log_ratios: Vec<f64> = spacing_ratios.iter()
        .map(|r| r.ln().abs())
        .collect();
    
    let corr_a = correlation(&asymmetries, &log_ratios);
    println!("Correlation(|max_pos - 0.5|, |ln(spacing_ratio)|): {:.4}", corr_a);
    
    if corr_a > 0.5 {
        println!("→ STRONG: Unequal spacings create asymmetric crossings.");
        println!("  When next zero is much closer/farther than previous,");
        println!("  the peak shifts away from center.");
    } else if corr_a > 0.2 {
        println!("→ MODERATE correlation.");
    } else {
        println!("→ WEAK: Spacing ratio doesn't strongly predict asymmetry.");
    }
    
    // TEST B: Correlate ellipse_std with crossing speed ratio
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST B: PHASE SPACE SHAPE vs CROSSING SPEED RATIO");
    println!("  If left and right crossings have very different speeds,");
    println!("  is the phase space trajectory more asymmetric?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let speed_ratios: Vec<f64> = analyses.iter()
        .map(|a| (a.crossing_speed_right / a.crossing_speed_left).ln().abs())
        .collect();
    
    let ellipse_stds: Vec<f64> = analyses.iter()
        .map(|a| a.ellipse_std)
        .collect();
    
    let corr_b = correlation(&speed_ratios, &ellipse_stds);
    println!("Correlation(|ln(speed_ratio)|, ellipse_std): {:.4}", corr_b);
    
    if corr_b > 0.5 {
        println!("→ STRONG: Unequal crossing speeds create asymmetric phase space.");
        println!("  When one crossing is much faster than the other,");
        println!("  the trajectory becomes spiky instead of elliptical.");
    } else if corr_b > 0.2 {
        println!("→ MODERATE correlation.");
    } else {
        println!("→ WEAK: Speed ratio doesn't strongly predict phase space shape.");
    }
    
    // TEST C: What about the absolute crossing speeds?
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST C: ABSOLUTE CROSSING SPEED vs PHASE SPACE SHAPE");
    println!("  Do very fast crossings (|Z'| > 10) tend to be asymmetric?");
    println!("═══════════════════════════════════════════════════════");
    
    // Split into fast and slow crossings
    let threshold = 5.0;
    let mut fast_ellipses = Vec::new();
    let mut slow_ellipses = Vec::new();
    
    for a in &analyses {
        let max_speed = a.crossing_speed_left.max(a.crossing_speed_right);
        if max_speed > threshold {
            fast_ellipses.push(a.ellipse_std);
        } else {
            slow_ellipses.push(a.ellipse_std);
        }
    }
    
    if !fast_ellipses.is_empty() && !slow_ellipses.is_empty() {
        let mean_fast = fast_ellipses.iter().sum::<f64>() / fast_ellipses.len() as f64;
        let mean_slow = slow_ellipses.iter().sum::<f64>() / slow_ellipses.len() as f64;
        
        println!();
        println!("Fast crossings (max|Z'| > {}):", threshold);
        println!("  Count: {}, mean ellipse_std: {:.4}", fast_ellipses.len(), mean_fast);
        println!("Slow crossings (max|Z'| ≤ {}):", threshold);
        println!("  Count: {}, mean ellipse_std: {:.4}", slow_ellipses.len(), mean_slow);
        println!();
        
        if mean_fast > mean_slow * 1.5 {
            println!("→ FAST crossings are MORE asymmetric (higher ellipse_std).");
            println!("  Sharp, fast crossings create spiky phase space trajectories.");
        } else if mean_slow > mean_fast * 1.5 {
            println!("→ SLOW crossings are MORE asymmetric.");
        } else {
            println!("→ No strong difference between fast and slow crossings.");
        }
    }
    
    // TEST D: Look at the most asymmetric intervals in detail
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST D: MOST ASYMMETRIC INTERVALS — WHAT'S SPECIAL?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Sort by ellipse_std (most asymmetric first)
    let mut sorted: Vec<_> = analyses.iter().enumerate().collect();
    sorted.sort_by(|a, b| b.1.ellipse_std.partial_cmp(&a.1.ellipse_std).unwrap());
    
    println!("Top 5 most asymmetric intervals:");
    for (rank, &(idx, a)) in sorted.iter().take(5).enumerate() {
        println!();
        println!("Rank {}: Interval [{:.3}, {:.3}] (width={:.3})", rank + 1, a.t_left, a.t_right, a.width);
        println!("  prev_spacing={:.3}, next_spacing={:.3}, ratio={:.2}", 
            a.prev_spacing, a.next_spacing, a.next_spacing/a.prev_spacing);
        println!("  max_pos={:.4} (center=0.5), skewness={:.4}", a.max_pos, a.skewness);
        println!("  speed_left={:.3}, speed_right={:.3}, ratio={:.2}", 
            a.crossing_speed_left, a.crossing_speed_right, 
            a.crossing_speed_right/a.crossing_speed_left);
        println!("  ellipse_std={:.4}", a.ellipse_std);
    }
    
    // TEST E: Most symmetric intervals for comparison
    println!();
    println!("Top 5 most SYMMETRIC intervals (for comparison):");
    sorted.sort_by(|a, b| a.1.ellipse_std.partial_cmp(&b.1.ellipse_std).unwrap());
    
    for (rank, &(idx, a)) in sorted.iter().take(5).enumerate() {
        println!();
        println!("Rank {}: Interval [{:.3}, {:.3}] (width={:.3})", rank + 1, a.t_left, a.t_right, a.width);
        println!("  prev_spacing={:.3}, next_spacing={:.3}, ratio={:.2}", 
            a.prev_spacing, a.next_spacing, a.next_spacing/a.prev_spacing);
        println!("  max_pos={:.4} (center=0.5), skewness={:.4}", a.max_pos, a.skewness);
        println!("  speed_left={:.3}, speed_right={:.3}, ratio={:.2}", 
            a.crossing_speed_left, a.crossing_speed_right, 
            a.crossing_speed_right/a.crossing_speed_left);
        println!("  ellipse_std={:.4}", a.ellipse_std);
    }
    
    // Summary
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  INTERPRETATION");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("If asymmetric intervals have:");
    println!("  • Unequal spacings (ratio >> 1 or << 1)");
    println!("  • Unequal crossing speeds (ratio >> 1 or << 1)");
    println!("  • Shifted peak position (max_pos far from 0.5)");
    println!();
    println!("Then the crossing character is determined by the LOCAL CONTEXT:");
    println!("how this zero relates to its neighbors in both position and dynamics.");
    println!();
    println!("This would mean each zero's 'shape' is not intrinsic but RELATIONAL —");
    println!("defined by its relationship to nearby zeros. The continuum view holds,");
    println!("but the local geometry at each point depends on the global configuration.");
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
    if n < 3.0 { return 0.0; }
    
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
