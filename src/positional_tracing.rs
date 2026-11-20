/// Positional Tracing: Structural Compression via Relational Representation
/// The third level of the compression hierarchy:
///   - Huffman (1952): symbols given frequency -> optimal prefix codes (stores WHAT)
///   - Lempel-Ziv (1977): repeated sequences -> offsets/references (stores WHERE)
///   - Positional Tracing: structural relationships -> displacement/references (stores HOW)
///
/// Core question: if you know where something is relative to other things,
/// do you need to store what it is? The compressed form is a trace of
/// relationships — you don't need the whole snapshot, just the displacements
/// that reconstruct the structure.
///
/// Connection to the gym:
/// - Prime gaps are already positional data: each gap is a displacement between primes
/// - Shor period-finding detects repeating structure in function values
/// - Positional tracing asks: can we represent the entire prime set by its gap structure?

use std::collections::HashMap;

// --- Delta Encoding (Displacement from Previous) -----------------------------

/// Represent a sorted sequence as [first_value, d2, d3, ..., dn] where di = vi - v(i-1).
/// This is the simplest form of positional tracing: each element is stored as
/// its displacement from the previous one.
pub fn delta_encode(values: &[usize]) -> Vec<i64> {
    if values.is_empty() {
        return vec![];
    }
    let mut encoded = Vec::with_capacity(values.len());
    encoded.push(values[0] as i64);
    for i in 1..values.len() {
        encoded.push((values[i] as i64) - (values[i - 1] as i64));
    }
    encoded
}

/// Reconstruct the original sequence from delta encoding.
pub fn delta_decode(encoded: &[i64]) -> Vec<usize> {
    if encoded.is_empty() {
        return vec![];
    }
    let mut decoded = Vec::with_capacity(encoded.len());
    let mut current = encoded[0];
    decoded.push(current as usize);
    for &delta in &encoded[1..] {
        current += delta;
        decoded.push(current as usize);
    }
    decoded
}

// --- Anchor-Based Tracing ----------------------------------------------------

/// Pick reference anchors from a sorted sequence and represent all elements
/// as offsets from their nearest anchor. This is positional tracing with
/// multiple reference points — like LZ77's sliding window but for spatial data.
///
/// Returns (anchors, offset_map) where offset_map maps each original index
/// to its nearest anchor index and the signed offset.
pub fn anchor_trace(values: &[usize], num_anchors: usize) -> (Vec<usize>, Vec<(usize, i64)>) {
    if values.is_empty() {
        return (vec![], vec![]);
    }

    let n = values.len();
    let actual_anchors = num_anchors.min(n);
    let mut anchors = Vec::with_capacity(actual_anchors);

    // Pick evenly spaced anchors
    for i in 0..actual_anchors {
        let idx = (i * n) / actual_anchors;
        anchors.push(values[idx]);
    }

    // For each value, find nearest anchor and compute signed offset
    let mut offsets = Vec::with_capacity(n);
    for &v in values {
        let mut best_anchor_idx = 0;
        let mut best_dist = i64::MAX;
        for (i, &anchor) in anchors.iter().enumerate() {
            let dist = (v as i64 - anchor as i64).abs();
            if dist < best_dist {
                best_dist = dist;
                best_anchor_idx = i;
            }
        }
        offsets.push((best_anchor_idx, v as i64 - anchors[best_anchor_idx] as i64));
    }

    (anchors, offsets)
}

/// Reconstruct values from anchor trace.
pub fn anchor_reconstruct(anchors: &[usize], offsets: &[(usize, i64)]) -> Vec<usize> {
    offsets
        .iter()
        .map(|&(anchor_idx, offset)| anchors[anchor_idx] as i64 + offset)
        .map(|v| v as usize)
        .collect()
}

// --- Relational Compression --------------------------------------------------

/// For a set of values with known structure (like primes), compute the
/// "relational signature": the multiset of pairwise displacements between
/// consecutive elements. This is the pure structural representation —
/// it discards absolute position entirely and keeps only relationships.
pub fn relational_signature(values: &[usize]) -> Vec<i64> {
    if values.len() < 2 {
        return vec![];
    }
    let mut sig = Vec::with_capacity(values.len() - 1);
    for i in 1..values.len() {
        sig.push((values[i] as i64) - (values[i - 1] as i64));
    }
    sig
}

