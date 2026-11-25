use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    let t_max = 100.0;
    println!("Running Test 25 with t_max = {}...", t_max);
    let report = shape_test_topological_orientation(t_max);
    println!("{}", report);
}
