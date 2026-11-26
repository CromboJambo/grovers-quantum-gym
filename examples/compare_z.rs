use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    let test_points = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
        40.918719012147495,
        52.970325814,
        20.0,
        35.0,
        60.0,
    ];
    
    for &t in &test_points {
        let z = zeta_z_function(t);
        println!("{} {}", t, z);
    }
}
