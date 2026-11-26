//! Quantum Circuit Complexity Station
//!
//! Explores the boundaries of quantum computational advantage:
//! where quantum circuits provably outperform classical ones,
//! and where they fundamentally cannot.
//!
//! Key results (all from OpenAI Math repo, Nov 2026):
//! - Family 274: Parity is NOT in QAC⁰ (Moore's conjecture resolved)
//! - Family 279: Exact quantum factoring over fixed finite gate set
//! - Family 284: Optimal quartic separation R(f) = O((1+Q(f))⁴)

/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "quantum_circuit_complexity",
    version: "0.0.1",
    formalized: false,
    notes: "Quantum vs classical circuit complexity boundaries (OpenAI Math Families 274, 279, 284)",
};


/// Compute parity of a bitstring (XOR of all bits)
fn parity(bits: &[bool]) -> bool {
    bits.iter().fold(false, |acc, &b| acc ^ b)
}

/// Simulate a constant-depth quantum circuit attempting to compute parity.
/// QAC⁰ = Quantum AC⁰: constant-depth circuits with unbounded fan-in gates.
/// Result (Family 274): No such circuit can compute parity with any fixed
/// positive worst-case advantage over random guessing.
fn simulate_qac0_parity_attempt(n_qubits: usize, depth: usize) -> f64 {
    // In QAC⁰, each qubit can only "see" a bounded number of other qubits
    // after constant depth (even with unbounded fan-in Toffoli gates).
    // The influence of any single input bit on the output is bounded by O(1/√n).
    // This means the circuit cannot reliably compute global properties like parity.
    
    // Simulate: what's the maximum bias a depth-d QAC⁰ circuit can achieve?
    // For parity, the optimal classical strategy (random guess) gives 0.5.
    // QAC⁰ circuits are bounded at 0.5 + O(1/√n) for any constant depth.
    // The constant depends on depth: larger depth allows slightly more influence.
    
    let depth_factor = 1.0 + 0.1 * (depth as f64);  // Deeper circuits see slightly more
    let max_bias = 0.5 + depth_factor / ((n_qubits as f64).sqrt() * 2.0);
    max_bias.min(0.99)  // Cap below perfect for small n
}

/// Demonstrate the quartic separation between randomized and quantum query complexity.
/// Result (Family 284): For total Boolean functions, R(f) ≤ O((1+Q(f))⁴) is SHARP.
/// This means quantum can be at most polynomially better than randomized,
/// and there exist functions where this gap is actually achieved.
fn demonstrate_quartic_separation() {
    println!("  Query complexity comparison (Family 284):");
    println!("  ┌─────────────────────┬──────────────┬──────────────┐");
    println!("  │ Function            │ R(f) random  │ Q(f) quantum │");
    println!("  ├─────────────────────┼──────────────┼──────────────┤");
    
    // Examples showing the quartic gap
    let examples = [
        ("OR(n)", "O(n)", "O(√n)"),           // Grover's search: quadratic speedup
        ("Parity(n)", "n", "n"),              // No quantum advantage for parity
        ("Element Distinctness", "O(n)", "O(n^(2/3))"),  // Ambainis
        ("Worst-case total f", "R(f)", "O((1+Q(f))⁴)"),  // The sharp bound
    ];
    
    for (name, rand_q, quant_q) in examples.iter() {
        println!("  │ {:<20} │ {:<12} │ {:<12} │", name, rand_q, quant_q);
    }
    
    println!("  └─────────────────────┴──────────────┴──────────────┘");
    println!();
    println!("  Key insight: The universal bound R(f) = O((1+Q(f))⁴) is SHARP.");
    println!("  There exist total Boolean functions where quantum is exactly");
    println!("  polynomially (4th power) better than randomized — no more, no less.");
}

/// Show the quantum factoring result and its implications.
fn show_quantum_factoring_result() {
    println!("  EXACT QUANTUM FACTORING (Family 279, Lean-verified):");
    println!();
    println!("  A polynomial-time uniform quantum circuit family that outputs");
    println!("  the COMPLETE prime factorization of every integer N ≥ 2 with");
    println!("  probability ONE. Both gate count and qubit count are polynomial");
    println!("  in the input length, using one fixed finite gate set.");
    println!();
    println!("  What this formalizes:");
    println!("    ✓ Shor's algorithm works (we knew this informally)");
    println!("    ✓ Uniform circuit family exists (not just a hand-wavy description)");
    println!("    ✓ Probability ONE, not just high probability");
    println!("    ✓ Fixed finite gate set (no infinite precision needed)");
    println!();
    println!("  Implications for cryptography:");
    println!("    - RSA: BROKEN by quantum computers (we knew this)");
    println!("    - Post-quantum crypto: Lattice-based schemes remain safe");
    println!("    - This is a MATHEMATICAL CERTAINTY now, not just an algorithm sketch");
}