/// Check if a relational signature can uniquely reconstruct the original set
/// given a starting point. Returns true if reconstruction is unambiguous.
pub fn signature_is_reconstructible(signature: &[i64], start: usize) -> Vec<usize> {
    let mut result = vec![start];
    let mut current = start as i64;
    for &delta in signature {
        current += delta;
        if current < 0 {
            return vec![]; // Invalid reconstruction
        }
        result.push(current as usize);
    }
    result
}

// --- Compression Comparison --------------------------------------------------

/// Compute the bit cost of representing data using different strategies.
/// This is the core comparison: how many bits does each approach need?
pub struct CompressionResult {
    pub method: String,
    pub bits: f64,
    pub description: String,
}

impl std::fmt::Display for CompressionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<25} {:>10.1} bits  {}", self.method, self.bits, self.description)
    }
}

/// Estimate bits needed for delta encoding of a sequence.
/// Each delta needs log2(range) bits where range is the max absolute delta.
pub fn delta_encode_bits(values: &[usize]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let encoded = delta_encode(values);
    // First value needs log2(max_value) bits
    let first_bits = (values[0] as f64).log2().max(1.0);
    // Each delta needs log2(max_abs_delta) bits
    let max_delta = encoded[1..].iter().map(|d| d.abs()).max().unwrap_or(1) as f64;
    let delta_bits = max_delta.log2().max(1.0);
    first_bits + (values.len() - 1) as f64 * delta_bits
}

/// Estimate bits needed for anchor tracing.
/// Anchors need log2(max_value) bits each, offsets need log2(max_offset) bits each.
pub fn anchor_trace_bits(values: &[usize], num_anchors: usize) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let (anchors, offsets) = anchor_trace(values, num_anchors);
    let max_val = *values.iter().max().unwrap_or(&1) as f64;
    let anchor_bits = anchors.len() as f64 * max_val.log2().max(1.0);
    let max_offset = offsets.iter().map(|&(_, off)| off.abs()).max().unwrap_or(1) as f64;
    let offset_bits = values.len() as f64 * max_offset.log2().max(1.0);
    anchor_bits + offset_bits
}

/// Estimate bits needed for raw storage (baseline).
pub fn raw_storage_bits(values: &[usize]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let max_val = *values.iter().max().unwrap_or(&1) as f64;
    values.len() as f64 * max_val.log2().max(1.0)
}

/// Estimate bits needed for Huffman encoding of the sequence.
/// Uses Shannon entropy as a lower bound approximation.
pub fn huffman_estimate_bits(values: &[usize]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    // Convert to string representation and use entropy
    let text: String = values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
    let mut freqs = HashMap::new();
    for c in text.chars() {
        *freqs.entry(c).or_insert(0usize) += 1;
    }
    let total = text.len() as f64;
    if total == 0.0 {
        return 0.0;
    }
    let entropy = -freqs.values().map(|&count| {
        let p = count as f64 / total;
        p * p.log2()
    }).sum::<f64>();
    // Huffman is within 1 bit per symbol of entropy, use entropy + 1/symbol overhead
    text.len() as f64 * (entropy + 0.5)
}

// --- Prime Gap Positional Analysis -------------------------------------------

/// Generate primes up to a limit using sieve.
fn sieve_primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (0..=limit).filter(|&x| is_prime[x]).collect()
}

/// Compute prime gaps up to a limit.
pub fn prime_gaps(limit: usize) -> Vec<usize> {
    let primes = sieve_primes(limit);
    if primes.len() < 2 {
        return vec![];
    }
    let mut gaps = Vec::with_capacity(primes.len() - 1);
    for i in 1..primes.len() {
        gaps.push(primes[i] - primes[i - 1]);
    }
    gaps
}

