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


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "riemann",
    version: "0.0.1",
    formalized: false,
    notes: "Zeta function computation on critical strip (Euler-Maclaurin)",
};


mod integer_structure;
mod prime_gaps;
pub mod rh_verification;
mod zeta_analysis;
pub mod von_mangoldt;
pub mod zeta_riemann_siegel;

use integer_structure::{factorize, mobius, omega, big_omega, is_semiprime, is_prime_power, is_squarefree};
use prime_gaps::{prime_gaps_up_to, gap_distribution, maximal_gaps, twin_primes, experiment_d_parity_control, experiment_e_residues, experiment_f_topology, experiment_g_endpoint_conditioned, experiment_h_largest_prime_factor, experiment_i_lpf_ratio, experiment_j_factorization_conditioned, experiment_k_shield_stress_test, experiment_l_neighbor_relationships, experiment_m_ordering_analysis, experiment_n_conditional_structure, experiment_o_transition_permutation, experiment_p_magnitude_replication};
use rh_verification::{rh_verification_report, verify_known_zeros, scan_critical_line, zeta};
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

    // Experiment I: Pattern propagation — LPF/n ratios
    // Do integers between consecutive primes have unusually large prime factors?
    // If the "pattern" of primality propagates into the gap, we might see many
    // integers with LPF close to n itself (i.e., nearly-prime numbers).
    println!("\n--- EXPERIMENT I: LARGEST PRIME FACTOR / N RATIOS ---");
    println!("Do integers between consecutive primes have unusually large prime factors?");
    println!("If the 'pattern' of primality propagates into the gap, we might see many");
    println!("integers with LPF close to n itself (i.e., nearly-prime numbers).");
    println!("\nThis tests the hypothesis that consecutive primes shield their gap not just");
    println!("from small factors but also that large prime factors are overrepresented.");

    let exp_i_results = experiment_i_lpf_ratio(limit);
    for (gap_size, stats) in &exp_i_results {
        println!("\nGap size {}:", gap_size);
        for bucket in ["≥0.9", "0.7-0.9", "0.5-0.7", "<0.5"] {
            let pct = *stats.get(bucket).unwrap();
            println!("  LPF/n {}: {:.1}%", bucket, pct);
        }
    }
    println!("If the ≥0.9 bucket is significantly populated, it suggests that consecutive");
    println!("primes leave a 'residue' of near-primality in their gap — integers that are");
    println!("close to being prime themselves (having one very large prime factor).");

    // Experiment J: Factorization-conditioned neighborhoods
    // Five comparison populations to isolate which constraints produce distinctive distributions
    println!("\n--- EXPERIMENT J: FACTORIZATION-CONDITIONED NEIGHBORHOODS ---");
    println!("Five comparison populations to isolate the source of factorization structure:");
    println!("  1. Consecutive-prime intervals (actual prime gaps)");
    println!("  2. Endpoint-conditioned intervals (coprime endpoints, no interior primes required)");
    println!("  3. Parity-matched intervals (same parity pattern as prime gaps)");
    println!("  4. Composite-interior intervals (nonprime endpoints, may contain interior primes)");
    println!("  5. Magnitude-matched intervals (random at similar magnitude)");

    let exp_j_results = experiment_j_factorization_conditioned(limit);

    // Group by gap size
    let gap_sizes = [2u64, 4, 6, 8];
    for &gap_size in &gap_sizes {
        println!("Gap size {}:", gap_size);

        // Get all five populations for this gap size
        let populations = ["prime_bounded", "endpoint_conditioned", "parity_matched",
                          "composite_interior", "magnitude_matched"];

        let mut population_stats: Vec<(String, std::collections::HashMap<String, f64>)> = Vec::new();
        for pop in &populations {
            let key = format!("{}_{}", gap_size, pop);
            if let Some(stats) = exp_j_results.get(&key) {
                population_stats.push((pop.to_string(), stats.clone()));
            }
        }

        if population_stats.is_empty() { continue; }

        // Print LPF distribution comparison
        println!("  Largest Prime Factor Distribution:");
        for bucket in ["<=11", "12-31", "32-100", ">100"] {
            print!("    LPF {}: ", bucket);
            let mut values = Vec::new();
            for (name, stats) in &population_stats {
                let key = format!("lpf_{}", bucket);
                if let Some(&pct) = stats.get(&key) {
                    values.push(format!("{}={:.1}%", name, pct));
                }
            }
            println!("{}", values.join(", "));
        }

        // Print LPF/n ratio comparison
        println!("  LPF/n Ratio Distribution:");
        for bucket in [">=0.9", "0.7-0.9", "0.5-0.7", "<0.5"] {
            print!("    Ratio {}: ", bucket);
            let mut values = Vec::new();
            for (name, stats) in &population_stats {
                let key = format!("lpf_ratio_{}", bucket);
                if let Some(&pct) = stats.get(&key) {
                    values.push(format!("{}={:.1}%", name, pct));
                }
            }
            println!("{}", values.join(", "));
        }

        // Print omega distribution comparison
        println!("  Number of Distinct Prime Factors ω(n):");
        for bucket in ["1", "2", "3", "4+"] {
            print!("    ω(n)={}: ", bucket);
            let mut values = Vec::new();
            for (name, stats) in &population_stats {
                let key = format!("omega_{}", bucket);
                if let Some(&pct) = stats.get(&key) {
                    values.push(format!("{}={:.1}%", name, pct));
                }
            }
            println!("{}", values.join(", "));
        }
    }

    println!("\nIf consecutive-prime intervals show a distinctive factorization distribution");
    println!("compared to all five controls, the prime-shield effect is real and not");
    println!("an artifact of modular constraints, parity, or magnitude.");

    // Experiment K: The shield stress test
    println!("\n--- EXPERIMENT K: THE SHIELD STRESS TEST ---");
    println!("Actively searching for counterexamples to the prime-shield hypothesis");
    println!("at multiple limits (1000, 5000, 10000):\n");

    let k_limits = [1000usize, 5000, 10000];
    let exp_k_results = experiment_k_shield_stress_test(&k_limits);

    println!("Limit | Prime-Bounded Ints | Max LPF | LPF>100 Count | LPF>100 % | Control LPF>100 %");
    println!("------|-------------------|---------|---------------|-----------|-----------------");
    for stats in &exp_k_results {
        let limit = *stats.get("limit").unwrap() as usize;
        let total = *stats.get("total_prime_bounded_integers").unwrap() as usize;
        let max_lpf = *stats.get("lpf_maximum").unwrap() as u64;
        let lpf_over_100_count = *stats.get("lpf_over_100_count").unwrap() as usize;
        let lpf_over_100_pct = *stats.get("lpf_over_100_pct").unwrap();
        let control_lpf_over_100_pct = stats.get("control_lpf_over_100_pct").copied().unwrap_or(0.0);

        println!("{:5} | {:>17} | {:>7} | {:>13} | {:>9.1}% | {:>15.1}%",
            limit, total, max_lpf, lpf_over_100_count, lpf_over_100_pct, control_lpf_over_100_pct);

        // Report first counterexample if found
        let ce_n = *stats.get("first_counterexample_n").unwrap();
        if ce_n > 0.0 {
            let ce_p1 = *stats.get("first_counterexample_p1").unwrap() as u64;
            let ce_p2 = *stats.get("first_counterexample_p2").unwrap() as u64;
            println!("  First counterexample: {} between {} and {} (LPF > 100)", ce_n as u64, ce_p1, ce_p2);
        } else {
            println!("  No counterexamples found up to {}", limit);
        }
    }

    // Factor composition comparison at largest limit
    if let Some(last_stats) = exp_k_results.last() {
        println!("\nFactor composition (prime-bounded vs control) at limit {}:", k_limits.last().unwrap());
        println!("Factor | Prime-Bounded % | Control % | Difference");
        println!("-------|-----------------|-----------|-----------");
        let small_primes = [2u64, 3, 5, 7, 11, 13, 17, 19];
        for &p in &small_primes {
            let pb_pct = last_stats.get(&format!("factor_{}_prime_bounded_pct", p)).copied().unwrap_or(0.0);
            let ctrl_pct = last_stats.get(&format!("factor_{}_control_pct", p)).copied().unwrap_or(0.0);
            println!("{:6} | {:>15.1}% | {:>9.1}% | {:>+9.1}%", p, pb_pct, ctrl_pct, pb_pct - ctrl_pct);
        }
    }

    println!("\nIf LPF > 100 eventually appears in prime-bounded intervals at larger limits,");
    println!("the 'prime shield' is a redistribution effect (less frequent) rather than");
    println!("a true exclusion (impossible).");

    // Experiment L: Neighbor relationship analysis
    println!("\n--- EXPERIMENT L: NEIGHBOR RELATIONSHIP ANALYSIS ---");
    println!("Comparing structural properties of integers between consecutive primes");
    println!("to control intervals of matching length and magnitude:\n");

    let exp_l_results = experiment_l_neighbor_relationships(200);
    println!("Gap Size | Adjacent Sharing (PB %) | Adjacent Sharing (Ctrl %) | Diff | Omega Similarity (PB %) | SqF Transitions (PB %)");
    println!("---------|-------------------------|---------------------------|------|------------------------|---------------------");
    for (gap_size, stats) in &exp_l_results {
        let adj_pb = *stats.get("adjacent_sharing_prime").unwrap();
        let adj_ctrl = *stats.get("adjacent_sharing_control").unwrap();
        let adj_diff = *stats.get("adjacent_sharing_diff").unwrap();
        let omega_sim = *stats.get("omega_similarity_prime").unwrap();
        let sqf_trans = *stats.get("sqf_transitions_prime").unwrap();
        
        println!("{:8} | {:>21.1}% | {:>23.1}% | {:>+4.1} | {:>20.1}% | {:>19.1}%",
            gap_size, adj_pb, adj_ctrl, adj_diff, omega_sim, sqf_trans);
    }

    // Experiment M: Ordering analysis
    println!("\n--- EXPERIMENT M: ORDERING ANALYSIS ---");
    println!("Autocorrelation of omega values in prime-bounded intervals vs shuffled controls:\n");

    let exp_m_results = experiment_m_ordering_analysis(500);
    println!("Gap Size | Intervals | Mean Autocorr (PB) | Mean Autocorr (Ctrl) | Diff");
    println!("---------|-----------|--------------------|----------------------|------");
    for (gap_size, stats) in &exp_m_results {
        let num_intervals = *stats.get("num_intervals").unwrap() as usize;
        if num_intervals < 5 { continue; }
        
        let autocorr_pb = stats.get("mean_autocorr_prime").copied().unwrap_or(0.0);
        let autocorr_ctrl = stats.get("mean_autocorr_control").copied().unwrap_or(0.0);
        let diff = stats.get("autocorr_diff").copied().unwrap_or(0.0);
        
        println!("{:8} | {:>9} | {:>18.4} | {:>20.4} | {:>+6.4}",
            gap_size, num_intervals, autocorr_pb, autocorr_ctrl, diff);
    }

    // Experiment N: Conditional structure analysis
    println!("\n--- EXPERIMENT N: CONDITIONAL STRUCTURE ANALYSIS ---");
    println!("Conditional persistence of structural properties in prime-bounded intervals:\n");

    let exp_n_stats = experiment_n_conditional_structure(500);
    if let Some(sqf_pb) = exp_n_stats.get("sqf_persistence_prime") {
        let sqf_ctrl = *exp_n_stats.get("sqf_persistence_control").unwrap();
        let sqf_diff = *exp_n_stats.get("sqf_persistence_diff").unwrap();
        println!("Squarefree persistence (n is SF → n+1 is SF):");
        println!("  Prime-bounded: {:.1}%", sqf_pb);
        println!("  Control:       {:.1}%", sqf_ctrl);
        println!("  Difference:    {:+.1}%", sqf_diff);
    }
    if let Some(omega_pb) = exp_n_stats.get("omega_stability_prime") {
        let omega_ctrl = *exp_n_stats.get("omega_stability_control").unwrap();
        let omega_diff = *exp_n_stats.get("omega_stability_diff").unwrap();
        println!("\nOmega stability (ω(n) == ω(n+1)):");
        println!("  Prime-bounded: {:.1}%", omega_pb);
        println!("  Control:       {:.1}%", omega_ctrl);
        println!("  Difference:    {:+.1}%", omega_diff);
    }

    // Experiment O: Transition permutation analysis
    println!("\n--- EXPERIMENT O: TRANSITION PERMUTATION ANALYSIS ---");
    println!("Total variation distance between transition distributions:\n");

    let exp_o_stats = experiment_o_transition_permutation(500);
    if let Some(tv) = exp_o_stats.get("tv_distance") {
        println!("TV distance (observed vs control transitions): {:.4}", tv);
        println!("  0.0 = identical distributions, 1.0 = completely different");
        
        // Show state counts
        for state in ["sf_low_omega", "sf_high_omega", "nsf_low_omega", "nsf_high_omega"] {
            if let Some(count) = exp_o_stats.get(&format!("state_{}_observed_count", state)) {
                println!("  State {}: {} observed transitions", state, *count as usize);
            }
        }
    }

    // Experiment P: Magnitude replication test
    println!("\n--- EXPERIMENT P: MAGNITUDE REPLICATION TEST ---");
    println!("Testing whether structural effects replicate across magnitude bands:\n");

    let exp_p_results = experiment_p_magnitude_replication(5000);
    println!("Band         | Prime Intervals | SqF Persistence (PB %) | SqF Persistence (Ctrl %) | Diff");
    println!("-------------|-----------------|--------------------------|--------------------------|------");
    for stats in &exp_p_results {
        let band_start = *stats.get("band_start").unwrap() as u64;
        let band_end = *stats.get("band_end").unwrap() as u64;
        let intervals = *stats.get("prime_intervals").unwrap() as usize;
        
        if intervals < 3 { continue; }
        
        let sqf_pb = stats.get("sqf_persistence_prime").copied().unwrap_or(0.0);
        let sqf_ctrl = stats.get("sqf_persistence_control").copied().unwrap_or(0.0);
        let sqf_diff = stats.get("sqf_persistence_diff").copied().unwrap_or(0.0);
        
        println!("{:5}-{:<10} | {:>13} | {:>24.1}% | {:>24.1}% | {:>+6.1}%",
            band_start, band_end, intervals, sqf_pb, sqf_ctrl, sqf_diff);
    }

    println!("\nIf effects are consistent across bands, they are robust structural properties.");
    println!("If they vary significantly, the 'shield' effect may be magnitude-dependent.");
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


    println!("\n=== VON MANGOLT SUM ANALYSIS (Post-RH-Proof Anchor) ===");
    println!("Alpoge proof (Nov 2026) controls weighted sums S(t,X) = sum of Lambda(n)(n-1)^(-it).");
    println!("Direct computation available via: cargo run --example von_mangoldt");
    println!("Key objects: Λ(n), Ψ(x), zero-density peaks, explicit formula convergence.");
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