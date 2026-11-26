use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;
use num_complex::Complex64;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  EM VALIDATION AT s = 2 (ζ(2) = π²/6 = 1.6449340668...)");
    println!("═══════════════════════════════════════════════════════");
    
    let exact = std::f64::consts::PI * std::f64::consts::PI / 6.0;
    println!("  Exact: {:.12}", exact);
    println!();
    
    for &n in &[10, 50, 100, 500, 1000] {
        let s = Complex64::new(2.0, 0.0);
        let zeta_val = zeta_euler_maclaurin(s, n);
        let error = (zeta_val.re - exact).abs();
        println!("  N={:>5}: ζ(2) = {:.12}, error = {:.2e}", n, zeta_val.re, error);
    }
    
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  EM AT s = 0.5 + 14.1347i (known zero)");
    println!("  Comparing partial sum vs correction terms");
    println!("═══════════════════════════════════════════════════════");
    
    let t = 14.134725141734693;
    let s = Complex64::new(0.5, t);
    let n_terms = 1000 as f64;
    
    // Compute partial sum manually
    let mut partial = Complex64::new(0.0, 0.0);
    for k in 1..n_terms as usize {
        let log_k = (k as f64).ln();
        partial += (-s * log_k).exp();
    }
    
    // Compute correction terms manually
    let n_complex = Complex64::new(n_terms, 0.0);
    let one_minus_s = 1.0 - s;
    let term1 = n_complex.powc(one_minus_s) / one_minus_s;
    let term2 = 0.5 * n_complex.powc(-s);
    let term3 = (s / 12.0) * n_complex.powc(-s - 1.0);
    let term4 = (-1.0 / 720.0) * s * (s + 1.0) * (s + 2.0) * (s + 3.0) * n_complex.powc(-s - 3.0);
    
    println!("  Partial sum: {:.8} + {:.8}i  |partial| = {:.6}", partial.re, partial.im, partial.norm());
    println!("  Term 1 (integral): {:.8} + {:.8}i", term1.re, term1.im);
    println!("  Term 2 (B1): {:.8} + {:.8}i", term2.re, term2.im);
    println!("  Term 3 (B2): {:.8} + {:.8}i", term3.re, term3.im);
    println!("  Term 4 (B4): {:.8} + {:.8}i", term4.re, term4.im);
    
    let corrections = term1 + term2 + term3 + term4;
    println!("  Total corrections: {:.8} + {:.8}i  |corr| = {:.6}", corrections.re, corrections.im, corrections.norm());
    println!("  Sum (partial+corr): {:.8} + {:.8}i  |sum| = {:.6}", 
             (partial + corrections).re, (partial + corrections).im, (partial + corrections).norm());
    
    // Check: do partial and corrections ADD or CANCEL?
    let dot_product = partial.re * corrections.re + partial.im * corrections.im;
    let angle = dot_product / (partial.norm() * corrections.norm()).max(1e-15);
    println!();
    println!("  Angle between partial and corrections: {:.4} rad ({:.2}°)", 
             angle.acos(), angle.acos() * 180.0 / std::f64::consts::PI);
    println!("  (0° = same direction, 180° = opposite)");
    
    // Now try with CORRECTED term4 (only 3 factors instead of 4)
    let term4_fixed = (-1.0 / 720.0) * s * (s + 1.0) * (s + 2.0) * n_complex.powc(-s - 3.0);
    println!();
    println!("  Term 4 FIXED (3 factors): {:.8} + {:.8}i", term4_fixed.re, term4_fixed.im);
    let corrections_fixed = term1 + term2 + term3 + term4_fixed;
    let sum_fixed = partial + corrections_fixed;
    println!("  Sum with fixed term4: {:.8} + {:.8}i  |sum| = {:.6}", 
             sum_fixed.re, sum_fixed.im, sum_fixed.norm());
}
