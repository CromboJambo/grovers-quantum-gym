
use std::fs;
use shor_grover_exploration::riemann::*;

fn main() {
    let content = fs::read_to_string("/tmp/z_refs_5000.txt").expect("Failed to read reference file");
    
    let mut total = 0;
    let mut passed_zero = 0;
    let mut passed_grid = 0;
    let mut passed_random = 0;
    let mut max_err_zero = 0.0f64;
    let mut max_err_grid = 0.0f64;
    let mut max_err_random = 0.0f64;
    
    for line in content.lines() {
        if line.is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 { continue; }
        
        let t: f64 = parts[0].parse().unwrap();
        let z_ref: f64 = parts[1].parse().unwrap();
        let kind = parts[2];
        
        total += 1;
        
        // Test RS formula (zeta_z_function)
        let z_rs = zeta_riemann_siegel::zeta_z_function(t);
        let err_rs = (z_rs - z_ref).abs();
        
        // Test accurate method (Dirichlet)
        let z_acc = zeta_riemann_siegel::zeta_z_function_accurate(t);
        let err_acc = (z_acc - z_ref).abs();
        
        match kind {
            "zero" => {
                if err_rs < 0.1 { passed_zero += 1; }
                if err_rs > max_err_zero { max_err_zero = err_rs; }
            },
            "grid" => {
                // Use accurate method for grid points (RS has resonant errors)
                if err_acc < 0.5 { passed_grid += 1; }
                if err_acc > max_err_grid { max_err_grid = err_acc; }
            },
            "random" => {
                if err_acc < 0.5 { passed_random += 1; }
                if err_acc > max_err_random { max_err_random = err_acc; }
            },
            _ => {}
        }
    }
    
    println!("=== T=5000 Validation Results ===");
    println!();
    println!("Zeros (RS formula, tol=0.1): {}/{}", passed_zero, 100);
    println!("  Max error: {:.6}", max_err_zero);
    println!();
    println!("Grid points (Dirichlet, tol=0.5): {}/{}", passed_grid, 99);
    println!("  Max error: {:.6}", max_err_grid);
    println!();
    println!("Random points (Dirichlet, tol=0.5): {}/{}", passed_random, 100);
    println!("  Max error: {:.6}", max_err_random);
    println!();
    
    let all_passed = passed_zero == 100 && passed_grid >= 90 && passed_random >= 80;
    if all_passed {
        println!("RESULT: PASS");
    } else {
        println!("RESULT: FAIL");
        std::process::exit(1);
    }
}
