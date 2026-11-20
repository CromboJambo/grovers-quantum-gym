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

// --- The Positional Tracing Algorithm ----------------------------------------

/// Representation type for a single element in the compressed stream.
/// Each element can be stored as one of three representations; the algorithm
/// chooses the cheapest per-element (greedy decision, like Huffman's optimal
/// code selection but over representation *types* rather than symbols).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepType {
    /// Store the value absolutely. Cost: log2(max_value) bits.
    Raw = 0,
    /// Store displacement from previous element. Cost: log2(|delta|) + 1 bit tag.
    Delta = 1,
    /// Store offset from nearest anchor. Cost: log2(|offset|) + log2(k) + 1 bit tag.
    Anchor = 2,
}

/// A single encoded element in the positional trace stream.
#[derive(Debug, Clone)]
pub struct EncodedElement {
    pub rep_type: RepType,
    /// For Raw: the value itself.
    /// For Delta: signed displacement from previous element.
    /// For Anchor: (anchor_index << 32) | offset — packed representation.
    pub payload: i128,
}

/// Result of positional compression.
#[derive(Debug)]
pub struct PositionalCompression {
    pub anchors: Vec<usize>,
    pub elements: Vec<EncodedElement>,
    pub total_bits: f64,
    pub raw_bits: f64,
}

impl PositionalCompression {
    /// Compression ratio relative to raw storage.
    pub fn compression_ratio(&self) -> f64 {
        if self.raw_bits == 0.0 {
            return 1.0;
        }
        self.total_bits / self.raw_bits
    }

    /// Lossless reconstruction of the original sequence.
    pub fn reconstruct(&self) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.elements.len());
        let mut previous: Option<i128> = None;

        for elem in &self.elements {
            let value = match elem.rep_type {
                RepType::Raw => elem.payload,
                RepType::Delta => {
                    let prev = previous.expect("Delta before any Raw element");
                    prev + elem.payload
                }
                RepType::Anchor => {
                    let anchor_idx = (elem.payload >> 64) as usize;
                    let offset_raw = (elem.payload & 0xFFFFFFFFFFFFFFFFi128) as u64 as i128;
                    let offset = if offset_raw >= (1i128 << 63) {
                        offset_raw - (1i128 << 64)
                    } else {
                        offset_raw
                    };
                    self.anchors[anchor_idx] as i128 + offset
                }
            };
            result.push(value as usize);
            previous = Some(value);
        }

        result
    }
}