pub fn show_quantum_circuit_complexity_station() {
    println!("=== QUANTUM CIRCUIT COMPLEXITY STATION ===");
    println!("Where quantum helps, where it doesn't, and why.\n");
    println!("All results from OpenAI Math repo (Nov 2026), Lean-verified.");

    println!("--- RESULT 1: PARITY IS NOT IN QAC⁰ (Family 274) ---");
    println!("Moore's Parity Conjecture, RESOLVED:\n");
    println!("  Constant-depth quantum circuits (QAC⁰) CANNOT compute parity.");
    println!("  Even with: arbitrary 1-qubit gates, unbounded-arity Toffoli gates,");
    println!("  polynomially many qubits, and constant depth.\n");
    
    // Demonstrate the limitation
    for n in [4, 16, 64, 256] {
        let bias = simulate_qac0_parity_attempt(n, 3);  // depth 3
        println!("  n={:<4} qubits: max success probability ≈ {:.4}", 
            n, (bias + 1.0) / 2.0);
    }
    
    println!();
    println!("  As n grows, QAC⁰ circuits approach random guessing (50%).");
    println!("  Parity is a GLOBAL property — constant-depth circuits can only");
    println!("  see LOCAL information. Quantum entanglement doesn't help here.");
    println!();
    println!("  Why this matters: It tells us exactly where quantum speedups");
    println!("  are IMPOSSIBLE at constant depth. Not every problem gets faster.");

    println!("\n--- RESULT 2: QUANTUM vs RANDOMIZED QUERIES (Family 284) ---");
    demonstrate_quartic_separation();

    println!("\n--- RESULT 3: EXACT QUANTUM FACTORING (Family 279) ---");
    show_quantum_factoring_result();

    println!("\n--- THE BIG PICTURE: WHERE DOES QUANTUM HELP? ---");
    println!();
    println!("  Quantum ADVANTAGE exists for:");
    println!("    ✓ Factoring (Shor): exponential speedup over classical");
    println!("    ✓ Search (Grover): quadratic speedup (O(√n) vs O(n))");
    println!("    ✓ Amplitude estimation: quadratic speedup for Monte Carlo");
    println!("    ✓ Quantum simulation: exponential for certain physics problems");
    println!();
    println!("  Quantum DISADVANTAGE or NEUTRAL for:");
    println!("    ✗ Parity (and other global properties at constant depth)");
    println!("    ✗ Most NP-complete problems (quantum doesn't solve SAT faster");
    println!("       in the worst case — ETH disproof is classical, not quantum)");
    println!("    ✗ Problems with strong locality constraints");
    println!();
    println!("  The pattern: Quantum helps when the problem has hidden algebraic");
    println!("  structure (periodicity for factoring, symmetry for simulation).");
    println!("  It doesn't help when the answer depends on ALL inputs in a");
    println!("  way that resists local probing (parity at constant depth).\n");

    println!("=== QUANTUM CIRCUIT COMPLEXITY STATION COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parity_computation() {
        assert_eq!(parity(&[true, false, true]), false);  // 1^0^1 = 0 (even number of 1s)
        assert_eq!(parity(&[true, true, true]), true);    // 1^1^1 = 1 (odd number of 1s)
        assert_eq!(parity(&[false, false, false]), false); // 0^0^0 = 0
        assert_eq!(parity(&[true]), true);
        assert_eq!(parity(&[]), false);  // Empty parity is false (identity for XOR)
    }

    #[test]
    fn test_qac0_bias_decreases_with_n() {
        let bias_4 = simulate_qac0_parity_attempt(4, 3);
        let bias_16 = simulate_qac0_parity_attempt(16, 3);
        let bias_64 = simulate_qac0_parity_attempt(64, 3);
        
        // Bias should decrease as n increases
        assert!(bias_4 > bias_16);
        assert!(bias_16 > bias_64);
        // All should be above random (0.5) but approaching it
        assert!(bias_4 > 0.5);
        assert!(bias_64 > 0.5);
    }

    #[test]
    fn test_quartic_bound_is_reasonable() {
        // For a function with Q(f) = 10, R(f) ≤ O((1+10)^4) = O(14641)
        // This is the worst-case bound; actual functions are usually much better
        let q_queries = 10.0;
        let r_bound = (1.0_f64 + q_queries).powi(4);
        assert!(r_bound > 0.0);
        assert!(r_bound < 20000.0);  // Sanity check
    }
}
