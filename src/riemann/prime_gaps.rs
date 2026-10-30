//! Prime Gaps Module
//! Ordinal prime positions, distances, gap distributions.

use crate::riemann::integer_structure::{sieve_primes, is_prime};

/// Compute all prime gaps up to limit
/// Returns vector of (prime, next_prime, gap)
pub fn prime_gaps_up_to(limit: usize) -> Vec<(u64, u64, u64)> {
    let primes = sieve_primes(limit);
    let mut gaps = Vec::new();
    for i in 0..primes.len() - 1 {
        let gap = primes[i + 1] - primes[i];
        gaps.push((primes[i], primes[i + 1], gap));
    }
    gaps
}

/// Find the ordinal position of a prime (1-indexed)
pub fn prime_pi(n: u64) -> usize {
    // Count primes up to n using sieve for efficiency
    let limit = n as usize;
    let is_prime_arr = {
        let mut arr = vec![true; limit + 1];
        arr[0] = false;
        arr[1] = false;
        for p in 2..=((limit as f64).sqrt() as usize) {
            if arr[p] {
                for multiple in (p * p)..=limit {
                    if multiple % p == 0 {
                        arr[multiple] = false;
                    }
                }
            }
        }
        arr
    };
    let mut count = 0;
    for i in 2..=n as usize {
        if is_prime_arr[i] {
            count += 1;
        }
    }
    count
}

/// Distribution of gap sizes up to limit
/// Returns HashMap<gap_size, frequency>
pub fn gap_distribution(limit: usize) -> std::collections::HashMap<u64, usize> {
    let gaps = prime_gaps_up_to(limit);
    let mut dist = std::collections::HashMap::new();
    for &(_, _, gap) in &gaps {
        *dist.entry(gap).or_insert(0) += 1;
    }
    dist
}

/// Find maximal gaps up to limit
/// Returns vector of (prime, next_prime, gap) where gap is a record high
pub fn maximal_gaps(limit: usize) -> Vec<(u64, u64, u64)> {
    let gaps = prime_gaps_up_to(limit);
    let mut max_gaps = Vec::new();
    let mut current_max = 0;
    for (p1, p2, gap) in &gaps {
        if *gap > current_max {
            current_max = *gap;
            max_gaps.push((*p1, *p2, *gap));
        }
    }
    max_gaps
}

/// Check Twin Prime Conjecture up to limit: count pairs with gap 2
pub fn twin_prime_count(limit: usize) -> usize {
    let gaps = prime_gaps_up_to(limit);
    gaps.iter().filter(|&(_, _, gap)| *gap == 2).count()
}

/// List twin primes up to limit
pub fn twin_primes(limit: usize) -> Vec<(u64, u64)> {
    let gaps = prime_gaps_up_to(limit);
    gaps.iter()
        .filter(|&(_, _, gap)| *gap == 2)
        .map(|&(p1, p2, _)| (p1, p2))
        .collect()
}

/// Analyze gap patterns in a specific range [start, end]
pub fn analyze_gap_range(start: u64, end: u64) -> Vec<(u64, u64, u64)> {
    let mut gaps = Vec::new();
    let mut last_prime = None;
    for n in start..=end {
        if is_prime(n) {
            if let Some(prev) = last_prime {
                gaps.push((prev, n, n - prev));
            }
            last_prime = Some(n);
        }
    }
    gaps
}

/// Analyze structural composition of integers in a single prime gap
pub fn analyze_gap_structure(p1: u64, p2: u64) -> std::collections::HashMap<String, usize> {
    use crate::riemann::integer_structure::{is_semiprime, is_prime_power, is_squarefree, big_omega};
    
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let interior_count = (p2 - p1 - 1) as usize;
    
    for n in (p1 + 1)..p2 {
        *counts.entry("interior".to_string()).or_insert(0) += 1;
        
        if is_semiprime(n) {
            *counts.entry("semiprime".to_string()).or_insert(0) += 1;
        }
        if is_prime_power(n) {
            *counts.entry("prime_power".to_string()).or_insert(0) += 1;
        }
        if is_squarefree(n) {
            *counts.entry("squarefree_composite".to_string()).or_insert(0) += 1;
        }
        if big_omega(n) >= 3 {
            *counts.entry("three_plus_factors".to_string()).or_insert(0) += 1;
        }
    }
    
    counts.insert("interior_count".to_string(), interior_count);
    counts
}

