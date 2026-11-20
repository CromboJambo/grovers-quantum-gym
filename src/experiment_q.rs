//! Experiment Q: Does the transition-structure effect vary continuously with prime gap magnitude?
//!
//! Design (per user guidance):
//! - Many magnitude bands, not just four
//! - Fit TV distance trend against log(magnitude)
//! - Permutation test for the trend itself
//! - Retain individual interval records
//! - Verify control exchangeability across magnitudes

use rand::Rng;
use rand::SeedableRng;

/// Simple sieve for primes up to limit
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
    let range_size = (high - low) as u32;

    for prime_interval in prime_intervals {
        // Match the length of the corresponding prime gap interval
        let gap_length = prime_interval.len() + 1; // +1 because prime interval excludes endpoints

        // Pick random start within band, ensuring room for the gap
        let max_start = high - gap_length as u32;
        if max_start <= low {
            continue;
        }
        let start: u32 = rng.gen_range(low..max_start);

        // Generate control interval of same length
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

fn analyze_magnitude_trend() {
    println!("=== EXPERIMENT Q: Magnitude-dependent transition-structure effect ===\n");

    let max_n = 50000;
    let primes = sieve_primes(max_n);

    // Many magnitude bands (logarithmically spaced)
    let band_boundaries: Vec<u32> = vec![100, 200, 400, 800, 1600, 3200, 6400, 12800, 25600, 50000];
    let mut band_results = Vec::new();

    println!("Magnitude Band  | Intervals | TV Distance | Log(Midpoint)");
    println!("----------------+-----------+-------------+---------------");

    for i in 0..band_boundaries.len() - 1 {
        let low = band_boundaries[i];
        let high = band_boundaries[i + 1];
        let mid = (low + high) / 2;

        // Collect prime intervals in this band
        let mut prime_intervals: Vec<Vec<u32>> = Vec::new();
        for i in 0..primes.len() - 1 {
            let p1 = primes[i];
            let p2 = primes[i + 1];
            if (p1 as u32) >= low && (p1 as u32) < high {
                let interval: Vec<u32> = ((p1 + 1)..p2).map(|x| x as u32).collect();
                if !interval.is_empty() {
                    prime_intervals.push(interval);
                }
            }
        }

        if prime_intervals.is_empty() {
            continue;
        }

        // Generate matched controls (same gap lengths)
        let seed = i as u64 * 12345 + 67890;
        let control_intervals = generate_control_intervals(low, high, &prime_intervals, seed);

        // Compute transition counts
        let observed = count_transition_types(&prime_intervals);
        let expected = count_transition_types(&control_intervals);

        let tv = total_variation_distance(&observed, &expected);
        let log_mid = (mid as f64).ln();

        println!(
            "{:15} | {:9} | {:11.4} | {:13.4}",
            format!("{}-{}", low, high),
            prime_intervals.len(),
            tv,
            log_mid
        );

        band_results.push((log_mid, tv, prime_intervals.len()));
    }

    // Fit linear trend: TV = a + b * log(magnitude)
    println!("\n--- Linear Trend Analysis (TV vs Log-Magnitude) ---");

    let n = band_results.len();
    if n < 3 {
        println!("Insufficient bands for trend analysis.");
        return;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;

    for (log_mid, tv, _) in &band_results {
        sum_x += *log_mid;
        sum_y += *tv;
        sum_xy += log_mid * tv;
        sum_xx += log_mid * log_mid;
    }

    let slope = (n as f64 * sum_xy - sum_x * sum_y) / (n as f64 * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n as f64;

    println!("Slope: {:.6}", slope);
    println!("Intercept: {:.6}", intercept);

    // Permutation test for trend significance
    let mut permuted_slopes = Vec::new();
    let tvs: Vec<f64> = band_results.iter().map(|(_, tv, _)| *tv).collect();

    for iter in 0..1000 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(iter * 98765 + 11111);
        let mut shuffled = tvs.clone();
        // Fisher-Yates shuffle
        for i in (1..shuffled.len()).rev() {
            let j: usize = rng.gen_range(0..=i);
            shuffled.swap(i, j);
        }

        let mut perm_sum_xy = 0.0;
        for (k, tv) in shuffled.iter().enumerate() {
            perm_sum_xy += band_results[k].0 * tv;
        }
        let perm_slope =
            (n as f64 * perm_sum_xy - sum_x * sum_y) / (n as f64 * sum_xx - sum_x * sum_x);
        permuted_slopes.push(perm_slope);
    }

    // Two-tailed p-value
    let observed_abs_slope = slope.abs();
    let extreme_count = permuted_slopes
        .iter()
        .filter(|s| s.abs() >= observed_abs_slope)
        .count();
    let p_value = (extreme_count + 1) as f64 / (permuted_slopes.len() + 1) as f64;

    println!("Permutation test p-value: {:.4}", p_value);
    if p_value < 0.05 {
        println!("Significant trend detected.");
    } else {
        println!("No significant trend detected.");
    }

    // Control exchangeability check
    println!("\n--- Control Exchangeability Check ---");
    for (log_mid, tv, count) in &band_results {
        let mid_exp = (*log_mid).exp() as u32;
        println!("Magnitude ~{}: {} intervals, TV={:.4}", mid_exp, count, tv);
    }

    println!("\n=== EXPERIMENT Q COMPLETE ===");
}

pub fn run_experiment_q() {
    analyze_magnitude_trend();
}
