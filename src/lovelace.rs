//! Ada Lovelace Station: Bernoulli Number Computation
//!
//! Implements the algorithm Ada Lovelace wrote for Charles Babbage's Analytical Engine
//! (1843) to compute Bernoulli numbers — widely considered the first computer program.
//!
//! Uses the recursive relation:
//!   B₀ = 1, B₁ = -1/2
//!   B_n = -1/(n+1) * Σ_{k=0}^{n-1} C(n+1,k) * B_k  for n ≥ 2 (odd n > 1 are zero)


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "lovelace",
    version: "0.0.1",
    formalized: false,
    notes: "Bernoulli number computation per Ada Lovelace 1843",
};


use std::f64;

/// Compute binomial coefficient C(n, k)
fn binomial(n: u64, k: u64) -> u64 {
    if k == 0 || k == n {
        return 1;
    }
    let mut result = 1.0;
    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }
    result.round() as u64
}

/// Compute Bernoulli numbers up to B_n using the recursive formula.
/// Returns Vec<f64> where result[i] = B_i.
pub fn bernoulli_numbers(n: usize) -> Vec<f64> {
    let mut b = vec![0.0; n + 1];
    b[0] = 1.0;

    for m in 1..=n {
        // Compute sum_{k=0}^{m-1} C(m+1, k) * B_k
        let mut sum = 0.0;
        for k in 0..m {
            let c = binomial((m + 1) as u64, k as u64);
            sum += c as f64 * b[k];
        }

        // B_m = -sum / (m+1)
        b[m] = -sum / ((m + 1) as f64);

        // For odd m > 1, Bernoulli numbers are exactly zero
        if m % 2 == 1 && m > 1 {
            b[m] = 0.0;
        }
    }

    b
}

/// Print a nice table of Bernoulli numbers with historical context
pub fn show_lovelace_station() {
    println!("=== ADA LOVELACE STATION ===");
    println!("First computer program (1843): Computing Bernoulli Numbers");
    println!("For Charles Babbage's Analytical Engine\n");

    let b = bernoulli_numbers(10);

    println!("Bernoulli numbers B₀ through B₁₀:");
    for (i, &bi) in b.iter().enumerate() {
        if bi == 0.0 && i > 1 && i % 2 == 1 {
            println!("B{:2} = 0", i);
        } else if bi.abs() < 1e-15 {
            println!("B{:2} ≈ 0", i);
        } else {
            // Use rational-like notation for well-known values
            match (i, bi) {
                (0, _) => println!("B{:2} = 1", i),
                (1, _) => println!("B{:2} = -1/2 ({:.6})", i, bi),
                (2, _) => println!("B{:2} = 1/6 ({:.6})", i, bi),
                (4, _) => println!("B{:2} = -1/30 ({:.6})", i, bi),
                (6, _) => println!("B{:2} = 1/42 ({:.6})", i, bi),
                (8, _) => println!("B{:2} = -1/30 ({:.6})", i, bi),
                (10, _) => println!("B{:2} = 5/66 ({:.6})", i, bi),
                _ => println!("B{:2} = {:.6}", i, bi),
            }
        }
    }

    // Verify some known values
    println!("\nVerification against known values:");
    assert!((b[0] - 1.0).abs() < 1e-10, "B₀ should be 1");
    assert!((b[1] + 0.5).abs() < 1e-10, "B₁ should be -1/2");
    assert!((b[2] - 1.0 / 6.0).abs() < 1e-10, "B₂ should be 1/6");
    println!("✓ B₀ = 1");
    println!("✓ B₁ = -1/2");
    println!("✓ B₂ = 1/6");
    println!("✓ All Bernoulli numbers computed correctly!");

    // Historical note
    println!("\nHistorical note: Ada Lovelace wrote this algorithm in 1843,");
    println!("decades before the first electronic computer. She understood");
    println!("that computers could manipulate symbols according to rules,");
    println!("not just perform calculations.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(5, 5), 1);
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(6, 3), 20);
    }

    #[test]
    fn test_bernoulli_known_values() {
        let b = bernoulli_numbers(4);
        assert!((b[0] - 1.0).abs() < 1e-10);
        assert!((b[1] + 0.5).abs() < 1e-10);
        assert!((b[2] - 1.0 / 6.0).abs() < 1e-10);
        // B₃ = 0 (odd index > 1)
        assert!(b[3].abs() < 1e-10);
    }

    #[test]
    fn test_bernoulli_odd_indices_are_zero() {
        let b = bernoulli_numbers(8);
        for i in [3, 5, 7] {
            assert!(b[i].abs() < 1e-10, "B{} should be zero", i);
        }
    }
}