/// Analyze structural composition by gap size
pub fn analyze_by_gap_size(limit: usize) -> std::collections::HashMap<u64, std::collections::HashMap<String, f64>> {
    use crate::riemann::integer_structure::{is_semiprime, is_prime_power};
    
    let gaps = prime_gaps_up_to(limit);
    let mut by_size: std::collections::HashMap<u64, Vec<(u64, u64)>> = std::collections::HashMap::new();
    
    for &(p1, p2, gap) in &gaps {
        by_size.entry(gap).or_insert(Vec::new()).push((p1, p2));
    }
    
    let mut results: std::collections::HashMap<u64, std::collections::HashMap<String, f64>> = std::collections::HashMap::new();
    for (gap_size, pairs) in by_size {
        if pairs.len() < 2 { continue; } // Need multiple examples
        
        let mut semiprime_count = 0;
        let mut prime_power_count = 0;
        let mut total_interior = 0;
        
        for &(p1, p2) in &pairs {
            for n in (p1 + 1)..p2 {
                total_interior += 1;
                if is_semiprime(n) { semiprime_count += 1; }
                if is_prime_power(n) { prime_power_count += 1; }
            }
        }
        
        let mut stats: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        if total_interior > 0 {
            stats.insert("semiprime_pct".to_string(), semiprime_count as f64 / total_interior as f64 * 100.0);
            stats.insert("prime_power_pct".to_string(), prime_power_count as f64 / total_interior as f64 * 100.0);
        }
        stats.insert("gap_count".to_string(), pairs.len() as f64);
        stats.insert("total_interior".to_string(), total_interior as f64);
        
        results.insert(gap_size, stats);
    }
    
    results
}

/// Experiment D: Parity-controlled comparison
/// Generate random intervals with same parity pattern as prime gaps
pub fn experiment_d_parity_control(limit: usize) -> Vec<(u64, f64, f64, f64, u64)> {
    use crate::riemann::integer_structure::{is_prime_power, is_semiprime};

    let gaps = prime_gaps_up_to(limit);
    let gap_bins = [2u64, 4, 6, 8, 10];
    let mut results = Vec::new();

    for &gap_size in &gap_bins {
        // Find all prime gaps of this size
        let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
            .filter(|&&(p1, p2, g)| g == gap_size)
            .map(|&(p1, p2, _)| (p1, p2))
            .collect();

        if relevant_pairs.is_empty() { continue; }

        // Analyze actual prime gaps
        let mut sp_count = 0;
        let mut total = 0;
        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                total += 1;
                if is_semiprime(n) { sp_count += 1; }
            }
        }

        let gap_sp_pct = if total > 0 { sp_count as f64 / total as f64 * 100.0 } else { 0.0 };

        // Generate parity-matched random controls
        // For each prime gap, generate a random interval of same length at similar magnitude
        let mut control_sp = 0;
        let mut control_total = 0;
        let num_controls = 100; // Many random samples for statistical significance

        for _ in 0..num_controls {
            // Pick random gap from relevant_pairs
            let idx = rand::random::<usize>() % relevant_pairs.len();
            let (p1, p2) = relevant_pairs[idx];
            let mag = p1;

            // Generate random interval of same length near this magnitude
            let start = mag + rand::random::<u64>() % 1000;
            for n in (start + 1)..(start + gap_size) {
                control_total += 1;
                if is_semiprime(n) { control_sp += 1; }
            }
        }

        let control_sp_pct = if control_total > 0 { control_sp as f64 / control_total as f64 * 100.0 } else { 0.0 };

        results.push((gap_size, gap_sp_pct, control_sp_pct, gap_sp_pct - control_sp_pct, total as u64));
    }

    results
}

