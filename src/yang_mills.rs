//! Yang-Mills Existence and Mass Gap Millennium Prize Problem Station
//!
//! Explores the Clay Mathematics Institute's $1 million question:
//! Does a quantum Yang-Mills theory exist with a mass gap?
//!
//! Demonstrates via lattice gauge theory concepts and Wilson loop computation.

/// Compute Wilson loop expectation for a rectangular loop on a lattice
/// This is simplified — real lattice QCD uses Monte Carlo simulation
fn wilson_loop_expectation(area: usize, coupling: f64) -> f64 {
    // Area law: <W(C)> ~ exp(-σ * A) where σ is string tension
    // For small loops, perimeter law dominates; for large loops, area law
    let sigma = 1.0 - coupling; // Simplified string tension
    (-sigma * area as f64).exp()
}

/// Estimate mass gap from Wilson loop decay
/// M ~ -ln(<W(C)>) / T where T is temporal extent
fn estimate_mass_gap(wilson_expectation: f64, time_extent: usize) -> f64 {
    if wilson_expectation <= 0.0 {
        return f64::INFINITY;
    }
    -wilson_expectation.ln() / (time_extent as f64)
}

/// Compute partition function for toy SU(2) lattice gauge theory
/// Using strong-coupling expansion (first few terms)
fn toy_partition_function(beta: f64, n_terms: usize) -> f64 {
    let mut z = 1.0;
    let mut term = 1.0;
    
    for n in 1..=n_terms {
        // Strong coupling expansion coefficients (simplified)
        term *= beta / n as f64;
        z += term;
    }
    
    z
}

/// Demonstrate confinement via potential between quarks
fn quark_antiquark_potential(distance: usize, string_tension: f64) -> f64 {
    // Linear potential at large distances (confinement)
    // Plus Coulomb-like term at short distances
    let coulomb = -0.5 / ((distance + 1) as f64);
    let linear = string_tension * distance as f64;
    coulomb + linear
}

pub fn show_yang_mills_station() {
    println!("=== YANG-MILLS EXISTENCE AND MASS GAP PROBLEM ===");
    println!("Clay Mathematics Institute: $1,000,000 prize for proof\n");
    
    println!("THE QUESTION: Does a quantum Yang-Mills theory exist with a mass gap?");
    println!("Specifically: Prove that for any compact simple gauge group G, there");
    println!("exists a non-trivial quantum Yang-Mills theory on R^4 with a positive");
    println!("mass gap Δ > 0.\n");
    
    println!("In simpler terms: Why do gluons (force carriers of strong force)");
    println!("have mass, even though the classical Yang-Mills equations predict");
    println!("massless particles? This explains quark confinement.\n");
    
    println!("--- LATTICE GAUGE THEORY APPROACH ---");
    println!("Numerical approach: discretize spacetime into a lattice and compute");
    println!("Wilson loop expectations to estimate the mass gap.\n");
    
    println!("Computing Wilson loop expectations for increasing loop areas:");
    println!("(Using simplified area law model)\n");
    
    let coupling = 0.1; // 't Hooft coupling
    println!("Loop Area | <W(C)>       | Notes");
    println!("----------|-------------|------");
    
    let areas = [1, 2, 4, 6, 8, 10];
    for &area in &areas {
        let w = wilson_loop_expectation(area, coupling);
        println!("{:9} | {:11.6} |", area, w);
    }
    
    println!("\nWilson loop decays exponentially with area → confinement");
    println!("This is the 'area law' vs 'perimeter law' distinction.\n");
    
    println!("--- MASS GAP ESTIMATION ---");
    println!("From Wilson loop decay rate, we can estimate the mass gap:\n");
    
    let time_extents = [2, 4, 6];
    for &t in &time_extents {
        let w = wilson_loop_expectation(t * 2, coupling);
        let m_gap = estimate_mass_gap(w, t);
        println!("Time extent T={}: mass gap estimate ≈ {:.4}", t, m_gap);
    }
    
    println!("\nPositive mass gap means the lightest particle has non-zero mass.");
    println!("This explains why the strong force is short-range despite massless");
    println!("gluons in the Lagrangian.\n");
    
    println!("--- CONFINEMENT ---");
    println!("Potential energy between quark-antiquark pair at increasing separation:\n");
    
    let string_tension = 0.9;
    println!("Distance | Potential Energy | Notes");
    println!("---------|------------------|------");
    for d in 1..=5 {
        let v = quark_antiquark_potential(d, string_tension);
        let note = if d >= 3 { "Linear rise → confinement" } else { "" };
        println!("{:8} | {:16.4} | {}", d, v, note);
    }
    
    println!("\nThe potential grows linearly at large distances — it takes infinite");
    println!("energy to separate quarks completely. This is why we never observe");
    println!("isolated quarks (confinement).\n");
    
    println!("--- THE MATHEMATICAL CHALLENGE ---");
    println!("• Construct the quantum field theory rigorously from axioms");
    println!("• Prove existence of the continuum limit (remove lattice spacing)");
    println!("• Show mass gap Δ > 0 in the continuum theory");
    println!("• Current approaches: constructive QFT, renormalization group\n");
    
    println!("--- CONNECTION TO REAL PHYSICS ---");
    println!("This problem underlies our understanding of:");
    println!("• Why protons and neutrons have mass (99% from QCD binding energy)");
    println!("• The structure of atomic nuclei");
    println!("• The early universe (quark-gluon plasma phase)");
    println!("• Lattice QCD computations used in particle physics\n");
    
    println!("=== YANG-MILLS STATION COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wilson_loop_decreases() {
        // Wilson loop expectation should decrease with area
        let w1 = wilson_loop_expectation(1, 0.1);
        let w2 = wilson_loop_expectation(4, 0.1);
        assert!(w1 > w2);
    }

    #[test]
    fn test_mass_gap_positive() {
        let w = wilson_loop_expectation(4, 0.1);
        let m = estimate_mass_gap(w, 2);
        assert!(m > 0.0);
    }

    #[test]
    fn test_partition_function_converges() {
        // Strong coupling expansion should converge for small beta
        let z = toy_partition_function(0.1, 100);
        assert!(z.is_finite());
        assert!(z > 1.0);
    }

    #[test]
    fn test_confinement_potential_rises() {
        // Potential should increase with distance at large separations
        let v1 = quark_antiquark_potential(2, 0.9);
        let v2 = quark_antiquark_potential(5, 0.9);
        assert!(v2 > v1);
    }
}
