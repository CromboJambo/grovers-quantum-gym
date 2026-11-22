/// Jacob Ziv & Abraham Lempel - LZ77/LZ78 Compression (1977-1978)
/// The most direct mathematical embodiment of the spreadsheet offset idea:
/// instead of storing data values, store (offset, length) references back to
/// previously seen patterns. The compressed form is a trace of structure -
/// you do not need the whole snapshot, just the offsets that recreate it.
///
/// Connection to the gym:
/// - Shor period-finding: detects repeating structure in function values
/// - LZ77: detects repeating structure in data streams via offset references
/// - Both reduce "what is this?" to "where have I seen this before?"
/// - The compressed form preserves all information (lossless) while discarding redundancy


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "lemper_ziv",
    version: "0.0.1",
    formalized: false,
    notes: "Lempel-Ziv compression algorithms",
};


use std::collections::HashMap;

// --- LZ77 Compression -------------------------------------------------------

/// An LZ77 token: either a literal character or a back-reference.
#[derive(Debug, Clone, PartialEq)]
pub enum Lz77Token {
    Literal(char),
    Reference { offset: usize, length: usize },
}

impl std::fmt::Display for Lz77Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lz77Token::Literal(c) => {
                let display = if *c == ' ' { "SPACE".to_string() } else { format!("{}", c) };
                write!(f, "L({})", display)
            }
            Lz77Token::Reference { offset, length } => write!(f, "R(-{}, {})", offset, length),
        }
    }
}

/// LZ77 compress a string using a sliding window.
pub fn lz77_compress(input: &str, window_size: usize, max_match_length: usize) -> Vec<Lz77Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < chars.len() {
        let search_start = pos.saturating_sub(window_size);
        let mut best_offset = 0;
        let mut best_length = 0;

        for i in (search_start..pos).rev() {
            let mut match_len = 0;
            while pos + match_len < chars.len()
                && i + match_len < pos
                && match_len < max_match_length
                && chars[pos + match_len] == chars[i + match_len]
            {
                match_len += 1;
            }

            if match_len > best_length {
                best_length = match_len;
                best_offset = pos - i;
            }
        }

        if best_length >= 2 {
            tokens.push(Lz77Token::Reference {
                offset: best_offset,
                length: best_length,
            });
            pos += best_length;
        } else {
            tokens.push(Lz77Token::Literal(chars[pos]));
            pos += 1;
        }
    }

    tokens
}

/// Decompress LZ77 tokens back to the original string.
pub fn lz77_decompress(tokens: &[Lz77Token]) -> String {
    let mut output: Vec<char> = Vec::new();

    for token in tokens {
        match token {
            Lz77Token::Literal(c) => {
                output.push(*c);
            }
            Lz77Token::Reference { offset, length } => {
                let start = output.len() - *offset;
                for i in 0..*length {
                    let c = output[start + i];
                    output.push(c);
                }
            }
        }
    }

    output.into_iter().collect()
}

// --- LZ78 Compression -------------------------------------------------------

/// An LZ78 token: an index into the dictionary and a new character.
#[derive(Debug, Clone, PartialEq)]
pub struct Lz78Token {
    pub dict_index: usize,
    pub new_char: char,
}

impl std::fmt::Display for Lz78Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_c = if self.new_char == ' ' { String::from("SPACE") } else { format!("{}", self.new_char) };
        write!(f, "[{}, {}]", self.dict_index, display_c)
    }
}