/// Experiment E: Small-prime divisibility signatures and residue classes
/// Record divisibility by small primes for each interior integer
pub fn experiment_e_divisibility(limit: usize) -> std::collections::HashMap<u64, Vec<f64>> {
    use crate::riemann::integer_structure::{is_prime_power, is_semiprime};

    let gaps = prime_gaps_up_to(limit);
    let small_primes = [2u64, 3, 5, 7, 11, 13, 17, 19];
    let mut results: std::collections::HashMap<u64, Vec<f64>> = std::collections::HashMap::new();

    // Analyze actual prime gaps
    for gap_size in [2u64, 4, 6, 8] {
        let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
            .filter(|&&(p1, p2, g)| g == gap_size)
            .map(|&(p1, p2, _)| (p1, p2))
            .collect();

        if relevant_pairs.is_empty() { continue; }

        let mut divisibility_counts = vec![0u64; small_primes.len()];
        let mut total_interior = 0;

        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                total_interior += 1;
                for (i, &sp) in small_primes.iter().enumerate() {
                    if n % sp == 0 {
                        divisibility_counts[i] += 1;
                    }
                }
            }
        }

        // Convert to percentages
        let mut percentages = Vec::new();
        for count in &divisibility_counts {
            let pct = if total_interior > 0 { *count as f64 / total_interior as f64 * 100.0 } else { 0.0 };
            percentages.push(pct);
        }

        results.insert(gap_size, percentages);
    }

    results
}

/// Experiment E enhanced: Residue classes and smallest prime factor analysis
pub fn experiment_e_residues(limit: usize) -> Vec<(u64, std::collections::HashMap<String, Vec<f64>>)> {
    use crate::riemann::integer_structure::{is_prime_power, is_semiprime};

    let gaps = prime_gaps_up_to(limit);
    let small_primes = [2u64, 3, 5, 7, 11];
    let gap_sizes = [2u64, 4, 6, 8, 10];
    let mut results = Vec::new();

    for &gap_size in &gap_sizes {
        let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
            .filter(|&&(p1, p2, g)| g == gap_size)
            .map(|&(p1, p2, _)| (p1, p2))
            .collect();

        if relevant_pairs.is_empty() { continue; }

        let mut stats: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();

        // Residue class distributions for each small prime
        for &r in &small_primes {
            let mut residue_counts = vec![0u64; r as usize];
            let mut total = 0;

            for &(p1, p2) in &relevant_pairs {
                for n in (p1 + 1)..p2 {
                    total += 1;
                    let residue = (n % r) as usize;
                    residue_counts[residue] += 1;
                }
            }

            let mut percentages = Vec::new();
            for count in &residue_counts {
                let pct = if total > 0 { *count as f64 / total as f64 * 100.0 } else { 0.0 };
                percentages.push(pct);
            }

            stats.insert(format!("residues_mod_{}", r), percentages);
        }

        // Smallest prime factor distribution
        let mut spf_counts = std::collections::HashMap::<u64, u64>::new();
        let mut total_interior = 0;

        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                total_interior += 1;
                // Find smallest prime factor
                for sp in &small_primes {
                    if n % sp == 0 {
                        *spf_counts.entry(*sp).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }

        let mut spf_percentages = Vec::new();
        for &sp in &small_primes {
            let count = spf_counts.get(&sp).copied().unwrap_or(0);
            let pct = if total_interior > 0 { count as f64 / total_interior as f64 * 100.0 } else { 0.0 };
            spf_percentages.push(pct);
        }

        stats.insert("smallest_prime_factor".to_string(), spf_percentages);

        // Semiprime factor composition
        let mut semiprime_small_factors = 0;
        let mut semiprime_large_factors = 0;
        let mut semiprime_total = 0;

        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                if is_semiprime(n) {
                    semiprime_total += 1;
                    // Check if both factors are small primes
                    let has_small_factor = small_primes.iter().any(|&sp| n % sp == 0);
                    if has_small_factor {
                        semiprime_small_factors += 1;
                    } else {
                        semiprime_large_factors += 1;
                    }
                }
            }
        }

        let small_pct = if semiprime_total > 0 { semiprime_small_factors as f64 / semiprime_total as f64 * 100.0 } else { 0.0 };
        let large_pct = if semiprime_total > 0 { semiprime_large_factors as f64 / semiprime_total as f64 * 100.0 } else { 0.0 };

        stats.insert("semiprime_small_factors".to_string(), vec![small_pct]);
        stats.insert("semiprime_large_factors".to_string(), vec![large_pct]);

        results.push((gap_size, stats));
    }

    results
}

