/// Claude Shannon Information Theory (1948)
/// The mathematical foundation for shape vs content: entropy measures the minimal
/// description length of a source - its structural shape - independent of what the
/// symbols actually mean.
///
/// Connection to the gym:
/// - Shor period-finding detects repeating structure in function values
/// - Shannon entropy measures how much structure (redundancy) exists in a source
/// - Both ask: what is the minimal representation that preserves the essential pattern?

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// --- Entropy ---------------------------------------------------------------

/// Calculate Shannon entropy H(X) = -sum(p_i * log2(p_i)) for a string.
pub fn string_entropy(s: &str) -> f64 {
    let mut freqs = HashMap::new();
    for c in s.chars() {
        *freqs.entry(c).or_insert(0usize) += 1;
    }
    let total = s.chars().count() as f64;
    if total == 0.0 {
        return 0.0;
    }
    -freqs.values()
        .map(|&count| {
            let p = count as f64 / total;
            p * p.log2()
        })
        .sum::<f64>()
}

/// Calculate entropy from a frequency map directly.
pub fn entropy_from_frequencies(freqs: &HashMap<char, usize>) -> f64 {
    let total: usize = freqs.values().sum();
    if total == 0 {
        return 0.0;
    }
    let total_f = total as f64;
    -freqs.values()
        .map(|&count| {
            let p = count as f64 / total_f;
            p * p.log2()
        })
        .sum::<f64>()
}

// --- Huffman Coding --------------------------------------------------------

/// A node in the Huffman tree.
#[derive(Debug, Clone)]
enum HuffmanNode {
    Leaf { symbol: char, freq: usize },
    Internal {
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
        freq: usize,
    },
}

/// Priority queue item for Huffman tree construction.
struct HeapItem {
    priority: usize,
    node: Box<HuffmanNode>,
}

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for HeapItem {}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

/// Build a Huffman tree from symbol frequencies.
fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Option<Box<HuffmanNode>> {
    let mut heap = BinaryHeap::new();

    for (&symbol, &freq) in frequencies {
        heap.push(HeapItem {
            priority: freq,
            node: Box::new(HuffmanNode::Leaf { symbol, freq }),
        });
    }

    if heap.len() == 1 {
        return Some(heap.pop().unwrap().node);
    }

    while heap.len() > 1 {
        let right = heap.pop().unwrap();
        let left = heap.pop().unwrap();
        let combined_freq = left.priority + right.priority;
        let combined = Box::new(HuffmanNode::Internal {
            left: left.node,
            right: right.node,
            freq: combined_freq,
        });
        heap.push(HeapItem {
            priority: combined_freq,
            node: combined,
        });
    }

    heap.pop().map(|item| item.node)
}

/// Generate Huffman codes from the tree.
fn generate_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { symbol, .. } => {
            let code = if prefix.is_empty() { "0".to_string() } else { prefix };
            codes.insert(*symbol, code);
        }
        HuffmanNode::Internal { left, right, .. } => {
            generate_codes(left, format!("{}0", prefix), codes);
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}

/// Encode a string using Huffman coding. Returns (code_table, compressed_bits, tree).
pub fn huffman_encode(s: &str) -> (HashMap<char, String>, String, Option<Box<HuffmanNode>>) {
    let mut freqs = HashMap::new();
    for c in s.chars() {
        *freqs.entry(c).or_insert(0usize) += 1;
    }

    if freqs.is_empty() {
        return (HashMap::new(), String::new(), None);
    }
    if freqs.len() == 1 {
        let symbol = *freqs.keys().next().unwrap();
        let mut codes = HashMap::new();
        codes.insert(symbol, "0".to_string());
        let compressed: String = s.chars().map(|_| "0").collect();
        let tree = Some(Box::new(HuffmanNode::Leaf { symbol, freq: 1 }));
        return (codes, compressed, tree);
    }

    let tree = build_huffman_tree(&freqs).expect("non-empty frequencies");
    let mut codes = HashMap::new();
    generate_codes(&tree, String::new(), &mut codes);

    let compressed: String = s
        .chars()
        .filter_map(|c| codes.get(&c).map(|code| code.as_str()))
        .collect();
    (codes, compressed, Some(tree))
}

