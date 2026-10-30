/// Hugh Everett III Station (1957)
///
/// Demonstrates the Many-Worlds Interpretation through a branching simulation.
/// Shows how quantum measurement leads to universe splitting rather than wave
/// function collapse, and explores the measure problem (how probability emerges).
///
/// Historical context:
/// - Born 1930, raised in Washington D.C.
/// - Princeton PhD 1957 under John Wheeler ("On the foundations of quantum mechanics")
/// - Proposed "relative state" interpretation — later called Many-Worlds by Bryce DeWitt (1970)
/// - Largely ignored until 1970s when quantum decoherence gave it credibility
/// - Died at 51 in 1982, long before MWI became dominant
/// - Einstein responded to his letter at age 12

#[derive(Debug, Clone)]
struct Branch {
    id: usize,
    history: String,
    amplitude: f64,
}

pub fn show_everett_station() {
    println!("=== HUGH EVERETT III STATION (1957) ===");
    println!();
    print_biography();
    println!();
    
    demonstrate_many_worlds();
    println!();
    
    demonstrate_measure_problem();
    println!();
    
    demonstrate_decoherence();
}

fn print_biography() {
    println!("Hugh Everett III (1930-1982)");
    println!("American physicist, Princeton PhD 1957 under John Wheeler");
    println!("Proposed the relative state interpretation of quantum mechanics");
    println!("Later called 'Many-Worlds Interpretation' by Bryce DeWitt (1970)");
    println!();
    println!("Key insight: The wave function never collapses. Instead, all possible");
    println!("outcomes occur in branching universes. The observer becomes entangled");
    println!("with the system, experiencing only one branch.");
}

fn demonstrate_many_worlds() {
    println!("--- DEMONSTRATION: Many-Worlds vs Copenhagen ---");
    println!();
    
    // Simulate a quantum measurement in both interpretations
    println!("Scenario: Measure spin of electron (up or down)");
    println!();
    
    println!("COPENHAGEN INTERPRETATION:");
    println!("  Before measurement: |ψ⟩ = α|↑⟩ + β|↓⟩ (superposition)");
    println!("  After measurement:   Wave function COLLAPSES to either |↑⟩ or |↓⟩");
    println!("  Only ONE outcome exists after measurement");
    println!();
    
    println!("MANY-WORLDS INTERPRETATION (Everett):");
    println!("  Before measurement: |ψ⟩ = α|↑⟩ + β|↓⟩ (superposition)");
    
    // Simulate branching
    let branches = simulate_branching(3);
    
    println!("  After measurement:   Universe SPLITS into branches:");
    for branch in &branches {
        print!("    Branch {}: {} (amplitude² = {:.2})", 
            branch.id, branch.history, branch.amplitude * branch.amplitude);
        if branch.id == 1 {
            print!(" <- You are here!");
        }
        println!();
    }
    
    println!();
    println!("  Both outcomes exist in different branches of the universal wave function");
    println!("  No collapse — just branching");
}

fn simulate_branching(depth: usize) -> Vec<Branch> {
    let mut branches = vec![Branch {
        id: 0,
        history: String::new(),
        amplitude: 1.0,
    }];
    
    for step in 0..depth {
        let mut new_branches = Vec::new();
        
        for branch in &branches {
            // Branch into two outcomes with equal probability (simplified)
            let up = Branch {
                id: branch.id * 2,
                history: format!("{}↑", branch.history),
                amplitude: branch.amplitude / 1.41421356, // divide by sqrt(2)
            };
            
            let down = Branch {
                id: branch.id * 2 + 1,
                history: format!("{}↓", branch.history),
                amplitude: branch.amplitude / 1.41421356, // divide by sqrt(2)
            };
            
            new_branches.push(up);
            new_branches.push(down);
        }
        
        branches = new_branches;
    }
    
    // Return only the first few for display
    branches.into_iter().take(4).collect()
}

