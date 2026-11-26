/// Matrix Multiplication Exponent: Past, Present, and Future
///
/// Educational station demonstrating the progression of matrix multiplication
/// complexity from O(n³) to ω ≤ 9/4 (2.25), with practical benchmarks showing
/// why theoretical bounds don't immediately translate to speedups.
///
/// Key results covered:
/// - Naive: O(n³) — the standard triple loop
/// - Strassen (1969): O(n^2.807) — 7 multiplications instead of 8
/// - Coppersmith-Winograd (1990): O(n^2.376) — tensor power method
/// - Alman-Williams (2021): ω < 2.371 — current best before OpenAI math
/// - OpenAI Math Family 107 (Nov 2026): ω ≤ 9/4 = 2.25 over ℂ (Lean-verified)
///
/// The central lesson: asymptotic complexity tells you what's possible for
/// n → ∞, but constants and cache effects determine what's fast in practice.

use std::time::Instant;

pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "matmul_exponent",
    version: "0.0.1",
    formalized: false,
    notes: "Matrix multiplication exponent progression: O(n³) → Strassen 2.807 → ω ≤ 9/4 (OpenAI Math Family 107, Lean-verified)",
};

/// A simple dense matrix using row-major storage.
#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn random(rows: usize, cols: usize, seed: u64) -> Self {
        let mut m = Matrix::new(rows, cols);
        // Simple LCG for deterministic "random" values in [-1, 1]
        let mut state = seed;
        for v in m.data.iter_mut() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *v = ((state >> 33) as f64 / (u64::MAX as f64)) * 2.0 - 1.0;
        }
        m
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.cols + j]
    }

    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.data[i * self.cols + j] = v;
    }
}

/// Naive O(n³) matrix multiplication.
pub fn matmul_naive(a: &Matrix, b: &Matrix) -> Matrix {
    assert_eq!(a.cols, b.rows);
    let mut c = Matrix::new(a.rows, b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = 0.0;
            for k in 0..a.cols {
                sum += a.get(i, k) * b.get(k, j);
            }
            c.set(i, j, sum);
        }
    }
    c
}

