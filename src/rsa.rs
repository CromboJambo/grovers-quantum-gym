/// RSA ENCRYPTION STATION (1977)
///
/// Rivest-Shamir-Adleman public-key cryptography.
/// Security based on difficulty of factoring large numbers —
/// which is why Shor's quantum algorithm threatens it!
use std::collections::HashSet;

/// Compute modular exponentiation: (base^exp) mod modulus
fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1u64;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128) as u64 % modulus;
        }
        exp >>= 1;
        if exp > 0 {
            base = (base as u128 * base as u128) as u64 % modulus;
        }
    }

    result
}

/// Extended Euclidean algorithm for modular inverse
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let mut m0 = m;
    let mut y = 0i64;
    let mut x = 1i64;

    if m.abs() < 2 {
        return None;
    }

    let mut a = a;
    let mut m = m;

    while a.abs() > 1 {
        let q = a / m;
        let new_m = a - q * m;
        a = m;
        m = new_m;

        let new_y = x - q * y;
        x = y;
        y = new_y;
    }

    if x < 0 {
        x += m0;
    }
    Some(x)
}

pub fn demonstrate_rsa() {
    println!("--- RSA ENCRYPTION (Rivest-Shamir-Adleman, 1977) ---");
    println!();
    println!("RSA uses two keys: public (for encryption) and private (for decryption).");
    println!("Security relies on the difficulty of factoring large numbers.");
    println!();

    // Step 1: Choose two prime numbers
    let p = 61;
    let q = 53;
    println!("Step 1: Choose two primes: p={}, q={}", p, q);

    // Step 2: Compute n = p * q
    let n = p * q;
    println!("Step 2: Compute n = p*q = {}", n);

    // Step 3: Compute Euler's totient function φ(n) = (p-1)(q-1)
    let phi = (p - 1) * (q - 1);
    println!("Step 3: Compute φ(n) = (p-1)(q-1) = {}", phi);

    // Step 4: Choose public exponent e coprime to φ(n)
    let e = 17;
    println!("Step 4: Choose public exponent e={} (coprime to φ(n))", e);

    // Step 5: Compute private exponent d such that e*d ≡ 1 (mod φ(n))
    let d = mod_inverse(e as i64, phi as i64).unwrap() as u64;
    println!(
        "Step 5: Compute private exponent d={} where e*d ≡ 1 (mod φ(n))",
        d
    );

    println!("\nPublic key:  (e={}, n={})", e, n);
    println!("Private key: (d={}, n={})", d, n);
    println!();

    // Encrypt a message
    let plaintext = 42u64;
    println!("Encrypting message: {}", plaintext);
    let ciphertext = mod_pow(plaintext, e, n as u64);
    println!("Ciphertext: {}", ciphertext);

    // Decrypt the message
    let decrypted = mod_pow(ciphertext, d, n as u64);
    println!("Decrypted:  {}", decrypted);
    println!();

    if plaintext == decrypted {
        println!("✓ Encryption/decryption successful!");
    } else {
        println!("✗ Error in encryption/decryption");
    }

    println!("\n--- WHY SHOR'S ALGORITHM MATTERS ---");
    println!("RSA security depends on factoring n=p*q being hard.");
    println!("For classical computers, this takes exponential time.");
    println!("But Shor's quantum algorithm can factor in polynomial time!");
    println!("A large enough quantum computer could break RSA encryption.");
}

pub fn demonstrate_prime_factorization_difficulty() {
    println!("\n--- FACTORIZATION DIFFICULTY ---");

    // Small example that we can factor easily
    let small_n = 15;
    println!("Factoring {} (easy):", small_n);
    for i in 2..=small_n {
        if small_n % i == 0 {
            println!("  {} × {}", i, small_n / i);
            break;
        }
    }

    // Larger example that's harder
    let large_n = 187;
    println!("Factoring {} (harder):", large_n);
    for i in 2..=large_n {
        if large_n % i == 0 {
            println!("  {} × {}", i, large_n / i);
            break;
        }
    }

    // Real RSA uses numbers with hundreds of digits
    println!("\nReal RSA uses primes with ~1024 bits (309 digits)!");
    println!("The largest number ever factored has 829 digits.");
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
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7), Some(5)); // 3*5 = 15 ≡ 1 (mod 7)
        assert_eq!(mod_inverse(4, 9), Some(7)); // 4*7 = 28 ≡ 1 (mod 9)
    }

    #[test]
    fn test_rsa_encrypt_decrypt() {
        let p = 61;
        let q = 53;
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let e = 17;
        let d = mod_inverse(e as i64, phi as i64).unwrap() as u64;

        let message = 42u64;
        let encrypted = mod_pow(message, e, n);
        let decrypted = mod_pow(encrypted, d, n);

        assert_eq!(message, decrypted);
    }
}
