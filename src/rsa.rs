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


/// Speculate on what an AI-driven cryptanalysis breakthrough would look like,
/// based on the November 2026 momentum and observed patterns.
pub fn demonstrate_breakthrough_scenario() {
    println!("\n--- WHAT WOULD A BREAKTHROUGH LOOK LIKE? (Nov 2026 assessment) ---");
    println!();
    println!("Based on the observed pattern of AI math results, here's what a");
    println!("cryptanalysis breakthrough would likely look like:\n");

    println!("PATTERN RECOGNITION FROM NOVEMBER 2026 CLUSTER:");
    println!("  1. Results are ASYMPTOTIC improvements, not magic bullets");
    println!("     - omega <= 9/4 (improvement over 2.373, still not O(n^2))");
    println!("     - L_N[1/3, O(1)] factoring (same class as GNFS, better constants)");
    println!("     - VP != VNP (separation, but doesn't give algorithms)");
    println!();
    println!("  2. Everything ships with Lean certificates (machine-verified proofs)");
    println!("     - No 'trust me' papers — the proof IS the artifact");
    println!("     - Independent verification by Lean community is the trust anchor");
    println!();
    println!("  3. Results land on GitHub repos, not journal announcements");
    println!("     - openai/math, anthropics/formal-math, Palomar registry");
    println!("     - Quiet publication: code + proof + comparator, no press release");
    println!();

    println!("MOST LIKELY BREAKTHROUGH SCENARIOS (ranked by probability):\n");

    println!("SCENARIO A: Improved factoring asymptotics (P ~ 60%)");
    println!("  What: New algorithm beating L_N[1/3, O(1)] — maybe L_N[1/c] for c > 3,");
    println!("        or polylog factors that make it practical at smaller key sizes.");
    println!("  Why likely: AI excels at finding structure in algebraic number theory.");
    println!("              The VP!=VNP proof used unexpected algebraic geometry connections.");
    println!("              A similar 'aha' moment for factoring is the natural next step.");
    println!("  Impact: RSA-2048 remains safe for now, but key sizes need to grow.");
    println!("          Migration timeline compresses from decades to years.");
    println!();

    println!("SCENARIO B: Discrete log breakthrough (P ~ 25%)");
    println!("  What: Subexponential algorithm for ECDLP or DLP with better constants.");
    println!("  Why likely: Aaronson specifically mentioned this as rumored. ECC is used");
    println!("              in Bitcoin, TLS, and most modern protocols — higher stakes.");
    println!("              If labs have this, it's the 'bombshell' Aaronson alludes to.");
    println!("  Impact: MORE DANGEROUS than RSA factoring break. Bitcoin, Signal,");
    println!("          TLS certificates all rely on ECDLP. No easy migration path.");
    println!();

    println!("SCENARIO C: Unexpected reduction (P ~ 10%)");
    println!("  What: Proof that factoring/DLP reduces to a problem already in P,");
    println!("        or a new complexity class collapse (like P=NP but narrower).");
    println!("  Why less likely: These are 'holy grail' results. AI hasn't shown");
    println!("                   this level of conceptual leap yet — November cluster");
    println!("                   was all about optimization within known frameworks.");
    println!("  Impact: Existential for public-key crypto. Total migration required.");
    println!();

    println!("SCENARIO D: Number theory enabler (P ~ 5%)");
    println!("  What: RH proof or related result that enables efficient algorithms");
    println!("        (e.g., deterministic poly-time primality with better bounds,");
    println!("        or new sieve methods).");
    println!("  Why least likely: RH formalization is happening (Alpöge/Anthropic),");
    println!("                    but proving it is different from applying it.");
    println!("  Impact: Indirect — improves all of number theory, crypto benefits later.");
    println!();

    println!("WHAT TO WATCH FOR (in order of appearance):\n");
    println!("  1. A new repo or directory in openai/math or anthropics/formal-math");
    println!("     with a Challenge.lean / Solution.lean pair about factoring or DLP");
    println!("  2. Aaronson blog post saying 'the paper has landed' (his pattern)");
    println!("  3. Lean community verification discussion on Zulip/GitHub");
    println!("  4. Nature/Science news article (always lagging by weeks)");
    println!("  5. Bitcoin dormant wallets moving (if ECC is affected) — OUR MONITOR");
    println!();

    println!("TIMELINE IMPLICATIONS FOR DEPLOYED CRYPTO:\n");
    println!("  Aaronson's key point: 'would not endanger any currently deployed");
    println!("  cryptosystem' immediately. Why?\n");
    println!("  - Asymptotic improvements need LARGE inputs to matter");
    println!("  - L_N[1/3, O(1)] at RSA-2048 still takes ~10^15 operations");
    println!("  - Practical break requires BOTH better asymptotics AND engineering");
    println!("  - Migration is slow: PKI, hardware tokens, embedded systems take years");
    println!();
    println!("  BUT: The window between 'theoretical break' and 'practical break'");
    println!("  is where the real danger lies. Harvest-now-decrypt-later attacks");
    println!("  become viable immediately once a theoretical path exists.");
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
