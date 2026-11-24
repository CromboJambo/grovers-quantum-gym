//! Riemann-Siegel formula demonstration for Grover's Quantum Gym.
//!
//! Shows how the Riemann-Siegel Z-function enables O(t^{1/4}) zeta computation,
//! making zero verification at high ordinates feasible.

use shor_grover_exploration::riemann::zeta_riemann_siegel::{
    riemann_siegel_report, find_zeros_rs, count_zeros_rs,
};

fn main() {
    println!("=== Riemann-Siegel Z-Function Demo ===\n");
    
    // 1. Show the computational advantage
    println!("Computational complexity comparison:");
    for &t in &[100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0] {
        let two_pi = 2.0 * std::f64::consts::PI;
        let n_rs = (t / two_pi).sqrt().floor() as usize;
        println!("  t={:9.0}: RS needs {:6} terms | EM needs ~{:10.0}", t, n_rs, t);
    }
    
    // 2. Zero count predictions
    println!("\nRiemann-von Mangoldt formula (zero counting):");
    for &t in &[100.0, 1_000.0, 10_000.0] {
        let n = count_zeros_rs(t);
        println!("  N({:.0}) ≈ {} zeros below t", t, n);
    }
    
    // 3. Find zeros using RS (for t ≥ 50)
    println!("\nFinding zeros in [50, 80] via Z(t) sign changes:");
    let zeros = find_zeros_rs(50.0, 80.0, 0.3);
    for &z in &zeros {
        println!("  Zero at t={:.9}", z);
    }
    
    // 4. Full report
    println!("\n{}", riemann_siegel_report());
    
    println!("\nNote: Simplified RS implementation. Accuracy improves with t.");
    println!("For production zero-finding, use full Riemann-Siegel with all");
    println!("correction terms and higher precision arithmetic.");
}
