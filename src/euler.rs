//! Euler: The Structural Unifier (1707-1783)
//!
//! Demonstrates Leonhard Euler's unifying contributions to mathematics:
//! - Euler's identity: e^(iπ) + 1 = 0
//! - Euler's formula connecting exponentials and trigonometry
//! - Graph theory foundations (Königsberg bridge problem)
//!
//! Theme: Mathematics as connected structure rather than isolated tricks.

use std::f64::consts::PI;

// Simple complex number type for demonstration
#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

// Compute e^(ix) using Taylor series for sin and cos (Euler's formula)
fn euler_formula(x: f64) -> Complex {
    // e^(ix) = cos(x) + i*sin(x)
    let cos_x = x.cos();
    let sin_x = x.sin();
    Complex::new(cos_x, sin_x)
}

// Verify Euler's identity: e^(iπ) + 1 = 0
fn verify_eulers_identity() -> bool {
    let result = euler_formula(PI);
    // Should be approximately -1 + 0i
    let real_part = result.re + 1.0; // Add 1 to check if result was -1
    let imag_part = result.im;
    
    println!("e^(iπ) computed: {:.6} + {:.6}i", result.re, result.im);
    println!("e^(iπ) + 1 = {:.6} + {:.6}i (should be 0 + 0i)", real_part, imag_part);
    
    let close_enough = real_part.abs() < 1e-10 && imag_part.abs() < 1e-10;
    if close_enough {
        println!("✓ Euler's identity verified!");
    } else {
        println!("✗ Euler's identity failed (numerical error?)");
    }
    
    close_enough
}

// Demonstrate the connection between exponential growth and rotation
fn show_rotation_connection() {
    println!("\nEuler's formula: e^(ix) = cos(x) + i*sin(x)");
    println!("Connects exponential functions to circular motion.\n");
    
    let angles = [0.0, PI / 4.0, PI / 2.0, PI, 3.0 * PI / 2.0, 2.0 * PI];
    let labels = ["0", "π/4", "π/2", "π", "3π/2", "2π"];
    
    println!("Angle | e^(ix) value");
    println!("------|-------------");
    for (i, &angle) in angles.iter().enumerate() {
        let result = euler_formula(angle);
        println!("{:5} | {:.4} + {:.4}i", labels[i], result.re, result.im);
    }
    
    println!("\nNote how e^(ix) traces the unit circle as x increases.");
    println!("Exponential growth in the complex plane becomes rotation!");
}

// Simple graph: Königsberg bridge problem (simplified)
// Can you cross each bridge exactly once? Euler proved NO.
fn konigsberg_bridge_problem() {
    println!("\n=== GRAPH THEORY: The Königsberg Bridge Problem ===");
    println!("Can you walk through the city crossing each bridge exactly once?\n");
    
    // Simplified graph: 4 land masses, 7 bridges
    // Euler proved impossible when more than 2 vertices have odd degree
    
    let mut adj = vec![vec![]; 4]; // Adjacency list for 4 vertices
    
    // Add the 7 bridges (edges)
    let bridges = [
        (0, 1), (0, 2), (0, 3), // Bridges from vertex 0
        (1, 2), (1, 3),         // Bridges from vertex 1
        (2, 3),                 // Bridge between 2 and 3
        (1, 2),                 // Second bridge between 1 and 2
    ];
    
    for &(u, v) in &bridges {
        adj[u].push(v);
        adj[v].push(u);
    }
    
    println!("Vertex degrees:");
    let mut odd_degree_count = 0;
    for i in 0..4 {
        let degree = adj[i].len();
        println!("  Vertex {}: degree {}", i, degree);
        if degree % 2 == 1 {
            odd_degree_count += 1;
        }
    }
    
    println!("\nEuler's theorem: An Eulerian path exists iff at most 2 vertices");
    println!("have odd degree. We have {} vertices with odd degree.", odd_degree_count);
    
    if odd_degree_count > 2 {
        println!("Result: IMPOSSIBLE to cross each bridge exactly once.");
        println!("(This is how graph theory was born!)");
    } else {
        println!("Result: Possible!");
    }
}

// Demonstrate structural unification theme
fn show_structural_unification() {
    println!("\n=== STRUCTURAL UNIFICATION THEME ===");
    println!("Euler showed that seemingly unrelated mathematical objects");
    println!("are connected through common underlying structures.\n");
    
    println!("Five fundamental constants in one equation:");
    println!("  e (growth) + i (imaginary) + π (circles) + 1 (unity) + 0 (nothing)");
    println!("  all tied together by e^(iπ) + 1 = 0\n");
    
    println!("This mirrors how modern software works:");
    println!("  - Source code → Abstract Syntax Tree (structure over syntax)");
    println!("  - Data formats → Schema definitions (structure over bytes)");
    println!("  - Programs → Graphs of dependencies (structure over statements)");
}

pub fn show_euler_station() {
    println!("=== EULER: THE STRUCTURAL UNIFIER (1707-1783) ===");
    println!("Connecting mathematics through structure and relationships\n");
    
    println!("--- Part 1: Euler's Identity ---");
    verify_eulers_identity();
    
    show_rotation_connection();
    
    konigsberg_bridge_problem();
    
    show_structural_unification();
    
    println!("\n=== EULER STATION COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_eulers_identity() {
        assert!(verify_eulers_identity());
    }
    
    #[test]
    fn test_euler_formula_unit_circle() {
        // e^(ix) should always have magnitude 1
        for angle in [0.0, 1.0, 2.0, PI, 3.0*PI/4.0] {
            let result = euler_formula(angle);
            assert!((result.abs() - 1.0).abs() < 1e-10);
        }
    }
    
    #[test]
    fn test_complex_arithmetic() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        
        let product = a.mul(b);
        // (1+2i)(3+4i) = 3+4i+6i+8i² = 3+10i-8 = -5+10i
        assert!((product.re + 5.0).abs() < 1e-10);
        assert!((product.im - 10.0).abs() < 1e-10);
    }
}
