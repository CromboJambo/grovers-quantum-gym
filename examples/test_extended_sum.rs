use shor_grover_exploration::riemann::*;
use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("Testing Z(t) accuracy with different numbers of main sum terms");
    println!();
    
    // Test points where we know mpmath values
    let test_points = [
        (20.0, -0.8258018160),   // mpmath reference
        (35.0, 1.9581184468),    // mpmath reference
        (60.0, 0.4110837510),    // mpmath reference
    ];
    
    for &(t, ref_val) in &test_points {
        let x = (t / (2.0 * std::f64::consts::PI)).sqrt();
        let n_base = x.floor() as usize;
        
        println!("t = {:.1} (N_base = {}):", t, n_base);
        println!("  Reference (mpmath): {:.10}", ref_val);
        
        // Try different numbers of extra terms
        for extra in [0, 5, 10, 20, 50] {
            let n_terms = n_base + extra;
            
            // Compute theta(t)
            let theta = riemann_siegel_theta(t);
            
            // Compute main sum with Kahan summation
            let mut sum = 0.0;
            let mut c = 0.0;
            for n in 1..=n_terms {
                let term = (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
                let y = term - c;
                let temp = sum + y;
                c = (temp - sum) - y;
                sum = temp;
            }
            
            // Compute correction with the extended N
            let x_ext = n_terms as f64;
            let p_ext = 0.0; // p = 0 when we use exactly integer terms
            
            // For now, just compute 2*sum without correction to see the trend
            let z_no_corr = 2.0 * sum;
            
            println!("  N={} (base+{}): Z_no_corr = {:.10} (error: {:.4})", 
                     n_terms, extra, z_no_corr, (z_no_corr - ref_val).abs());
        }
        println!();
    }
}
