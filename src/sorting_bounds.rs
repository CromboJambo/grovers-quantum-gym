/// SORTING LOWER BOUNDS STATION
///
/// Proving that comparison-based sorting requires Ω(n log n) comparisons
/// in the worst case. Uses information-theoretic arguments and decision trees.

fn count_permutations(n: usize) -> u64 {
    let mut result: u64 = 1;
    for i in 2..=n {
        result *= i as u64;
    }
    result
}

fn log2_factorial_approx(n: usize) -> f64 {
    // Stirling's approximation: ln(n!) ≈ n*ln(n) - n + 0.5*ln(2πn)
    // Then convert to log base 2
    let ln_n = (n as f64).ln();
    let ln_factorial =
        n as f64 * ln_n - n as f64 + 0.5 * (2.0 * std::f64::consts::PI * n as f64).ln();
    ln_factorial / std::f64::consts::LN_2
}

pub fn demonstrate_decision_tree(n: usize) {
    println!("--- DECISION TREE MODEL ---");
    println!();
    println!("Any comparison-based sort can be viewed as a decision tree:");
    println!("- Each internal node represents a comparison (a < b?)");
    println!("- Each leaf represents a permutation of the input");
    println!("- The path from root to leaf is the sequence of comparisons");
    println!();

    let permutations = count_permutations(n);
    println!(
        "For n={} elements, there are {} possible orderings.",
        n, permutations
    );
    println!("Each comparison has 2 outcomes (yes/no), so a tree of depth d");
    println!("can have at most 2^d leaves.");
    println!();

    // Find minimum depth needed
    let mut depth = 0;
    while (1u64 << depth) < permutations {
        depth += 1;
    }

    println!(
        "To distinguish all {} orderings, we need at least {} comparisons.",
        permutations, depth
    );
    println!("This is log₂({}!) ≈ {} comparisons.", n, depth);

    let stirling_approx = log2_factorial_approx(n);
    println!(
        "Using Stirling's approximation: log₂({}!) ≈ {:.1}",
        n, stirling_approx
    );
    println!();
}

pub fn demonstrate_information_theoretic_argument() {
    println!("--- INFORMATION-THEORETIC ARGUMENT ---");
    println!();
    println!("Sorting requires determining which of n! permutations we have.");
    println!("Each permutation is equally likely, so the information content is:");
    println!();

    for n in 3..=8 {
        let bits = log2_factorial_approx(n);
        println!(
            "  n={}: {}! permutations → {:.1} bits of information",
            n,
            count_permutations(n),
            bits
        );
    }

    println!();
    println!("Each comparison provides at most 1 bit of information (yes/no).");
    println!("Therefore, we need at least log₂(n!) comparisons.");
    println!();
    println!("Since log₂(n!) = Θ(n log n), comparison-based sorting is Ω(n log n).");
}

pub fn demonstrate_linear_time_sorts() {
    println!("--- BUT WAIT: LINEAR-TIME SORTS EXIST! ---");
    println!();
    println!("The Ω(n log n) bound only applies to COMPARISON-BASED sorts.");
    println!("If we know more about the data, we can sort faster!");
    println!();

    println!("Counting Sort: O(n + k) where k = range of values");
    println!("  - Works when values are in a known small range");
    println!("  - Example: sorting ages (0-150) or test scores (0-100)");
    println!();

    println!("Radix Sort: O(d * n) where d = number of digits");
    println!("  - Processes numbers digit by digit");
    println!("  - Works well for fixed-length strings or integers");
    println!();

    println!("Bucket Sort: O(n) average case");
    println!("  - Distributes elements into buckets, sorts each bucket");
    println!("  - Requires uniform distribution of input");
    println!();

    // Demonstrate counting sort conceptually
    println!("Example: Counting sort for small integers");
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let max_val = *data.iter().max().unwrap();

    // Build count array
    let mut counts = vec![0; max_val + 1];
    for &x in &data {
        counts[x] += 1;
    }

    println!("Input: {:?}", data);
    println!("Counts: {:?}", counts);

    // Reconstruct sorted output
    let mut sorted = Vec::new();
    for (value, count) in counts.iter().enumerate() {
        for _ in 0..*count {
            sorted.push(value);
        }
    }

    println!("Sorted: {:?}", sorted);
}

fn demonstrate_adversarial_argument() {
    println!("--- ADVERSARIAL ARGUMENT (ALTERNATIVE PROOF) ---");
    println!();
    println!("An adversary can force any comparison sort to make Ω(n log n) comparisons.");
    println!("Strategy: always answer comparisons in a way that keeps as many");
    println!("permutations consistent as possible.");
    println!();
    println!("At each step, the adversary maintains a set of permutations consistent");
    println!("with previous answers. To minimize information revealed, the adversary");
    println!("answers so that roughly half the permutations remain viable.");
    println!();
    println!("This requires at least log₂(n!) = Ω(n log n) comparisons to narrow it");
    println!("down to a single permutation (the sorted order).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_counting() {
        assert_eq!(count_permutations(1), 1);
        assert_eq!(count_permutations(2), 2);
        assert_eq!(count_permutations(3), 6);
        assert_eq!(count_permutations(4), 24);
    }

    #[test]
    fn test_log2_factorial_approximation() {
        // log₂(3!) = log₂(6) ≈ 2.585
        let approx = log2_factorial_approx(3);
        assert!((approx - 2.585).abs() < 0.1);
    }

    #[test]
    fn test_counting_sort_concept() {
        // Verify the counting sort approach works correctly
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let max_val = *data.iter().max().unwrap();

        let mut counts = vec![0; max_val + 1];
        for &x in &data {
            counts[x] += 1;
        }

        let mut sorted = Vec::new();
        for (value, count) in counts.iter().enumerate() {
            for _ in 0..*count {
                sorted.push(value);
            }
        }

        assert_eq!(sorted, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    }
}
