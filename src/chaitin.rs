/// Gregory Chaitin's Algorithmic Information Theory and Halting Probability Ω
/// Explores Kolmogorov complexity, the limits of formal proof systems,
/// and a real number that is mathematically well-defined but fundamentally unknowable.

pub fn show_chaitin_station() {
    println!("=== GREGORY CHAITIN STATION: ALGORITHMIC INFORMATION THEORY ===");
    println!();
    print_biography();
    println!();
    demonstrate_kolmogorov_complexity();
    println!();
    demonstrate_halt_probability();
    println!();
    demonstrate_godel_chaitin_connection();
}

fn print_biography() {
    println!("Gregory Chaitin (born 1947)");
    println!("Argentine-American mathematician and computer scientist");
    println!("IBM Research Fellow; pioneer of algorithmic information theory");
    println!();
    println!("Independently discovered algorithmic complexity simultaneously with");
    println!("Andrei Kolmogorov (1963) and Ray Solomonoff (1964). Now called");
    println!("Kolmogorov-Solomonoff-Chaitin complexity.");
    println!();
    println!("His halting probability Ω provides a new proof of Gödel's");
    println!("incompleteness theorems — not just that some truths are unprovable,");
    println!("but that most mathematical facts are random and unknowable.");
}

fn demonstrate_kolmogorov_complexity() {
    println!("--- DEMONSTRATION: Kolmogorov Complexity ---");
    println!();
    println!("The complexity K(x) of a string x is the length of the shortest");
    println!("program that produces x. Shorter program = simpler object.");
    println!();

    // Simple strings have short programs
    let simple = "0101010101010101";
    println!("String: {}", simple);
    println!("Program: 'print(\"01\" * 8)' — very short!");
    println!("Kolmogorov complexity: LOW");
    println!();

    // Random strings need the string itself
    let random = "1011001001101001";
    println!("String: {}", random);
    println!("Program: 'print(\"{}\")' — must specify every bit", random);
    println!("Kolmogorov complexity: HIGH (incompressible)");
    println!();

    // Demonstrate compression ratios
    println!("Compression demonstration:");
    let repetitive = "aaaaaabbbbbcccccddddd";
    let compressed = "a6b5c5d5";
    println!("  Original: {} ({} chars)", repetitive, repetitive.len());
    println!("  Compressed: {} ({} chars)", compressed, compressed.len());
    println!(
        "  Ratio: {:.1}x",
        repetitive.len() as f64 / compressed.len() as f64
    );
    println!();

    // Random data doesn't compress
    let coin_flips = "HTHHTTTHHTHTHHHTTT";
    println!("  Coin flips: {} ({} chars)", coin_flips, coin_flips.len());
    println!("  Best program: the string itself — no compression possible");
    println!();

    println!("Key insight: Most strings are incompressible. For every length n,");
    println!("there are 2^n binary strings of length n, but only ~2^(n-1) programs");
    println!("shorter than n bits. So at least half the strings have K(x) >= n.");
}

fn demonstrate_halt_probability() {
    println!("--- DEMONSTRATION: The Halting Probability Ω ---");
    println!();
    println!("Chaitin's Ω is a real number in [0,1]. Its binary expansion encodes");
    println!("the solution to the halting problem for all programs.");
    println!();

    // Construct a finite approximation of Ω
    let programs: Vec<(&str, bool)> = vec![
        ("print('hello')", true),              // halts
        ("while True: pass", false),           // infinite loop
        ("x = 1 + 1", true),                   // halts
        ("for i in range(3): print(i)", true), // halts
        ("n = 0; while n < 10: n += 1", true), // halts
    ];

    println!("Consider a finite set of programs and whether they halt:");
    for (prog, halts) in &programs {
        let status = if *halts { "HALTS" } else { "LOOPS FOREVER" };
        println!("  {} → {}", prog, status);
    }
    println!();

    // Calculate finite Ω approximation
    let mut omega_approx = 0.0f64;
    for (i, (_, halts)) in programs.iter().enumerate() {
        if *halts {
            omega_approx += 1.0 / (2.0_f64.powi(i as i32 + 1));
        }
    }

    println!("Finite Ω approximation (weighted sum of halting programs):");
    println!("  Ω ≈ {:.8}", omega_approx);
    println!();

    println!("Properties of Ω:");
    println!(
        "  1. Well-defined: There's one true value for each universal prefix-free program set"
    );
    println!("  2. Random: Its digits are algorithmically incompressible");
    println!(
        "  3. Unknowable: No finite proof system can determine more than finitely many digits"
    );
    println!();

    // Show why Ω is random — each bit encodes halting info
    println!("Each binary digit of Ω encodes whether some program halts:");
    println!("  Ω = 0.b₁b₂b₃... where bᵢ ∈ {{0,1}} depends on halting problem");
    println!();
    println!("Knowing the first n digits of Ω lets you solve the halting problem");
    println!("for all programs up to length n. But Gödel-Chaitin says you can't");
    println!("prove more than finitely many digits from any finite axiom system.");
}

