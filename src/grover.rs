//! Grover's search algorithm - classical simulation of amplitude amplification.
//!
//! Demonstrates O(√N) speedup over classical O(N) linear search for unstructured
//! search problems, using probability amplitude manipulation.

/// Oracle trait: given a candidate index, returns true if it's the target.
pub trait GroverOracle {
    fn evaluate(&self, x: usize) -> bool;
}

/// Classical linear search baseline.
pub fn linear_search<F>(n: usize, oracle: F) -> Option<usize>
where
    F: Fn(usize) -> bool,
{
    for i in 0..n {
        if oracle(i) {
            return Some(i);
        }
    }
    None
}

/// Grover's search algorithm (classical simulation).
///
/// Simulates the quantum circuit using probability amplitudes as complex numbers.
/// Returns the found index, or None if iterations exhausted.
pub fn grover_search<F>(n: usize, oracle: F) -> Option<usize>
where
    F: Fn(usize) -> bool + Clone,
{
    // Count marked states to determine optimal iterations
    let mut marked_count = 0;
    for i in 0..n {
        if oracle(i) {
            marked_count += 1;
        }
    }

    if marked_count == 0 {
        return None;
    }

    // Optimal iterations: π/4 * sqrt(N/M) where M is number of marked states
    let iterations = ((std::f64::consts::PI / 4.0) * (n as f64 / marked_count as f64).sqrt()) as usize;
    if iterations < 1 {
        return Some(0); // Trivial case
    }

    // Initialize uniform superposition: all amplitudes equal
    let mut amplitudes = vec![1.0 / (n as f64).sqrt(); n];

    for _ in 0..iterations {
        // Oracle step: flip phase of marked state(s)
        for i in 0..n {
            if oracle(i) {
                amplitudes[i] = -amplitudes[i];
            }
        }

        // Diffusion operator: reflect about mean
        let mean: f64 = amplitudes.iter().sum::<f64>() / (n as f64);
        for i in 0..n {
            amplitudes[i] = 2.0 * mean - amplitudes[i];
        }
    }

    // Measure: find index with highest probability amplitude
    let mut best_idx = 0;
    let mut best_prob = f64::NEG_INFINITY;
    for (i, &amp) in amplitudes.iter().enumerate() {
        let prob = amp * amp;
        if prob > best_prob {
            best_prob = prob;
            best_idx = i;
        }
    }

    // Verify the result satisfies the oracle
    if oracle(best_idx) {
        Some(best_idx)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_basic() {
        let result = linear_search(10, |x| x == 7);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_grover_search_small() {
        // Search for value 3 in space of size 8
        let result = grover_search(8, |x| x == 3);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_grover_search_larger() {
        // Search for value 42 in space of size 100
        let result = grover_search(100, |x| x == 42);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_grover_multiple_targets() {
        // Multiple marked states: multiples of 3 in range [0, 9)
        // Grover should find one of them
        let result = grover_search(10, |x| x % 3 == 0);
        assert!(result.is_some());
        // Verify it's actually a multiple of 3
        let found = result.unwrap();
        assert_eq!(found % 3, 0, "Found {} which is not a multiple of 3", found);
    }
}