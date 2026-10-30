//! Riemann Hypothesis Millennium Prize Problem Station
//!
//! Explores the Clay Mathematics Institute's $1 million question:
//! Do all non-trivial zeros of the Riemann zeta function lie on the
//! critical line Re(s) = 1/2?
//!
//! Architecture: Three separate representations sharing underlying data:
//! - integer_structure: factorization, prime powers, Möbius function
//! - prime_gaps: ordinal positions, distances, gap distributions
//! - zeta_analysis: Euler product, zeta zeros, approximation errors

mod integer_structure;
mod prime_gaps;
mod zeta_analysis;

use integer_structure::{factorize, mobius, omega, big_omega, is_semiprime, is_prime_power, is_squarefree};
use prime_gaps::{prime_gaps_up_to, gap_distribution, maximal_gaps, twin_primes, experiment_d_parity_control, experiment_e_residues, experiment_f_topology, experiment_g_endpoint_conditioned, experiment_h_largest_prime_factor};
use zeta_analysis::{verify_euler_product, verify_mobius_identity, pnt_comparison, rh_gap_bound};

/// Experiment: Structural classification using independent properties
fn structural_experiment(limit: usize) {
    println!("--- STRUCTURAL CLASSIFICATION EXPERIMENT (Independent Properties) ---");
    println!("Using ω(n), Ω(n), is_semiprime, is_prime_power, is_squarefree\n");

    // Example: analyze specific intervals with detailed properties
    let test_intervals = [(7u64, 11u64), (23u64, 29u64), (89u64, 97u64)];
    
    for &(p1, p2) in &test_intervals {
        println!("Interval ({}, {}):", p1, p2);
        for n in (p1 + 1)..p2 {
            let factors = factorize(n);
            let w = omega(n);
            let big_w = big_omega(n);
            let sp = is_semiprime(n);
            let pp = is_prime_power(n);
            let sf = is_squarefree(n);
            
            println!("  {} = {:?} | ω={} Ω={} semiprime={} prime_power={} squarefree={}",
                n, factors, w, big_w, sp, pp, sf);
        }
    }

    // Experiment D: Parity-controlled comparison
    println!("\n--- EXPERIMENT D: PARITY-CONTROLLED COMPARISON ---");
    println!("Comparing prime-bounded intervals vs parity-matched random controls\n");
    
    let exp_d_results = experiment_d_parity_control(limit);
    println!("Gap Size | Prime-Bounded % | Control % | Difference | Interior Count");
    println!("---------|-----------------|-----------|------------|--------------");
    for (gap_size, gap_pct, control_pct, diff, interior) in &exp_d_results {
        println!("{:8} | {:13.2}% | {:9.2}% | {:>+10.2}% | {:?}", 
            gap_size, gap_pct, control_pct, diff, interior);
    }

    // Experiment E enhanced: Residue classes and smallest prime factor analysis
    println!("\n--- EXPERIMENT E ENHANCED: RESIDUE CLASSES AND SMALLEST PRIME FACTOR ---");
    println!("Analyzing modular structure of interior integers\n");
    
    let exp_e_res = experiment_e_residues(limit);
    for (gap_size, stats) in &exp_e_res {
        println!("Gap size {}:", gap_size);
        
        // Show residue distributions for key small primes
        if let Some(residues_3) = stats.get("residues_mod_3") {
            println!("  Residues mod 3: [0]={:.1}% [1]={:.1}% [2]={:.1}%",
                residues_3[0], residues_3[1], residues_3[2]);
        }
        
        if let Some(residues_5) = stats.get("residues_mod_5") {
            println!("  Residues mod 5: [0]={:.1}% [1]={:.1}% [2]={:.1}% [3]={:.1}% [4]={:.1}%",
                residues_5[0], residues_5[1], residues_5[2], residues_5[3], residues_5[4]);
        }
        
        // Smallest prime factor distribution
        if let Some(spf) = stats.get("smallest_prime_factor") {
            println!("  Smallest prime factor: [2]={:.1}% [3]={:.1}% [5]={:.1}% [7]={:.1}% [11]={:.1}%",
                spf[0], spf[1], spf[2], spf[3], spf[4]);
        }
        
        // Semiprime factor composition
        if let Some(small) = stats.get("semiprime_small_factors") {
            if let Some(large) = stats.get("semiprime_large_factors") {
                println!("  Semiprimes with small factors: {:.1}%", small[0]);
                println!("  Semiprimes with large factors: {:.1}%", large[0]);
            }
        }
    }

    // Experiment F: Factorization topology
    println!("\n--- EXPERIMENT F: FACTORIZATION TOPOLOGY ---");
    println!("Distribution of structural types within prime gaps\n");
    
    let exp_f_results = experiment_f_topology(limit);
    println!("Gap Size | Prime % | Prime Power % | Semiprime % | Higher Composite %");
    println!("---------|----------|---------------|-------------|-------------------");
    for (gap_size, prime_pct, pp_pct, sp_pct, hc_pct) in &exp_f_results {
        println!("{:8} | {:6.2}% | {:13.2}% | {:11.2}% | {:>17.2}%",
            gap_size, prime_pct, pp_pct, sp_pct, hc_pct);
    }

    // Experiment G: Endpoint-conditioned controls
    println!("\n--- EXPERIMENT G: ENDPOINT-CONDITIONED CONTROLS ---");
    println!("Comparing prime-bounded intervals vs intervals with coprime endpoints\n");
    
    let exp_g_results = experiment_g_endpoint_conditioned(limit);
    println!("Gap Size | Prime-Bounded % | Endpoint-Cond. % | Difference");
    println!("---------|-----------------|------------------|-----------");
    for (gap_size, stats) in &exp_g_results {
        let prime_pct = *stats.get("prime_bounded_pct").unwrap();
        let control_pct = *stats.get("endpoint_conditioned_pct").unwrap();
        let diff = *stats.get("difference").unwrap();
        println!("{:8} | {:13.2}% | {:14.2}% | {:>+9.2}%",
            gap_size, prime_pct, control_pct, diff);
    }

    println!("\nIf endpoint-conditioned controls reproduce the observed distributions,");
    println!("the mechanism is identified as modular constraints on endpoints.");
    println!("If not, something additional about consecutive primes remains unexplained.");

    // Experiment H: Largest prime factor distribution
    println!("\n--- EXPERIMENT H: LARGEST PRIME FACTOR DISTRIBUTION ---");
    println!("Does the 'shield' effect extend to large prime factors?");
    println!("If p and q are consecutive primes, can their gap contain integers with");
    println!("very large prime factors? If not, this would be a deeper structural property.\n");

    let exp_h_results = experiment_h_largest_prime_factor(limit);
    for (gap_size, stats) in &exp_h_results {
        println!("Gap size {}:", gap_size);
        for bucket in ["≤11", "12-31", "32-100", ">100"] {
            let prime_pct = *stats.get(&format!("prime_{}", bucket)).unwrap();
            let control_pct = *stats.get(&format!("control_{}", bucket)).unwrap();
            println!("  LPF {}: prime-bounded={:.1}%, endpoint-conditioned={:.1}%",
                bucket, prime_pct, control_pct);
        }
    }
    println!("\nIf the distributions are similar, the 'shield' effect is limited to small primes.");
    println!("This would confirm that consecutive primes impose modular constraints");
    println!("on their gap's factorization structure, but not a deeper property.");
}

