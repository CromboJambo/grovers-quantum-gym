use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

#[derive(Debug, Clone)]
struct IntervalData {
    idx: usize,
    t_left: f64,
    t_right: f64,
    width: f64,
    shapes: Vec<f64>,
    phase_x: Vec<f64>,
    phase_y: Vec<f64>,
    n_inflections: usize,
}

impl IntervalData {
    fn ellipse_std(&self) -> f64 {
        let n = self.phase_x.len();
        if n == 0 { return 0.0; }
        
        let mut values = Vec::new();
        for j in 0..n {
            values.push(self.phase_x[j]*self.phase_x[j] + self.phase_y[j]*self.phase_y[j]);
        }
        
        let mean = values.iter().sum::<f64>() / n as f64;
        let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
        var.sqrt()
    }
}



fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  GEOMETRIC MAP OF Z(t) INTERVALS");
    println!("  Narrow (simple) vs Wide (complex) trajectories");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Find zeros
    let mut found_zeros = Vec::new();
    let t_start = 10.0;
    let t_end = 250.0;
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
    
    let n_samples = 101;
    
    // Collect interval data

    
    let mut intervals = Vec::new();
    
    for i in 1..std::cmp::min(found_zeros.len() - 1, 70) + 1 {
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
        let max_dz = dzs.iter().map(|z| z.abs()).fold(0.0, f64::max);
        
        let shapes: Vec<f64> = if max_z > 1e-15 {
            zs.iter().map(|z| z.abs() / max_z).collect()
        } else {
            vec![0.0; n_samples]
        };
        
        let phase_x: Vec<f64> = if max_z > 1e-15 {
            zs.iter().map(|z| z / max_z).collect()
        } else {
            vec![0.0; n_samples]
        };
        
        let phase_y: Vec<f64> = if max_dz > 1e-15 {
            dzs.iter().map(|z| z / max_dz).collect()
        } else {
            vec![0.0; n_samples]
        };
        
        let mut n_inflections = 0;
        for j in 1..d2zs.len() {
            if (d2zs[j-1] < 0.0 && d2zs[j] > 0.0) || (d2zs[j-1] > 0.0 && d2zs[j] < 0.0) {
                n_inflections += 1;
            }
        }
        
        intervals.push(IntervalData {
            idx: i, t_left, t_right, width,
            shapes, phase_x, phase_y,
            n_inflections,
        });
    }
    
    // ═══════════════════════════════════════════════════
    // MAP 1: NARROW vs WIDE SHAPE COMPARISON
    // ═══════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════");
    println!("  MAP 1: SHAPE COMPARISON (normalized to unit width)");
    println!("  |Z(t)| / max|Z| vs u = (t - t_left) / width");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Pick 3 narrowest and 3 widest
    let mut sorted_by_width = intervals.clone();
    sorted_by_width.sort_by(|a, b| a.width.partial_cmp(&b.width).unwrap());
    
    let narrow_picks = &sorted_by_width[0..3];
    let wide_picks = &sorted_by_width[sorted_by_width.len()-3..];
    
    // ASCII plot parameters
    let plot_width = 60;
    let plot_height = 12;
    
    println!("─── NARROW INTERVALS (simple, peaked) ───");
    for iv in narrow_picks {
        println!();
        println!("Interval #{}: [{:.3}, {:.3}] width={:.3} inflections={}", 
            iv.idx, iv.t_left, iv.t_right, iv.width, iv.n_inflections);
        ascii_plot_shape(&iv.shapes, plot_width, plot_height);
    }
    
    println!();
    println!("─── WIDE INTERVALS (complex, flat-topped) ───");
    for iv in wide_picks {
        println!();
        println!("Interval #{}: [{:.3}, {:.3}] width={:.3} inflections={}", 
            iv.idx, iv.t_left, iv.t_right, iv.width, iv.n_inflections);
        ascii_plot_shape(&iv.shapes, plot_width, plot_height);
    }
    
    // ═══════════════════════════════════════════════════
    // MAP 2: PHASE SPACE TRAJECTORIES
    // ═══════════════════════════════════════════════════
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  MAP 2: PHASE SPACE (Z vs Z')");
    println!("  Trajectory shape reveals complexity");
    println!("═══════════════════════════════════════════════════════");
    
    // Pick one narrow and one wide for phase space
    if !narrow_picks.is_empty() && !wide_picks.is_empty() {
        let narrow = &narrow_picks[0];
        let wide = &wide_picks[wide_picks.len()-1];
        
        println!();
        println!("NARROW interval #{} (width={:.3}):", narrow.idx, narrow.width);
        ascii_plot_phase(&narrow.phase_x, &narrow.phase_y, 50, 25);
        
        println!();
        println!("WIDE interval #{} (width={:.3}):", wide.idx, wide.width);
        ascii_plot_phase(&wide.phase_x, &wide.phase_y, 50, 25);
    }
    
    // ═══════════════════════════════════════════════════
    // MAP 3: WIDTH vs COMPLEXITY SCATTER
    // ═══════════════════════════════════════════════════
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  MAP 3: ALL INTERVALS — WIDTH vs COMPLEXITY");
    println!("  Each row = one interval. Width on x-axis, complexity as density.");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Create a scatter plot: x = width (binned), y = complexity
    let n_width_bins = 40;
    let max_width = intervals.iter().map(|iv| iv.width).fold(0.0, f64::max);
    
    // For each width bin, compute mean complexity
    let mut bins: Vec<(f64, f64, usize)> = vec![(0.0, 0.0, 0); n_width_bins];
    
    for iv in &intervals {
        let bin = ((iv.width / max_width) * (n_width_bins as f64)) as usize;
        let bin_idx = std::cmp::min(bin, n_width_bins - 1);
        bins[bin_idx].0 += iv.ellipse_std();
        bins[bin_idx].1 += 1.0;
    }
    
    println!("Width →  ");
    for i in 0..n_width_bins {
        if bins[i].1 > 0.0 {
            let mean_c = bins[i].0 / bins[i].1;
            let bar_len = (mean_c * 30.0).round() as usize;
            let bar: String = "█".repeat(bar_len);
            println!("Bin {:2} ({:.2}-{:.2}): {} n={}", 
                i, 
                (i as f64) * max_width / (n_width_bins as f64),
                ((i+1) as f64) * max_width / (n_width_bins as f64),
                bar, bins[i].1 as usize);
        }
    }
    
    // ═══════════════════════════════════════════════════
    // MAP 4: THE CONTINUUM — ALL INTERVALS IN SEQUENCE
    // ═══════════════════════════════════════════════════
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  MAP 4: THE CONTINUUM (intervals in sequence)");
    println!("  Each character = one interval. Height = width, shade = complexity.");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let max_w = intervals.iter().map(|iv| iv.width).fold(0.0, f64::max);
    let max_c = intervals.iter().map(|iv| iv.ellipse_std()).fold(0.0, f64::max);
    
    println!("t-range: {:.1} to {:.1}", found_zeros[1], found_zeros[std::cmp::min(found_zeros.len()-1, 71)]);
    println!();
    
    // Print in rows of 40 intervals
    for row_start in (0..intervals.len()).step_by(40) {
        let row_end = std::cmp::min(row_start + 40, intervals.len());
        
        // Find max width/complexity in this row for scaling
        let row_max_w = intervals[row_start..row_end].iter().map(|iv| iv.width).fold(0.0, f64::max);
        let row_max_c = intervals[row_start..row_end].iter().map(|iv| iv.ellipse_std()).fold(0.0, f64::max);
        
        // 5 rows of characters per interval (vertical bar chart)
        for h in 0..5 {
            let mut line = String::new();
            for i in row_start..row_end {
                let iv = &intervals[i];
                let w_frac = if row_max_w > 0.0 { iv.width / row_max_w } else { 0.0 };
                let c_frac = if row_max_c > 0.0 { iv.ellipse_std() / row_max_c } else { 0.0 };
                
                let height_threshold = (h as f64) / 5.0;
                let char_idx = if w_frac > height_threshold {
                    (c_frac * (chars.len() as f64 - 1.0)) as usize
                } else {
                    0
                };
                line.push(chars[char_idx]);
            }
            println!("{}", line);
        }
        
        // Label row
        let label_start = found_zeros[row_start + 1];
        let label_end = found_zeros[row_end];
        println!("     {:.1}", label_start);
        if row_end < intervals.len() {
            println!();
        }
    }
    
    println!();
    println!("Legend:  . : - = + * # % @  (low to high complexity)");
    println!("         Height of bar = interval width");
    println!();
    println!("Pattern: Dark tall bars = wide, complex intervals.");
    println!("         Light short bars = narrow, simple intervals.");
    println!("         Look for clusters — do complex intervals group together?");
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

fn ascii_plot_shape(shapes: &[f64], width: usize, height: usize) {
    let mut grid = vec![vec![' '; width]; height];
    
    for j in 0..shapes.len() {
        let x = (shapes[j] * ((width - 1) as f64)).round() as usize;
        let y_val = shapes[j];
        let y = ((1.0 - y_val) * (height as f64 - 1.0)).round() as usize;
        
        if x < width && y < height {
            grid[y][x] = '●';
        }
    }
    
    // Also draw a baseline
    for x in 0..width {
        if grid[height-1][x] == ' ' {
            grid[height-1][x] = '_';
        }
    }
    
    for row in &grid {
        println!("  {}", row.iter().collect::<String>());
    }
}

fn ascii_plot_phase(xs: &[f64], ys: &[f64], width: usize, height: usize) {
    let mut grid = vec![vec![' '; width]; height];
    
    // Draw axes
    let mid_y = height / 2;
    let mid_x = width / 2;
    for x in 0..width {
        if grid[mid_y][x] == ' ' { grid[mid_y][x] = '-'; }
    }
    for y in 0..height {
        if grid[y][mid_x] == ' ' { grid[y][mid_x] = '|'; }
    }
    
    // Plot trajectory
    let n = xs.len();
    for j in 0..n {
        // x: -1 to 1 → 0 to width
        let px = (((xs[j] + 1.0) / 2.0) * ((width - 1) as f64)).round() as usize;
        // y: -1 to 1 → height to 0 (flip)
        let py = ((1.0 - (ys[j] + 1.0) / 2.0) * ((height - 1) as f64)).round() as usize;
        
        if px < width && py < height {
            // Use different characters based on position in trajectory
            let frac = j as f64 / n as f64;
            let chars = ['·', 'o', 'O', '●'];
            grid[py][px] = chars[(frac * (chars.len() as f64 - 1.0)) as usize];
        }
    }
    
    // Mark start and end
    if n > 0 {
        let px_start = (((xs[0] + 1.0) / 2.0) * ((width - 1) as f64)).round() as usize;
        let py_start = ((1.0 - (ys[0] + 1.0) / 2.0) * ((height - 1) as f64)).round() as usize;
        if px_start < width && py_start < height { grid[py_start][px_start] = 'S'; }
        
        let px_end = (((xs[n-1] + 1.0) / 2.0) * ((width - 1) as f64)).round() as usize;
        let py_end = ((1.0 - (ys[n-1] + 1.0) / 2.0) * ((height - 1) as f64)).round() as usize;
        if px_end < width && py_end < height { grid[py_end][px_end] = 'E'; }
    }
    
    println!("  Z' (velocity) ↑");
    for row in &grid {
        println!("  {}", row.iter().collect::<String>());
    }
    println!("              → Z (position)");
}
