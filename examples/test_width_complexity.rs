use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  WIDTH vs COMPLEXITY: CAUSAL ANALYSIS");
    println!("  Does width cause complexity, or complexity cause width?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Find zeros
    let mut found_zeros = Vec::new();
    let t_start = 10.0;
    let t_end = 300.0;
    let step = 0.005;
    
    let mut prev_t = t_start;
    let mut prev_z = basic_rs_sum(t_start);
    
    let mut t = t_start + step;
    while t <= t_end {
        let z = basic_rs_sum(t);
        if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
            let zero_t = bisect_zero(prev_t, t, prev_z);
            found_zeros.push(zero_t);
        }
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Found {} zeros in [{}, {}]", found_zeros.len(), t_start, t_end);
    println!();
    
    // For each interval, compute width and complexity measures
    let n_intervals = std::cmp::min(found_zeros.len() - 1, 80);
    let n_samples = 201;
    
    #[derive(Debug)]
    struct IntervalData {
        t_left: f64,
        t_right: f64,
        width: f64,
        ellipse_std: f64,
        max_pos: f64,
        skewness: f64,
        n_inflections: usize,
        normalized_shape: Vec<f64>,
    }
    
    let mut intervals = Vec::new();
    
    for i in 1..n_intervals + 1 {
        let t_left = found_zeros[i];
        let t_right = found_zeros[i + 1];
        let width = t_right - t_left;
        
        let mut zs = Vec::new();
        let mut dzs = Vec::new();
        let mut d2zs = Vec::new();
        
        for j in 0..n_samples {
            let tt = t_left + width * (j as f64) / ((n_samples - 1) as f64);
            zs.push(basic_rs_sum(tt));
            
            let dt = 0.002;
            let z_plus = basic_rs_sum(tt + dt);
            let z_minus = basic_rs_sum(tt - dt);
            dzs.push((z_plus - z_minus) / (2.0 * dt));
            
            let d2t = 0.01;
            let z_pp = basic_rs_sum(tt + d2t);
            let z_pm = basic_rs_sum(tt - d2t);
            d2zs.push((z_pp - 2.0 * zs[j] + z_pm) / (d2t * d2t));
        }
        
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let normalized_shape: Vec<f64> = if max_z > 1e-15 {
            zs.iter().map(|z| z / max_z).collect()
        } else {
            vec![0.0; n_samples]
        };
        
        let max_idx = normalized_shape.iter().enumerate()
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
            .unwrap().0;
        let max_pos = max_idx as f64 / (n_samples - 1) as f64;
        
        let mean_z = zs.iter().sum::<f64>() / n_samples as f64;
        let var_z = zs.iter().map(|z| (z - mean_z).powi(2)).sum::<f64>() / n_samples as f64;
        let std_z = var_z.sqrt();
        let skewness = if std_z > 1e-15 {
            zs.iter().map(|z| ((z - mean_z) / std_z).powi(3)).sum::<f64>() / n_samples as f64
        } else { 0.0 };
        
        let mut n_inflections = 0;
        for j in 1..d2zs.len() {
            if (d2zs[j-1] < 0.0 && d2zs[j] > 0.0) || (d2zs[j-1] > 0.0 && d2zs[j] < 0.0) {
                n_inflections += 1;
            }
        }
        
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
        
        intervals.push(IntervalData {
            t_left, t_right, width,
            ellipse_std, max_pos, skewness,
            n_inflections,
            normalized_shape,
        });
    }
    
    // EXPERIMENT 1: Width vs Complexity
    println!("═══════════════════════════════════════════════════════");
    println!("  EXPERIMENT 1: WIDTH vs COMPLEXITY");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let bins = [(0.0, 2.0), (2.0, 4.0), (4.0, 6.0), (6.0, 8.0), (8.0, 15.0)];
    
    for &(lo, hi) in &bins {
        let in_bin: Vec<&IntervalData> = intervals.iter()
            .filter(|iv| iv.width >= lo && iv.width < hi)
            .collect();
        
        if !in_bin.is_empty() {
            let mean_complexity = in_bin.iter().map(|iv| iv.ellipse_std).sum::<f64>() / in_bin.len() as f64;
            let mean_width = in_bin.iter().map(|iv| iv.width).sum::<f64>() / in_bin.len() as f64;
            let mean_inflections = in_bin.iter().map(|iv| iv.n_inflections as f64).sum::<f64>() / in_bin.len() as f64;
            
            println!("Width [{:.0}, {:.0}): n={:3}, mean_width={:.2}, mean_complexity={:.4}, mean_inflections={:.1}", 
                lo, hi, in_bin.len(), mean_width, mean_complexity, mean_inflections);
        }
    }
    
    let widths: Vec<f64> = intervals.iter().map(|iv| iv.width).collect();
    let complexities: Vec<f64> = intervals.iter().map(|iv| iv.ellipse_std).collect();
    let corr_wc = correlation(&widths, &complexities);
    println!();
    println!("Correlation(width, ellipse_std): {:.4}", corr_wc);
    
    // EXPERIMENT 2: Normalized Shape Comparison
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  EXPERIMENT 2: NORMALIZED SHAPE COMPARISON");
    println!("  Rescale all intervals to u in [0,1], compare shapes");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let narrow: Vec<&IntervalData> = intervals.iter().filter(|iv| iv.width < 3.0).collect();
    let wide: Vec<&IntervalData> = intervals.iter().filter(|iv| iv.width > 5.0).collect();
    
    println!("Narrow intervals (width < 3): n={}", narrow.len());
    println!("Wide intervals (width > 5): n={}", wide.len());
    println!();
    
    let mut var_narrow = 0.0f64;
    let mut var_wide = 0.0f64;
    let mut corr_shapes = 0.0f64;
    
    if !narrow.is_empty() && !wide.is_empty() {
        // Mean normalized shapes
        let mut mean_narrow = vec![0.0; n_samples];
        for iv in &narrow {
            for j in 0..n_samples {
                mean_narrow[j] += iv.normalized_shape[j];
            }
        }
        for j in 0..n_samples {
            mean_narrow[j] /= narrow.len() as f64;
        }
        
        let mut mean_wide = vec![0.0; n_samples];
        for iv in &wide {
            for j in 0..n_samples {
                mean_wide[j] += iv.normalized_shape[j];
            }
        }
        for j in 0..n_samples {
            mean_wide[j] /= wide.len() as f64;
        }
        
        // Within-group variance
        var_narrow = narrow.iter()
            .map(|iv| {
                iv.normalized_shape.iter().zip(mean_narrow.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>() / n_samples as f64
            })
            .sum::<f64>() / narrow.len() as f64;
        
        var_wide = wide.iter()
            .map(|iv| {
                iv.normalized_shape.iter().zip(mean_wide.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>() / n_samples as f64
            })
            .sum::<f64>() / wide.len() as f64;
        
        corr_shapes = correlation(&mean_narrow, &mean_wide);
        
        println!("WITHIN-GROUP VARIANCE (after normalization to unit width):");
        println!("  Narrow group: {:.6}", var_narrow);
        println!("  Wide group:   {:.6}", var_wide);
        println!();
        
        if var_wide > var_narrow * 2.0 {
            println!("→ Wide intervals have MORE internal variation even after normalization.");
            println!("  Complexity is NOT just a scaling effect — wide intervals have");
            println!("  genuinely more diverse internal structure.");
        } else if var_narrow > var_wide * 2.0 {
            println!("→ Narrow intervals have MORE internal variation (unexpected).");
        } else {
            println!("→ Similar internal variation in both groups after normalization.");
            println!("  Complexity may be primarily a scaling effect.");
        }
        
        println!();
        println!("Correlation(mean narrow shape, mean wide shape): {:.4}", corr_shapes);
        
        if corr_shapes > 0.95 {
            println!("→ Mean shapes are nearly identical after normalization.");
            println!("  Width primarily affects SCALE, not SHAPE.");
        } else if corr_shapes > 0.8 {
            println!("→ Mean shapes are similar but not identical.");
            println!("  Some genuine shape difference persists.");
        } else {
            println!("→ Mean shapes are significantly different.");
            println!("  Wide and narrow intervals have fundamentally different geometries.");
        }
        
        println!();
        println!("Mean normalized shapes at key positions:");
        println!("{:>6} {:>12} {:>12}", "u", "narrow_mean", "wide_mean");
        for u in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0] {
            let idx = (u * (n_samples - 1) as f64).round() as usize;
            println!("{:>6.1} {:>12.4} {:>12.4}", u, mean_narrow[idx], mean_wide[idx]);
        }
    }
    
    // EXPERIMENT 3: Inflection Points vs Width
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  EXPERIMENT 3: INFLECTION POINTS vs WIDTH");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    for &(lo, hi) in &bins {
        let in_bin: Vec<&IntervalData> = intervals.iter()
            .filter(|iv| iv.width >= lo && iv.width < hi)
            .collect();
        
        if !in_bin.is_empty() {
            let inflections: Vec<usize> = in_bin.iter().map(|iv| iv.n_inflections).collect();
            let mean_inf = inflections.iter().sum::<usize>() as f64 / inflections.len() as f64;
            let min_inf = *inflections.iter().min().unwrap();
            let max_inf = *inflections.iter().max().unwrap();
            
            println!("Width [{:.0}, {:.0}): n={:3}, inflection points: mean={:.1}, min={}, max={}", 
                lo, hi, in_bin.len(), mean_inf, min_inf, max_inf);
        }
    }
    
    let inflections: Vec<f64> = intervals.iter().map(|iv| iv.n_inflections as f64).collect();
    let corr_wi = correlation(&widths, &inflections);
    println!();
    println!("Correlation(width, n_inflections): {:.4}", corr_wi);
    
    if corr_wi > 0.3 {
        println!("→ Wider intervals have MORE inflection points.");
        println!("  More room = more internal structure (curvature changes).");
    } else {
        println!("→ No strong relationship between width and inflection count.");
    }
    
    // SUMMARY
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  SUMMARY");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("Key measurements:");
    println!("  Correlation(width, complexity): {:.4}", corr_wc);
    println!("  Correlation(width, inflections): {:.4}", corr_wi);
    if !narrow.is_empty() && !wide.is_empty() {
        println!("  Within-group variance (narrow): {:.6}", var_narrow);
        println!("  Within-group variance (wide):   {:.6}", var_wide);
        println!("  Shape correlation (narrow vs wide): {:.4}", corr_shapes);
    }
    
    println!();
    println!("CAUSAL INTERPRETATION:");
    if !narrow.is_empty() && !wide.is_empty() {
        if corr_shapes > 0.95 && var_wide < var_narrow * 2.0 {
            println!("  Width is primarily a SCALING parameter.");
            println!("  The function generates the same basic shape at all scales.");
            println!("  Wide intervals look like stretched narrow intervals.");
            println!("  Causality: width determines complexity via available scale.");
        } else if corr_shapes < 0.8 || var_wide > var_narrow * 2.0 {
            println!("  Wide intervals have genuinely different internal structures.");
            println!("  They are not just stretched versions of narrow intervals.");
            println!("  Causality: complexity causes width (complex paths take more time).");
        } else {
            println!("  Mixed evidence — both scaling and genuine structure present.");
        }
    }
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

fn bisect_zero(t_a: f64, t_b: f64, z_a: f64) -> f64 {
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
