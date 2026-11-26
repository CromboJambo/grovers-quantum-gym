use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  Z(t) ACCURACY: Basic RS + Newton Refinement");
    println!("═══════════════════════════════════════════════════════");
    
    // Known zeros for validation
    let known_zeros = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
        32.935061587739189,
        37.586178158825670,
        40.918719012147495,
        43.327073280914000,
    ];
    
    for &gamma_true in &known_zeros {
        // Start near the known zero (within 0.1)
        let t_start = gamma_true - 0.05;
        
        // Newton's method: t_{n+1} = t_n - Z(t_n)/Z'(t_n)
        let mut t = t_start;
        let mut converged = false;
        
        for iter in 0..50 {
            let z_val = basic_rs_z(t);
            let z_deriv = basic_rs_z_prime(t);
            
            if z_deriv.abs() < 1e-15 {
                break; // derivative too small
            }
            
            let t_new = t - z_val / z_deriv;
            
            if (t_new - t).abs() < 1e-12 {
                t = t_new;
                converged = true;
                break;
            }
            t = t_new;
        }
        
        let z_at_zero = basic_rs_z(t);
        let error = (t - gamma_true).abs();
        
        println!("  True: {:.10}  Found: {:.10}  |Z|={:.2e}  Err={:.2e}  {}", 
                 gamma_true, t, z_at_zero.abs(), error,
                 if converged { "CONVERGED" } else { "NOT CONVERGED" });
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}

/// Basic RS Z-function with optimal N
fn basic_rs_z(t: f64) -> f64 {
    let two_pi = 2.0 * std::f64::consts::PI;
    let theta = riemann_siegel_theta(t);
    let n_terms = ((t / two_pi).sqrt() as usize) + 5;
    
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;
    for n in 1..=n_terms {
        let phase = theta - t * (n as f64).ln();
        let term = phase.cos() / (n as f64).sqrt();
        
        let y = term - compensation;
        let temp = sum + y;
        compensation = (temp - sum) - y;
        sum = temp;
    }
    2.0 * sum
}

/// Derivative of basic RS Z-function
fn basic_rs_z_prime(t: f64) -> f64 {
    let h = 1e-7;
    (basic_rs_z(t + h) - basic_rs_z(t - h)) / (2.0 * h)
}
