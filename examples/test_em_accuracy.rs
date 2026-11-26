use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;
use shor_grover_exploration::riemann::zeta_riemann_siegel::riemann_siegel_theta;
use num_complex::Complex64;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  EM ACCURACY AT KNOWN ZEROS (s = 1/2 + it)");
    println!("═══════════════════════════════════════════════════════");
    
    let known_zeros = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
    ];
    
    for &t in &known_zeros {
        let s = Complex64::new(0.5, t);
        
        println!();
        println!("─── t = {:.9}", t);
        println!("  Terms   |ζ(s)|       Re         Im");
        println!("  -------|-----------|----------|----------");
        
        for &n in &[100, 500, 1000, 2000, 5000] {
            let zeta_val = zeta_euler_maclaurin(s, n);
            println!("  {:>7}   |{:>11.6}|  {:>10.6}  {:>10.6}", 
                     n, zeta_val.norm(), zeta_val.re, zeta_val.im);
        }
        
        // Also compute Z(t) = e^{iθ} ζ(s) and check if it's real
        let theta = riemann_siegel_theta(t);
        let rotation = Complex64::new(theta.cos(), theta.sin());
        let z_func = rotation * zeta_euler_maclaurin(s, 2000);
        println!("  Z(t) via EM: {:.8} + {:.8}i", z_func.re, z_func.im);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
}