/// LZ78 compress a string using a growing dictionary.
/// LZ78 compress a string using a growing dictionary.
/// Token encoding: dict_index == 0 means "literal character", otherwise
/// it references an existing dictionary entry to extend with new_char.
pub fn lz78_compress(input: &str) -> Vec<Lz78Token> {
    let mut tokens = Vec::new();
    let mut dict: HashMap<String, usize> = HashMap::new();
    let mut next_idx = 1; // 0 is reserved for the "literal" sentinel
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut phrase = String::new();

        // Extend phrase as long as it exists in the dictionary
        while i + phrase.len() < chars.len() {
            let candidate = format!("{}{}", phrase, chars[i + phrase.len()]);
            if dict.contains_key(&candidate) {
                phrase = candidate;
            } else {
                break;
            }
        }

        if phrase.is_empty() {
            // No dictionary match - emit literal
            let c = chars[i];
            tokens.push(Lz78Token { dict_index: 0, new_char: c });
            dict.insert(c.to_string(), next_idx);
            next_idx += 1;
            i += 1;
        } else if i + phrase.len() < chars.len() {
            // Found a match - emit reference to phrase + next char
            let next_c = chars[i + phrase.len()];
            tokens.push(Lz78Token {
                dict_index: dict[&phrase],
                new_char: next_c,
            });
            let new_phrase = format!("{}{}", phrase, next_c);
            dict.insert(new_phrase, next_idx);
            next_idx += 1;
            i += phrase.len() + 1;
        } else {
            // Phrase matches to end of input - emit as literal chars
            for &c in &chars[i..] {
                tokens.push(Lz78Token { dict_index: 0, new_char: c });
            }
            break;
        }
    }

    tokens
}

/// Decompress LZ78 tokens back to the original string.
/// Decompress LZ78 tokens back to the original string.
pub fn lz78_decompress(tokens: &[Lz78Token]) -> String {
    let mut output = String::new();
    let mut dict: Vec<String> = Vec::new(); // dict[i] holds phrase at index i+1

    for token in tokens {
        if token.dict_index == 0 {
            // Literal character
            output.push(token.new_char);
            dict.push(token.new_char.to_string());
        } else {
            // Reference to existing phrase + new char
            let prev = &dict[token.dict_index - 1]; // 1-based to 0-based
            let phrase = format!("{}{}", prev, token.new_char);
            output.push_str(&phrase);
            dict.push(phrase);
        }
    }

    output
}

// --- Offset Pattern Analysis -------------------------------------------------

/// Analyze the offset patterns in LZ77 compression.
pub fn analyze_offset_patterns(tokens: &[Lz77Token]) -> OffsetAnalysis {
    let mut literal_count = 0;
    let mut reference_count = 0;
    let mut total_copied = 0usize;
    let mut offset_histogram: HashMap<usize, usize> = HashMap::new();
    let mut length_histogram: HashMap<usize, usize> = HashMap::new();

    for token in tokens {
        match token {
            Lz77Token::Literal(_) => literal_count += 1,
            Lz77Token::Reference { offset, length } => {
                reference_count += 1;
                total_copied += *length;
                *offset_histogram.entry(*offset).or_insert(0) += 1;
                *length_histogram.entry(*length).or_insert(0) += 1;
            }
        }
    }

    let total_tokens = tokens.len();
    OffsetAnalysis {
        literal_count,
        reference_count,
        total_copied,
        total_tokens,
        offset_histogram,
        length_histogram,
        compression_ratio: if total_tokens > 0 {
            (literal_count + reference_count) as f64 / (literal_count + total_copied) as f64
        } else {
            1.0
        },
    }
}

/// Statistical summary of LZ77 offset patterns.
#[derive(Debug)]
pub struct OffsetAnalysis {
    pub literal_count: usize,
    pub reference_count: usize,
    pub total_copied: usize,
    pub total_tokens: usize,
    pub offset_histogram: HashMap<usize, usize>,
    pub length_histogram: HashMap<usize, usize>,
    pub compression_ratio: f64,
}

impl OffsetAnalysis {
    /// The most common offset - this is the period of the data.
    pub fn dominant_period(&self) -> Option<(usize, usize)> {
        self.offset_histogram
            .iter()
            .max_by_key(|&(_, count)| *count)
            .map(|(&offset, &count)| (offset, count))
    }

    /// Top N most common offsets.
    pub fn top_offsets(&self, n: usize) -> Vec<(usize, usize)> {
        let mut entries: Vec<_> = self.offset_histogram.iter().collect();
        entries.sort_by_key(|&(_, count)| *count);
        entries.into_iter().rev().take(n).map(|(&o, &c)| (o, c)).collect()
    }
}

// --- Station Demonstration ---------------------------------------------------

