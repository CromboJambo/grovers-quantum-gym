use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST WITH FIXED GAMMA_UPPER (via R-function)");
    println!("═══════════════════════════════════════════════════════");
    
    // Test at larger t where R-function is active (t >= 50)
    println!();
    println!("  Basic RS vs Full RS at various t:");
    for &t in &[50.0, 100.0, 200.0, 500.0] {
        let theta = riemann_siegel_theta(t);
        let n_terms = ((t / (2.0 * std::f64::consts::PI)).sqrt() as usize) + 5;
        
        let mut sum = 0.0_f64;
        for n in 1..=n_terms {
            let phase = theta - t * (n as f64).ln();
            sum += phase.cos() / (n as f64).sqrt();
        }
        let basic_z = 2.0 * sum;
        
        // The public zeta_z_function now uses basic RS without R-function
        // So we can't directly test the R-function through it
        // But we can at least verify gamma_upper is working by checking
        // that the build succeeds and basic RS still works
        
        println!("  t = {:.1}: Basic RS Z(t) = {:.10}", t, basic_z);
    }
    
    // Now let's make gamma_upper public so we can test it directly
    println!();
    println!("  Need to make gamma_upper public for direct testing...");
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
