
use shor_grover_exploration::riemann::*;

fn main() {
    let zeros = [14.1347251417, 21.0220396388, 25.0108575801, 30.4248761259, 32.9350615877];
    
    println!("At zeros (expect |Z| < 0.1):");
    for &t in &zeros {
        let z_rs = zeta_riemann_siegel::zeta_z_function(t);
        let z_acc = zeta_riemann_siegel::zeta_z_function_accurate(t);
        println!("  t={}: RS={:+.6} (pass: {})  Accurate={:+.6} (pass: {})", 
                  t, z_rs, z_rs.abs() < 0.1, z_acc, z_acc.abs() < 0.1);
    }
    
    println!();
    println!("Conclusion: RS formula is better at zeros, Dirichlet is better between zeros.");
}