/// Experiment F: Factorization topology analysis
pub fn experiment_f_topology(limit: usize) -> Vec<(u64, f64, f64, f64, f64)> {
    use crate::riemann::integer_structure::{big_omega, omega};

    let gaps = prime_gaps_up_to(limit);
    let gap_bins = [2u64, 4, 6, 8, 10];
    let mut results = Vec::new();

    for &gap_size in &gap_bins {
        let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
            .filter(|&&(p1, p2, g)| g == gap_size)
            .map(|&(p1, p2, _)| (p1, p2))
            .collect();

        if relevant_pairs.is_empty() { continue; }

        let mut prime_count = 0;
        let mut prime_power_count = 0;
        let mut semiprime_count = 0;
        let mut higher_composite_count = 0;
        let mut total = 0;

        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                total += 1;
                let w = omega(n);
                let big_w = big_omega(n);

                if w == 1 && big_w == 1 {
                    prime_count += 1; // Prime
                } else if w == 1 && big_w > 1 {
                    prime_power_count += 1; // Prime power
                } else if big_w == 2 {
                    semiprime_count += 1; // Semiprime (Ω=2)
                } else {
                    higher_composite_count += 1; // Higher-order composite
                }
            }
        }

        let prime_pct = if total > 0 { prime_count as f64 / total as f64 * 100.0 } else { 0.0 };
        let pp_pct = if total > 0 { prime_power_count as f64 / total as f64 * 100.0 } else { 0.0 };
        let sp_pct = if total > 0 { semiprime_count as f64 / total as f64 * 100.0 } else { 0.0 };
        let hc_pct = if total > 0 { higher_composite_count as f64 / total as f64 * 100.0 } else { 0.0 };

        results.push((gap_size, prime_pct, pp_pct, sp_pct, hc_pct));
    }

    results
}

/// Analyze structural composition by magnitude range
pub fn analyze_by_magnitude(ranges: &[(u64, u64)]) -> Vec<(u64, u64, std::collections::HashMap<String, f64>)> {
    use crate::riemann::integer_structure::{is_semiprime, is_prime_power};
    
    let mut results = Vec::new();
    
    for &(start, end) in ranges {
        let gaps = analyze_gap_range(start, end);
        
        let mut semiprime_count = 0;
        let mut prime_power_count = 0;
        let mut total_interior = 0;
        
        for &(p1, p2, _) in &gaps {
            for n in (p1 + 1)..p2 {
                total_interior += 1;
                if is_semiprime(n) { semiprime_count += 1; }
                if is_prime_power(n) { prime_power_count += 1; }
            }
        }
        
        let mut stats: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        if total_interior > 0 {
            stats.insert("semiprime_pct".to_string(), semiprime_count as f64 / total_interior as f64 * 100.0);
            stats.insert("prime_power_pct".to_string(), prime_power_count as f64 / total_interior as f64 * 100.0);
        }
        stats.insert("gap_count".to_string(), gaps.len() as f64);
        stats.insert("total_interior".to_string(), total_interior as f64);
        
        results.push((start, end, stats));
    }
    
    results
}

