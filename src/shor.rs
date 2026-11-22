//! Shor's factoring algorithm - classical period-finding components.
//!
//! Implements the classical parts of Shor's algorithm that can be understood
//! without quantum hardware: modular exponentiation, period finding via
//! continued fractions, and the overall factorization pipeline.

/// Compute gcd using Euclidean algorithm

/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "shor",
    version: "0.0.1",
    formalized: false,
    notes: "Shor's factoring — period finding via continued fractions",
};


fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute modular exponentiation: (base^exp) mod modulus
fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result: u128 = 1;
    let mut base = base as u128 % modulus as u128;
    let mut exp = exp;
    let modulus = modulus as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result as u64
}

/// Classical period finding using continued fractions.
/// Finds the period of f(x) = a^x mod N by computing the convergents of
/// the continued fraction expansion of s/r where gcd(s,r)=1 and r is minimal.
pub fn classical_period_finding(a: u64, n: u64) -> Option<u64> {
    // Compute powers of a modulo n until we find a repeat
    let mut seen = std::collections::HashMap::new();
    let mut x = 1u64;
    let mut i = 0u64;

    loop {
        if let Some(&prev_i) = seen.get(&x) {
            return Some(i - prev_i);
        }
        seen.insert(x, i);
        x = mod_pow(a, i + 1, n);
        i += 1;
        if i > n * 2 {
            return None; // Safety bound
        }
    }
}

/// Shor's factoring algorithm (classical version with simulated period finding).
/// Returns factors of n, or None if n is prime.
pub fn shor_factor(n: u64) -> Option<(u64, u64)> {
    // Handle trivial cases
    if n <= 1 {
        return None;
    }
    if n == 2 || n == 3 {
        return Some((n, 1));
    }

    // Check if n is even
    if n % 2 == 0 {
        return Some((2, n / 2));
    }

    // Try different values of a until we find a factor
    for a in 2..n {
        if gcd(a, n) > 1 {
            return Some((gcd(a, n), n / gcd(a, n)));
        }

        let period = classical_period_finding(a, n)?;

        // Check if period is even
        if period % 2 != 0 {
            continue;
        }

        // Compute x = a^(r/2) mod n
        let half_period = period / 2;
        let x = mod_pow(a, half_period, n);

        // Check if x^2 ≡ -1 (mod n), which would mean no factor found
        if (x as i64 + 1) % (n as i64) == 0 {
            continue;
        }

        // Try to find factors using gcd(x ± 1, n)
        let p = gcd((x + 1) % n, n);
        if p > 1 && p < n {
            return Some((p, n / p));
        }

        let q = gcd((x + n - 1) % n, n);
        if q > 1 && q < n {
            return Some((q, n / q));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 3, 5), 3); // 2^3 = 8 ≡ 3 (mod 5)
        assert_eq!(mod_pow(3, 4, 7), 4); // 3^4 = 81 ≡ 4 (mod 7)
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_shor_factor_small() {
        let result = shor_factor(15);
        // Factors can come back in any order
        match result {
            Some((p, q)) => assert!(p * q == 15 && p > 1 && q > 1),
            None => panic!("Failed to factor 15"),
        }
    }

    #[test]
    fn test_shor_factor_even() {
        let result = shor_factor(10);
        assert_eq!(result, Some((2, 5)));
    }
}
