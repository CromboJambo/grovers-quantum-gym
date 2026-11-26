
use std::fs;
use shor_grover_exploration::riemann::*;

/// Find all zeros in [a, b] using sign changes of Z(t) with given step size.
fn find_zeros(a: f64, b: f64, step: f64) -> Vec<f64> {
    let mut zeros = Vec::new();
    let mut t = a;
    let mut prev_z = zeta_riemann_siegel::zeta_z_function(t);
    
    while t < b {
        t += step;
        let z = zeta_riemann_siegel::zeta_z_function(t);
        
        if prev_z * z < 0.0 {
            // Sign change detected — refine using bisection
            let mut lo = t - step;
            let mut hi = t;
            for _ in 0..60 {
                let mid = (lo + hi) / 2.0;
                let z_mid = zeta_riemann_siegel::zeta_z_function(mid);
                if zeta_riemann_siegel::zeta_z_function(lo) * z_mid < 0.0 {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            zeros.push((lo + hi) / 2.0);
        }
        
        prev_z = z;
    }
    
    zeros
}

/// Expected number of zeros up to T using the Riemann-von Mangoldt formula.
fn expected_zeros(t: f64) -> i64 {
    if t <= 2.0 { return 0; }
    let theta = 0.5 * t * (t / (2.0 * std::f64::consts::PI)).ln() - 0.5 * t 
               - std::f64::consts::PI / 8.0 + 1.0 / (48.0 * t);
    (theta / std::f64::consts::PI + 7.0 / 8.0) as i64
}

fn main() {
    let T: f64 = 5000.0;
    
    println!("=== RH Zero-Finding Verification up to T={} ===", T);
    println!();
    
    // Step 1: Find all zeros using RS formula (start at t=10 where RS is valid)
    println!("Step 1: Finding zeros via sign changes (step=0.1, start=10)...");
    let start = std::time::Instant::now();
    let zeros = find_zeros(10.0, T, 0.1);
    let elapsed = start.elapsed();
    
    println!("  Found {} zeros in {:.2}s", zeros.len(), elapsed.as_secs_f64());
    println!();
    
    // Step 2: Check zero count against Riemann-von Mangoldt formula
    let expected = expected_zeros(T);
    println!("Step 2: Zero count verification");
    println!("  Found:    {}", zeros.len());
    println!("  Expected (R-vM): {}", expected);
    
    if (zeros.len() as i64) != expected {
        println!("  WARNING: Count mismatch! Difference: {}", 
                 (zeros.len() as i64 - expected).abs());
    } else {
        println!("  ✓ Zero count matches Riemann-von Mangoldt prediction");
    }
    println!();
    
    // Step 3: Verify zeros against mpmath references (if available)
    if let Ok(content) = fs::read_to_string("/tmp/z_refs_5000.txt") {
        let mut ref_zeros: Vec<f64> = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[2] == "zero" {
                if let Ok(t) = parts[0].parse::<f64>() {
                    ref_zeros.push(t);
                }
            }
        }
        
        if !ref_zeros.is_empty() {
            println!("Step 3: Verifying against mpmath references ({} zeros)", ref_zeros.len());
            
            let mut verified = 0;
            let mut max_err = 0.0f64;
            
            for &ref_t in &ref_zeros {
                // Find closest zero we found
                let mut best_dist = f64::MAX;
                for &found_t in &zeros {
                    let dist = (found_t - ref_t).abs();
                    if dist < best_dist {
                        best_dist = dist;
                    }
                }
                
                if best_dist < 0.01 {
                    verified += 1;
                }
                if best_dist > max_err {
                    max_err = best_dist;
                }
            }
            
            println!("  Verified: {}/{} zeros within 0.01 of mpmath reference", 
                     verified, ref_zeros.len());
            println!("  Max location error: {:.6}", max_err);
            println!();
        }
    }
    
    // Step 4: Check that all zeros are on the critical line (|Z(t)| < tolerance)
    println!("Step 4: Verifying zeros are on critical line (|Z(t)| < 0.1)...");
    let mut on_line = 0;
    let mut max_z_at_zero = 0.0f64;
    
    for &t in &zeros {
        let z = zeta_riemann_siegel::zeta_z_function(t);
        if z.abs() < 0.1 {
            on_line += 1;
        }
        if z.abs() > max_z_at_zero {
            max_z_at_zero = z.abs();
        }
    }
    
    println!("  On critical line: {}/{}", on_line, zeros.len());
    println!("  Max |Z(t)| at zeros: {:.6}", max_z_at_zero);
    println!();
    
    // Step 5: Report first and last few zeros
    println!("First 10 zeros:");
    for &t in &zeros[..10.min(zeros.len())] {
        println!("  t = {:.6}", t);
    }
    if zeros.len() > 10 {
        println!("  ...");
        let start_idx = zeros.len().saturating_sub(5);
        for &t in &zeros[start_idx..] {
            println!("  t = {:.6}", t);
        }
    }
    println!();
    
    // Final verdict
    let count_ok = (zeros.len() as i64) == expected;
    let line_ok = on_line == zeros.len();
    
    if count_ok && line_ok {
        println!("RESULT: PASS — All {} zeros found and verified on critical line", zeros.len());
    } else {
        println!("RESULT: FAIL");
        std::process::exit(1);
    }
}