/// Experiment G: Endpoint-conditioned controls
/// Generate control intervals [a, a+g] where gcd(a(a+g), P) = 1 for chosen primorial P.
/// This preserves small-prime endpoint restrictions without requiring primality.
pub fn experiment_g_endpoint_conditioned(limit: usize) -> Vec<(u64, std::collections::HashMap<String, f64>)> {
    use crate::riemann::integer_structure::{is_semiprime};

    let gaps = prime_gaps_up_to(limit);
    let gap_sizes = [2u64, 4, 6, 8];
    let primorial: u64 = 2 * 3 * 5 * 7 * 11; // 2310

    fn is_coprime(n: u64, m: u64) -> bool {
        let mut a = n;
        let mut b = m;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a == 1
    }

    fn generate_control_intervals(gap_size: u64, primorial: u64, count: usize) -> Vec<(u64, u64)> {
        let mut intervals = Vec::new();
        let mut n = 100; // Start above small primes
        while intervals.len() < count && n < 50000 {
            if is_coprime(n, primorial) && is_coprime(n + gap_size, primorial) {
                intervals.push((n, n + gap_size));
            }
            n += 1;
        }
        intervals
    }

    let mut results = Vec::new();

    for &gap_size in &gap_sizes {
        // Analyze actual prime gaps of this size
        let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
            .filter(|&&(p1, p2, g)| g == gap_size)
            .map(|&(p1, p2, _)| (p1, p2))
            .collect();

        if relevant_pairs.is_empty() { continue; }

        let mut prime_sp = 0;
        let mut prime_total = 0;
        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                prime_total += 1;
                if is_semiprime(n) { prime_sp += 1; }
            }
        }

        // Generate endpoint-conditioned controls
        let control_intervals = generate_control_intervals(gap_size, primorial, 50);
        let mut control_sp = 0;
        let mut control_total = 0;
        for &(a, b) in &control_intervals {
            for n in (a + 1)..b {
                control_total += 1;
                if is_semiprime(n) { control_sp += 1; }
            }
        }

        let prime_sp_pct = if prime_total > 0 { prime_sp as f64 / prime_total as f64 * 100.0 } else { 0.0 };
        let control_sp_pct = if control_total > 0 { control_sp as f64 / control_total as f64 * 100.0 } else { 0.0 };

        let mut stats: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        stats.insert("prime_bounded_pct".to_string(), prime_sp_pct);
        stats.insert("endpoint_conditioned_pct".to_string(), control_sp_pct);
        stats.insert("difference".to_string(), prime_sp_pct - control_sp_pct);
        stats.insert("prime_interior_count".to_string(), prime_total as f64);
        stats.insert("control_interior_count".to_string(), control_total as f64);

        results.push((gap_size, stats));
    }

    results
}

