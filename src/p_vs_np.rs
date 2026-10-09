//! P vs NP Millennium Prize Problem Station
//!
//! Explores the Clay Mathematics Institute's $1 million question:
//! Does P = NP? Can every problem whose solution can be verified quickly
//! also be solved quickly?
//!
//! Demonstrates through brute-force SAT solving, exponential blowup,
//! and the fundamental difficulty of NP-complete problems.

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

/// Brute-force SAT solver: try all 2^n assignments
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
        
        clauses.push(vec![
            (v1, s1),
            (v2, s2),
            (v3, s3),
        ]);
    }
    clauses
}

pub fn show_p_vs_np_station() {
    println!("=== P VS NP MILLENNIUM PRIZE PROBLEM ===");
    println!("Clay Mathematics Institute: $1,000,000 prize for proof\n");

    println!("THE QUESTION: Does P = NP?");
    println!("- P: Problems solvable in polynomial time (efficiently)");
    println!("- NP: Problems whose solutions can be VERIFIED in polynomial time");
    println!("\nIf P = NP, then every problem whose solution can be checked quickly");
    println!("can also be SOLVED quickly. Most believe P ≠ NP.");

    println!("\n--- EXPERIMENT 1: SAT IS NP-COMPLETE ---");
    println!("Cook-Levin Theorem (1971): Boolean satisfiability is NP-complete.");
    println!("Every NP problem reduces to SAT.\n");

    // Demonstrate SAT solving at increasing sizes
    let sizes = [4, 6, 8, 10];
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

    println!("\nNotice the exponential growth: 2^n possibilities.");
    println!(
        "At n=40, we'd try 1 trillion combinations. At n=100, more than atoms in the universe."
    );

    println!("\n--- EXPERIMENT 2: THE VERIFICATION TRICK ---");
    println!("NP means 'nondeterministic polynomial' — or equivalently,");
    println!("'polynomial-time verifiable.' Here's why that matters:\n");

    // Create a known satisfiable formula
    let easy_formula = vec![
        vec![(0, true), (1, true), (2, false)], // x0 OR x1 OR !x2
        vec![(0, false), (1, true), (2, true)], // !x0 OR x1 OR x2
    ];

    println!("Formula: (x0 ∨ x1 ∨ ¬x2) ∧ (¬x0 ∨ x1 ∨ x2)");
    println!("Can we VERIFY a solution quickly? Yes — just plug it in!");

    let test_assignment = vec![true, false, true]; // x0=T, x1=F, x2=T
    let verified = easy_formula
        .iter()
        .all(|clause| eval_clause(clause, &test_assignment));
    println!(
        "Assignment (T,F,T): {}",
        if verified { "SATISFIES" } else { "FAILS" }
    );

    println!("\nVerification: O(n) — linear in formula size.");
    println!("Finding solution: O(2^n) — exponential in number of variables.");

    println!("\n--- EXPERIMENT 3: WHY NO ONE HAS PROVEN P ≠ NP ---");
    println!("Despite decades of effort by the world's best logicians...");
    println!("- The problem is deeply connected to the structure of computation itself");
    println!("- Lower bound proofs are notoriously difficult (relativity, circuit complexity)");
    println!("- Many attempted 'proofs' have failed subtle technical hurdles\n");

    println!("If P = NP:");
    println!("  - All cryptographic schemes break (RSA, AES, SHA-256)");
    println!("  - Traveling salesman becomes polynomial-time solvable");
    println!("  - Drug discovery, protein folding, scheduling become tractable\n");

    println!("If P ≠ NP:");
    println!("  - Exponential blowup is fundamental, not a lack of cleverness");
    println!("  - Some problems are inherently hard to solve (but easy to check)");
    println!("  - We must accept approximation algorithms for NP-hard problems\n");

    println!("=== P VS NP STATION COMPLETE ===");
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
        // (x0 OR x1) AND (!x0 OR x1) — satisfiable with x1=true
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
}
