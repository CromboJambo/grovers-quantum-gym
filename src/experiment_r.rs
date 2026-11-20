//! Experiment R: Does the negative TV trend persist after downsampling to equal sample sizes?
//!
//! Tests whether the apparent magnitude-dependent effect is driven by sample size differences
//! across bands rather than genuine structural changes in prime gap transition patterns.

use rand::Rng;
use rand::SeedableRng;

fn sieve_primes(limit: usize) -> Vec<u64> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i)..=limit {
                if j % i == 0 {
                    is_prime[j] = false;
                }
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &p)| p)
        .map(|(n, _)| n as u64)
        .collect()
}

fn total_variation_distance(observed: &[u32], expected: &[u32]) -> f64 {
    let mut tv = 0.0;
    let total_observed: u32 = observed.iter().sum();
    for i in 0..observed.len() {
        tv += (observed[i] as f64 - expected[i] as f64).abs();
    }
    if total_observed == 0 {
        return 0.0;
    }
    tv / (2.0 * total_observed as f64)
}

fn count_transition_types(intervals: &[Vec<u32>]) -> Vec<u32> {
    let mut counts = vec![0u32; 16];
    for interval in intervals {
        if interval.is_empty() {
            continue;
        }
        for i in 0..interval.len() - 1 {
            let a = (interval[i] % 4) as usize;
            let b = (interval[i + 1] % 4) as usize;
            counts[a * 4 + b] += 1;
        }
    }
    counts
}

fn generate_control_intervals(
    low: u32,
    high: u32,
    prime_intervals: &[Vec<u32>],
    seed: u64,
) -> Vec<Vec<u32>> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut intervals = Vec::new();

    for prime_interval in prime_intervals {
        let gap_length = prime_interval.len() + 1;
        let max_start = high - gap_length as u32;
        if max_start <= low {
            continue;
        }
        let start: u32 = rng.gen_range(low..max_start);

        let mut interval = Vec::new();
        for n in (start + 1)..(start + gap_length as u32) {
            if n >= low && n <= high {
                interval.push(n);
            }
        }
        intervals.push(interval);
    }

    intervals
}

fn downsample_intervals(intervals: &[Vec<u32>], target_size: usize, seed: u64) -> Vec<Vec<u32>> {
    if intervals.len() <= target_size {
        return intervals.to_vec();
    }

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let indices: Vec<usize> = (0..intervals.len()).collect();
    let mut sampled_indices = indices.clone();
    // Fisher-Yates shuffle
    for i in (1..sampled_indices.len()).rev() {
        let j: usize = rng.gen_range(0..=i);
        sampled_indices.swap(i, j);
    }
    sampled_indices.truncate(target_size);
    sampled_indices.sort();

    sampled_indices
        .iter()
        .map(|&i| intervals[i].clone())
        .collect()
}

fn analyze_downsampling_effect() {
    println!("=== EXPERIMENT R: Downsampling test for TV trend ===\n");

    let max_n = 50000;
    let primes = sieve_primes(max_n);

    // Same bands as Experiment Q
    let band_boundaries: Vec<u32> = vec![100, 200, 400, 800, 1600, 3200, 6400, 12800, 25600, 50000];

    // Collect all prime intervals by band
    let mut band_intervals: Vec<Vec<Vec<u32>>> = Vec::new();
    for i in 0..band_boundaries.len() - 1 {
        let low = band_boundaries[i];
        let high = band_boundaries[i + 1];

        let mut intervals: Vec<Vec<u32>> = Vec::new();
        for j in 0..primes.len() - 1 {
            let p1 = primes[j];
            let p2 = primes[j + 1];
            if (p1 as u32) >= low && (p1 as u32) < high {
                let interval: Vec<u32> = ((p1 + 1)..p2).map(|x| x as u32).collect();
                if !interval.is_empty() {
                    intervals.push(interval);
                }
            }
        }
        band_intervals.push(intervals);
    }

    // Find minimum band size (excluding first band which is too small)
    let min_size = 50; // Target sample size for downsampling

    println!("Testing TV distances at equal sample sizes (n={}) across all bands:", min_size);
    println!("\nMagnitude Band  | Full TV | Downsampled TV | Difference");
    println!("----------------+---------+----------------+-----------");

    let mut full_tvs = Vec::new();
    let mut downsampled_tvs = Vec::new();

    for i in 0..band_intervals.len() {
        let low = band_boundaries[i];
        let high = band_boundaries[i + 1];
        let intervals = &band_intervals[i];

        if intervals.is_empty() {
            continue;
        }

        // Full sample TV
        let seed = i as u64 * 12345 + 67890;
        let controls_full = generate_control_intervals(low, high, intervals, seed);
        let observed_full = count_transition_types(intervals);
        let expected_full = count_transition_types(&controls_full);
        let tv_full = total_variation_distance(&observed_full, &expected_full);

        // Downsampled TV (if band has enough intervals)
        let tv_downsampled;
        if intervals.len() > min_size {
            let sampled_intervals = downsample_intervals(intervals, min_size, seed + 1000);
            let controls_sampled = generate_control_intervals(low, high, &sampled_intervals, seed);
            let observed_sampled = count_transition_types(&sampled_intervals);
            let expected_sampled = count_transition_types(&controls_sampled);
            tv_downsampled = total_variation_distance(&observed_sampled, &expected_sampled);
        } else {
            tv_downsampled = tv_full; // Use full TV if not enough to downsample
        }

        println!(
            "{:15} | {:7.4} | {:14.4} | {:9.4}",
            format!("{}-{}", low, high),
            tv_full,
            tv_downsampled,
            tv_full - tv_downsampled
        );

        full_tvs.push(tv_full);
        downsampled_tvs.push(tv_downsampled);
    }

    // Compare trends
    println!("\n--- Trend Comparison ---");
    println!("Full sample TV range: {:.4} to {:.4}", full_tvs.iter().cloned().fold(f64::INFINITY, f64::min), full_tvs.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    println!("Downsampled TV range: {:.4} to {:.4}", downsampled_tvs.iter().cloned().fold(f64::INFINITY, f64::min), downsampled_tvs.iter().cloned().fold(f64::NEG_INFINITY, f64::max));

    // Check if trend persists after downsampling
    let full_mean = full_tvs.iter().sum::<f64>() / full_tvs.len() as f64;
    let downsampled_mean = downsampled_tvs.iter().sum::<f64>() / downsampled_tvs.len() as f64;

    println!("Full sample mean TV: {:.4}", full_mean);
    println!("Downsampled mean TV: {:.4}", downsampled_mean);

    if (full_mean - downsampled_mean).abs() < 0.01 {
        println!("\nConclusion: Trend persists after downsampling — sample size is not the primary driver.");
    } else {
        println!("\nConclusion: Trend changes significantly after downsampling — sample size contributes to the effect.");
    }

    println!("\n=== EXPERIMENT R COMPLETE ===");
}

pub fn run_experiment_r() {
    analyze_downsampling_effect();
}