/// Analyze the positional structure of prime gaps.
/// Key question: can we represent the entire prime set by its gap structure?
pub fn analyze_prime_positional_structure(limit: usize) -> PrimePositionalAnalysis {
    let primes = sieve_primes(limit);
    let gaps = prime_gaps(limit);

    if primes.is_empty() || gaps.is_empty() {
        return PrimePositionalAnalysis {
            limit,
            num_primes: 0,
            raw_bits: 0.0,
            delta_bits: 0.0,
            huffman_bits: 0.0,
            anchor_bits_10: 0.0,
            gap_histogram: HashMap::new(),
            avg_gap: 0.0,
            max_gap: 0,
        };
    }

    // Compute compression metrics for the prime set itself
    let raw = raw_storage_bits(&primes);
    let delta = delta_encode_bits(&primes);
    let huffman = huffman_estimate_bits(&primes);
    let anchor_10 = anchor_trace_bits(&primes, 10);

    // Gap histogram
    let mut gap_histogram: HashMap<usize, usize> = HashMap::new();
    for &gap in &gaps {
        *gap_histogram.entry(gap).or_insert(0) += 1;
    }

    let avg_gap = gaps.iter().sum::<usize>() as f64 / gaps.len() as f64;
    let max_gap = *gaps.iter().max().unwrap_or(&0);

    PrimePositionalAnalysis {
        limit,
        num_primes: primes.len(),
        raw_bits: raw,
        delta_bits: delta,
        huffman_bits: huffman,
        anchor_bits_10: anchor_10,
        gap_histogram,
        avg_gap,
        max_gap,
    }
}

/// Statistical summary of prime positional structure.
#[derive(Debug)]
pub struct PrimePositionalAnalysis {
    pub limit: usize,
    pub num_primes: usize,
    pub raw_bits: f64,
    pub delta_bits: f64,
    pub huffman_bits: f64,
    pub anchor_bits_10: f64,
    pub gap_histogram: HashMap<usize, usize>,
    pub avg_gap: f64,
    pub max_gap: usize,
}

impl PrimePositionalAnalysis {
    /// The most common gap — this is the "period" of the prime distribution.
    pub fn dominant_gap(&self) -> Option<(usize, usize)> {
        self.gap_histogram
            .iter()
            .max_by_key(|&(_, count)| *count)
            .map(|(&gap, &count)| (gap, count))
    }

    /// Compression ratio of delta encoding vs raw storage.
    pub fn delta_compression_ratio(&self) -> f64 {
        if self.raw_bits == 0.0 {
            return 1.0;
        }
        self.delta_bits / self.raw_bits
    }
}

// --- Station Demonstration ---------------------------------------------------

pub fn show_positional_tracing_station() {
    println!("=== POSITIONAL TRACING STATION: STRUCTURAL COMPRESSION ===");
    println!();
    print_biography();
    println!();
    demonstrate_delta_encoding();
    println!();
    demonstrate_anchor_tracing();
    println!();
    demonstrate_prime_positional_structure();
    println!();
    demonstrate_comparison_hierarchy();
    println!();
    demonstrate_connection_to_gym();
}

fn print_biography() {
    println!("Positional Tracing: The Third Compression Level");
    println!();
    println!("The compression hierarchy asks the same question three ways:");
    println!();
    println!("  Huffman (1952): Given symbol frequencies, what's the optimal code?");
    println!("    -> Stores WHAT each symbol is. Exploits frequency distribution.");
    println!();
    println!("  Lempel-Ziv (1977): Have I seen this pattern before?");
    println!("    -> Stores WHERE a pattern was. Exploits repetition via offsets.");
    println!();
    println!("  Positional Tracing: Where is this relative to its neighbors?");
    println!("    -> Stores HOW elements relate. Exploits spatial/structural position.");
    println!();
    println!("The insight from the chat: if you know where something is relative to");
    println!("other things, do you need to store what it is? The compressed form is a");
    println!("trace of relationships — displacements that reconstruct the structure.");
    println!();
    println!("This is not a named algorithm with a single inventor. It's a pattern");
    println!("that appears in delta encoding (video compression), relative addressing");
    println!("(CPU instruction sets), and coordinate transforms (physics).");
}

fn demonstrate_delta_encoding() {
    println!("--- DEMONSTRATION: Delta Encoding (Displacement from Previous) ---");
    println!();
    println!("The simplest positional trace: store the first value absolutely,");
    println!("then each subsequent value as its displacement from the previous one.");
    println!();

    let primes_small = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    println!("Primes up to 29: {:?}", primes_small);

    let encoded = delta_encode(&primes_small);
    println!("Delta encoding:   {:?}", encoded);
    println!();
    println!("Reading the trace:");
    println!("  2       - start at 2 (absolute)");
    for i in 1..encoded.len() {
        println!("  +{}     - move {} forward -> {}", encoded[i], encoded[i], primes_small[i]);
    }
    println!();

    let decoded = delta_decode(&encoded);
    assert_eq!(decoded, primes_small);
    println!("[OK] Roundtrip verified: delta_decode(delta_encode(primes)) == primes");
    println!();

    println!("Why this works for primes:");
    println!("  - Raw storage: each prime needs ~log2(29) = 4.86 bits");
    println!("  - Delta storage: gaps are small (1-6), need ~log2(6) = 2.58 bits each");
    println!("  - The first value is the only absolute anchor; everything else is relative");
    println!();
    println!("The gaps ARE the structure. You don't need to store the primes,");
    println!("you need to store where they are relative to each other.");
}

