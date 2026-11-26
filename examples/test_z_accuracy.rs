use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  Z(t) ACCURACY DIAGNOSTIC AT KNOWN ZEROS");
    println!("  Testing different N values to find optimal cutoff");
    println!("═══════════════════════════════════════════════════════");
    
    // Known zeros (verified from literature)
    let known_zeros = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
        32.935061587739189,
    ];
    
    for &gamma in &known_zeros {
        println!();
        println!("─── Zero at t = {:.9}", gamma);
        
        let two_pi = 2.0 * std::f64::consts::PI;
        let optimal_n = (gamma / two_pi).sqrt().floor() as usize;
        let theta = riemann_siegel_theta(gamma);
        
        println!("  Optimal N = {}", optimal_n);
        println!("  θ(t) = {:.10}", theta);
        println!();
        println!("  N_offset   |Z(t)|     Terms");
        println!("  ----------|----------|------");
        
        for offset in [0, 2, 3, 4, 5, 7, 10, 15] {
            let n_terms = optimal_n + offset;
            if n_terms < 1 { continue; }
            
            let mut sum = 0.0_f64;
            let mut compensation = 0.0_f64;
            for n in 1..=n_terms {
                let phase = theta - gamma * (n as f64).ln();
                let term = phase.cos() / (n as f64).sqrt();
                
                let y = term - compensation;
                let temp = sum + y;
                compensation = (temp - sum) - y;
                sum = temp;
            }
            let z_val = 2.0 * sum;
            
            println!("  {:>8}   |{:.6}|  N={}", offset, z_val.abs(), n_terms);
        }
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  CONCLUSION: Which N_offset gives smallest |Z(t)|?");
    println!("═══════════════════════════════════════════════════════");
}