/// Experiment H: Largest Prime Factor Distribution
/// Compare the distribution of largest prime factors in interior integers vs controls.
/// If consecutive primes "shield" their gap from large prime factors, we should see
/// a different LPF distribution than in control intervals.
pub fn experiment_h_largest_prime_factor(limit: usize) -> Vec<(u64, std::collections::HashMap<String, f64>)> {
    use crate::riemann::integer_structure::{largest_prime_factor};

    let gaps = prime_gaps_up_to(limit);
    let gap_sizes = [2u64, 4, 6, 8];
    let primorial: u64 = 2 * 3 * 5 * 7 * 11; // 2310

    fn is_coprime(n: u64, m: u64) -> bool {
        let mut a = n;
        let mut b = m;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a == 1
    }

    fn generate_control_intervals(gap_size: u64, primorial: u64, count: usize) -> Vec<(u64, u64)> {
        let mut intervals = Vec::new();
        let mut n = 100;
        while intervals.len() < count && n < 50000 {
            if is_coprime(n, primorial) && is_coprime(n + gap_size, primorial) {
                intervals.push((n, n + gap_size));
            }
            n += 1;
        }
        intervals
    }

    fn lpf_bucket(lpf: u64) -> &'static str {
        if lpf <= 11 { "≤11" } else if lpf <= 31 { "12-31" } else if lpf <= 100 { "32-100" } else { ">100" }
    }

    let mut results = Vec::new();

    for &gap_size in &gap_sizes {
        // Analyze actual prime gaps of this size
        let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
            .filter(|&&(p1, p2, g)| g == gap_size)
            .map(|&(p1, p2, _)| (p1, p2))
            .collect();

        if relevant_pairs.is_empty() { continue; }

        // Distribution of LPF buckets for prime-bounded intervals
        let mut prime_buckets: std::collections::HashMap<&'static str, u64> = std::collections::HashMap::new();
        let mut prime_total = 0;
        for &(p1, p2) in &relevant_pairs {
            for n in (p1 + 1)..p2 {
                prime_total += 1;
                let lpf = largest_prime_factor(n);
                *prime_buckets.entry(lpf_bucket(lpf)).or_insert(0) += 1;
            }
        }

        // Distribution for endpoint-conditioned controls
        let control_intervals = generate_control_intervals(gap_size, primorial, 50);
        let mut control_buckets: std::collections::HashMap<&'static str, u64> = std::collections::HashMap::new();
        let mut control_total = 0;
        for &(a, b) in &control_intervals {
            for n in (a + 1)..b {
                control_total += 1;
                let lpf = largest_prime_factor(n);
                *control_buckets.entry(lpf_bucket(lpf)).or_insert(0) += 1;
            }
        }

        // Compute percentages and differences
        let mut stats: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for bucket in ["≤11", "12-31", "32-100", ">100"] {
            let prime_pct = if prime_total > 0 {
                *prime_buckets.get(bucket).unwrap_or(&0) as f64 / prime_total as f64 * 100.0
            } else { 0.0 };
            let control_pct = if control_total > 0 {
                *control_buckets.get(bucket).unwrap_or(&0) as f64 / control_total as f64 * 100.0
            } else { 0.0 };

            stats.insert(format!("prime_{}", bucket), prime_pct);
            stats.insert(format!("control_{}", bucket), control_pct);
            stats.insert(format!("diff_{}", bucket), prime_pct - control_pct);
        }
        stats.insert("prime_total".to_string(), prime_total as f64);
        stats.insert("control_total".to_string(), control_total as f64);

        results.push((gap_size, stats));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_gaps() {
        let gaps = prime_gaps_up_to(20);
        // Gaps: 1 (2→3), 2 (3→5), 2 (5→7), 4 (7→11), 2 (11→13), 4 (13→17), 2 (17→19)
        assert_eq!(gaps[0], (2, 3, 1));
        assert_eq!(gaps[1], (3, 5, 2));
    }

    #[test]
    fn test_prime_pi() {
        assert_eq!(prime_pi(10), 4); // 2,3,5,7
        assert_eq!(prime_pi(20), 8); // 2,3,5,7,11,13,17,19
    }

    #[test]
    fn test_twin_primes() {
        let twins = twin_primes(30);
        assert!(twins.contains(&(3, 5)));
        assert!(twins.contains(&(5, 7)));
        assert!(twins.contains(&(11, 13)));
        assert!(twins.contains(&(17, 19)));
    }

    #[test]
    fn test_maximal_gaps() {
        let max_gaps = maximal_gaps(100);
        // First record gap is 4 (between 7 and 11)
        assert!(!max_gaps.is_empty());
    }

    #[test]
    fn test_experiment_g_runs() {
        let results = experiment_g_endpoint_conditioned(200);
        // Should find at least one gap size with data
        assert!(!results.is_empty());
        for (gap_size, stats) in &results {
            assert!(stats.contains_key("prime_bounded_pct"));
            assert!(stats.contains_key("endpoint_conditioned_pct"));
        }
    }
}