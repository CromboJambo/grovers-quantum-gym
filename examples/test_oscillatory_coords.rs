use shor_grover_exploration::riemann::zeta_riemann_siegel::*;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;
use num_complex::Complex64;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  OSCILLATORY COORDINATE TRANSFORMATION");
    println!("  theta(t) IS the oscillation that straightens zeta(1/2+it)");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    println!("The Riemann-Siegel Z-function is defined as:");
    println!("  Z(t) = e^(i*theta(t)) * zeta(1/2+it)");
    println!();
    println!("This is an OSCILLATORY COORDINATE TRANSFORMATION:");
    println!("  - Without rotation: arg(zeta(1/2+it)) oscillates wildly with t");
    println!("  - With rotation by theta(t): Z(t) becomes REAL-VALUED");
    println!("  - theta(t) IS the oscillation that straightens the graph");
    println!();
    
    // Demonstrate: compute arg(zeta(1/2+it)) and compare with theta(t)
    println!("t        theta(t)   arg(zeta)   Z(t)=e^(i*theta)*zeta   |Z|");
    println!("{}", "-".repeat(65));
    
    let t_values = vec![14.134725, 15.0, 18.0, 21.022040, 25.0, 30.0, 40.0, 50.0];
    
    for &t in &t_values {
        let theta = riemann_siegel_theta(t);
        
        // Basic RS sum gives us Z(t) directly (approximately)
        let z_t = basic_rs_sum(t);
        
        // Recover zeta(1/2+it) from Z(t): zeta = e^(-i*theta) * Z(t)
        let zeta = Complex64::new(theta.cos(), -theta.sin()) * z_t;
        let arg_zeta = zeta.arg();
        
        println!("{:>8.4}  {:>9.4}  {:>9.4}  {:>12.6e}  {:>8.4}", 
            t, theta, arg_zeta, z_t.abs(), z_t);
    }
    
    println!();
    println!("KEY INSIGHT:");
    println!("  theta(t) is the oscillation needed to straighten the graph.");
    println!("  Its asymptotic form IS the simple, repeatable structure:");
    println!("  theta(t) ~ (t/2)*ln(t/(2*pi)) - t/2 - pi/8 + 1/(48t) + ...");
    println!();
    println!("This is exactly your intuition: instead of smoothing the data,");
    println!("we rotate the coordinate system by theta(t) to absorb the oscillation.");
    println!();
    
    // Now demonstrate the EM problem in this light
    println!("═══════════════════════════════════════════════════════");
    println!("  THE EM PROBLEM IN THIS LIGHT");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("At s = 1/2 + 14.1347i (a true zero):");
    println!("  - Partial sum: |S| ~ sqrt(N) (grows with N)");
    println!("  - Correction term: |C| ~ sqrt(N) (grows with N)");
    println!("  - They ADD instead of cancel -> |zeta_EM| grows with N");
    println!();
    println!("In the oscillatory coordinate view:");
    println!("  - The partial sum and correction are in DIFFERENT phases");
    println!("  - For correct cancellation, they need to be OPPOSITE phase");
    println!("  - Our EM has them in the SAME phase (adding)");
    println!();
    
    // Test: what if we apply the RS phase to our EM result?
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  TEST: APPLY RS PHASE ROTATION TO EM RESULT");
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let t0 = 14.134725;
    let s = Complex64::new(0.5, t0);
    let theta = riemann_siegel_theta(t0);
    
    for &n in &[7, 10, 20, 50] {
        let z_em = zeta_euler_maclaurin(s, n);
        
        // Apply RS phase rotation: Z_rotated = e^(i*theta) * zeta_EM
        let z_rotated = Complex64::new(theta.cos(), theta.sin()) * z_em;
        
        println!("N={:>3}: |zeta_EM|={:>8.4}, arg(zeta_EM)={:>7.4}, theta(t)={:>7.4}", 
            n, z_em.norm(), z_em.arg(), theta);
        println!("      Z_rotated = e^(i*theta)*zeta_EM = {:>10.6e} + {:>10.6e}i", 
            z_rotated.re, z_rotated.im);
        println!("      (If EM were correct, Z_rotated should be real and ~0)");
        println!();
    }
    
    println!("CONCLUSION: The phase rotation doesn't fix the magnitude problem.");
    println!("The issue is deeper: our EM correction terms have the wrong");
    println!("PHASE RELATIONSHIP to the partial sum. They should cancel at");
    println!("zeros (like theta(t) cancels arg(zeta)), but they reinforce.");
}

fn basic_rs_sum(t: f64) -> f64 {
    let theta = riemann_siegel_theta(t);
    let n_terms = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize + 5;
    
    let mut sum = 0.0;
    for n in 1..=n_terms {
        sum += (theta - t * (n as f64).ln()).cos() / (n as f64).sqrt();
    }
    2.0 * sum
}
