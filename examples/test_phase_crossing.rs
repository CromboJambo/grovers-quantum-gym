use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  PHASE SPACE & CROSSING GEOMETRY ANALYSIS");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Find zeros
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
            let zero_t = bisect_zero(prev_t, t, prev_z, z);
            found_zeros.push(zero_t);
        }
        prev_t = t;
        prev_z = z;
        t += step;
    }
    
    println!("Found {} zeros in [{}, {}]", found_zeros.len(), t_start, t_end);
    println!();
    
    // ════════════════════════════════════════════════════
    // TEST 1: PHASE SPACE TRAJECTORY (Z vs Z')
    // ════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 1: PHASE SPACE — (Z(t), Z'(t)) TRAJECTORIES");
    println!("  For a sine wave, this would be an ellipse.");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // For each of the first 5 intervals, compute phase space trajectory
    let n_intervals = std::cmp::min(5, found_zeros.len() - 1);
    let n_samples = 201;
    
    for i in 0..n_intervals {
        let t_left = found_zeros[i];
        let t_right = found_zeros[i + 1];
        let width = t_right - t_left;
        
        let mut zs = Vec::new();
        let mut dzs = Vec::new(); // Z'(t) approximated numerically
        
        for j in 0..n_samples {
            let tt = t_left + width * (j as f64) / ((n_samples - 1) as f64);
            let z = basic_rs_sum(tt);
            zs.push(z);
            
            // Numerical derivative: central difference
            let dt = 0.001;
            let z_plus = basic_rs_sum(tt + dt);
            let z_minus = basic_rs_sum(tt - dt);
            dzs.push((z_plus - z_minus) / (2.0 * dt));
        }
        
        // Normalize for comparison
        let max_z = zs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        let max_dz = dzs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        
        // Check if trajectory is elliptical: for an ellipse, 
        // (Z/max_z)^2 + (Z'/max_dz)^2 should be constant (= 1)
        let mut ellipse_values = Vec::new();
        for j in 0..n_samples {
            let x = zs[j] / max_z;
            let y = dzs[j] / max_dz;
            ellipse_values.push(x*x + y*y);
        }
        
        // Compute mean and std of ellipse values
        let mean_ellipse = ellipse_values.iter().sum::<f64>() / ellipse_values.len() as f64;
        let var_ellipse = ellipse_values.iter()
            .map(|v| (v - mean_ellipse).powi(2))
            .sum::<f64>() / ellipse_values.len() as f64;
        let std_ellipse = var_ellipse.sqrt();
        
        // Also compute the "eccentricity" — ratio of max extent in Z vs Z'
        // For a circle in normalized coords, mean_ellipse should be 1.0 and std small
        
        println!("Interval [{:.3}, {:.3}] (width={:.3}):", t_left, t_right, width);
        println!("  max|Z|={:.4e}, max|Z'|={:.4e}", max_z, max_dz);
        println!("  Ellipse test: mean(Z²+Z'²)={:.4}, std={:.4}", mean_ellipse, std_ellipse);
        
        if std_ellipse < 0.1 {
            println!("  → Nearly ELLIPTICAL (std < 0.1)");
        } else if std_ellipse < 0.3 {
            println!("  → APPROXIMATELY ELLIPTICAL (std < 0.3)");
        } else {
            println!("  → NOT ELLIPTICAL (std ≥ 0.3) — complex trajectory");
        }
        
        // Show a few sample points from the phase space trajectory
        println!("  Phase space samples (normalized Z, normalized Z'):");
        for k in [0, n_samples/4, n_samples/2, 3*n_samples/4, n_samples-1] {
            let x = zs[k] / max_z;
            let y = dzs[k] / max_dz;
            println!("    t={:.3}: ({:>7.4}, {:>7.4})", 
                t_left + width * (k as f64) / ((n_samples - 1) as f64), x, y);
        }
        println!();
    }
    
    // ════════════════════════════════════════════════════
    // TEST 2: CROSSING GEOMETRY — SPEED AT ZEROS
    // ════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 2: CROSSING SPEED |Z'(t)| AT EACH ZERO");
    println!("  How fast does the trajectory pass through each zero?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let mut crossing_speeds = Vec::new();
    let mut spacings = Vec::new();
    
    for i in 0..found_zeros.len() {
        let t0 = found_zeros[i];
        
        // Compute Z'(t0) numerically
        let dt = 0.001;
        let z_plus = basic_rs_sum(t0 + dt);
        let z_minus = basic_rs_sum(t0 - dt);
        let dzdt = (z_plus - z_minus) / (2.0 * dt);
        
        crossing_speeds.push(dzdt.abs());
        
        // Spacing to next zero
        if i < found_zeros.len() - 1 {
            spacings.push(found_zeros[i+1] - t0);
        }
    }
    
    println!("Zero #    t-value     |Z'(t)|     Spacing");
    println!("{}", "-".repeat(50));
    
    for i in 0..std::cmp::min(20, found_zeros.len()) {
        let spacing = if i < spacings.len() { spacings[i] } else { 0.0 };
        println!("{:>6}   {:>9.4}   {:>10.4e}   {:>8.4}", 
            i + 1, found_zeros[i], crossing_speeds[i], spacing);
    }
    
    // Statistics
    let mean_speed = crossing_speeds.iter().sum::<f64>() / crossing_speeds.len() as f64;
    let var_speed = crossing_speeds.iter()
        .map(|s| (s - mean_speed).powi(2))
        .sum::<f64>() / crossing_speeds.len() as f64;
    let std_speed = var_speed.sqrt();
    
    println!();
    println!("Crossing speed statistics:");
    println!("  Mean |Z'(t)|: {:.4e}", mean_speed);
    println!("  Std dev:      {:.4e}", std_speed);
    println!("  Min:          {:.4e}", crossing_speeds.iter().cloned().fold(f64::MAX, f64::min));
    println!("  Max:          {:.4e}", crossing_speeds.iter().cloned().fold(0.0, f64::max));
    println!();
    
    // Test: Does crossing speed correlate with local zero density?
    // (High density = small spacing = should we expect faster or slower crossings?)
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 3: CROSSING SPEED vs ZERO SPACING");
    println!("  Is there a relationship between how fast the trajectory");
    println!("  crosses and how close the next zero is?");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Correlate |Z'(t_i)| with spacing to next zero s_i
    let n_pairs = std::cmp::min(crossing_speeds.len(), spacings.len());
    
    let mean_s = spacings[..n_pairs].iter().sum::<f64>() / n_pairs as f64;
    let mean_v = crossing_speeds[..n_pairs].iter().sum::<f64>() / n_pairs as f64;
    
    let mut cov = 0.0;
    let mut var_s = 0.0;
    let mut var_v = 0.0;
    
    for i in 0..n_pairs {
        let ds = spacings[i] - mean_s;
        let dv = crossing_speeds[i] - mean_v;
        cov += ds * dv;
        var_s += ds * ds;
        var_v += dv * dv;
    }
    
    if var_s > 1e-30 && var_v > 1e-30 {
        let corr = cov / (var_s.sqrt() * var_v.sqrt());
        println!("Correlation(|Z'(t)|, spacing to next zero): {:.4}", corr);
        println!();
        
        if corr.abs() < 0.2 {
            println!("→ NO significant correlation.");
            println!("  Crossing speed is independent of local zero density.");
        } else if corr > 0.5 {
            println!("→ POSITIVE correlation: faster crossings near dense zeros.");
        } else if corr < -0.5 {
            println!("→ NEGATIVE correlation: slower crossings near dense zeros.");
        } else {
            println!("→ WEAK correlation ({:.2}).", corr);
        }
    }
    
    // Test: Does crossing speed vary systematically with t?
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 4: CROSSING SPEED vs t (SYSTEMATIC TREND?)");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Correlate |Z'(t)| with t itself
    let mean_t = found_zeros[..n_pairs].iter().sum::<f64>() / n_pairs as f64;
    let mut cov_t = 0.0;
    let mut var_t = 0.0;
    
    for i in 0..n_pairs {
        let dt_val = found_zeros[i] - mean_t;
        let dv = crossing_speeds[i] - mean_v;
        cov_t += dt_val * dv;
        var_t += dt_val * dt_val;
    }
    
    if var_t > 1e-30 && var_v > 1e-30 {
        let corr_t = cov_t / (var_t.sqrt() * var_v.sqrt());
        println!("Correlation(|Z'(t)|, t): {:.4}", corr_t);
        println!();
        
        if corr_t.abs() < 0.2 {
            println!("→ NO systematic trend with t.");
            println!("  Crossing speeds don't grow or shrink as t increases.");
        } else if corr_t > 0.5 {
            println!("→ POSITIVE trend: crossings get FASTER at higher t.");
        } else if corr_t < -0.5 {
            println!("→ NEGATIVE trend: crossings get SLOWER at higher t.");
        } else {
            println!("→ WEAK trend ({:.2}).", corr_t);
        }
    }
    
    // Test 5: Relationship between crossing speed and interval width
    // For a sine wave A*sin(ωt), |Z'| at zero = A*ω
    // And interval width = π/ω
    // So |Z'| * width should be proportional to A (amplitude)
    // If this product is constant across intervals, the motion is "self-similar"
    
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST 5: SELF-SIMILARITY CHECK");
    println!("  For sine wave: |Z'(zero)| × interval_width ∝ amplitude");
    println!("  If product is constant → self-similar motion");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let mut products = Vec::new();
    for i in 0..n_pairs {
        let product = crossing_speeds[i] * spacings[i];
        products.push(product);
    }
    
    let mean_prod = products.iter().sum::<f64>() / products.len() as f64;
    let var_prod = products.iter()
        .map(|p| (p - mean_prod).powi(2))
        .sum::<f64>() / products.len() as f64;
    let std_prod = var_prod.sqrt();
    
    println!("Product |Z'(t)| × spacing statistics:");
    println!("  Mean: {:.4e}", mean_prod);
    println!("  Std dev: {:.4e}", std_prod);
    println!("  Coefficient of variation: {:.3}", std_prod / mean_prod);
    println!();
    
    if std_prod / mean_prod < 0.2 {
        println!("→ NEARLY CONSTANT product → motion is SELF-SIMILAR");
        println!("  Each zero crossing looks the same when scaled by local width.");
    } else if std_prod / mean_prod < 0.5 {
        println!("→ MODERATE variation → partial self-similarity");
    } else {
        println!("→ SIGNIFICANT variation → motion is NOT self-similar");
        println!("  Each crossing has unique character.");
    }
    
    // Summary
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  SUMMARY: CONTINUUM VIEW OF ZERO CROSSINGS");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("If phase space trajectories are elliptical AND crossing speeds");
    println!("are self-similar (|Z'| × width ≈ constant), then:");
    println!();
    println!("  → Zeros are moments in a continuous, self-similar oscillation");
    println!("  → No special 'zero geometry' — just the same motion at different scales");
    println!("  → The 'intersection as continuum' intuition is mathematically supported");
    println!();
    println!("If trajectories are complex AND speeds vary significantly:");
    println!();
    println!("  → Each zero crossing has unique character");
    println!("  → The continuum view holds but with rich local variation");
    println!("  → The 'shape' of each crossing encodes information about that zero");
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
