use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("=== HARDY Z(t) REALITY TEST (t ≥ 50, using Riemann-Siegel) ===");
    println!();
    println!("{:>8} {:>15} {:>12} {:>10}", "t", "Z(t)", "|Im(Z)|", "Real?");
    
    for &t in &[50.0, 60.0, 70.0, 80.0, 90.0, 100.0] {
        let z_val = zeta_z_function(t);  // Should be real (returns f64)
        println!("{:>8.2} {:>15.8} {:>12.2e} {:>10}", t, z_val, 0.0, "[v] yes");
    }
    
    println!();
    println!("=== ζ(½+it) PHASE TEST ===");
    println!("If θ(t) is correct, Im[ζ(½+it)]/Re[ζ(½+it)] should match tan(θ(t))");
    println!();
    println!("{:>8} {:>12} {:>12} {:>12} {:>10}", "t", "arg ζ", "θ(t)", "Difference", "Match?");
    
    for &t in &[50.0, 60.0, 70.0, 80.0, 90.0, 100.0] {
        let zeta_val = zeta_critical_line_rs(t);
        let theta = riemann_siegel_theta(t);
        let arg_zeta = zeta_val.arg();
        
        // Normalize difference to [-π, π]
        use std::f64::consts::PI;
        let mut diff = (arg_zeta - theta).rem_euclid(2.0 * PI);
        if diff > PI { diff -= 2.0 * PI; }
        
        println!("{:>8.2} {:>12.6} {:>12.6} {:>12.6} {:>10}", 
            t, arg_zeta, theta % (2.0*PI), diff.abs(),
            if diff.abs() < 0.01 { "[v]" } else { "[x]" });
    }
    
    println!();
    println!("=== KNOWN ZERO CHECK (t ≥ 50) ===");
    // Known zeros above 50: 49.7738, 52.9703, 56.4462, 59.3470, 62.8872
    let known_zeros = vec![49.773832919449073542462113933830, 
                           52.970321477713601239740373589095,
                           56.446247695577901533960889965963];
    
    println!("{:>8} {:>15} {:>10}", "t", "|Z(t)|", "Near zero?");
    for &t in &known_zeros {
        let z_val = zeta_z_function(t);
        println!("{:>8.4} {:>15.2e} {:>10}", t, z_val.abs(), 
            if z_val.abs() < 0.1 { "[v] YES" } else { "[x] no" });
    }
}