pub fn show_lz_station() {
    println!("=== LEMPEL-ZIV STATION: OFFSET-BASED COMPRESSION (1977-1978) ===");
    println!();
    print_biography();
    println!();
    demonstrate_lz77_offsets();
    println!();
    demonstrate_spreadsheet_connection();
    println!();
    demonstrate_period_detection();
    println!();
    demonstrate_lz78_dictionary();
    println!();
    demonstrate_connection_to_shor();
}

fn print_biography() {
    println!("Abraham Lempel (1936-2024) & Jacob Ziv (1922-2006)");
    println!("Israeli-American computer scientists; pioneers of data compression.");
    println!();
    println!("LZ77 (1977): A Universal Algorithm for Sequential Data Compression");
    println!("  - Store (offset, length) references to previously seen patterns.");
    println!("LZ78 (1978): Compression of Individual Sequences via Variable-Rate Coding");
    println!("  - Build a dictionary of phrases; reference entries by index.");
    println!();
    println!("Their insight is the most direct formalization of the spreadsheet");
    println!("analogy: you do not need to store the data itself, just the offsets");
    println!("that tell you where each piece came from. The compressed form is a");
    println!("trace of structure - a map of repetitions and relationships.");
    println!();
    println!("LZ77 became the foundation for gzip, PNG, ZIP, and DEFLATE.");
    println!("Every time you download a file over HTTP, Lempel-Ziv is working.");
}

fn demonstrate_lz77_offsets() {
    println!("--- DEMONSTRATION: LZ77 Offset Compression ---");
    println!();
    println!("LZ77 scans the input left to right. For each position, it asks:");
    println!("  Have I seen this pattern before? If so, how far back?");
    println!();
    println!("If yes -> emit Reference(offset, length): copy from offset chars back");
    println!("If no  -> emit Literal(char): write the character directly");
    println!();

    let text = "abcabcabcabc";
    println!("Input: {}", text);
    let tokens = lz77_compress(text, 12, 6);
    println!("Tokens: {}", tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" "));
    println!();
    println!("Reading the trace:");
    println!("  L(a) L(b) L(c)  - write abc (3 literals)");
    println!("  R(-3, 3)        - copy 3 chars from 3 back -> abc");
    println!("  R(-6, 3)        - copy 3 chars from 6 back -> abc");
    println!("  R(-9, 3)        - copy 3 chars from 9 back -> abc");
    println!();
    println!("Original: 12 chars = 96 bits (at 8 bits/char)");
    println!("Compressed: 6 tokens (each ~10-12 bits in practice) = 60-72 bits");
    println!("The offsets ARE the structure. You can reconstruct everything.");

    let decompressed = lz77_decompress(&tokens);
    assert_eq!(decompressed, text);
    println!("\n[OK] Roundtrip verified: decompressed == original");
}

fn demonstrate_spreadsheet_connection() {
    println!("--- DEMONSTRATION: The Spreadsheet Connection ---");
    println!();
    println!("The chat analogy: If you filter rows in a spreadsheet and resort by");
    println!("each column, the offsets tell you the shape of your data.");
    println!();
    println!("LZ77 does exactly this on a 1D stream:");
    println!("  - Each position is a row");
    println!("  - Looking back for matches is filtering by pattern");
    println!("  - The offset is how far did this move from its original position?");
    println!();

    let csv_like = "id,name,dept\n1,Alice,eng\n2,Bob,eng\n3,Carol,sales\n4,Dave,eng\n5,Eve,sales";
    println!("CSV-like data (the spreadsheet):");
    for line in csv_like.lines() {
        println!("  {}", line);
    }
    println!();

    let tokens = lz77_compress(csv_like, 64, 16);
    println!("LZ77 tokens:");
    for token in &tokens {
        println!("  {}", token);
    }

    let analysis = analyze_offset_patterns(&tokens);
    println!("\nOffset analysis:");
    println!("  Literals: {}, References: {}", analysis.literal_count, analysis.reference_count);
    println!("  Total chars copied by references: {}", analysis.total_copied);
    println!(
        "  Effective compression: {:.1}% of original size",
        analysis.compression_ratio * 100.0
    );

    if let Some((offset, count)) = analysis.dominant_period() {
        println!("\nDominant offset: {} (appears {} times)", offset, count);
        println!("This is the period of the data - the repeating structure.");
        println!("In a spreadsheet, this would be the row height or column pattern.");
    }
}

