//! Exponential Time Hypothesis (ETH) Station
//!
//! Explores the recently claimed disproof of ETH by OpenAI and Anthropic (Oct 2026),
//! which found a subexponential-time algorithm for 3-SAT.
//!
//! Demonstrates implications for Grover's algorithm, quantum vs classical complexity,
//! and what "subexponential" actually means in practice.

use std::time::Instant;

/// Evaluate a 3-CNF clause (literal1 OR literal2 OR literal3)
fn eval_clause(lits: &[(usize, bool)], assignment: &[bool]) -> bool {
    for &(var, sign) in lits {
        let val = assignment[var];
        if sign && val {
            return true;
        } // Positive literal satisfied
        if !sign && !val {
            return true;
        } // Negative literal satisfied
    }
    false
}

/// Brute-force SAT solver: try all 2^n assignments (the ETH baseline)
fn brute_force_sat(clauses: &[Vec<(usize, bool)>], n_vars: usize) -> Option<Vec<bool>> {
    let total = 1 << n_vars;

    for mask in 0..total {
        // Decode assignment from bit pattern
        let mut assignment = vec![false; n_vars];
        for i in 0..n_vars {
            if (mask >> i) & 1 == 1 {
                assignment[i] = true;
            }
        }

        // Check all clauses
        let mut satisfied = true;
        for clause in clauses {
            if !eval_clause(clause, &assignment) {
                satisfied = false;
                break;
            }
        }

        if satisfied {
            return Some(assignment);
        }
    }

    None
}

/// Generate a 3-CNF formula deterministically (for demonstration)
fn generate_3sat(n_vars: usize, n_clauses: usize) -> Vec<Vec<(usize, bool)>> {
    let mut clauses = vec![];
    for i in 0..n_clauses {
        // Deterministic "random" variable selection based on clause index
        let v1 = (i * 7 + 3) % n_vars;
        let v2 = (i * 11 + 5) % n_vars;
        let v3 = (i * 13 + 9) % n_vars;

        // Alternate signs for variety
        let s1 = (i & 1) == 0;
        let s2 = (i & 2) == 0;
        let s3 = (i & 4) == 0;

        clauses.push(vec![(v1, s1), (v2, s2), (v3, s3)]);
    }
    clauses
}

/// Compute the theoretical running time for various algorithms on n variables
fn compute_complexity_comparison(n_vars: usize) -> (f64, f64, f64) {
    let n = n_vars as f64;
    let log_n = n.log2();
    let log_log_n = if log_n > 1.0 { log_n.log2() } else { 1.0 };

    // Brute force: O(2^n)
    let brute = 2.0_f64.powf(n);

    // Grover's algorithm: O(2^(n/2))
    let grover = 2.0_f64.powf(n / 2.0);

    // Claimed subexponential: O(2^(n * log log n / sqrt(log n)))
    // Simplified for demonstration (ignoring polynomial factors)
    let exponent = if log_n > 1.0 {
        n * log_log_n / log_n.sqrt()
    } else {
        n
    };
    let subexponential = 2.0_f64.powf(exponent);

    (brute, grover, subexponential)
}