/// Decode a Huffman-encoded bitstring back to the original string.
pub fn huffman_decode(bits: &str, tree: &HuffmanNode) -> String {
    let mut result = String::new();
    let mut current = tree;

    for bit in bits.chars() {
        match current {
            HuffmanNode::Internal { left, right, .. } => {
                current = match bit {
                    '0' => left.as_ref(),
                    _ => right.as_ref(),
                };
                // Emit immediately when we land on a leaf
                if let HuffmanNode::Leaf { symbol, .. } = current {
                    result.push(*symbol);
                    current = tree;
                }
            }
            HuffmanNode::Leaf { .. } => {
                // Shouldn't happen (we reset after emitting), but be safe
                current = tree;
            }
        }
    }

    result
}

// --- Station Demonstration --------------------------------------------------

pub fn show_shannon_station() {
    println!("=== SHANNON STATION: INFORMATION THEORY (1948) ===");
    println!();
    print_biography();
    println!();
    demonstrate_entropy();
    println!();
    demonstrate_huffman();
    println!();
    demonstrate_shape_vs_content();
    println!();
    demonstrate_connection_to_quantum();
}

fn print_biography() {
    println!("Claude Shannon (1916-2001)");
    println!("Father of information theory. 1948 paper: A Mathematical Theory of Communication");
    println!();
    println!("His insight: you can separate the SHAPE of information from its CONTENT.");
    println!("Entropy H(X) = -sum(p_i * log2(p_i)) measures the minimal number of bits");
    println!("needed per symbol - the irreducible structural complexity of a source.");
    println!();
    println!("This is the theoretical foundation for everything from gzip to DNA sequencing,");
    println!("from error-correcting codes to neural network regularization.");
}

fn demonstrate_entropy() {
    println!("--- DEMONSTRATION: Entropy as Shape Measurement ---");
    println!();
    println!("Entropy measures the SHAPE of a probability distribution:");
    println!("  - Uniform (flat) = maximum entropy = maximum uncertainty");
    println!("  - Peaked (concentrated) = low entropy = predictable structure");
    println!();

    let uniform = "abcde".repeat(20);
    let h_uniform = string_entropy(&uniform);
    println!("Uniform source abcde x 20 (each symbol p=0.2):");
    println!("  H = {:.4} bits/symbol (maximum for alphabet size 5)", h_uniform);
    println!("  Shape: flat - every symbol equally likely, maximum uncertainty");
    println!();

    let english = "the quick brown fox jumps over the lazy dog";
    let h_english = string_entropy(english);
    println!("English sentence ({} chars):", english.len());
    println!("  H = {:.4} bits/symbol", h_english);
    println!("  Shape: intermediate - structure exists but not trivial");
    println!();

    let biased_binary = format!("{}{}", "0".repeat(95), "1".repeat(5));
    let h_binary = string_entropy(&biased_binary);
    println!("Binary source 0x95 + 1x5 (p_0=0.95, p_1=0.05):");
    println!("  H = {:.4} bits/symbol", h_binary);
    println!("  Shape: very peaked - almost always 0, barely any information");
    println!();

    println!("KEY INSIGHT (from the chat): You do not need to know WHAT the symbols are.");
    println!("The SHAPE of their distribution tells you everything about compressibility.");
}

fn demonstrate_huffman() {
    println!("--- DEMONSTRATION: Huffman Coding - Exploiting Shape ---");
    println!();
    println!("Huffman coding assigns short codes to frequent symbols and long codes");
    println!("to rare ones. It directly exploits the SHAPE of the frequency distribution.");
    println!();

    let text = "the quick brown fox";
    println!("Original text: \"{}\" ({} chars = {} bits)", text, text.len(), text.len() * 8);

    let (codes, compressed, tree) = huffman_encode(text);

    println!("\nHuffman codes (sorted by length):");
    let mut sorted_codes: Vec<_> = codes.iter().collect();
    sorted_codes.sort_by_key(|(_, code)| code.len());
    for (symbol, code) in &sorted_codes {
        let display_sym = if **symbol == ' ' {
            "SPACE".to_string()
        } else {
            symbol.to_string()
        };
        println!("  {:5} -> {} ({})", display_sym, code, code.len());
    }

    println!("\nCompressed bitstring: {}", compressed);
    println!("Compressed length: {} bits", compressed.len());
    println!(
        "Compression ratio: {:.1}x ({} bits -> {} bits)",
        (text.len() * 8) as f64 / compressed.len() as f64,
        text.len() * 8,
        compressed.len()
    );

    if let Some(ref t) = tree {
        let decoded = huffman_decode(&compressed, t);
        assert_eq!(decoded, text);
        println!("\n[OK] Roundtrip verified: decoded == original");
    }
}

