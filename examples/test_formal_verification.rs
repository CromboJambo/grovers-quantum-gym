use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    let t_max = 100.0;
    println!("Running SHAPE TEST 26 — Formal Verification of Argument Principle...");
    println!();
    let report = shape_test_formal_verification(t_max);
    println!("{}", report);
}
