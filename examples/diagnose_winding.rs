use num_complex::Complex64;
use shor_grover_exploration::riemann::rh_verification::zeta_euler_maclaurin;
use std::f64::consts::PI;

fn main() {
    let gamma = 14.13472514173469379045725198356247; // First zero
    
    println!("Diagnosing winding number around ρ₁ = ½ + {:.4}i\n", gamma);
    println!("{:>6} {:>8} {:>10} {:>10} {:>12}", "N_terms", "Radius", "Winding", "|ζ|min", "|ζ|max");
    
    for &n in &[500, 1000, 2000, 5000] {
        for &r in &[0.3, 0.5, 0.8] {
            let mut total_phase = 0.0;
            let mut prev_phase: Option<f64> = None;
            let mut min_mag = f64::MAX;
            let mut max_mag: f64 = 0.0;
            let steps = 500;
            
            for i in 0..steps {
                let angle = 2.0 * PI * (i as f64) / (steps as f64);
                let s_re = 0.5 + r * angle.cos();
                let s_im = gamma + r * angle.sin();
                let s = Complex64::new(s_re, s_im);
                
                let zv = zeta_euler_maclaurin(s, n);
                let mag = zv.norm();
                
                if mag.is_nan() || mag.is_infinite() {
                    continue;
                }
                
                min_mag = min_mag.min(mag);
                max_mag = max_mag.max(mag);
                
                let phase = zv.arg();
                if let Some(prev) = prev_phase {
                    let mut delta = phase - prev;
                    while delta > PI { delta -= 2.0 * PI; }
                    while delta < -PI { delta += 2.0 * PI; }
                    total_phase += delta;
                }
                prev_phase = Some(phase);
            }
            
            let w = total_phase / (2.0 * PI);
            println!("{:>6} {:>8.2} {:>+10.4} {:>10.6} {:>12.4}", n, r, w, min_mag, max_mag);
        }
        println!();
    }
}
