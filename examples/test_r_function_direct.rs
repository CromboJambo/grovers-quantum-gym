use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  R-FUNCTION DIRECT TEST (fixed gamma_upper)");
    println!("═══════════════════════════════════════════════════════");
    
    // Known zeros where R-function should be active (t >= 50)
    let known_zeros = [
        52.970325814,
        56.636158063,
        60.831859482,
        65.798444358,
        69.549652190,
        72.063184930,
    ];
    
    println!();
    println!("  Known zeros (t >= 50):");
    for &t in &known_zeros {
        let r = riemann_siegel_r_function(t);
        println!("  R({:.9}) = {:.10}", t, r);
    }
    
    // Test full RS with R-function at these zeros
    println!();
    println!("  Full RS (basic + R) at known zeros:");
    for &t in &known_zeros {
        let theta = riemann_siegel_theta(t);
        let n_terms = ((t / (2.0 * std::f64::consts::PI)).sqrt() as usize) + 5;
        
        // Basic RS sum with Kahan summation
        let mut sum = 0.0_f64;
        let mut c = 0.0_f64;
        for n in 1..=n_terms {
            let phase = theta - t * (n as f64).ln();
            let term = phase.cos() / (n as f64).sqrt();
            let y = term - c;
            let temp = sum + y;
            c = (temp - sum) - y;
            sum = temp;
        }
        let basic_z = 2.0 * sum;
        
        // Full RS with R-function
        let r = riemann_siegel_r_function(t);
        let full_z = basic_z + r;
        
        println!("  t = {:.9}: Basic = {:.10}, R = {:.10}, Full = {:.10}", 
                 t, basic_z, r, full_z);
    }
    
    // Compare with zeta_z_function (which uses basic RS only)
    println!();
    println!("  Comparison with zeta_z_function:");
    for &t in &known_zeros {
        let z = zeta_z_function(t);
        let full = {
            let theta = riemann_siegel_theta(t);
            let n_terms = ((t / (2.0 * std::f64::consts::PI)).sqrt() as usize) + 5;
            let mut sum = 0.0_f64;
            let mut c = 0.0_f64;
            for n in 1..=n_terms {
                let phase = theta - t * (n as f64).ln();
                let term = phase.cos() / (n as f64).sqrt();
                let y = term - c;
                let temp = sum + y;
                c = (temp - sum) - y;
                sum = temp;
            }
            2.0 * sum + riemann_siegel_r_function(t)
        };
        println!("  t = {:.9}: zeta_z_func = {:.10}, full_RS = {:.10}", 
                 t, z, full);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
