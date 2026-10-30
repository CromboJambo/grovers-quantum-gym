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

/// Experiment C: Granular gap-size bins stratified by magnitude with control intervals
pub fn experiment_c_stratified(limit: usize) -> Vec<(u64, u64, u64, std::collections::HashMap<String, f64>)> {
    use crate::riemann::integer_structure::{is_semiprime, is_prime_power};
    
    let gaps = prime_gaps_up_to(limit);
    
    // Define gap size bins and magnitude bands
    let gap_bins = [2u64, 4, 6, 8, 10, 12, 14, 16];
    let mag_bands = [(10u64, 100), (100, 1000), (1000, 5000)];
    
    let mut results = Vec::new();
    
    for &gap_size in &gap_bins {
        for &(mag_start, mag_end) in &mag_bands {
            // Find prime gaps of this size in this magnitude band
            let relevant_pairs: Vec<(u64, u64)> = gaps.iter()
                .filter(|&&(p1, p2, g)| g == gap_size && p1 >= mag_start && p1 < mag_end)
                .map(|&(p1, p2, _)| (p1, p2))
                .collect();
            
            if relevant_pairs.is_empty() { continue; }
            
            // Analyze actual prime gaps
            let mut sp_count = 0;
            let mut pp_count = 0;
            let mut total = 0;
            for &(p1, p2) in &relevant_pairs {
                for n in (p1 + 1)..p2 {
                    total += 1;
                    if is_semiprime(n) { sp_count += 1; }
                    if is_prime_power(n) { pp_count += 1; }
                }
            }
            
            // Control: randomly selected intervals of same length and magnitude
            let mut control_sp = 0;
            let mut control_total = 0;
            for _ in 0..relevant_pairs.len() {
                let start = mag_start + (rand::random::<u64>() % (mag_end - mag_start - gap_size));
                for n in (start + 1)..(start + gap_size) {
                    control_total += 1;
                    if is_semiprime(n) { control_sp += 1; }
                }
            }
            
            let mut stats: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
            if total > 0 {
                stats.insert("gap_sp_pct".to_string(), sp_count as f64 / total as f64 * 100.0);
                stats.insert("gap_pp_pct".to_string(), pp_count as f64 / total as f64 * 100.0);
            }
            if control_total > 0 {
                stats.insert("control_sp_pct".to_string(), control_sp as f64 / control_total as f64 * 100.0);
            }
            stats.insert("gap_count".to_string(), relevant_pairs.len() as f64);
            stats.insert("total_interior".to_string(), total as f64);
            
            results.push((gap_size, mag_start, mag_end, stats));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_gaps() {
        let gaps = prime_gaps_up_to(20);
        // Gaps: 1, 2, 2, 4, 2, 4, 2, 4 (between consecutive primes)
        assert_eq!(gaps[0], (2, 3, 1));
        assert_eq!(gaps[1], (3, 5, 2));
        assert_eq!(gaps.last().unwrap().2, 4); // 17→19 gap is 2, but last is 19 with no next
    }

    #[test]
    fn test_prime_pi() {
        assert_eq!(prime_pi(10), 4);  // 2,3,5,7
        assert_eq!(prime_pi(20), 8);  // 2,3,5,7,11,13,17,19
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
        assert_eq!(max_gaps[0].2, 4);
    }
}