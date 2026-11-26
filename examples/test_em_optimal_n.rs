use num_complex::Complex64;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;

fn main() {
    println!("=== EM WITH OPTIMAL N SELECTION ===");
    println!();
    
    // For s = 0.5 + it, optimal N for EM is roughly sqrt(t) or similar
    // Let's test various small N values at the first zero
    
    let t0 = 14.134725;
    let s = Complex64::new(0.5, t0);
    
    println!("ζ(0.5 + {:.6}i) should be ≈ 0", t0);
    println!("{:>8} {:>14}", "N", "|ζ|");
    println!("{}", "-".repeat(24));
    
    for n in [1, 2, 3, 4, 5, 7, 10, 15, 20, 30, 50, 100].iter() {
        let z = zeta_euler_maclaurin(s, *n);
        println!("{:>8} {:>14.6e}", n, z.norm());
    }
    
    println!();
    println!("=== RS OPTIMAL N FOR COMPARISON ===");
    // RS uses N = sqrt(t / 2π)
    let rs_n = (t0 / (2.0 * std::f64::consts::PI)).sqrt();
    println!("RS optimal N for t={:.4}: {:.2}", t0, rs_n);
    
    println!();
    println!("=== TEST AT HIGHER t WHERE RS WORKS ===");
    // At higher t, EM with optimal N might work better
    for &t in &[50.0, 100.0, 200.0] {
        let s = Complex64::new(0.5, t);
        let optimal_n = (t / (2.0 * std::f64::consts::PI)).sqrt() as usize;
        
        println!("t={:.0}, optimal N={}", t, optimal_n);
        for &n in &[optimal_n.saturating_sub(5), optimal_n, optimal_n + 5] {
            if n > 0 {
                let z = zeta_euler_maclaurin(s, n);
                println!("  N={:>4}: |ζ| = {:>12.6e}", n, z.norm());
            }
        }
    }
    
    println!();
    println!("=== CHECK: IS OUR EM FORMULA SIGN-CORRECT? ===");
    // For large real s > 1, ζ(s) → 1 from above
    // Test at s = 10 where ζ(10) ≈ 1.000994575
    let s10 = Complex64::new(10.0, 0.0);
    for &n in &[2, 3, 5, 10] {
        let z = zeta_euler_maclaurin(s10, n);
        println!("ζ(10) with N={}: {:.12}", n, z.re);
    }
    println!("Expected: ζ(10) ≈ 1.000994575");
}
