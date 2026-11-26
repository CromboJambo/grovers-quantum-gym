use num_complex::Complex64;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;

fn main() {
    println!("=== EM CONVERGENCE AT CRITICAL LINE ZERO ===");
    println!("Testing ζ(0.5 + 14.1347i) which should be ≈ 0");
    println!();
    
    let s = Complex64::new(0.5, 14.134725);
    
    for &n in &[100, 500, 1000, 2000, 5000] {
        let z = zeta_euler_maclaurin(s, n);
        println!("N={:>6}: |ζ| = {:>12.6e}", n, z.norm());
    }
    
    println!();
    println!("=== EM CONVERGENCE AT ζ(2) (Re(s)=2, no cancellation issue) ===");
    let s2 = Complex64::new(2.0, 0.0);
    let expected = std::f64::consts::PI.powi(2) / 6.0;
    
    for &n in &[100, 500, 1000, 2000] {
        let z = zeta_euler_maclaurin(s2, n);
        println!("N={:>6}: ζ(2) = {:.10}, error = {:.2e}", n, z.re, (z.re - expected).abs());
    }
    
    println!();
    println!("=== EM CONVERGENCE AT ζ(1.5) (Re(s)=1.5, mild cancellation) ===");
    let s3 = Complex64::new(1.5, 0.0);
    // ζ(1.5) ≈ 2.612375
    for &n in &[100, 500, 1000, 2000] {
        let z = zeta_euler_maclaurin(s3, n);
        println!("N={:>6}: ζ(1.5) = {:.8}, error = {:.2e}", n, z.re, (z.re - 2.612375).abs());
    }
    
    println!();
    println!("=== EM CONVERGENCE AT ζ(0.5+it₀) FOR DIFFERENT t₀ ===");
    for &t in &[14.134725, 21.022040, 25.010858] {
        let s = Complex64::new(0.5, t);
        let z = zeta_euler_maclaurin(s, 2000);
        println!("t={:>10}: |ζ| = {:>12.6e}", t, z.norm());
    }
    
    println!();
    println!("=== PARTIAL SUM VS CORRECTION TERM MAGNITUDES ===");
    let s = Complex64::new(0.5, 14.134725);
    let n = 2000.0;
    
    // Compute partial sum magnitude
    let mut sum = Complex64::new(0.0, 0.0);
    for k in 1..2000 {
        sum += (-s * (k as f64).ln()).exp();
    }
    
    // Compute correction term N^(1-s)/(1-s)
    let one_minus_s = 1.0 - s;
    let corr = Complex64::new(n, 0.0).powc(one_minus_s) / one_minus_s;
    
    println!("|Partial sum|   = {:.6e}", sum.norm());
    println!("|Correction|    = {:.6e}", corr.norm());
    println!("|Sum + Corr|    = {:.6e}", (sum + corr).norm());
    println!("Cancellation: partial and correction are same order, result is much smaller");
}