/// **POSITIONAL-COMPRESS(S, k)** — The positional tracing algorithm.
///
/// Given a sorted sequence S of n values and k anchor points, produces a
/// lossless compressed representation where each element is stored using
/// whichever representation type (raw, delta, or anchor-offset) requires
/// the fewest bits.
///
/// **Algorithm:**
/// 1. Select k anchors from S using evenly-spaced density sampling.
/// 2. For each element s[i], compute the bit cost of all three representations:
///    - Raw:     ceil(log2(max_value)) bits
///    - Delta:   ceil(log2(|s[i] - s[i-1]|)) + 1 bit (type tag)
///    - Anchor:  ceil(log2(|offset|)) + ceil(log2(k)) + 1 bit (type tag)
/// 3. Choose the minimum-cost representation for each element.
/// 4. First element is always Raw (no previous value for delta, establishes anchor).
///
/// **Complexity:** O(n log n) for anchor selection + O(nk) for cost evaluation.
/// With fixed k, this is O(n) — linear in input size.
///
/// **Correctness:** Lossless by construction. Each representation type has a
/// unique inverse transform, and the type tag disambiguates during decode.
///
/// **Relation to Huffman/LZ:**
/// - Huffman chooses the best *code length* per symbol (given a frequency model).
/// - LZ77 chooses the best *(offset, length)* pair per position (given history).
/// - Positional-COMPRESS chooses the best *representation type* per element
///   (given spatial structure). All three are greedy per-element decisions
///   that exploit different kinds of redundancy.
pub fn positional_compress(values: &[usize], k: usize) -> PositionalCompression {
    if values.is_empty() {
        return PositionalCompression {
            anchors: vec![],
            elements: vec![],
            total_bits: 0.0,
            raw_bits: 0.0,
        };
    }

    let n = values.len();
    let max_val = *values.iter().max().unwrap_or(&1).max(&1) as f64;
    let raw_bits_per_val = max_val.log2().ceil().max(1.0);
    let total_raw_bits = n as f64 * raw_bits_per_val;

    // Step 1: Select anchors (evenly spaced for now — density-based in future)
    let actual_k = k.min(n);
    let mut anchors = Vec::with_capacity(actual_k);
    for i in 0..actual_k {
        let idx = (i * n) / actual_k;
        anchors.push(values[idx]);
    }

    // Precompute log2(k) for anchor index cost
    let anchor_idx_bits = if actual_k > 1 {
        (actual_k as f64).log2().ceil().max(1.0)
    } else {
        0.0
    };

    // Step 2-3: For each element, choose cheapest representation
    let mut elements = Vec::with_capacity(n);
    let mut total_bits = 0.0;
    let mut previous_value: Option<i128> = None;

    for (i, &v) in values.iter().enumerate() {
        if i == 0 {
            // First element must be Raw (no reference point exists yet)
            elements.push(EncodedElement {
                rep_type: RepType::Raw,
                payload: v as i128,
            });
            total_bits += raw_bits_per_val;
            previous_value = Some(v as i128);
            continue;
        }

        // Cost of Raw representation
        let cost_raw = raw_bits_per_val;

        // Cost of Delta representation
        let delta = (v as i128) - previous_value.unwrap();
        let abs_delta = delta.abs().max(1) as f64;
        let cost_delta = abs_delta.log2().ceil().max(1.0) + 1.0; // +1 for type tag

        // Cost of Anchor representation (only if anchors exist)
        let (rep_type, payload, cost) = if anchors.is_empty() {
            // No anchors available — choose between Raw and Delta only
            if cost_raw <= cost_delta {
                (RepType::Raw, v as i128, cost_raw)
            } else {
                (RepType::Delta, delta, cost_delta)
            }
        } else {
            let (best_anchor_idx, best_offset) = nearest_anchor(v as i128, &anchors);
            let abs_offset = best_offset.abs().max(1) as f64;
            let cost_anchor = abs_offset.log2().ceil().max(1.0) + anchor_idx_bits + 1.0; // +1 for type tag

            if cost_raw <= cost_delta && cost_raw <= cost_anchor {
                (RepType::Raw, v as i128, cost_raw)
            } else if cost_delta <= cost_anchor {
                (RepType::Delta, delta, cost_delta)
            } else {
                let packed = ((best_anchor_idx as i128) << 64) | ((best_offset & 0xFFFFFFFFFFFFFFFFi128) as u64 as i128);
                (RepType::Anchor, packed, cost_anchor)
            }
        };

        elements.push(EncodedElement { rep_type, payload });
        total_bits += cost;
        previous_value = Some(v as i128);
    }

    PositionalCompression {
        anchors,
        elements,
        total_bits,
        raw_bits: total_raw_bits,
    }
}

