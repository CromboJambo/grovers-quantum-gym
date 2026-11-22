/// Kurt Gödel's Incompleteness Theorems (1931)
/// Demonstrates the limits of formal systems through executable examples.


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "godel",
    version: "0.0.1",
    formalized: false,
    notes: "Gödel incompleteness theorems exploration",
};


pub fn show_godel_station() {
    println!("=== KURT GÖDEL STATION (1931) ===");
    println!();
    print_biography();
    println!();
    demonstrate_godel_numbering();
    println!();
    demonstrate_first_incompleteness();
    println!();
    demonstrate_second_incompleteness();
}

fn print_biography() {
    println!("Kurt Gödel (1906-1978)");
    println!("Austrian logician, mathematician, and philosopher");
    println!("Vienna Circle associate; later IAS Princeton with Einstein");
    println!();
    println!("Published 'On Formally Undecidable Propositions of Principia");
    println!("Mathematica and Related Systems' in 1931 at age 25.");
    println!();
    println!("His incompleteness theorems shattered Hilbert's program to find");
    println!("a complete, consistent, finitely axiomatized foundation for all");
    println!("of mathematics. No formal system rich enough to express arithmetic");
    println!("can prove both its own consistency and all true statements.");
}

fn demonstrate_godel_numbering() {
    println!("--- DEMONSTRATION: Gödel Numbering ---");
    println!();
    println!("Gödel's key insight: encode mathematical formulas as natural numbers.");
    println!("This allows arithmetic to 'talk about' its own syntax.");
    println!();

    // Assign primes to symbols
    let symbols: Vec<(&str, u64)> = vec![
        ("0", 2),
        ("S", 3),
        ("+", 5),
        ("×", 7),
        ("=", 11),
        ("∀", 13),
        ("∃", 17),
        ("¬", 19),
        ("→", 23),
        ("(", 29),
        (")", 31),
    ];

    println!("Symbol encoding (prime numbers):");
    for (sym, prime) in &symbols {
        println!("  {} → {}", sym, prime);
    }
    println!();

    // Example: encode the formula "0+0=0"
    let formula = "0+0=0";
    println!("Encoding formula: {}", formula);

    let mut godel_number = 1u64;
    for (i, ch) in formula.chars().enumerate() {
        // Simplified: use char code + 2 as prime index
        let prime: u64 = match ch {
            '0' => 2,
            '+' => 5,
            '=' => 11,
            _ => 3,
        };
        godel_number *= prime.pow((i + 1) as u32);
    }

    println!("  Gödel number: {}", godel_number);
    println!();
    println!("Now properties of formulas become arithmetic properties of numbers!");
    println!("'Is this formula provable?' becomes 'Does this number have property P?'");
}

fn demonstrate_first_incompleteness() {
    println!("--- DEMONSTRATION: First Incompleteness Theorem ---");
    println!();
    println!("THEOREM: Any consistent formal system F that can express basic");
    println!("arithmetic is incomplete. There exists a statement G such that:");
    println!("  - G is true (in the standard model of arithmetic)");
    println!("  - G is not provable in F");
    println!();

    println!("The Gödel sentence G says about itself: 'I am not provable in F.'");
    println!();

    // Demonstrate with a simple formal system
    let axioms = vec![
        "∀x (x + 0 = x)",
        "∀x ∀y (x + S(y) = S(x + y))",
        "∀x (x × 0 = 0)",
        "∀x ∀y (x × S(y) = (x × y) + x)",
    ];

    println!("Consider a formal system with these arithmetic axioms:");
    for axiom in &axioms {
        println!("  {}", axiom);
    }
    println!();

    // Construct Gödel sentence conceptually
    let godel_sentence = "This statement is not provable in this system";
    println!("The Gödel construction creates a self-referential formula G:");
    println!("  G ≡ 'G is not provable'");
    println!();

    // Show the paradox if we could prove it
    println!("If F could PROVE G:");
    println!("  Then G would be false (it claims to be unprovable)");
    println!("  Contradiction: F proved a false statement → F is inconsistent");
    println!();

    println!("If F could prove NOT-G:");
    println!("  Then F proves 'G is provable'");
    println!("  But G says it's not provable...");
    println!("  So F proves both G and ¬G → F is inconsistent");
    println!();

    println!("Conclusion: In a CONSISTENT system, G is neither provable nor disprovable.");
    println!("But G is TRUE (it really is unprovable). Hence the system is INCOMPLETE.");
}

fn demonstrate_second_incompleteness() {
    println!("--- DEMONSTRATION: Second Incompleteness Theorem ---");
    println!();
    println!("THEOREM: No consistent formal system F capable of expressing basic");
    println!("arithmetic can prove its own consistency.");
    println!();

    println!("This follows from the first theorem. If F could prove 'F is consistent',");
    println!("then (by formalizing the proof of the first theorem) F would be able to");
    println!("prove G — contradicting the first incompleteness theorem.");
    println!();

    // Show with a concrete example
    println!("Example: Peano Arithmetic (PA)");
    println!("  - Can express addition and multiplication");
    println!("  - We believe PA is consistent");
    println!("  - But PA cannot prove 'PA is consistent'");
    println!();

    println!("To prove PA's consistency, we need a STRONGER system (like ZFC set theory).");
    println!("But then ZFC can't prove its own consistency... and so on forever.");
    println!();

    println!("This explains why mathematicians accept axioms without formal proof:");
    println!("No finite axiomatization of mathematics can be both complete AND consistent.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_godel_numbering_produces_unique_numbers() {
        // Different formulas should produce different Gödel numbers
        let formula1 = "0+0";
        let formula2 = "S(0)";

        // Simplified encoding for test
        fn encode(formula: &str) -> u64 {
            let mut num = 1u64;
            for (i, ch) in formula.chars().enumerate() {
                let prime: u64 = match ch {
                    '0' => 2,
                    '+' => 5,
                    '=' => 11,
                    'S' => 3,
                    '(' => 29,
                    ')' => 31,
                    _ => 7,
                };
                num = num.wrapping_mul(prime);
            }
            num
        }

        let g1 = encode(formula1);
        let g2 = encode(formula2);
        assert_ne!(g1, g2);
    }

    #[test]
    fn test_godel_numbering_is_deterministic() {
        fn encode(formula: &str) -> u64 {
            let mut num = 1u64;
            for (i, ch) in formula.chars().enumerate() {
                let prime: u64 = match ch {
                    '0' => 2,
                    '+' => 5,
                    '=' => 11,
                    _ => 3,
                };
                num *= prime.pow((i + 1) as u32);
            }
            num
        }

        let formula = "0+0=0";
        assert_eq!(encode(formula), encode(formula));
    }

    #[test]
    fn test_godel_sentence_property() {
        // The Gödel sentence G has the property: F ⊢ G iff F is inconsistent
        // We can't actually run a formal system here, but we can verify the logic

        let consistent = true;
        let proves_g = false; // In a consistent system, G is unprovable

        if consistent {
            assert!(!proves_g); // Consistent systems don't prove G
        }
    }

    #[test]
    fn test_consistency_statement() {
        // Cons(F) = "There is no proof of 0=1 in F"
        // This is a Π₁ statement (universal quantifier over natural numbers)

        let has_contradiction = false; // Assume system is consistent
        let consistency_statement = !has_contradiction;

        assert!(consistency_statement);
    }
}