fn demonstrate_period_detection() {
    println!("--- DEMONSTRATION: Period Detection via Offsets ---");
    println!();
    println!("Here is where LZ77 connects to Shor algorithm:");
    println!("  - Shor finds the period of a modular exponential function");
    println!("  - LZ77 finds the period of a data stream via offset patterns");
    println!("  - Both reduce \"what is this?\" to \"how often does it repeat?\"");
    println!();

    let periodic = "ABCD".repeat(10);
    println!("Periodic data: {}... (period=4)", &periodic[..16]);
    let tokens = lz77_compress(&periodic, 40, 8);
    let analysis = analyze_offset_patterns(&tokens);

    if let Some((offset, count)) = analysis.dominant_period() {
        println!("Dominant offset: {} -> detected period of {}", offset, offset);
    }
    println!(
        "References: {}, total copied: {} (vs {} original chars)",
        analysis.reference_count, analysis.total_copied, periodic.len()
    );

    println!("\nData with hidden structure (alternating patterns):");
    let hidden = "xy".repeat(50);
    let tokens_hidden = lz77_compress(&hidden, 100, 10);
    let analysis_hidden = analyze_offset_patterns(&tokens_hidden);

    if let Some((offset, count)) = analysis_hidden.dominant_period() {
        println!("Dominant offset: {} -> detected period of {}", offset, offset);
    }
    println!(
        "Compression ratio: {:.1}% (periodic data compresses extremely well)",
        analysis_hidden.compression_ratio * 100.0
    );

    println!("\nRandom data (no structure - incompressible):");
    let random = "xkqzjwvbnm".repeat(5);
    let tokens_random = lz77_compress(&random, 20, 4);
    let analysis_random = analyze_offset_patterns(&tokens_random);

    println!(
        "References: {} (very few - nothing to reference)",
        analysis_random.reference_count
    );
    println!(
        "Compression ratio: {:.1}% (near 100% = incompressible)",
        analysis_random.compression_ratio * 100.0
    );

    println!("\nKEY INSIGHT:");
    println!("  Compression ratio IS a measure of structure.");
    println!("  - High compression -> strong periodic/structural patterns");
    println!("  - Low compression -> random/incompressible (high entropy)");
    println!("  This is the same duality as Shor algorithm:");
    println!("  - Periodic function -> easy to factor (Shor wins)");
    println!("  - Random function -> hard to factor (classical needed)");
}

fn demonstrate_lz78_dictionary() {
    println!("--- DEMONSTRATION: LZ78 Dictionary Compression ---");
    println!();
    println!("LZ78 takes a different approach: instead of looking back at raw data,");
    println!("it builds a dictionary of phrases. Each token says:");
    println!("  [dictionary_index, new_char] = append new_char to phrase #index");
    println!();
    println!("The dictionary IS the shape - it captures all structural patterns.");
    println!("This is like building a thesaurus of your data vocabulary.");
    println!();

    let text = "ababababab";
    println!("Input: {}", text);
    let tokens = lz78_compress(text);
    println!("Tokens: {}", tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" "));
    println!();

    let mut dict: Vec<String> = Vec::new();
    let mut phrase = String::new();
    for c in text.chars() {
        let candidate = format!("{}{}", phrase, c);
        if !dict.contains(&candidate) && !phrase.is_empty() {
            dict.push(candidate.clone());
        }
        phrase = candidate;
    }

    println!("Dictionary built during compression:");
    for (i, entry) in dict.iter().enumerate().take(10) {
        println!("  [{}] = {}", i + 1, entry);
    }

    let decompressed = lz78_decompress(&tokens);
    assert_eq!(decompressed, text);
    println!("\n[OK] Roundtrip verified: decompressed == original");
    println!();
    println!("LZ78 vs LZ77:");
    println!("  - LZ77: look back at raw data (sliding window) - simpler, faster");
    println!("  - LZ78: build a dictionary of phrases - better for long-range patterns");
    println!("  - Both are lossless: the trace preserves all information");
}

