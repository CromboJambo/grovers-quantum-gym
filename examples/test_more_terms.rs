use shor_grover_exploration::riemann::zeta_riemann_siegel::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  BASIC RS WITH MORE TERMS AT KNOWN ZEROS");
    println!("═══════════════════════════════════════════════════════");
    
    let known_zeros = [
        52.970325814,
        56.636158063,
        60.831859482,
        65.798444358,
        69.549652190,
        72.063184930,
    ];
    
    println!();
    println!("  t=60.83 (known zero) with different term counts:");
    let t = 60.831859482;
    let theta = riemann_siegel_theta(t);
    let n_base = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize;
    
    for extra in [0, 5, 10, 20, 50, 100] {
        let n_terms = n_base + extra;
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
        let z = 2.0 * sum;
        println!("    N+{} terms (N={}): Z(t) = {:.10}", extra, n_base, z);
    }
    
    println!();
    println!("  All known zeros with N+20 terms:");
    for &t in &known_zeros {
        let theta = riemann_siegel_theta(t);
        let n_terms = ((t / (2.0 * std::f64::consts::PI)).sqrt() as usize) + 20;
        
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
        
        // Add R-function
        let r = riemann_siegel_r_function(t);
        let full_z = basic_z + r;
        
        println!("  t = {:.9}: Basic(N+20) = {:.10}, Full = {:.10}", 
                 t, basic_z, full_z);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