fn demonstrate_shape_vs_content() {
    println!("--- DEMONSTRATION: Shape vs Content ---");
    println!();
    println!("Two completely different strings can have identical entropy shapes:");
    println!();

    let s1 = "ababababababababab";
    let s2 = "xyxyxyxyxyxyxyxyxy";
    let h1 = string_entropy(s1);
    let h2 = string_entropy(s2);
    println!("s1 = \"{}...\"  H = {:.4}", &s1[..10], h1);
    println!("s2 = \"{}...\"  H = {:.4}", &s2[..10], h2);
    println!("Different CONTENT, identical SHAPE.");
    println!();

    let repetitive = "ab".repeat(50);
    let h_rep = string_entropy(&repetitive);
    println!("Entropy: {:.4} bits/symbol - low because the shape is peaked", h_rep);
    println!();
    println!("The repeated ab pattern creates a peaked frequency distribution.");
    println!("Huffman exploits this shape to assign short codes to common symbols.");
}

fn demonstrate_connection_to_quantum() {
    println!("--- CONNECTION TO QUANTUM ALGORITHMS ---");
    println!();
    println!("Shannon entropy and quantum algorithms share a deep connection:");
    println!();
    println!("  Shannon: H(X) = -sum(p_i * log2(p_i))  - classical uncertainty");
    println!("  Von Neumann: S(rho) = -Tr(rho * log2(rho))  - quantum entropy");
    println!();
    println!("Both measure the SHAPE of a probability distribution:");
    println!("  - Pure state / deterministic source -> entropy = 0 (fully known)");
    println!("  - Mixed state / random source -> entropy > 0 (uncertainty)");
    println!();
    println!("Shor algorithm works because modular exponentiation creates a");
    println!("PERIODIC shape in the probability distribution after Fourier transform.");
    println!("The quantum Fourier transform is essentially measuring the SHAPE");
    println!("of the distribution in frequency space - exactly what Shannon");
    println!("told us to do with his entropy and information measures.");
    println!();
    println!("Grover algorithm amplifies amplitude toward the solution - it");
    println!("reshapes the probability distribution from uniform (high entropy)");
    println!("to peaked on the answer (low entropy). Each Grover iteration");
    println!("reduces entropy by rotating the state vector.");
}

// --- Tests ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_uniform() {
        let uniform = "abcde".repeat(20);
        let h = string_entropy(&uniform);
        assert!((h - 2.3219).abs() < 0.001);
    }

    #[test]
    fn test_entropy_deterministic() {
        let s = "aaaaaa";
        let h = string_entropy(s);
        assert!(h.abs() < 0.0001);
    }

    #[test]
    fn test_entropy_binary_50_50() {
        let s = "01".repeat(50);
        let h = string_entropy(&s);
        assert!((h - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_huffman_roundtrip() {
        let text = "the quick brown fox jumps over the lazy dog";
        let (codes, compressed, _tree) = huffman_encode(text);
        assert!(!compressed.is_empty());
        let codes_vec: Vec<_> = codes.values().collect();
        for (i, c1) in codes_vec.iter().enumerate() {
            for (j, c2) in codes_vec.iter().enumerate() {
                if i != j {
                    assert!(!c2.starts_with(c1.as_str()), "Code {} is prefix of {}", c1, c2);
                }
            }
        }
    }

    #[test]
    fn test_huffman_compression_ratio() {
        let text = "ababababababababab";
        let (_codes, compressed, _tree) = huffman_encode(text);
        assert!(compressed.len() < text.len() * 8);
    }

    #[test]
    fn test_huffman_decode_roundtrip() {
        let text = "hello world";
        let (_codes, compressed, tree) = huffman_encode(text);
        if let Some(ref tree) = tree {
            let decoded = huffman_decode(&compressed, tree);
            assert_eq!(decoded, text);
        }
    }

    #[test]
    fn test_entropy_from_frequencies() {
        let mut freqs = HashMap::new();
        freqs.insert('a', 50);
        freqs.insert('b', 50);
        let h = entropy_from_frequencies(&freqs);
        assert!((h - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_huffman_single_char() {
        let text = "aaaa";
        let (_codes, compressed, _tree) = huffman_encode(text);
        assert_eq!(compressed.len(), 4);
    }

    #[test]
    fn test_huffman_empty() {
        let (codes, compressed, _tree) = huffman_encode("");
        assert!(codes.is_empty());
        assert!(compressed.is_empty());
    }

    #[test]
    fn test_shape_invariance() {
        let s1 = "ab".repeat(25);
        let s2 = "xy".repeat(25);
        assert!((string_entropy(&s1) - string_entropy(&s2)).abs() < 0.0001);
    }
}