fn demonstrate_connection_to_shor() {
    println!("--- CONNECTION TO SHOR ALGORITHM ---");
    println!();
    println!("Shor algorithm factors N by finding the period r of f(x) = a^x mod N.");
    println!("The period-finding step is where quantum speedup happens.");
    println!();
    println!("LZ77 does classical period finding on data streams:");
    println!();
    println!("  Shor:      f(x + r) = f(x) for all x  ->  period r in function values");
    println!("  LZ77:      s[i..i+n] = s[i-r..i-r+n]  ->  offset r in data stream");
    println!();
    println!("Both ask the same fundamental question:");
    println!("  How far back does this pattern repeat?");
    println!();
    println!("The difference is the domain:");
    println!("  - Shor works on modular exponentiation (algebraic structure)");
    println!("  - LZ77 works on arbitrary data streams (statistical structure)");
    println!();
    println!("And the consequence:");
    println!("  - Shor period -> factors of N (breaks RSA)");
    println!("  - LZ77 periods -> compression ratio (makes files smaller)");
    println!();
    println!("Both are instances of the same principle from the chat:");
    println!("  You do not need the whole snapshot. You need the trace.");
    println!("  The trace is the set of offsets that reconstructs everything.");
}

// --- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz77_roundtrip_simple() {
        let text = "hello world";
        let tokens = lz77_compress(text, 32, 8);
        let decompressed = lz77_decompress(&tokens);
        assert_eq!(decompressed, text);
    }

    #[test]
    fn test_lz77_roundtrip_repetitive() {
        let text = "abcabcabcabc";
        let tokens = lz77_compress(text, 32, 8);
        let decompressed = lz77_decompress(&tokens);
        assert_eq!(decompressed, text);
    }

    #[test]
    fn test_lz77_repetitive_compression() {
        let text = "ababababab";
        let tokens = lz77_compress(text, 20, 6);
        let has_reference = tokens.iter().any(|t| matches!(t, Lz77Token::Reference { .. }));
        assert!(has_reference);
    }

    #[test]
    fn test_lz77_no_repetition() {
        let text = "abcdef";
        let tokens = lz77_compress(text, 10, 4);
        assert_eq!(tokens.len(), 6);
        for token in &tokens {
            assert!(matches!(token, Lz77Token::Literal(_)));
        }
    }

    #[test]
    fn test_lz78_roundtrip() {
        let text = "abababab";
        let tokens = lz78_compress(text);
        let decompressed = lz78_decompress(&tokens);
        assert_eq!(decompressed, text);
    }

    #[test]
    fn test_lz78_roundtrip_text() {
        let text = "the quick brown fox";
        let tokens = lz78_compress(text);
        let decompressed = lz78_decompress(&tokens);
        assert_eq!(decompressed, text);
    }

    #[test]
    fn test_offset_analysis_detects_period() {
        let text = "ABCD".repeat(10);
        let tokens = lz77_compress(&text, 40, 4);
        let analysis = analyze_offset_patterns(&tokens);

        if let Some((offset, _)) = analysis.dominant_period() {
            assert_eq!(offset, 4);
        } else {
            panic!("No dominant period found in periodic data");
        }
    }

    #[test]
    fn test_offset_analysis_random_data() {
        let text = "xkqzjwvbnmxcv";
        let tokens = lz77_compress(text, 10, 4);
        let analysis = analyze_offset_patterns(&tokens);
        assert!(analysis.literal_count > analysis.reference_count);
    }

    #[test]
    fn test_lz77_empty_string() {
        let tokens = lz77_compress("", 10, 4);
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_lz77_single_char() {
        let tokens = lz77_compress("a", 10, 4);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0], Lz77Token::Literal('a')));
    }

    #[test]
    fn test_lz78_empty_string() {
        let tokens = lz78_compress("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_lz77_long_range_reference() {
        let text = "xyz".repeat(3) + "abc" + "xyz";
        let tokens = lz77_compress(&text, 50, 6);
        let decompressed = lz77_decompress(&tokens);
        assert_eq!(decompressed, text);
    }

    #[test]
    fn test_lz78_growing_dictionary() {
        let text = "ababab";
        let tokens = lz78_compress(text);
        assert!(tokens.len() < text.len());
    }
}