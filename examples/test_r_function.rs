use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  FULL RS (WITH R-FUNCTION) AT KNOWN ZEROS");
    println!("═══════════════════════════════════════════════════════");
    
    let known_zeros = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
    ];
    
    for &t in &known_zeros {
        let z_val = zeta_z_function(t);
        println!("  t = {:.9}: Z(t) = {:.10}  |Z| = {:.2e}", t, z_val, z_val.abs());
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  COMPARISON: Basic RS vs Full RS (with R)");
    println!("═══════════════════════════════════════════════════════");
    
    for &t in &known_zeros {
        let full_z = zeta_z_function(t);
        
        // Compute basic RS manually
        let two_pi = 2.0 * std::f64::consts::PI;
        let theta = riemann_siegel_theta(t);
        let n_terms = ((t / two_pi).sqrt() as usize) + 5;
        
        let mut sum = 0.0_f64;
        for n in 1..=n_terms {
            let phase = theta - t * (n as f64).ln();
            sum += phase.cos() / (n as f64).sqrt();
        }
        let basic_z = 2.0 * sum;
        
        println!("  t = {:.9}: Basic={:.10}  Full={:.10}  Diff(R)={:.10}", 
                 t, basic_z, full_z, full_z - basic_z);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
