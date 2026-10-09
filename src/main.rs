use shor_grover_exploration::grover::{grover_search, linear_search};
use shor_grover_exploration::shor::shor_factor;
use std::time::Instant;

fn main() {
    println!("=== QUANTUM GYM: EXPLORATION SESSION ===\n");

    // Grover's Search - scaling study
    println!("--- GROVER'S SEARCH: SCALING STUDY ---");
    let sizes = [8, 64, 256, 1024];
    for &n in &sizes {
        let target = n / 2;
        
        // Classical
        let start = Instant::now();
        let classical_result = linear_search(n, |x| x == target);
        let classical_time = start.elapsed();
        
        // Grover's
        let start = Instant::now();
        let grover_result = grover_search(n, |x| x == target);
        let grover_time = start.elapsed();
        
        println!("N={:5}: classical {:6.3}ms (found={}) vs Grover {:6.3}ms (found={})", 
            n, classical_time.as_secs_f64() * 1000.0, classical_result.is_some(),
            grover_time.as_secs_f64() * 1000.0, grover_result.is_some());
    }
    
    // Grover's with multiple marked states
    println!("\n--- GROVER'S SEARCH: MULTIPLE TARGETS ---");
    let result = grover_search(100, |x| x % 7 == 0);
    println!("Find multiples of 7 in [0,100): found={}", result.map(|r| r.to_string()).unwrap_or_default());
    
    // Shor's Factoring - stress test with larger numbers
    println!("\n--- SHOR'S FACTORING: STRESS TEST ---");
    let numbers = [15u64, 21, 35, 55, 77, 91, 143, 221, 323, 391];
    for &n in &numbers {
        let result = shor_factor(n);
        match result {
            Some((p, q)) => println!("{:4} = {} × {}", n, p, q),
            None => println!("{:4} = (failed to factor)", n),
        }
    }
    
    // NP-Hard Problems - Grover vs Classical comparison
    println!("\n--- NP-HARD PROBLEMS ---");
    
    // Subset Sum
    let numbers = vec![3, 34, 4, 5, 6, 12, 9, 8];
    let target = 27;
    let result = shor_grover_exploration::problems::subset_sum_classical(&numbers, target);
    println!("Subset sum (target={})", target);
    if let Some(indices) = result {
        let mut sum = 0;
        let mut parts = Vec::new();
        for &i in &indices {
            parts.push(numbers[i].to_string());
            sum += numbers[i];
        }
        println!("  Found: {} = {}", parts.join(" + "), sum);
    }
    
    // SAT Problem
    println!("\nSAT problem (3-CNF with 4 variables):");
    // (x1 ∨ x2) ∧ (¬x1 ∨ x3) ∧ (x2 ∨ ¬x3 ∨ x4)
    let clauses = vec![
        vec![1, 2],      // x1 ∨ x2
        vec![-1, 3],     // ¬x1 ∨ x3
        vec![2, -3, 4],  // x2 ∨ ¬x3 ∨ x4
    ];
    let result = shor_grover_exploration::problems::sat_classical(&clauses, 4);
    match result {
        Some(assignment) => {
            println!("  Satisfiable! Assignment:");
            for (i, &val) in assignment.iter().enumerate() {
                println!("    x{} = {}", i + 1, if val { "true" } else { "false" });
            }
        }
        None => println!("  Unsatisfiable"),
    }
    
    // TSP - Nearest Neighbor vs Brute Force Comparison
    println!("\nTSP (Nearest Neighbor Heuristic):");
    let distances = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let (tour, total) = shor_grover_exploration::problems::tsp_nearest_neighbor(&distances);
    println!("  Tour: {}", tour.iter().map(|&c| c.to_string()).collect::<Vec<_>>().join(" → "));
    println!("  Total distance: {}", total);
    
    println!("\n--- ADA LOVELACE STATION ---");
    shor_grover_exploration::lovelace::show_lovelace_station();

    println!("\n--- THOMPSON REGEX ENGINE (1968) ---");
    shor_grover_exploration::regex::show_regex_station();

    println!("\n--- ATTENTION IS ALL YOU NEED (2017) ---");
    shor_grover_exploration::attention::show_attention_station();

    println!("\n--- P VS NP MILLENNIUM PRIZE PROBLEM ---");
    shor_grover_exploration::p_vs_np::show_p_vs_np_station();

    println!("\n--- NAVIER-STOKES MILLENNIUM PRIZE PROBLEM ---");
    shor_grover_exploration::navier_stokes::show_navier_stokes_station();

    println!("\n--- RIEMANN HYPOTHESIS MILLENNIUM PRIZE PROBLEM ---");
    shor_grover_exploration::riemann::show_riemann_station();

    println!("\n--- YANG-MILLS EXISTENCE AND MASS GAP PROBLEM ---");
    shor_grover_exploration::yang_mills::show_yang_mills_station();

    println!("\n=== EXPLORATION COMPLETE ===");
}