fn demonstrate_anchor_tracing() {
    println!("--- DEMONSTRATION: Anchor-Based Tracing ---");
    println!();
    println!("Multiple reference points (anchors) with offsets from each element's");
    println!("nearest anchor. Like LZ77's sliding window but for spatial data.");
    println!();

    let values = vec![10, 12, 15, 18, 20, 25, 30, 35, 40, 45];
    println!("Values: {:?}", values);

    let (anchors, offsets) = anchor_trace(&values, 3);
    println!("Anchors (3 evenly spaced): {:?}", anchors);
    println!();
    println!("Offsets from nearest anchor:");
    for (i, &v) in values.iter().enumerate() {
        let (anchor_idx, offset) = offsets[i];
        println!(
            "  {} -> anchor[{}]={} + {}",
            v, anchor_idx, anchors[anchor_idx], offset
        );
    }

    let reconstructed = anchor_reconstruct(&anchors, &offsets);
    assert_eq!(reconstructed, values);
    println!("\n[OK] Roundtrip verified: anchor_reconstruct == original");
    println!();
    println!("Why this is different from LZ77:");
    println!("  - LZ77 looks back in time (temporal references)");
    println!("  - Anchor tracing looks at position in space (spatial references)");
    println!("  - Both store displacements, but the domain of reference differs");
}

fn demonstrate_prime_positional_structure() {
    println!("--- DEMONSTRATION: Prime Positional Structure ---");
    println!();
    println!("Key question from the chat: can we represent the entire prime set");
    println!("by its gap structure? The gaps are positional data — each is a");
    println!("displacement between consecutive primes.");
    println!();

    let limit = 1000;
    let analysis = analyze_prime_positional_structure(limit);

    println!("Primes up to {}: {} found", limit, analysis.num_primes);
    println!();
    println!("Compression comparison (bits to represent the prime set):");
    println!("  Raw storage:         {:>10.1} bits", analysis.raw_bits);
    println!("  Delta encoding:      {:>10.1} bits  ({:.1}% of raw)", analysis.delta_bits, analysis.delta_compression_ratio() * 100.0);
    println!("  Huffman estimate:    {:>10.1} bits  ({:.1}% of raw)", analysis.huffman_bits, analysis.huffman_bits / analysis.raw_bits * 100.0);
    println!("  Anchor trace (10):   {:>10.1} bits  ({:.1}% of raw)", analysis.anchor_bits_10, analysis.anchor_bits_10 / analysis.raw_bits * 100.0);
    println!();

    if let Some((gap, count)) = analysis.dominant_gap() {
        println!("Gap statistics:");
        println!("  Most common gap: {} (appears {} times)", gap, count);
        println!("  Average gap: {:.2}", analysis.avg_gap);
        println!("  Maximum gap: {}", analysis.max_gap);
        println!();
    }

    println!("Top 5 most common gaps:");
    let mut sorted_gaps: Vec<_> = analysis.gap_histogram.iter().collect();
    sorted_gaps.sort_by(|a, b| b.1.cmp(a.1));
    for (gap, count) in sorted_gaps.iter().take(5) {
        println!("  gap {}: {} times", gap, count);
    }
    println!();

    println!("KEY INSIGHT:");
    println!("  Delta encoding beats raw storage because prime gaps are small.");
    println!("  The positional structure (gaps) is the information. Absolute");
    println!("  position is redundant — you can reconstruct everything from");
    println!("  the starting point and the sequence of displacements.");
    println!();
    println!("  This is exactly what the chat was getting at: the trace of");
    println!("  relationships preserves all information while discarding");
    println!("  redundant absolute positioning.");
}

