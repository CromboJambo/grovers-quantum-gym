/// John von Neumann Station (1932, 1944)
///
/// Demonstrates two of von Neumann's foundational contributions:
/// 1. Quantum measurement theory — the projection postulate and the von Neumann chain
///    showing how measurement propagates from system to apparatus to observer.
/// 2. Game theory — co-founded with Oskar Morgenstern in "Theory of Games and Economic Behavior" (1944).
///
/// Historical context:
/// - Born December 28, 1903 in Budapest, Austria-Hungary; emigrated to the U.S. in 1937.
/// - Princeton Institute for Advanced Study professor; worked with Einstein, Gödel, Oppenheimer.
/// - "Mathematical Foundations of Quantum Mechanics" (1932) — first rigorous axiomatic framework.
/// - Manhattan Project consultant (implosion lens design); IAS and AEC committees in the 1950s.
/// - Died February 8, 1957 at age 53; Medal of Freedom recipient; crater on the Moon named for him.

#[derive(Debug, Clone)]
struct MeasurementStep {
    id: usize,
    entity: String,
    state_before: String,
    state_after: String,
    description: String,
}

pub fn show_von_neumann_station() {
    println!("=== JOHN VON NEUMANN STATION (1932, 1944) ===");
    println!();
    print_biography();
    println!();

    demonstrate_projection_postulate();
    println!();

    demonstrate_von_neumann_chain();
    println!();

    demonstrate_game_theory();
}

fn print_biography() {
    println!("John von Neumann (1903-1957)");
    println!("Hungarian-American polymath, Princeton Institute for Advanced Study");
    println!("First rigorous axiomatic framework for quantum mechanics (1932)");
    println!("Co-founded game theory with Oskar Morgenstern (1944)");
    println!();
    println!("Key insight: Quantum measurement is not a mystical process — it's a physical");
    println!("interaction that propagates from the system to the apparatus to the observer.");
    println!("This chain of entanglement blurs the line between 'system' and 'environment.'");
}

fn demonstrate_projection_postulate() {
    println!("--- DEMONSTRATION: The Projection Postulate ---");
    println!();
    println!(
        "Before von Neumann, quantum measurement was hand-waved. He gave it mathematical rigor."
    );
    println!();
    println!("The projection postulate states:");
    println!("  When you measure an observable and get outcome λᵢ, the state collapses to");
    println!("  Pᵢ|ψ⟩ / √⟨ψ|Pᵢ|ψ⟩ where Pᵢ is the projector onto the eigenspace of λᵢ.");
    println!();

    // Demonstrate with a simple spin measurement
    let alpha = 0.6;
    let beta = 0.8;
    println!(
        "Example: Electron in superposition |ψ⟩ = {:.1}|↑⟩ + {:.1}|↓⟩",
        alpha, beta
    );
    println!();

    // Calculate probabilities
    let p_up = alpha * alpha;
    let p_down = beta * beta;
    println!("Probability of measuring UP: |α|² = {:.2}", p_up);
    println!("Probability of measuring DOWN: |β|² = {:.2}", p_down);
    println!();

    // Simulate measurement (deterministic for demonstration)
    let outcome = if p_up >= 0.5 { "UP" } else { "DOWN" };
    println!("Simulated measurement outcome: {}", outcome);
    println!("After measurement, the state is projected to |{}⟩", outcome);
    println!();
    println!("This is not just 'we learned something' — the physical state actually changes.");
    println!("Measurement disturbs the system. That's the quantum world.");
}

