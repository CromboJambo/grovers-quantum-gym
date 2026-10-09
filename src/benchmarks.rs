//! Benchmarks comparing Grover vs classical approaches.

use crate::grover::{grover_search, linear_search};
use std::time::Instant;

pub fn benchmark_search(n: usize, target: usize) {
    println!("Search space size: {}", n);

    // Classical linear search
    let start = Instant::now();
    let result_classical = linear_search(n, |x| x == target);
    let elapsed_classical = start.elapsed().as_micros();
    println!(
        "  Classical linear search: {:?} (found: {:?})",
        elapsed_classical, result_classical
    );

    // Grover's search
    let start = Instant::now();
    let result_grover = grover_search(n, |x| x == target);
    let elapsed_grover = start.elapsed().as_micros();
    println!(
        "  Grover's search: {:?} (found: {:?})",
        elapsed_grover, result_grover
    );

    if elapsed_classical > 0 {
        let speedup = elapsed_classical as f64 / elapsed_grover as f64;
        println!("  Speedup factor: {:.2}x", speedup);
    }
}

pub fn run_all_benchmarks() {
    println!("=== Grover vs Classical Search Benchmarks ===\n");

    let sizes = [10, 100, 1000, 10000];
    for &n in &sizes {
        let target = n / 2;
        benchmark_search(n, target);
        println!();
    }
}