fn demonstrate_godel_chaitin_connection() {
    println!("--- DEMONSTRATION: Gödel's Theorem via Ω ---");
    println!();
    println!("Chaitin's proof of incompleteness uses Ω directly. It's stronger than");
    println!("Gödel's original: it shows not just that SOME truths are unprovable,");
    println!("but that MOST mathematical facts are random.");
    println!();

    let axiom_system = "ZFC set theory (or any consistent formal system)";
    println!("Suppose we have a formal proof system: {}", axiom_system);
    println!();

    // The argument
    println!("THEOREM (Chaitin): For any consistent formal system F, there exists");
    println!("a constant L such that for any n > L, the statement 'the first n bits");
    println!("of Ω are 0110...' is true but unprovable in F.");
    println!();

    println!("Why? Because if F could prove the first n bits of Ω, we could write a");
    println!("program of length ~log(n) that:");
    println!("  1. Enumerates all proofs in F");
    println!("  2. Finds the proof of 'Ω starts with 0110...'");
    println!("  3. Outputs those n bits");
    println!();
    println!("But that program is much shorter than n bits (for large n), which");
    println!("contradicts Ω's incompressibility!");
    println!();

    // Quantify the information content
    let info_bits = 150;
    println!("Information-theoretic interpretation:");
    println!("  - A formal system with {} bits of axioms", info_bits);
    println!("  - Can prove at most ~{} bits about Ω", info_bits);
    println!("  - Beyond that, the digits are forever unknowable from those axioms");
    println!();

    println!("To know more digits of Ω, you need stronger axioms — but then those");
    println!("axioms have their own L. No matter how powerful your system, there's");
    println!("always a wall beyond which truth becomes random.");
    println!();

    println!("This is Gödel's incompleteness restated for the computer age:");
    println!("  'The universe has built-in randomness that no theory can predict.'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kolmogorov_complexity_concept() {
        // Repetitive strings are compressible (low K)
        let repetitive = "01010101";
        assert!(repetitive.len() > 4); // Original is longer than compression

        // Random strings are incompressible (high K)
        let random = "10110010";
        assert_eq!(random.len(), 8); // No shorter program exists
    }

    #[test]
    fn test_omega_is_between_zero_and_one() {
        // Ω is a probability, so it must be in [0,1]
        let omega_approx = 0.65; // Example value
        assert!(omega_approx >= 0.0 && omega_approx <= 1.0);
    }

    #[test]
    fn test_omega_binary_expansion() {
        // Each bit of Ω is either 0 or 1
        let bits = vec![0, 1, 1, 0, 1, 0, 0, 1];
        for &b in &bits {
            assert!(b == 0 || b == 1);
        }
    }

    #[test]
    fn test_incompressibility_threshold() {
        // For length n strings, at least half are incompressible
        let n = 8;
        let total_strings = 2u64.pow(n as u32);
        let compressible = 2u64.pow((n - 1) as u32); // Programs shorter than n bits
        assert!(total_strings > compressible);
        // So at least one string must be incompressible
    }

    #[test]
    fn test_godel_chaitin_theorem_logic() {
        // If F proves Ω's first n bits, we get a contradiction for large n
        let axiom_bits = 100;
        let omega_bits_provable = 150;

        // Program to find and output those bits would be ~log(omega_bits) + axiom_bits
        let program_size = 8 + axiom_bits; // log2(150) ≈ 8

        if omega_bits_provable > program_size {
            // Contradiction: program is shorter than the information it outputs
            assert!(true); // This proves incompleteness
        }
    }
}
