use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    let t_max = if args.len() > 1 {
        args[1].parse::<f64>().unwrap_or(200.0)
    } else {
        200.0
    };
    
    println!("\n======================================================================");
    println!("SHAPE TESTING: Geometric structure in zero-count trajectory");
    println!("Zeros up to T = {}", t_max);
    println!("======================================================================\n");
    
    // Run all shape tests
    let report = all_shape_tests(t_max);
    println!("{}", report);
}