/// Strassen's O(n^log₂7) ≈ O(n^2.807) matrix multiplication.
/// Only works on square matrices with even dimensions (padded if needed).
pub fn matmul_strassen(a: &Matrix, b: &Matrix) -> Matrix {
    assert_eq!(a.rows, a.cols);
    assert_eq!(b.rows, b.cols);
    assert_eq!(a.rows, b.rows);
    
    let n = a.rows;
    // Base case: use naive for small matrices (Strassen overhead dominates)
    if n <= 64 {
        return matmul_naive(a, b);
    }
    
    let half = n / 2;
    
    // Split into quadrants
    let a11 = &a.data[0..half * half];
    let a12 = &a.data[half * half..half * (n + half)];
    let a21 = &a.data[half * n..half * n + half * half];
    let a22 = &a.data[half * n + half * half..];
    
    let b11 = &b.data[0..half * half];
    let b12 = &b.data[half * half..half * (n + half)];
    let b21 = &b.data[half * n..half * n + half * half];
    let b22 = &b.data[half * n + half * half..];
    
    // Helper to create a matrix from a slice
    let mat_from_slice = |slice: &[f64]| Matrix { rows: half, cols: half, data: slice.to_vec() };
    
    // Compute the 7 Strassen products
    // M1 = (A11 + A22) × (B11 + B22)
    let mut a11_a22 = mat_from_slice(a11);
    for i in 0..half * half {
        a11_a22.data[i] += a22[i];
    }
    let mut b11_b22 = mat_from_slice(b11);
    for i in 0..half * half {
        b11_b22.data[i] += b22[i];
    }
    let m1 = matmul_strassen(&a11_a22, &b11_b22);
    
    // M2 = (A21 + A22) × B11
    let mut a21_a22 = mat_from_slice(a21);
    for i in 0..half * half {
        a21_a22.data[i] += a22[i];
    }
    let b11_mat = mat_from_slice(b11);
    let m2 = matmul_strassen(&a21_a22, &b11_mat);
    
    // M3 = A11 × (B12 - B22)
    let mut b12_b22 = mat_from_slice(b12);
    for i in 0..half * half {
        b12_b22.data[i] -= b22[i];
    }
    let a11_mat = mat_from_slice(a11);
    let m3 = matmul_strassen(&a11_mat, &b12_b22);
    
    // M4 = A22 × (B21 - B11)
    let mut b21_b11 = mat_from_slice(b21);
    for i in 0..half * half {
        b21_b11.data[i] -= b11[i];
    }
    let a22_mat = mat_from_slice(a22);
    let m4 = matmul_strassen(&a22_mat, &b21_b11);
    
    // M5 = (A11 + A12) × B22
    let mut a11_a12 = mat_from_slice(a11);
    for i in 0..half * half {
        a11_a12.data[i] += a12[i];
    }
    let b22_mat = mat_from_slice(b22);
    let m5 = matmul_strassen(&a11_a12, &b22_mat);
    
    // M6 = (A21 - A11) × (B11 + B12)
    let mut a21_a11 = mat_from_slice(a21);
    for i in 0..half * half {
        a21_a11.data[i] -= a11[i];
    }
    let mut b11_b12 = mat_from_slice(b11);
    for i in 0..half * half {
        b11_b12.data[i] += b12[i];
    }
    let m6 = matmul_strassen(&a21_a11, &b11_b12);
    
    // M7 = (A12 - A22) × (B21 + B22)
    let mut a12_a22 = mat_from_slice(a12);
    for i in 0..half * half {
        a12_a22.data[i] -= a22[i];
    }
    let mut b21_b22 = mat_from_slice(b21);
    for i in 0..half * half {
        b21_b22.data[i] += b22[i];
    }
    let m7 = matmul_strassen(&a12_a22, &b21_b22);
    
    // Combine results
    let mut c = Matrix::new(n, n);
    for i in 0..half {
        for j in 0..half {
            // C11 = M1 + M4 - M5 + M7
            c.set(i, j, m1.get(i, j) + m4.get(i, j) - m5.get(i, j) + m7.get(i, j));
            // C12 = M3 + M5
            c.set(i, j + half, m3.get(i, j) + m5.get(i, j));
            // C21 = M2 + M4
            c.set(i + half, j, m2.get(i, j) + m4.get(i, j));
            // C22 = M1 - M2 + M3 + M6
            c.set(i + half, j + half, m1.get(i, j) - m2.get(i, j) + m3.get(i, j) + m6.get(i, j));
        }
    }
    c
}

/// Verify two matrices are close (for correctness checking).
fn matrices_close(a: &Matrix, b: &Matrix, tol: f64) -> bool {
    if a.rows != b.rows || a.cols != b.cols {
        return false;
    }
    for i in 0..a.data.len() {
        if (a.data[i] - b.data[i]).abs() > tol * (1.0 + a.data[i].abs()) {
            return false;
        }
    }
    true
}

/// Theoretical operation counts for different algorithms.
fn naive_ops(n: usize) -> u64 {
    (n as u64).pow(3)
}

fn strassen_ops(n: usize) -> u64 {
    // 7^(log2(n)) multiplications + additions
    if n <= 1 { return 0; }
    let log2_n = (n as f64).log2();
    7.0_f64.powf(log2_n) as u64
}

