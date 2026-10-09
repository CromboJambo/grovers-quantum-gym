/// FAST FOURIER TRANSFORM STATION (1965)
///
/// The Cooley-Tukey FFT algorithm — reducing DFT from O(n²) to O(n log n).
/// This breakthrough enabled digital signal processing, image compression,
/// and countless modern applications.
use std::f64::consts::PI;

#[derive(Debug, Clone)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }

    fn sub(&self, other: &Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }

    fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

/// Direct DFT computation — O(n²)
fn dft_direct(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let mut output = vec![Complex::new(0.0, 0.0); n];

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for j in 0..n {
            let angle = -2.0 * PI * k as f64 * j as f64 / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            sum = sum.add(&input[j].mul(&w));
        }
        output[k] = sum;
    }

    output
}

/// Cooley-Tukey FFT — O(n log n) using divide and conquer
fn fft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();

    if n == 1 {
        return vec![input[0].clone()];
    }

    // Divide into even and odd indexed elements
    let mut even = Vec::with_capacity(n / 2);
    let mut odd = Vec::with_capacity(n / 2);
    for i in 0..n {
        if i % 2 == 0 {
            even.push(input[i].clone());
        } else {
            odd.push(input[i].clone());
        }
    }

    // Recursive FFTs on halves
    let even_transform = fft(&even);
    let odd_transform = fft(&odd);

    // Combine using butterfly operations
    let mut output = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n / 2 {
        let angle = -2.0 * PI * k as f64 / n as f64;
        let w = Complex::new(angle.cos(), angle.sin());
        let t = w.mul(&odd_transform[k]);

        output[k] = even_transform[k].add(&t);
        output[k + n / 2] = even_transform[k].sub(&t);
    }

    output
}

pub fn demonstrate_fft() {
    println!("--- FAST FOURIER TRANSFORM (Cooley-Tukey, 1965) ---");
    println!();
    println!("The FFT transforms a signal from time domain to frequency domain.");
    println!("This reveals what frequencies are present in the signal.");
    println!();

    // Create a test signal: sum of two sinusoids
    let n = 8;
    let mut signal = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / n as f64;
        // Signal with frequencies 1 and 3 (out of 0 to n/2)
        let value = 2.0 * (2.0 * PI * t).sin() + 1.0 * (6.0 * PI * t).sin();
        signal.push(Complex::new(value, 0.0));
    }

    println!("Test signal: sum of two sinusoids at different frequencies");
    println!("Computing DFT using direct method (O(n²))...");
    let dft_result = dft_direct(&signal);

    println!("Computing FFT using Cooley-Tukey (O(n log n))...");
    let fft_result = fft(&signal);

    // Compare results — should be identical
    println!("\nComparing DFT vs FFT results:");
    for k in 0..n {
        let dft_mag =
            (dft_result[k].re * dft_result[k].re + dft_result[k].im * dft_result[k].im).sqrt();
        let fft_mag =
            (fft_result[k].re * fft_result[k].re + fft_result[k].im * fft_result[k].im).sqrt();

        println!(
            "Frequency {:1}: DFT magnitude={:8.3}  FFT magnitude={:8.3}",
            k, dft_mag, fft_mag
        );
    }

    println!("\nNotice the peaks at frequencies 1 and 3 (and their aliases).");
    println!("These correspond to the two sinusoids in our original signal!");
    println!();

    // Performance comparison with larger input
    println!("--- PERFORMANCE COMPARISON ---");
    let large_n = 256;
    let mut large_signal = Vec::with_capacity(large_n);
    for i in 0..large_n {
        large_signal.push(Complex::new(i as f64, 0.0));
    }

    // Count operations instead of timing (for deterministic comparison)
    let dft_ops = large_n * large_n;
    let fft_ops = large_n * (large_n as i32).ilog2() as usize;

    println!("For n = {}:", large_n);
    println!("  Direct DFT operations: {}", dft_ops);
    println!("  FFT operations:        {}", fft_ops);
    println!(
        "  Speedup factor:        {:.0}x",
        dft_ops as f64 / fft_ops as f64
    );

    println!("\nThe FFT made digital signal processing practical!");
    println!("Applications: MP3 compression, WiFi, MRI imaging, astronomy...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_vs_dft() {
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let dft_result = dft_direct(&input);
        let fft_result = fft(&input);

        for (dft_val, fft_val) in dft_result.iter().zip(fft_result.iter()) {
            assert!((dft_val.re - fft_val.re).abs() < 1e-10);
            assert!((dft_val.im - fft_val.im).abs() < 1e-10);
        }
    }

    #[test]
    fn test_fft_peaks() {
        // Signal with known frequency components should show peaks
        let n = 8;
        let mut signal = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f64 / n as f64;
            let value = (2.0 * PI * 2.0 * t).sin(); // Frequency 2
            signal.push(Complex::new(value, 0.0));
        }

        let result = fft(&signal);
        // Should have peak at frequency 2 and n-2=6
        let mag_2 = (result[2].re * result[2].re + result[2].im * result[2].im).sqrt();
        let mag_6 = (result[6].re * result[6].re + result[6].im * result[6].im).sqrt();

        assert!(mag_2 > 1.0); // Peak present
        assert!(mag_6 > 1.0); // Alias peak present
    }
}
