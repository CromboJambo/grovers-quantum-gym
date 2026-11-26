fn main() {
    println!("SINGULARITY ANALYSIS OF Phi(p)");
    println!();
    
    println!("Testing near p = 1/4 (where cos(2*pi*p) = 0):");
    for n in [3, 4, 5] {
        for delta in [-0.001_f64, -0.0001, 0.0001, 0.001] {
            let x_test = n as f64 + 0.25 + delta;
            let p_test = x_test - (x_test.floor() as f64);
            
            let two_pi_p = 2.0 * std::f64::consts::PI * p_test;
            let denom = two_pi_p.cos();
            let numer = (2.0 * std::f64::consts::PI * (p_test*p_test - p_test - 0.0625)).cos();
            
            if denom.abs() > 1e-15 {
                let phi = numer / denom;
                println!("  N={}, delta={}: p={}, cos(2*pi*p)={}, Phi(p)={}", 
                         n, delta, p_test, denom, phi);
            } else {
                println!("  N={}, delta={}: p={}, SINGULAR (denom={})", 
                         n, delta, p_test, denom);
            }
        }
    }
    
    println!();
    println!("Testing near p = 3/4 (where cos(2*pi*p) = 0):");
    for n in [3, 4, 5] {
        for delta in [-0.001_f64, -0.0001, 0.0001, 0.001] {
            let x_test = n as f64 + 0.75 + delta;
            let p_test = x_test - (x_test.floor() as f64);
            
            let two_pi_p = 2.0 * std::f64::consts::PI * p_test;
            let denom = two_pi_p.cos();
            let numer = (2.0 * std::f64::consts::PI * (p_test*p_test - p_test - 0.0625)).cos();
            
            if denom.abs() > 1e-15 {
                let phi = numer / denom;
                println!("  N={}, delta={}: p={}, cos(2*pi*p)={}, Phi(p)={}", 
                         n, delta, p_test, denom, phi);
            } else {
                println!("  N={}, delta={}: p={}, SINGULAR (denom={})", 
                         n, delta, p_test, denom);
            }
        }
    }
    
    // Compute limits via L'Hopital's rule
    println!();
    println!("Analytical limits via L'Hopital:");
    
    let p1 = 0.25_f64;
    let limit_1q = ((2.0 * std::f64::consts::PI * (p1*p1 - p1 - 0.0625)).sin() * (2.0*p1 - 1.0)) 
                  / (2.0 * std::f64::consts::PI * p1).sin();
    println!("  Limit as p->1/4: Phi(p) = {}", limit_1q);
    
    let p3 = 0.75_f64;
    let limit_3q = ((2.0 * std::f64::consts::PI * (p3*p3 - p3 - 0.0625)).sin() * (2.0*p3 - 1.0)) 
                  / (2.0 * std::f64::consts::PI * p3).sin();
    println!("  Limit as p->3/4: Phi(p) = {}", limit_3q);
}