pub fn show_riemann_station() {
    println!("=== RIEMANN HYPOTHESIS MILLENNIUM PRIZE PROBLEM ===");
    println!("Clay Mathematics Institute: $1,000,000 prize for proof\\n");

    println!("THE QUESTION: Do all non-trivial zeros of the Riemann zeta function");
    println!("ζ(s) = Σ(1/n^s) lie on the critical line Re(s) = 1/2?\\n");

    // Part 1: Euler Product Identity
    println!("=== PART 1: THE EULER PRODUCT IDENTITY ===");
    println!("Two representations of the same structure:\\n");
    println!("  Σ(1/n^s) over ALL natural numbers = Π(1 - p^(-s))^(-1) over PRIMES only\\n");

    let s_values = [2.0, 3.0, 4.0];
    println!("Verifying identity for s > 1:\\n");
    println!("s   | ζ(s) series | Euler product | Relative error");
    println!("----|-------------|---------------|---------------");
    for &s in &s_values {
        let (zeta, product, error) = verify_euler_product(s, 500);
        println!("{:3.1} | {:9.6} | {:11.6} | {:.2e}", s, zeta, product, error);
    }

    println!("\\nThis identity follows from unique prime factorization.");
    println!("The left side enumerates all natural numbers additively.");
    println!("The right side reconstructs them multiplicatively from primes.\\n");

    // Part 2: Möbius Function and Inverted Zeta
    println!("=== PART 2: THE MÖBIUS FUNCTION ===");
    println!("μ(n) encodes inclusion-exclusion over prime factors:\\n");
    println!("  μ(1) = 1");
    println!("  μ(p) = -1 for prime p");
    println!("  μ(pq) = +1 for distinct primes p,q");
    println!("  μ(n) = 0 if n has a squared prime factor\\n");

    let test_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print!("μ(n) for n=1..10: ");
    for &n in &test_values {
        print!("{:>3}", mobius(n));
    }
    println!();

    println!("\\nIdentity: Σ(μ(n)/n^s) = 1/ζ(s)\\n");
    let (mobius_sum, expected, error) = verify_mobius_identity(2.0, 1000);
    println!("Σ(μ(n)/n²) computed: {:.6}", mobius_sum);
    println!("1/ζ(2) expected:     {:.6}", expected);
    println!("Relative error:      {:.2e}\\n", error);

    // Part 3: Prime Gaps and Distribution
    println!("=== PART 3: PRIME GAP DISTRIBUTION ===");
    println!("The 'irregularity' of primes is not random — it's structured.\\n");

    let limit = 500;
    let gaps = prime_gaps_up_to(limit);
    println!("Prime gaps up to {}:", limit);
    println!("Gap size | Frequency | Primes involved (first few examples)");
    println!("---------|-----------|-----------------------------------");

    let dist = gap_distribution(limit);
    let mut sorted_gaps: Vec<(&u64, &usize)> = dist.iter().collect();
    sorted_gaps.sort_by_key(|&(gap, _)| *gap);

    for (gap_size, freq) in sorted_gaps {
        // Find example primes with this gap
        let examples = gaps.iter()
            .filter(|&(_, _, g)| *g == *gap_size)
            .take(2)
            .map(|&(p1, p2, _)| format!("{}→{}", p1, p2))
            .collect::<Vec<_>>()
            .join(", ");

        println!("{:8} | {:9} | {}", gap_size, freq, examples);
    }

    // Maximal gaps
    let max_gaps = maximal_gaps(limit);
    println!("\\nRecord-breaking gaps (maximal gaps):");
    for &(p1, p2, gap) in &max_gaps {
        println!("  {}→{}: gap of {} (first time a gap this large appears)", p1, p2, gap);
    }

    // Twin primes
    let twins = twin_primes(limit);
    println!("\\nTwin prime pairs (gap=2) up to {}: {}", limit, twins.len());
    if !twins.is_empty() {
        print!("  Examples: ");
        for (i, &(p1, p2)) in twins.iter().enumerate() {
            if i >= 5 { break; }
            print!("({},{}), ", p1, p2);
        }
        println!();
    }

    // Part 4: Prime Number Theorem and RH
    println!("\\n=== PART 4: PRIME NUMBER THEOREM AND RH ===");
    println!("PNT: π(x) ~ Li(x) as x→∞ (ratio approaches 1)\\n");

    let sample_points = [100u64, 500, 1000, 5000];
    println!("x     | π(x) actual | Li(x) approx | Ratio π/Li | Relative error");
    println!("------|-------------|------------|-----------|---------------");
    let results = pnt_comparison(5000, &sample_points);
    for &(x, pi_x, ratio, error) in &results {
        println!("{:5} | {:11} | {:10.2} | {:9.6} | {:.4e}", x, pi_x, 1.0/ratio * pi_x as f64, ratio, error);
    }

    println!("\\nUnder RH, the error term is bounded by O(√x · ln(x)):");
    for &x in &[100f64, 1000.0, 10000.0] {
        let bound = rh_gap_bound(x);
        println!("  x={:7.0}: RH error bound ≈ {:.0}", x, bound);
    }

    // Part 5: Structural Experiment
    structural_experiment(200);

    // Summary
    println!("\\n=== WHAT WE KNOW ===");
    println!("• First 10^13 zeros computed numerically — all on critical line Re(s)=1/2");
    println!("• No counterexample found despite extensive search");
    println!("• But no one has proven it for ALL zeros\\n");

    println!("=== WHY IT MATTERS ===");
    println!("The distribution of prime numbers is encoded in the zeta zeros.");
    println!("RH would give us the sharpest possible bound on how far primes");
    println!("can deviate from their 'expected' distribution.\\n");

    println!("=== CONNECTION TO PHYSICS ===");
    println!("Zeta zero spacing matches eigenvalue statistics of random Hermitian");
    println!("matrices (Gaussian Unitary Ensemble). Berry & Keating conjectured");
    println!("an underlying Hamiltonian whose spectrum gives the zeros.\\n");

    println!("=== ARCHITECTURAL INSIGHT ===");
    println!("This station demonstrates preserving multiple mathematical");
    println!("representations and studying their relationships:");
    println!("  - Additive enumeration (natural numbers)");
    println!("  - Multiplicative construction (prime factorization)");
    println!("  - Structural cancellation (Möbius function)");
    println!("Each representation reveals different aspects of the same structure.\\n");

    println!("=== RIEMANN HYPOTHESIS STATION COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_product_identity() {
        let (zeta, product, error) = verify_euler_product(2.0, 1000);
        assert!(error < 0.01);
    }

    #[test]
    fn test_mobius_identity() {
        let (_, _, error) = verify_mobius_identity(2.0, 1000);
        assert!(error < 0.01);
    }

    #[test]
    fn test_prime_gaps_exist() {
        let gaps = prime_gaps_up_to(100);
        assert!(!gaps.is_empty());
        // First gap is always 1 (2→3)
        assert_eq!(gaps[0].2, 1);
    }

    #[test]
    fn test_twin_primes_found() {
        let twins = twin_primes(50);
        assert!(twins.len() > 0);
        assert!(twins.contains(&(3, 5)));
    }

    #[test]
    fn test_structural_classification() {
        // Verify that integers between primes can be classified
        use crate::riemann::integer_structure::classify_range;
        let counts = classify_range(10, 20);
        assert!(counts.contains_key("prime"));
        assert!(*counts.get("prime").unwrap() > 0);
    }
}