/// Public entry point for the station.
pub fn show_matmul_station() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  MATRIX MULTIPLICATION EXPONENT: PAST, PRESENT, FUTURE   ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    println!("The exponent ω of matrix multiplication is the infimum of all α such that");
    println!("two n×n matrices can be multiplied in O(n^α) arithmetic operations.");
    println!();
    
    // === PAST ===
    println!("─── PAST: The Progression ─────────────────────────────────────────────");
    println!();
    println!("{:<20} {:<15} {:<20}", "Year", "Algorithm", "Complexity");
    println!("{:<20} {:<15} {:<20}", "────", "─────────", "────────");
    println!("{:<20} {:<15} {:<20}", "Ancient", "Naive (triple loop)", "O(n³) = n^3.000");
    println!("{:<20} {:<15} {:<20}", "1969", "Strassen", "O(n^log₂7) ≈ n^2.807");
    println!("{:<20} {:<15} {:<20}", "1971", "Pan (improved Strassen)", "O(n^2.795)");
    println!("{:<20} {:<15} {:<20}", "1990", "Coppersmith-Winograd", "O(n^2.376)");
    println!("{:<20} {:<15} {:<20}", "2014", "Stothers (CW refinement)", "O(n^2.373)");
    println!("{:<20} {:<15} {:<20}", "2021", "Alman-Williams", "ω < 2.371");
    println!();
    
    // === PRESENT: Practical Benchmark ===
    println!("─── PRESENT: Theory vs Practice ──────────────────────────────────────");
    println!();
    println!("Theoretical bounds assume n → ∞ and ignore constants, cache effects,");
    println!("and memory bandwidth. Here's what actually happens on real hardware:\n");
    
    let sizes = [16usize, 32, 64, 128, 256];
    println!("{:>6} {:>14} {:>14} {:>14} {:>10}", "n", "Naive (ms)", "Strassen (ms)", "Speedup", "Ops saved");
    println!("{:>6} {:>14} {:>14} {:>14} {:>10}", "──", "──────────", "─────────────", "───────", "─────────");
    
    for &n in &sizes {
        let a = Matrix::random(n, n, 42);
        let b = Matrix::random(n, n, 123);
        
        // Time naive
        let start = Instant::now();
        let _c_naive = matmul_naive(&a, &b);
        let naive_time = start.elapsed().as_micros() as f64 / 1000.0;
        
        // Time Strassen (only for powers of 2 >= 64)
        if n >= 64 && n.is_power_of_two() {
            let start = Instant::now();
            let c_strassen = matmul_strassen(&a, &b);
            let strassen_time = start.elapsed().as_micros() as f64 / 1000.0;
            
            // Verify correctness
            let c_naive = matmul_naive(&a, &b);
            let correct = matrices_close(&c_strassen, &c_naive, 1e-6);
            
            let speedup = if strassen_time > 0.0 { naive_time / strassen_time } else { 0.0 };
            let ops_saved = 1.0 - (strassen_ops(n) as f64 / naive_ops(n) as f64);
            
            println!("{:>6} {:>13.2}ms {:>13.2}ms {:>9.2}x {:>9.0}% {}", 
                n, naive_time, strassen_time, speedup, ops_saved * 100.0,
                if correct { "✓" } else { "✗ WRONG" });
        } else {
            println!("{:>6} {:>13.2}ms {:>14} {:>14} {:>14}", 
                n, naive_time, "(too small)", "", "");
        }
    }
    
    println!();
    println!("Notice: Strassen is SLOWER for small n despite fewer operations.");
    println!("Why? The 7 recursive multiplications create overhead (allocations,");
    println!("additions/subtractions) that dominate when n is small. BLAS libraries");
    println!("typically use naive multiplication up to n ≈ 64-128, then switch to");
    println!("Strassen or other methods only for very large matrices.");
    println!();
    
    // === FUTURE: ω ≤ 9/4 ===
    println!("─── FUTURE: The 9/4 Breakthrough (OpenAI Math, Nov 2026) ─────────────");
    println!();
    println!("Family 107 in the OpenAI math repository proves:");
    println!();
    println!("    ω ≤ 9/4 = 2.25 over ℂ");
    println!();
    println!("This is Lean-verified (lean/docs/107.md) and represents a significant");
    println!("improvement over the previous best bound of ω < 2.371 (Alman-Williams).");
    println!();
    println!("Additional bounds from the same work:");
    println!("  • ω < 2.258 outside finitely many positive characteristics");
    println!("  • ω < 2.371054886006746 over every fixed field");
    println!("  • Rectangular: ω(1, 0.709, 1) < 2.092 in characteristic zero");
    println!();
    
    // === IMPLICATIONS FOR LLM INFERENCE ===
    println!("─── WHAT THIS MEANS FOR PESTI / LLM INFERENCE ────────────────────────");
    println!();
    println!("LLM inference is dominated by GEMM (General Matrix Multiply) operations:");
    println!();
    println!("  Attention: QK^T → [batch × seq_len] × [seq_len × head_dim]");
    println!("             AV   → [batch × seq_len] × [seq_len × head_dim]");
    println!("  Feed-forward: W1 → [batch × hidden] × [hidden × 4×hidden]");
    println!();
    
    let batch = 8usize;
    let seq_len = 2048usize;
    let hidden = 4096usize;
    
    println!("Typical pesti workload (batch=8, seq=2048, hidden=4096):");
    println!("  Attention QK^T: {}×{} × {}×{} = {:.1} GFLOP per layer", 
        batch * seq_len, seq_len, seq_len, 128, 
        2.0 * (batch as f64) * (seq_len as f64).powi(3) * 128.0 / 1e9);
    println!("  Feed-forward W1: {}×{} × {}×{} = {:.1} GFLOP per layer", 
        batch, hidden, hidden, 4 * hidden, 
        2.0 * (batch as f64) * (hidden as f64).powi(3) * 4.0 / 1e9);
    println!();
    
    println!("If ω = 2.25 were practical at these sizes:");
    let n_attn = seq_len; // The dominant dimension in attention
    let speedup_attn = (n_attn as f64).powf(3.0 - 2.25);
    println!("  Attention speedup factor: n^0.75 = {:.1}x for seq_len={}", speedup_attn, n_attn);
    let n_ff = hidden;
    let speedup_ff = (n_ff as f64).powf(3.0 - 2.25);
    println!("  Feed-forward speedup: n^0.75 = {:.1}x for hidden={}", speedup_ff, n_ff);
    println!();
    
    println!("BUT — the catch (and this is the important lesson):");
    println!();
    println!("  1. Asymptotic bounds require n → ∞. The 9/4 algorithm likely has");
    println!("     enormous constants and only beats BLAS for n >> 10,000.");
    println!();
    println!("  2. Memory bandwidth often dominates compute in inference (especially");
    println!("     decode step with batch=1). Faster matmul doesn't help if you're");
    println!("     waiting on memory.");
    println!();
    println!("  3. GPU architectures are optimized for the specific access patterns");
    println!("     of BLAS/Strassen-style algorithms. New algorithms may not map");
    println!("     well to existing hardware without architectural changes.");
    println!();
    println!("  4. For pesti specifically: keep the GEMM layer pluggable. When (if)");
    println!("     someone implements ω < 2.5 with reasonable constants, we should");
    println!("     be able to swap it in without changing the inference stack.");
    println!();
    
    // === THE DEEPER PATTERN ===
    println!("─── THE DEEPER PATTERN ───────────────────────────────────────────────");
    println!();
    println!("The progression 3.000 → 2.807 → 2.376 → 2.371 → 2.25 shows:");
    println!();
    println!("  • Each improvement gets smaller and harder to achieve");
    println!("  • The gap between theory and practice WIDENS as bounds improve");
    println!("  • Practical speedups come from engineering (cache, parallelism,");
    println!("    hardware) more often than from asymptotic improvements");
    println!();
    println!("This mirrors a broader pattern in computer science: the best algorithms");
    println!("for real problems are often not the ones with the best asymptotic bounds.");
    println!("QuickSort (O(n log n) worst case is MergeSort, but QuickSort wins in");
    println!("practice due to cache behavior and small constants).");
    println!();
    println!("The 9/4 result is mathematically profound — it tells us the fundamental");
    println!("limit of what's possible. Whether it becomes practically useful depends");
    println!("on whether someone can tame the constants. That's the next challenge.");
    println!();
    
    // === VERIFICATION ===
    println!("─── CORRECTNESS VERIFICATION ─────────────────────────────────────────");
    println!();
    
    // Verify Strassen against naive for a few sizes
    let test_sizes = [64usize, 128, 256];
    for &n in &test_sizes {
        let a = Matrix::random(n, n, 999);
        let b = Matrix::random(n, n, 888);
        
        let c_naive = matmul_naive(&a, &b);
        let c_strassen = matmul_strassen(&a, &b);
        
        // Compute max relative error
        let mut max_err: f64 = 0.0;
        for i in 0..n * n {
            let rel_err = if c_naive.data[i].abs() > 1e-10 {
                (c_strassen.data[i] - c_naive.data[i]).abs() / c_naive.data[i].abs()
            } else {
                (c_strassen.data[i] - c_naive.data[i]).abs()
            };
            max_err = max_err.max(rel_err);
        }
        
        println!("  n={:>4}: Strassen vs Naive max relative error = {:.2e} {}", 
            n, max_err, if max_err < 1e-6 { "✓" } else { "✗" });
    }
    
    println!();
    println!("References:");
    println!("  • OpenAI Math Family 107: ω ≤ 9/4 (Lean-verified)");
    println!("    https://github.com/openai/math/blob/main/lean/docs/107.md");
    println!("  • Strassen (1969): Gaussian elimination is not optimal");
    println!("  • Coppersmith & Winograd (1990): Matrix multiplication via");
    println!("    tensor methods");
    println!("  • Alman & Williams (2021): Faster rectangular matrix");
    println!("    multiplication");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_naive_matmul_correctness() {
        // 2×2 × 2×2
        let a = Matrix { rows: 2, cols: 2, data: vec![1.0, 2.0, 3.0, 4.0] };
        let b = Matrix { rows: 2, cols: 2, data: vec![5.0, 6.0, 7.0, 8.0] };
        let c = matmul_naive(&a, &b);
        
        // Expected: [[19, 22], [43, 50]]
        assert!((c.get(0, 0) - 19.0).abs() < 1e-10);
        assert!((c.get(0, 1) - 22.0).abs() < 1e-10);
        assert!((c.get(1, 0) - 43.0).abs() < 1e-10);
        assert!((c.get(1, 1) - 50.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_strassen_matches_naive() {
        // Test at the base case boundary and above
        for &n in &[64usize, 128, 256] {
            let a = Matrix::random(n, n, 42);
            let b = Matrix::random(n, n, 123);
            
            let c_naive = matmul_naive(&a, &b);
            let c_strassen = matmul_strassen(&a, &b);
            
            assert!(matrices_close(&c_strassen, &c_naive, 1e-6), 
                "Strassen result differs from naive at n={}", n);
        }
    }
    
    #[test]
    fn test_rectangular_matmul() {
        // Naive handles rectangular matrices
        let a = Matrix::random(3, 4, 1);
        let b = Matrix::random(4, 5, 2);
        let c = matmul_naive(&a, &b);
        
        assert_eq!(c.rows, 3);
        assert_eq!(c.cols, 5);
        
        // Verify one entry manually
        let expected: f64 = (0..4).map(|k| a.get(0, k) * b.get(k, 0)).sum();
        assert!((c.get(0, 0) - expected).abs() < 1e-10);
    }
    
    #[test]
    fn test_operation_counts() {
        // Naive is exactly n³ for square matrices
        assert_eq!(naive_ops(2), 8);
        assert_eq!(naive_ops(10), 1000);
        
        // Strassen ops should be less than naive for large n
        assert!(strassen_ops(1024) < naive_ops(1024));
        assert!(strassen_ops(65536) < naive_ops(65536));
    }
    
    #[test]
    fn test_station_metadata() {
        assert_eq!(STATION.name, "matmul_exponent");
        assert!(!STATION.formalized);
        assert!(STATION.version.starts_with("0.0."));
    }
}
