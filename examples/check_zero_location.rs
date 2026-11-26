use num_complex::Complex64;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;

fn main() {
    let gamma = 14.13472514173469379045725198356247;
    
    println!("Checking |ζ(s)| at and near the first zero:");
    println!();
    
    // At the exact critical line point
    let s = Complex64::new(0.5, gamma);
    for &n in &[1000, 2000, 5000] {
        let zv = zeta_euler_maclaurin(s, n);
        println!("N={}: |ζ(½ + {:.4}i)| = {:.10}", n, gamma, zv.norm());
    }
    
    println!();
    println!("Scanning for actual zeros of EM approximation near t=14:");
    let mut prev_sign: Option<f64> = None;
    let mut prev_t: f64 = 13.0;
    for i in 0..200 {
        let t = 13.0 + 0.05 * (i as f64);
        let s = Complex64::new(0.5, t);
        let zv = zeta_euler_maclaurin(s, 2000);
        let re = zv.re; // Z(t) is real on the critical line
        
        if let Some(prev) = prev_sign {
            if (prev > 0.0 && re < 0.0) || (prev < 0.0 && re > 0.0) {
                println!("  Sign change between t={:.2} and t={:.2}", prev_t, t);
            }
        }
        prev_sign = Some(re);
        prev_t = t;
    }
    
    // Also check the Riemann-Siegel Z function directly
    println!();
    println!("Using Riemann-Siegel Z(t) instead:");
    for &t in &[14.0, 14.1, 14.13, 14.1347, 14.14, 14.2] {
        let z_val = shor_grover_exploration::riemann::zeta_riemann_siegel::zeta_critical_line_rs(t);
        println!("  Z({:.4}) = {:.8}", t, z_val);
    }
}