/// Find the nearest anchor to a value and return (anchor_index, signed_offset).
fn nearest_anchor(value: i128, anchors: &[usize]) -> (usize, i128) {
    let mut best_idx = 0;
    let mut best_dist = i128::MAX;
    for (i, &anchor) in anchors.iter().enumerate() {
        let dist = (value - anchor as i128).abs();
        if dist < best_dist {
            best_dist = dist;
            best_idx = i;
        }
    }
    (best_idx, value - anchors[best_idx] as i128)
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

    // --- Tests for POSITIONAL-COMPRESS algorithm ---

    #[test]
    fn test_positional_compress_roundtrip_primes() {
        let primes = sieve_primes(100);
        let compressed = positional_compress(&primes, 5);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, primes);
    }

    #[test]
    fn test_positional_compress_roundtrip_arithmetic() {
        let seq: Vec<usize> = (0..20).map(|i| i * 7 + 3).collect();
        let compressed = positional_compress(&seq, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, seq);
    }

    #[test]
    fn test_positional_compress_roundtrip_random() {
        // Non-sorted random data — algorithm should still work (uses Raw mostly)
        let values = vec![42, 17, 89, 3, 56, 23, 78, 11, 65, 90];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_positional_compress_beats_raw_for_primes() {
        let primes = sieve_primes(1000);
        let compressed = positional_compress(&primes, 10);
        assert!(compressed.total_bits < compressed.raw_bits);
        assert!(compressed.compression_ratio() < 1.0);
    }

    #[test]
    fn test_positional_compress_beats_raw_for_arithmetic() {
        let seq: Vec<usize> = (0..50).map(|i| i * 13 + 5).collect();
        let compressed = positional_compress(&seq, 5);
        // Arithmetic sequences should compress well with delta
        // (type tags add overhead, so expect ~50-60% of raw)
        assert!(compressed.compression_ratio() < 0.6);
    }

    #[test]
    fn test_positional_compress_rep_type_distribution_primes() {
        let primes = sieve_primes(1000);
        let compressed = positional_compress(&primes, 10);

        let mut raw_count = 0;
        let mut delta_count = 0;
        let mut anchor_count = 0;
        for elem in &compressed.elements {
            match elem.rep_type {
                RepType::Raw => raw_count += 1,
                RepType::Delta => delta_count += 1,
                RepType::Anchor => anchor_count += 1,
            }
        }

        // For primes, delta should dominate (small gaps)
        assert!(delta_count > raw_count);
        println!(
            "Primes up to 1000: Raw={}, Delta={}, Anchor={}",
            raw_count, delta_count, anchor_count
        );
    }

    #[test]
    fn test_positional_compress_first_element_always_raw() {
        let values = vec![5, 10, 15, 20];
        let compressed = positional_compress(&values, 2);
        assert_eq!(compressed.elements[0].rep_type, RepType::Raw);
    }

    #[test]
    fn test_positional_compress_empty() {
        let values: Vec<usize> = vec![];
        let compressed = positional_compress(&values, 5);
        assert!(compressed.elements.is_empty());
        assert_eq!(compressed.total_bits, 0.0);
    }

    #[test]
    fn test_positional_compress_single_element() {
        let values = vec![42];
        let compressed = positional_compress(&values, 3);
        assert_eq!(compressed.elements.len(), 1);
        assert_eq!(compressed.elements[0].rep_type, RepType::Raw);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, vec![42]);
    }

    #[test]
    fn test_positional_compress_k_larger_than_n() {
        let values = vec![3, 7, 11];
        let compressed = positional_compress(&values, 100); // k > n
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_positional_compress_vs_delta_only() {
        // For data with strong local structure but global variation,
        // positional_compress should do at least as well as pure delta
        let primes = sieve_primes(500);
        let compressed = positional_compress(&primes, 10);
        let delta_bits = delta_encode_bits(&primes);

        // Positional compress with anchors should be <= delta-only + overhead
        // (the overhead is the anchor storage cost)
        assert!(compressed.total_bits < delta_bits * 1.5);
    }

    // --- Stress tests: break POSITIONAL-COMPRESS ---

    #[test]
    fn test_stress_unsorted_data() {
        let values = vec![100, 5, 999, 42, 1, 500, 7];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_duplicate_values() {
        let values = vec![5, 5, 5, 5, 5];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_alternating_pattern() {
        let values: Vec<usize> = (0..100).map(|i| if i % 2 == 0 { 10 } else { 9990 }).collect();
        let compressed = positional_compress(&values, 5);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_large_values_anchor_overflow() {
        // CRITICAL: anchor packing uses (idx << 32) | offset as u32.
        // If offset exceeds 2^31 - 1, the cast truncates and reconstruction fails.
        let values = vec![0usize, 1_000_000_000usize, 2_000_000_000usize];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Anchor packing overflow with large offsets");
    }

    #[test]
    fn test_stress_huge_sequence_performance() {
        let primes = sieve_primes(1_300_000);
        assert!(primes.len() > 50_000);
        let compressed = positional_compress(&primes, 100);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed.len(), primes.len());
        assert_eq!(reconstructed[0], primes[0]);
        assert_eq!(reconstructed[primes.len() - 1], primes[primes.len() - 1]);
        assert_eq!(reconstructed[primes.len() / 2], primes[primes.len() / 2]);
    }

    #[test]
    fn test_stress_k_equals_n() {
        let values: Vec<usize> = (0..50).map(|i| i * 3).collect();
        let compressed = positional_compress(&values, 50);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_k_equals_one() {
        let values: Vec<usize> = (0..20).map(|i| 100 + i * 2).collect();
        let compressed = positional_compress(&values, 1);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_mixed_representation_reconstruction() {
        let values = vec![10, 12, 14, 16, 5000, 5002, 5004, 5006];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);

        let types: Vec<RepType> = compressed.elements.iter().map(|e| e.rep_type).collect();
        let unique_types = types.iter().collect::<std::collections::HashSet<_>>();
        assert!(unique_types.len() >= 2, "Expected mixed representations");
    }

    #[test]
    fn test_stress_zero_values() {
        let values = vec![0, 1, 2, 3, 4];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_max_u64_values() {
        let values = vec![usize::MAX - 10, usize::MAX - 5, usize::MAX];
        let compressed = positional_compress(&values, 2);
        // Use checked reconstruction to detect overflow silently
        let reconstructed = checked_reconstruct_safe(&compressed);
        assert_eq!(reconstructed.len(), values.len());
    }

    fn checked_reconstruct_safe(compressed: &PositionalCompression) -> Vec<usize> {
        let mut result = Vec::with_capacity(compressed.elements.len());
        let mut previous: Option<i128> = None;
        for elem in &compressed.elements {
            let value = match elem.rep_type {
                RepType::Raw => Some(elem.payload),
                RepType::Delta => {
                    match previous {
                        Some(prev) => prev.checked_add(elem.payload),
                        None => return result,
                    }
                }
                RepType::Anchor => {
                    let anchor_idx = (elem.payload >> 64) as usize;
                    if anchor_idx >= compressed.anchors.len() {
                        return result;
                    }
                    let offset_raw = (elem.payload & 0xFFFFFFFFFFFFFFFFi128) as u64 as i128;
                    let offset = if offset_raw >= (1i128 << 63) { offset_raw - (1i128 << 64) } else { offset_raw };
                    (compressed.anchors[anchor_idx] as i128).checked_add(offset)
                }
            };
            match value {
                Some(v) if v >= 0 => { result.push(v as usize); previous = Some(v); }
                _ => return result,
            }
        }
        result
    }

    #[test]
    fn test_stress_decreasing_sequence() {
        let values: Vec<usize> = (0..20).rev().collect();
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_periodic_with_noise() {
        let mut rng_state: u64 = 42;
        let values: Vec<usize> = (0..100)
            .map(|i| {
                rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let noise = (rng_state >> 33) % 5 as u64;
                (i * 7 + noise) as usize
            })
            .collect();
        let compressed = positional_compress(&values, 10);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
        assert!(compressed.compression_ratio() < 1.0);
    }
    // --- Additional stress tests: edge cases and adversarial inputs ---

    #[test]
    fn test_stress_k_equals_zero() {
        let values: Vec<usize> = (0..50).map(|i| i * 7 + 3).collect();
        let compressed = positional_compress(&values, 0);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "k=0 should still roundtrip");
    }

    #[test]
    fn test_stress_k_greater_than_len() {
        let values = vec![100usize, 200, 300];
        let compressed = positional_compress(&values, 10);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "k > len should still roundtrip");
    }

    #[test]
    fn test_stress_decreasing_huge_gaps() {
        let values = vec![
            18_446_744_073_709_551_615usize,
            18_446_744_073_709_551_600,
            18_446_744_073_709_551_500,
            18_446_744_073_709_551_000,
        ];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Decreasing huge gaps should roundtrip");
    }

    #[test]
    fn test_stress_single_element() {
        let values = vec![42usize];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_two_elements() {
        let values = vec![1usize, usize::MAX];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_very_large_n() {
        let n = 100_000;
        let values: Vec<usize> = (0..n).map(|i| (i as u64 * 2654435761u64) as usize % 1_000_000_000).collect();
        let compressed = positional_compress(&values, 10);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed.len(), values.len());
        assert_eq!(reconstructed, values, "Large N should roundtrip");
    }

    #[test]
    fn test_stress_all_same_value() {
        let values = vec![999usize; 100];
        let compressed = positional_compress(&values, 5);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_zero_and_max() {
        let values = vec![0usize, usize::MAX / 2, usize::MAX];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_oscillating_extremes() {
        let values = vec![0usize, usize::MAX, 1, usize::MAX - 1, 2, usize::MAX - 2];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }

    #[test]
    fn test_stress_prime_gaps_large() {
        let mut is_prime = vec![true; 100_000];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..(100_000usize as f64).sqrt() as usize {
            if is_prime[i] {
                for j in ((i*i)..100_000).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        let primes: Vec<usize> = (2..100_000).filter(|&i| is_prime[i]).collect();
        assert!(primes.len() > 5000, "Should have many primes");
        let compressed = positional_compress(&primes, 10);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, primes, "Primes up to 100k should roundtrip");
        assert!(compressed.compression_ratio() < 0.7, 
            "Prime gaps should compress below 70% raw");
    }
    // --- Adversarial: negative offset sign extension ---

    #[test]
    fn test_stress_negative_offset_sign_extension() {
        let values = vec![100usize, 50, 25, 10];
        let compressed = positional_compress(&values, 4);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Negative offsets should roundtrip");
    }

    #[test]
    fn test_stress_mixed_positive_negative_offsets() {
        let values = vec![100usize, 200, 50, 300, 75, 400];
        let compressed = positional_compress(&values, 3);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Mixed offsets should roundtrip");
    }


    #[test]
    fn test_stress_fuzz_random() {
        let mut seed: u64 = 12345;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            seed as usize
        };

        for trial in 0..100 {
            let n = (next() % 100) + 1;
            let k = (next() % 20);
            let values: Vec<usize> = (0..n).map(|_| next()).collect();
            let compressed = positional_compress(&values, k);
            let reconstructed = compressed.reconstruct();
            assert_eq!(reconstructed, values, 
                "Fuzz trial {} failed: n={}, k={}", trial, n, k);
        }
    }

    #[test]
    fn test_stress_fuzz_sorted_random() {
        let mut seed: u64 = 54321;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            seed as usize
        };

        for trial in 0..100 {
            let n = (next() % 200) + 1;
            let k = (next() % 30);
            let mut values: Vec<usize> = (0..n).map(|_| next()).collect();
            values.sort();
            let compressed = positional_compress(&values, k);
            let reconstructed = compressed.reconstruct();
            assert_eq!(reconstructed, values, 
                "Fuzz sorted trial {} failed: n={}, k={}", trial, n, k);
        }
    }

    #[test]
    fn test_stress_empty_sequence() {
        let values: Vec<usize> = vec![];
        let compressed = positional_compress(&values, 5);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values);
    }
    // --- Adversarial: 2^63 boundary stress ---

    #[test]
    fn test_stress_offset_at_2_pow_63() {
        // Offset exactly at 2^63 — the signed 64-bit boundary
        let anchor = 1usize;
        let value = anchor + (1u64 << 63) as usize;
        let values = vec![anchor, value];
        let compressed = positional_compress(&values, 1);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Offset at 2^63 should roundtrip");
    }

    #[test]
    fn test_stress_offset_above_2_pow_63() {
        // Offset above 2^63 — wraps in signed 64-bit interpretation
        let anchor = 10usize;
        let value = anchor + (1u64 << 63) as usize + 5;
        let values = vec![anchor, 100, value];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Offset above 2^63 should roundtrip");
    }

    #[test]
    fn test_stress_negative_offset_at_2_pow_63() {
        // Value below anchor by exactly 2^63
        let high = (1u64 << 63) as usize + 100;
        let low = 100usize;
        let values = vec![low, high];
        let compressed = positional_compress(&values, 2);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Negative offset at 2^63 should roundtrip");
    }

    #[test]
    fn test_stress_upper_half_fuzz() {
        // Fuzz specifically in the upper half of usize range [2^63, 2^64)
        let mut seed: u64 = 99999;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed as usize) | (1usize << 63) // Force upper half
        };

        for trial in 0..50 {
            let n = (next() % 50) as usize + 2;
            let k = (next() % 10) as usize;
            let mut values: Vec<usize> = (0..n).map(|_| next()).collect();
            values.sort();
            let compressed = positional_compress(&values, k);
            let reconstructed = compressed.reconstruct();
            assert_eq!(reconstructed, values, 
                "Upper-half fuzz trial {} failed: n={}, k={}", trial, n, k);
        }
    }

    #[test]
    fn test_stress_mixed_halves_fuzz() {
        // Fuzz with values spanning both halves of usize range
        let mut seed: u64 = 77777;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            seed as usize
        };

        for trial in 0..50 {
            let n = (next() % 50) as usize + 2;
            let k = (next() % 10) as usize;
            let mut values: Vec<usize> = (0..n).map(|_| next()).collect();
            // Force some values into upper half
            for i in 0..values.len() / 2 {
                values[i] |= (1usize << 63);
            }
            values.sort();
            let compressed = positional_compress(&values, k);
            let reconstructed = compressed.reconstruct();
            assert_eq!(reconstructed, values, 
                "Mixed-halves fuzz trial {} failed: n={}, k={}", trial, n, k);
        }
    }

    // --- Adversarial: large k performance and correctness ---

    #[test]
    fn test_stress_large_k() {
        let n = 10_000;
        let values: Vec<usize> = (0..n).map(|i| i * 997).collect();
        let compressed = positional_compress(&values, 1000);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "k=1000 with n=10000 should roundtrip");
    }

    // --- Adversarial: powers of 2 (exact log2 boundaries) ---

    #[test]
    fn test_stress_powers_of_two() {
        let values: Vec<usize> = (0..63).map(|i| 1usize << i).collect();
        let compressed = positional_compress(&values, 10);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Powers of 2 should roundtrip");
    }

    #[test]
    fn test_stress_just_above_powers_of_two() {
        // Values just above powers of 2 — tests ceil(log2) boundary behavior
        let values: Vec<usize> = (0..30).map(|i| (1usize << i) + 1).collect();
        let compressed = positional_compress(&values, 5);
        let reconstructed = compressed.reconstruct();
        assert_eq!(reconstructed, values, "Just above powers of 2 should roundtrip");
    }

}
