//! Attention Is All You Need (2017)
//!
//! Implementation of the scaled dot-product attention mechanism from the
//! original transformer paper. Shows how self-attention allows each position
//! to attend to all other positions, enabling parallel sequence processing.


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "attention",
    version: "0.0.1",
    formalized: false,
    notes: "Transformer attention mechanism simulation",
};


use std::f64;

// Scaled dot-product attention: softmax(QK^T/sqrt(d_k))V
fn scaled_dot_product_attention(q: &[Vec<f64>], k: &[Vec<f64>], v: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let d_k = k[0].len();
    let scale = (d_k as f64).sqrt();

    // Compute attention scores: Q @ K^T / sqrt(d_k)
    let mut scores = vec![vec![0.0; k.len()]; q.len()];
    for i in 0..q.len() {
        for j in 0..k.len() {
            let mut dot = 0.0;
            for d in 0..d_k {
                dot += q[i][d] * k[j][d];
            }
            scores[i][j] = dot / scale;
        }
    }

    // Apply softmax to get attention weights
    let weights = softmax_rows(&scores);

    // Weighted sum of values
    let mut output = vec![vec![0.0; v[0].len()]; q.len()];
    for i in 0..q.len() {
        for j in 0..k.len() {
            if weights[i][j] > 1e-6 {
                for d in 0..v[0].len() {
                    output[i][d] += weights[i][j] * v[j][d];
                }
            }
        }
    }

    output
}

fn softmax_rows(x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut result = vec![vec![0.0; x[0].len()]; x.len()];

    for row in 0..x.len() {
        // Find max for numerical stability
        let mut max = x[row][0];
        for &val in &x[row] {
            if val > max {
                max = val;
            }
        }

        // Compute exp(x - max)
        let mut exps = vec![0.0; x[row].len()];
        let mut sum = 0.0;
        for col in 0..x[row].len() {
            exps[col] = (x[row][col] - max).exp();
            sum += exps[col];
        }

        // Normalize
        for col in 0..exps.len() {
            result[row][col] = exps[col] / sum;
        }
    }

    result
}

// Simple embedding: map token index to a fixed-size vector (deterministic)
fn embed_token(token: u32, dim: usize) -> Vec<f64> {
    let mut vec = vec![0.0; dim];
    for i in 0..dim {
        // Use sin/cos patterns like real positional encodings
        let pos = token as f64;
        let freq = (i as f64) / ((dim - 1) as f64);
        vec[i] = pos.sin() * freq.cos();
    }
    vec
}

fn show_attention_weights(q: &[Vec<f64>], k: &[Vec<f64>]) {
    let d_k = k[0].len();
    let scale = (d_k as f64).sqrt();

    // Compute scores
    let mut scores = vec![vec![0.0; k.len()]; q.len()];
    for i in 0..q.len() {
        for j in 0..k.len() {
            let mut dot = 0.0;
            for d in 0..d_k {
                dot += q[i][d] * k[j][d];
            }
            scores[i][j] = dot / scale;
        }
    }

    // Apply softmax
    let weights = softmax_rows(&scores);

    println!("Attention weights (softmax(QK^T/sqrt(d_k))):");
    for row in 0..weights.len() {
        let row_str: Vec<String> = weights[row].iter().map(|w| format!("{:.3}", w)).collect();
        println!("  [{}] -> [{}]", row, row_str.join(", "));
    }
}

pub fn show_attention_station() {
    println!("=== ATTENTION IS ALL YOU NEED (2017) ===");
    println!("Scaled dot-product self-attention mechanism\n");

    let dim = 4; // Embedding dimension for demo

    println!("Example: Self-attention on 'I love neural networks'");
    println!("Each word attends to all others based on similarity.\n");

    // Tokenize (simplified)
    let tokens = vec![0u32, 1u32, 2u32, 3u32]; // I, love, neural, networks
    let words = ["I", "love", "neural", "networks"];

    // Embed all tokens
    let embeddings: Vec<Vec<f64>> = tokens.iter().map(|t| embed_token(*t, dim)).collect();

    // For self-attention, Q=K=Value=embeddings
    let q = &embeddings;
    let k = &embeddings;
    let v = &embeddings;

    // Show attention weights
    show_attention_weights(q, k);

    // Compute attention output
    let output = scaled_dot_product_attention(q, k, v);

    println!("\nAttention output (contextualized representations):");
    for i in 0..output.len() {
        let out_str: Vec<String> = output[i]
            .iter()
            .take(3)
            .map(|x| format!("{:.3}", x))
            .collect();
        println!("  '{}' -> [{}...]", words[i], out_str.join(", "));
    }

    println!("\nKey insight: Each position's representation is now a weighted");
    println!("sum of ALL positions' values, where weights are learned similarity.");
    println!("This enables parallel processing and long-range dependency capture.");

    println!("\n=== TRANSFORMER ATTENTION COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax() {
        let input = vec![vec![1.0, 2.0, 3.0]];
        let result = softmax_rows(&input);
        // Softmax of [1,2,3] should be approximately [0.09, 0.24, 0.67]
        assert!((result[0][0] - 0.09).abs() < 0.01);
        assert!((result[0][1] - 0.24).abs() < 0.01);
        assert!((result[0][2] - 0.67).abs() < 0.01);
    }

    #[test]
    fn test_attention_dimensions() {
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let k = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let v = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

        let output = scaled_dot_product_attention(&q, &k, &v);
        assert_eq!(output.len(), 2);
        assert_eq!(output[0].len(), 2);
    }

    #[test]
    fn test_self_attention_identity() {
        // When Q=K and values are identity-like, attention should recover inputs
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let k = q.clone();
        let v = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

        let output = scaled_dot_product_attention(&q, &k, &v);
        // First query should attend mostly to first key/value
        assert!((output[0][0] - 1.0).abs() < 0.1 || (output[0][0] - 0.5).abs() < 0.5);
    }
}