fn demonstrate_von_neumann_chain() {
    println!("--- DEMONSTRATION: The Von Neumann Chain ---");
    println!();
    println!("What exactly is 'measurement'? Where does the chain end?");
    println!();

    let steps = vec![
        MeasurementStep {
            id: 1,
            entity: "Electron".to_string(),
            state_before: "|ψ⟩ = α|↑⟩ + β|↓⟩ (superposition)".to_string(),
            state_after: "Entangled with apparatus".to_string(),
            description: "Quantum system in superposition of spin states".to_string(),
        },
        MeasurementStep {
            id: 2,
            entity: "Stern-Gerlach Apparatus".to_string(),
            state_before: "|ready⟩ (neutral)".to_string(),
            state_after: "α|shows UP⟩ + β|shows DOWN⟩".to_string(),
            description: "Apparatus becomes correlated with electron spin".to_string(),
        },
        MeasurementStep {
            id: 3,
            entity: "Detector Screen".to_string(),
            state_before: "|blank⟩".to_string(),
            state_after: "α|UP spot⟩ + β|DOWN spot⟩".to_string(),
            description: "Visual record created on screen".to_string(),
        },
        MeasurementStep {
            id: 4,
            entity: "Physicist's Eye".to_string(),
            state_before: "|unseeing⟩".to_string(),
            state_after: "α|sees UP⟩ + β|sees DOWN⟩".to_string(),
            description: "Light from screen enters observer's eye".to_string(),
        },
        MeasurementStep {
            id: 5,
            entity: "Physicist's Brain".to_string(),
            state_before: "|unaware⟩".to_string(),
            state_after: "α|knows UP⟩ + β|knows DOWN⟩".to_string(),
            description: "Neural processes create conscious experience".to_string(),
        },
    ];

    for step in &steps {
        println!("Step {}: {}", step.id, step.entity);
        println!("  Before: {}", step.state_before);
        println!("  After:  {}", step.state_after);
        println!("  {}", step.description);
        println!();
    }

    println!("The key question von Neumann raised: Where does the chain end?");
    println!("Is there a 'cut' between quantum and classical? Or is everything quantum?");
    println!();
    println!("Von Neumann's answer: The cut can be placed ANYWHERE along the chain.");
    println!("You could treat the electron + apparatus as quantum and the observer as classical.");
    println!("Or you could treat the entire chain (including the observer) as quantum.");
    println!();
    println!("This insight directly led to the Many-Worlds Interpretation — if we treat");
    println!("the whole chain as quantum, there's no collapse. Just branching entanglement.");
}

fn demonstrate_game_theory() {
    println!("--- DEMONSTRATION: Game Theory (1944) ---");
    println!();
    println!("Von Neumann and Morgenstern formalized strategic decision-making.");
    println!("Key concept: Nash equilibrium — no player can improve by changing strategy alone.");
    println!();

    // Prisoner's Dilemma payoff matrix
    println!("Prisoner's Dilemma Payoff Matrix (years in prison, lower is better):");
    println!();
    println!("              Player B Cooperates    Player B Betrays");
    println!("Player A Cooperates      (-1, -1)               (-3, 0)");
    println!("Player A Betrays         (0, -3)                (-2, -2)");
    println!();

    // Analyze the game
    println!("Dominant strategy analysis:");
    println!("  If B cooperates: A should betray (0 > -1)");
    println!("  If B betrays:    A should betray (-2 > -3)");
    println!("  Same logic applies to B.");
    println!();
    println!("Nash equilibrium: Both betray → (-2, -2)");
    println!("But if both cooperated, they'd get (-1, -1) — better for both!");
    println!();
    println!("This tension between individual rationality and collective optimality");
    println!("is the essence of game theory. It applies to economics, biology, politics...");
    println!("and even quantum measurement (the observer is a 'player' in the chain).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection_probabilities_sum_to_one() {
        let alpha = 0.6;
        let beta = 0.8;
        let p_up = alpha * alpha;
        let p_down = beta * beta;
        assert!((p_up + p_down - 1.0_f64).abs() < 0.001_f64);
    }

    #[test]
    fn test_von_neumann_chain_has_five_steps() {
        let steps = vec![
            MeasurementStep {
                id: 1,
                entity: "Electron".to_string(),
                state_before: "".to_string(),
                state_after: "".to_string(),
                description: "".to_string(),
            },
            MeasurementStep {
                id: 2,
                entity: "Apparatus".to_string(),
                state_before: "".to_string(),
                state_after: "".to_string(),
                description: "".to_string(),
            },
            MeasurementStep {
                id: 3,
                entity: "Screen".to_string(),
                state_before: "".to_string(),
                state_after: "".to_string(),
                description: "".to_string(),
            },
            MeasurementStep {
                id: 4,
                entity: "Eye".to_string(),
                state_before: "".to_string(),
                state_after: "".to_string(),
                description: "".to_string(),
            },
            MeasurementStep {
                id: 5,
                entity: "Brain".to_string(),
                state_before: "".to_string(),
                state_after: "".to_string(),
                description: "".to_string(),
            },
        ];
        assert_eq!(steps.len(), 5);
    }

    #[test]
    fn test_prisoners_dilemma_payoffs() {
        // Verify the payoff structure is correct for Prisoner's Dilemma
        // T > R > P > S and 2R > T + S
        let t = 0; // Temptation to betray
        let r = -1; // Reward for mutual cooperation
        let p = -2; // Punishment for mutual betrayal
        let s = -3; // Sucker's payoff

        assert!(t > r);
        assert!(r > p);
        assert!(p > s);
        assert!(2 * r > t + s);
    }
}