pub fn show_eth_station() {
    println!("=== EXPONENTIAL TIME HYPOTHESIS (ETH) STATION ===");
    println!("October 2026: OpenAI and Anthropic claim ETH is false\n");

    println!("THE HYPOTHESIS (1999-2026):");
    println!("Impagliazzo & Paturi conjectured that 3-SAT requires exponential time.");
    println!("Specifically, there exists s₃ > 0 such that no algorithm solves 3-SAT in");
    println!("time less than 2^(s₃·n). This became a foundational assumption in complexity theory.");

    println!("\nTHE CLAIMED DISPROOF (October 2026):");
    println!("OpenAI's model found a deterministic, polynomial-space algorithm that counts");
    println!("satisfying assignments of k-CNF formulas in subexponential time:");
    println!("  O(2^(n·log log n / √log n))");
    println!("Anthropic independently claimed the same result with their own model.");
    println!("Both proofs were formalized and checked in Lean theorem prover.");

    println!("\n--- EXPERIMENT 1: WHAT ETH MEANT FOR GROVER'S ALGORITHM ---");
    println!("Grover provides quadratic speedup for unstructured search: O(N) → O(√N).");
    println!("For SAT with n variables, this meant O(2^n) → O(2^(n/2)).");
    println!("If ETH was true, Grover's speedup was asymptotically optimal.");

    println!("\nNow, if ETH is false and classical algorithms can solve 3-SAT in");
    println!("subexponential time like O(2^(n·log log n / √log n)), the quantum advantage");
    println!("for unstructured search becomes less dramatic.");

    println!("\n--- EXPERIMENT 2: COMPLEXITY COMPARISON ---");
    println!("Theoretical operation counts for different algorithms:\n");

    let sizes = [10, 20, 50, 100];
    for n_vars in sizes {
        let (brute, grover, subexp) = compute_complexity_comparison(n_vars);

        println!("n = {:3} variables:", n_vars);
        println!("  Brute force:      O(2^n)       = {:.0e}", brute);
        println!("  Grover's algo:    O(2^(n/2))   = {:.0e}", grover);
        println!(
            "  Subexponential: O(2^(n·log log n/√log n)) = {:.0e}",
            subexp
        );

        // Compare Grover vs subexponential
        let ratio = grover / subexp;
        if ratio > 1.0 {
            println!(
                "  → Subexponential is {:.0}x faster than Grover",
                ratio
            );
        } else {
            println!(
                "  → Grover is {:.0}x faster than subexponential",
                1.0 / ratio
            );
        }
        println!();
    }

    println!("KEY INSIGHT: For large n, the subexponential algorithm outperforms");
    println!("Grover's quadratic speedup. The quantum advantage for unstructured search");
    println!("is reduced from 'exponential' to something more modest.");

    println!("\n--- EXPERIMENT 3: BRUTE FORCE SAT SOLVING ---");
    println!("Demonstrating the exponential blowup that ETH was about:\n");

    let sizes = [4, 6, 8];
    for n_vars in sizes {
        let n_clauses = n_vars * 3; // Dense formula
        println!(
            "Solving {}-variable 3-SAT ({} clauses)...",
            n_vars, n_clauses
        );

        let formula = generate_3sat(n_vars, n_clauses);
        let start = Instant::now();
        let result = brute_force_sat(&formula, n_vars);
        let elapsed = start.elapsed().as_micros();

        match result {
            Some(_assignment) => {
                println!(
                    "  SATISFIABLE! Found assignment in {:.3}ms (tried {} combinations)",
                    elapsed as f64 / 1000.0,
                    1 << n_vars
                );
            }
            None => {
                println!(
                    "  UNSATISFIABLE after trying all {} combinations ({:.3}ms)",
                    1 << n_vars,
                    elapsed as f64 / 1000.0
                );
            }
        }
    }

    println!("\nNotice: even at n=8, we're trying 256 combinations.");
    println!("At n=100, that's 2^100 ≈ 10^30 — more than atoms in the observable universe.");

    println!("\n--- EXPERIMENT 4: WHAT THIS MEANS FOR QUANTUM COMPUTING ---");
    println!("ETH being false changes our understanding of quantum advantage:");

    println!("\nBEFORE (ETH true):");
    println!("  Classical: O(2^n) for NP-complete problems");
    println!("  Quantum (Grover): O(2^(n/2)) — exponential speedup");
    println!("  Shor's algorithm: breaks RSA by factoring in polynomial time");

    println!("\nAFTER (ETH false, claimed):");
    println!("  Classical: O(2^(n·log log n / √log n)) for 3-SAT — subexponential!");
    println!("  Quantum (Grover): O(2^(n/2)) — still faster, but less dramatically so");
    println!("  Shor's algorithm: unaffected (factoring is not NP-complete)");

    println!("\n--- EXPERIMENT 5: WHY SHOR'S ALGORITHM IS STILL IMPORTANT ---");
    println!("Shor's algorithm factors integers in polynomial time on a quantum computer.");
    println!("ETH being false doesn't affect this because:");
    println!("  - Factoring is not known to be NP-complete");
    println!("  - The best classical factoring algorithm is still subexponential but slow");
    println!("  - Shor provides polynomial time, which is fundamentally different");

    println!("\n--- EXPERIMENT 6: IMPLICATIONS FOR CRYPTOGRAPHY ---");
    println!("If the claimed disproof holds up under peer review:");

    println!("\nRSA (based on factoring):");
    println!("  Still broken by Shor's algorithm on a quantum computer");
    println!("  Classical security unaffected (factoring remains hard classically)");

    println!("\nAES, SHA-256 (symmetric crypto):");
    println!("  Grover provides quadratic speedup for key search");
    println!("  If ETH is false, classical algorithms might be closer to breaking these");
    println!("  But still require exponential time — just less so than before");

    println!("\nPost-quantum cryptography:");
    println!("  Lattice-based schemes (NTRU, Kyber) remain secure");
    println!("  Based on problems like Learning With Errors, not NP-complete problems");

    println!("\n--- EXPERIMENT 7: STATUS OF THE CLAIM ---");
    println!("As of November 2026, the disproof is still under review by:");
    println!("  - Russell Impagliazzo (UC San Diego) — co-creator of ETH");
    println!("  - Ramamohan Paturi (UC San Diego) — co-creator of ETH");
    println!("  - Ryan Williams (MIT) — complexity theory expert");

    println!("\nThey have stated their involvement should not be taken as certification.");
    println!("Both OpenAI and Anthropic provided Lean formalizations that anyone can verify.");
    println!("The mathematical community is cautiously skeptical but engaged.");

    println!("\n=== ETH STATION COMPLETE ===");
    println!("Bottom line: If confirmed, this narrows the quantum advantage for");
    println!("unstructured search but doesn't eliminate it. Shor's algorithm remains");
    println!("the killer app for quantum computing, breaking RSA and other public-key");
    println!("cryptosystems that protect our digital world.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clause_evaluation() {
        // Clause: x0 OR !x1
        let clause = vec![(0, true), (1, false)];

        // x0=T, x1=F → T OR T = T
        assert!(eval_clause(&clause, &[true, false]));
        // x0=F, x1=T → F OR F = F
        assert!(!eval_clause(&clause, &[false, true]));
        // x0=F, x1=F → F OR T = T
        assert!(eval_clause(&clause, &[false, false]));
    }

    #[test]
    fn test_sat_solver() {
        // (x0 OR x1) — satisfiable with x0=true or x1=true
        let clauses = vec![vec![(0, true), (1, true)]];
        let result = brute_force_sat(&clauses, 2);
        assert!(result.is_some());
    }

    #[test]
    fn test_unsat_formula() {
        // (x0) AND (!x0) — unsatisfiable
        let clauses = vec![vec![(0, true)], vec![(0, false)]];
        let result = brute_force_sat(&clauses, 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_complexity_comparison() {
        // For n=10, Grover should be faster than brute force
        let (brute, grover, _subexp) = compute_complexity_comparison(10);
        assert!(grover < brute);
        assert!(grover > 0.0);
    }
}
