//! Integer Structure Module
//! Factorization, prime powers, squarefree classification, Möbius function.

/// Trial division factorization returning (prime, exponent) pairs
pub fn factorize(n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut divisor = 2;
    while divisor * divisor <= num {
        if num % divisor == 0 {
            let mut count = 0;
            while num % divisor == 0 {
                num /= divisor;
                count += 1;
            }
            factors.push((divisor, count));
        }
        divisor += 1;
    }
    if num > 1 {
        factors.push((num, 1));
    }
    factors
}

/// Check if n is prime using optimized trial division
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut divisor = 5;
    while divisor * divisor <= n {
        if n % divisor == 0 || n % (divisor + 2) == 0 {
            return false;
        }
        divisor += 6;
    }
    true
}

/// Generate primes up to limit using Sieve of Eratosthenes
pub fn sieve_primes(limit: usize) -> Vec<u64> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[p] {
            for multiple in (p * p)..=limit {
                if multiple % p == 0 {
                    is_prime[multiple] = false;
                }
            }
        }
    }
    let mut primes = Vec::new();
    for (i, &prime) in is_prime.iter().enumerate() {
        if prime {
            primes.push(i as u64);
        }
    }
    primes
}

/// Möbius function μ(n):
/// 1 if n=1, (-1)^k if n has k distinct squarefree prime factors, 0 otherwise
pub fn mobius(n: u64) -> i32 {
    if n == 1 {
        return 1;
    }
    let factors = factorize(n);
    // Check for squared prime factors
    for &(_, exp) in &factors {
        if exp > 1 {
            return 0;
        }
    }
    // Squarefree: return (-1)^k where k is number of distinct prime factors
    let k = factors.len();
    if k % 2 == 0 { 1 } else { -1 }
}

/// Check if n is squarefree (no squared prime factor)
pub fn is_squarefree(n: u64) -> bool {
    mobius(n) != 0
}

/// Count distinct prime factors ω(n)
pub fn omega(n: u64) -> usize {
    factorize(n).len()
}

/// Total number of prime factors counting multiplicity Ω(n)
pub fn big_omega(n: u64) -> usize {
    factorize(n).iter().map(|(_, exp)| *exp as usize).sum()
}

/// Check if n is a prime power p^k with k >= 2
pub fn is_prime_power(n: u64) -> bool {
    if n < 4 {
        return false;
    }
    let factors = factorize(n);
    factors.len() == 1 && factors[0].1 >= 2
}

/// Check if n is a semiprime (Ω(n) = 2)
pub fn is_semiprime(n: u64) -> bool {
    big_omega(n) == 2
}

/// Check if n is a perfect square
pub fn is_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

/// Largest prime factor of n
pub fn largest_prime_factor(n: u64) -> u64 {
    let factors = factorize(n);
    factors.last().map(|(p, _)| *p).unwrap_or(1)
}

/// Smallest prime factor of n
pub fn smallest_prime_factor(n: u64) -> u64 {
    let factors = factorize(n);
    factors.first().map(|(p, _)| *p).unwrap_or(1)
}

/// Classify integer by structural type (mutually exclusive for legacy use)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegerClass {
    One,
    Prime,
    PrimePower(u32),        // p^k where k > 1
    Semiprime,              // p*q where p≠q
    SquarefreeMulti(usize), // product of k distinct primes, k >= 3
    HasSquaredFactor,       // contains p^2 for some prime p
}

pub fn classify(n: u64) -> IntegerClass {
    if n == 1 {
        return IntegerClass::One;
    }
    let factors = factorize(n);
    if factors.len() == 1 {
        let (_, exp) = factors[0];
        if exp == 1 {
            return IntegerClass::Prime;
        } else {
            return IntegerClass::PrimePower(exp);
        }
    }
    // Multiple distinct prime factors
    let mut has_squared = false;
    for &(_, exp) in &factors {
        if exp > 1 {
            has_squared = true;
            break;
        }
    }
    if has_squared {
        return IntegerClass::HasSquaredFactor;
    }
    // Squarefree with multiple distinct primes
    if factors.len() == 2 {
        return IntegerClass::Semiprime;
    }
    IntegerClass::SquarefreeMulti(factors.len())
}

/// Count integers of each class in range [start, end]
pub fn classify_range(start: u64, end: u64) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for n in start..=end {
        match classify(n) {
            IntegerClass::One => *counts.entry("one".to_string()).or_insert(0) += 1,
            IntegerClass::Prime => *counts.entry("prime".to_string()).or_insert(0) += 1,
            IntegerClass::PrimePower(_) => {
                *counts.entry("prime_power".to_string()).or_insert(0) += 1
            }
            IntegerClass::Semiprime => *counts.entry("semiprime".to_string()).or_insert(0) += 1,
            IntegerClass::SquarefreeMulti(k) => {
                let key = format!("squarefree_{}", k);
                *counts.entry(key).or_insert(0) += 1;
            }
            IntegerClass::HasSquaredFactor => {
                *counts.entry("squared_factor".to_string()).or_insert(0) += 1
            }
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factorize(30), vec![(2, 1), (3, 1), (5, 1)]);
        assert_eq!(factorize(49), vec![(7, 2)]);
    }

    #[test]
    fn test_mobius() {
        assert_eq!(mobius(1), 1);
        assert_eq!(mobius(2), -1);
        assert_eq!(mobius(6), 1); // 2*3, two distinct primes
        assert_eq!(mobius(12), 0); // has squared factor (2^2)
        assert_eq!(mobius(30), -1); // 2*3*5, three distinct primes
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify(1), IntegerClass::One);
        assert_eq!(classify(7), IntegerClass::Prime);
        assert_eq!(classify(8), IntegerClass::PrimePower(3));
        assert_eq!(classify(6), IntegerClass::Semiprime);
        assert_eq!(classify(30), IntegerClass::SquarefreeMulti(3));
        assert_eq!(classify(12), IntegerClass::HasSquaredFactor);
    }

    #[test]
    fn test_sieve() {
        let primes = sieve_primes(20);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
