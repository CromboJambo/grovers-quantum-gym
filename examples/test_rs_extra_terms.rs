use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("=== RS WITH MORE TERMS THAN OPTIMAL ===");
    println!();
    
    let known_zeros = vec![14.134725, 21.022040, 25.010858];
    
    for &extra in &[0, 5, 10, 20, 50] {
        println!("Extra terms beyond optimal: {}", extra);
        for &t in &known_zeros {
            let z = rs_with_extra_terms(t, extra);
            println!("  t={:>10.6}: Z(t) = {:>14.8e}", t, z);
        }
        println!();
    }
    
    println!("=== SIGN CHANGES WITH EXTRA TERMS ===");
    for &extra in &[0, 10, 50] {
        let mut count = 0;
        let mut prev_t = 10.0;
        let mut prev_z = rs_with_extra_terms(prev_t, extra);
        
        let mut t = 10.0 + 0.005;
        while t < 30.0 {
            let z = rs_with_extra_terms(t, extra);
            if (prev_z < 0.0 && z > 0.0) || (prev_z > 0.0 && z < 0.0) {
                count += 1;
            }
            prev_t = t;
            prev_z = z;
            t += 0.005;
        }
        println!("Extra={:>3}: {} sign changes in [10,30]", extra, count);
    }
}

fn rs_with_extra_terms(t: f64, extra: usize) -> f64 {
    let theta = riemann_siegel_theta(t);
    let optimal_n = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize;
    let n_terms = optimal_n + extra;
    
    let mut sum = 0.0;
    for n in 1..=n_terms {
        sum += (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
    }
    2.0 * sum
}