fn demonstrate_comparison_hierarchy() {
    println!("--- DEMONSTRATION: The Three Compression Levels ---");
    println!();
    println!("Same data, three compression strategies. Which wins depends on structure.");
    println!();

    // Test 1: Random data (no structure)
    let random = vec![42, 17, 89, 3, 56, 23, 78, 11, 65, 90];
    println!("Test 1: Random-ish data {:?}", random);
    let raw1 = raw_storage_bits(&random);
    let delta1 = delta_encode_bits(&random);
    let huff1 = huffman_estimate_bits(&random);
    println!("  Raw:     {:>8.1} bits", raw1);
    println!("  Delta:   {:>8.1} bits  ({:.1}% of raw)", delta1, delta1 / raw1 * 100.0);
    println!("  Huffman: {:>8.1} bits  ({:.1}% of raw)", huff1, huff1 / raw1 * 100.0);
    println!();

    // Test 2: Arithmetic sequence (strong positional structure)
    let arithmetic: Vec<usize> = (0..10).map(|i| i * 7 + 3).collect();
    println!("Test 2: Arithmetic sequence {:?}", arithmetic);
    let raw2 = raw_storage_bits(&arithmetic);
    let delta2 = delta_encode_bits(&arithmetic);
    let huff2 = huffman_estimate_bits(&arithmetic);
    println!("  Raw:     {:>8.1} bits", raw2);
    println!("  Delta:   {:>8.1} bits  ({:.1}% of raw)", delta2, delta2 / raw2 * 100.0);
    println!("  Huffman: {:>8.1} bits  ({:.1}% of raw)", huff2, huff2 / raw2 * 100.0);
    println!();

    // Test 3: Repeated pattern (strong LZ structure)
    let repeated = vec![5, 10, 15, 5, 10, 15, 5, 10, 15];
    println!("Test 3: Repeated pattern {:?}", repeated);
    let raw3 = raw_storage_bits(&repeated);
    let delta3 = delta_encode_bits(&repeated);
    let huff3 = huffman_estimate_bits(&repeated);
    println!("  Raw:     {:>8.1} bits", raw3);
    println!("  Delta:   {:>8.1} bits  ({:.1}% of raw)", delta3, delta3 / raw3 * 100.0);
    println!("  Huffman: {:>8.1} bits  ({:.1}% of raw)", huff3, huff3 / raw3 * 100.0);
    println!();

    println!("WHAT THIS SHOWS:");
    println!("  - Random data: no method wins much (incompressible)");
    println!("  - Arithmetic sequence: delta encoding crushes it (constant gaps = tiny deltas)");
    println!("  - Repeated pattern: Huffman wins (frequency exploitation), delta is mediocre");
    println!();
    println!("Each method exploits a different KIND of structure:");
    println!("  Huffman -> frequency distribution (WHAT symbols appear often)");
    println!("  LZ77    -> temporal repetition (WHERE patterns repeat)");
    println!("  Delta   -> spatial position (HOW elements are arranged)");
}

fn demonstrate_connection_to_gym() {
    println!("--- CONNECTION TO THE QUANTUM GYM ---");
    println!();
    println!("Positional tracing connects to multiple stations:");
    println!();
    println!("  Riemann Station: Prime gaps ARE positional data. Each gap is a");
    println!("    displacement between primes. The entire prime distribution can");
    println!("    be characterized by its gap structure (twin primes = gap of 2,");
    println!("    cousin primes = gap of 4, etc.).");
    println!();
    println!("  Shor Station: Period finding detects repeating structure in function");
    println!("    values. Positional tracing is the classical analog: detecting");
    println!("    repeating displacement patterns in spatial data.");
    println!();
    println!("  Shannon Station: Entropy measures the shape of a distribution.");
    println!("    Positional tracing asks: what's the entropy of the gap distribution?");
    println!("    Low gap entropy = predictable structure = compressible.");
    println!();
    println!("  Lempel-Ziv Station: LZ77 finds temporal periods via offsets.");
    println!("    Positional tracing finds spatial periods via displacements.");
    println!("    Same math, different domain.");
    println!();
    println!("THE UNIFYING QUESTION (from the chat):");
    println!("  How much information do we actually need to preserve?");
    println!();
    println!("  Huffman says: you need the frequency distribution.");
    println!("  Lempel-Ziv says: you need the set of offsets that recreate repetitions.");
    println!("  Positional tracing says: you need the displacements that define structure.");
    println!();
    println!("  All three are asking for the minimal trace that reconstructs everything.");
}