fn demonstrate_measure_problem() {
    println!("--- DEMONSTRATION: The Measure Problem ---");
    println!();
    println!("How does probability emerge from Many-Worlds?");
    println!("If ALL outcomes occur, what does '50% chance' mean?");
    println!();
    
    // Simulate many measurements and count branch weights
    let total_measurements = 10;
    let mut up_count: f64 = 0.0;
    let mut down_count: f64 = 0.0;
    
    for _ in 0..total_measurements {
        // Each measurement creates two branches with equal weight (simplified)
        up_count += 0.5;
        down_count += 0.5;
    }
    
    println!("After {} measurements:", total_measurements);
    println!("  Total branch weight for UP: {:.1}", up_count);
    println!("  Total branch weight for DOWN: {:.1}", down_count);
    println!();
    println!("Everett's solution: Probability = measure of branches containing that outcome");
    println!("P(UP) = (weight of UP branches) / (total weight) = {:.0}", 
        up_count / (up_count + down_count) * 100.0);
    println!();
    println!("This is controversial — it requires deriving Born's rule from the theory itself.");
    println!("Everett argued this in his thesis, but it remains debated today.");
}

fn demonstrate_decoherence() {
    println!("--- DEMONSTRATION: Quantum Decoherence ---");
    println!();
    println!("Why don't we see superposition in everyday life?");
    println!("Decoherence explains how quantum systems become classical.");
    println!();
    
    // Simulate decoherence by showing interference terms disappear
    let system_states = vec![
        ("|0⟩", 1.0, 0.0),
        ("|1⟩", 0.0, 1.0),
    ];
    
    let environment_states = vec![
        ("|E₀⟩", 1.0, 0.0),
        ("|E₁⟩", 0.0, 1.0),
    ];
    
    println!("Before decoherence (isolated system):");
    println!("  |ψ⟩ = α|0⟩ + β|1⟩");
    println!("  Interference terms exist → quantum behavior");
    println!();
    
    println!("After interaction with environment:");
    println!("  |Ψ⟩ = α|0⟩|E₀⟩ + β|1⟩|E₁⟩");
    println!("  System is now ENTANGLED with environment");
    println!();
    
    // Show how interference disappears when we trace out the environment
    let alpha = 0.7;
    let beta = 0.7; // Not normalized for demonstration
    
    let interference_before = alpha * beta;
    let interference_after = alpha * beta * 0.0; // Environment states are orthogonal
    
    println!("Interference term before decoherence: {:.2}", interference_before);
    println!("Interference term after decoherence:  {:.2}", interference_after);
    println!();
    
    println!("Result: The system appears classical — superposition is 'hidden'");
    println!("in the environment. This explains why we see definite outcomes.");
    println!();
    println!("Decoherence theory (1970s) gave Many-Worlds its credibility boost.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_branching_creates_correct_number_of_branches() {
        let branches = simulate_branching(2);
        assert_eq!(branches.len(), 4); // 2^2 = 4 branches
        
        let branches = simulate_branching(3);
        assert_eq!(branches.len(), 4); // take(4) limits display to first 4
    }
    
    #[test]
    fn test_branch_amplitudes_sum_to_one() {
        let branches = simulate_branching(2);
        
        let total_weight: f64 = branches.iter()
            .map(|b| b.amplitude * b.amplitude)
            .sum();
        
        // Should be close to 1.0 (all branches accounted for)
        assert!((total_weight - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_branch_history_length() {
        let depth = 3;
        let branches = simulate_branching(depth);
        
        for branch in &branches {
            assert_eq!(branch.history.chars().count(), depth);
        }
    }
    
    #[test]
    fn test_measure_problem_symmetric() {
        // With equal amplitudes, up and down should have equal measure
        let total = 10;
        let up = total as f64 * 0.5;
        let down = total as f64 * 0.5;
        
        assert_eq!(up, down);
    }
}