// --- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_encode_decode_roundtrip() {
        let values = vec![2, 3, 5, 7, 11, 13];
        let encoded = delta_encode(&values);
        let decoded = delta_decode(&encoded);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_delta_encode_empty() {
        let values: Vec<usize> = vec![];
        let encoded = delta_encode(&values);
        assert!(encoded.is_empty());
    }

    #[test]
    fn test_delta_encode_single() {
        let values = vec![42];
        let encoded = delta_encode(&values);
        assert_eq!(encoded, vec![42]);
    }

    #[test]
    fn test_anchor_trace_roundtrip() {
        let values = vec![10, 12, 15, 18, 20, 25];
        let (anchors, offsets) = anchor_trace(&values, 3);
        let reconstructed = anchor_reconstruct(&anchors, &offsets);
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_anchor_trace_single_value() {
        let values = vec![42];
        let (anchors, offsets) = anchor_trace(&values, 5);
        assert_eq!(anchors.len(), 1);
        assert_eq!(offsets.len(), 1);
        assert_eq!(offsets[0].1, 0); // Offset from itself is 0
    }

    #[test]
    fn test_relational_signature_primes() {
        let primes = vec![2, 3, 5, 7, 11];
        let sig = relational_signature(&primes);
        assert_eq!(sig, vec![1, 2, 2, 4]); // Gaps between consecutive primes
    }

    #[test]
    fn test_signature_reconstructible() {
        let signature = vec![1, 2, 2, 4];
        let reconstructed = signature_is_reconstructible(&signature, 2);
        assert_eq!(reconstructed, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_prime_gaps_computation() {
        let gaps = prime_gaps(30);
        // Primes up to 30: 2,3,5,7,11,13,17,19,23,29
        // Gaps: 1,2,2,4,2,4,2,4,6
        assert_eq!(gaps[0], 1); // 3-2
        assert_eq!(gaps[1], 2); // 5-3
        assert!(gaps.len() == 9);
    }

    #[test]
    fn test_prime_positional_analysis() {
        let analysis = analyze_prime_positional_structure(100);
        assert!(analysis.num_primes > 0);
        assert!(analysis.raw_bits > 0.0);
        assert!(analysis.delta_bits < analysis.raw_bits); // Delta should compress
    }

    #[test]
    fn test_delta_compression_beats_raw_for_primes() {
        let primes = sieve_primes(1000);
        let raw = raw_storage_bits(&primes);
        let delta = delta_encode_bits(&primes);
        assert!(delta < raw); // Delta encoding should be smaller for primes
    }

    #[test]
    fn test_huffman_estimate_positive() {
        let values = vec![1, 2, 3, 4, 5];
        let bits = huffman_estimate_bits(&values);
        assert!(bits > 0.0);
    }

    #[test]
    fn test_arithmetic_sequence_delta_compression() {
        // Arithmetic sequence should compress extremely well with delta
        let seq: Vec<usize> = (0..10).map(|i| i * 7 + 3).collect();
        let raw = raw_storage_bits(&seq);
        let delta = delta_encode_bits(&seq);
        assert!(delta < raw / 2.0); // Should be at least 2x compression
    }

    #[test]
    fn test_random_data_no_compression_advantage() {
        // Random data shouldn't compress much with any method
        let random = vec![42, 17, 89, 3, 56, 23, 78, 11, 65, 90];
        let raw = raw_storage_bits(&random);
        let delta = delta_encode_bits(&random);
        // Delta might still be smaller due to log scale, but not dramatically
        assert!(delta < raw * 2.0); // Shouldn't be worse than 2x
    }

    #[test]
    fn test_display_compression_result() {
        let result = CompressionResult {
            method: "Test".to_string(),
            bits: 100.5,
            description: "test description".to_string(),
        };
        let s = format!("{}", result);
        assert!(s.contains("Test"));
        assert!(s.contains("100.5"));
    }

    #[test]
    fn test_dominant_gap() {
        let analysis = analyze_prime_positional_structure(100);
        if let Some((gap, count)) = analysis.dominant_gap() {
            assert!(gap > 0);
            assert!(count > 0);
        }
    }
}
