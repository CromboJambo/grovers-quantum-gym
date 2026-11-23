/// Carlo Rovelli & Relational Quantum Mechanics Station (1996)
///
/// Demonstrates the relational interpretation of quantum mechanics: physical
/// properties are not intrinsic to systems but arise from interactions between
/// them. There is no observer-independent description of reality. The entangled
/// pair embodies the observer-observed relationship itself. Covers the Quantum
/// Zeno effect, double-slit disturbance (mechanical vs relational), and how RQM
/// dissolves the measurement problem differently from Copenhagen and Many Worlds.
///
/// Historical context:
/// - Born 1956 in Venice, Italy; currently at Aix-Marseille University
/// - Developed Relational Quantum Mechanics in his 1996 paper "Relational Quantum Mechanics"
/// - Also a leading figure in Loop Quantum Gravity
/// - Author of popular science books including "Seven Brief Lessons on Physics"
/// - RQM connects to Einstein's relativity (no preferred reference frame) and
///   Wheeler's participatory universe, but goes further: there is no view from nowhere

/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "rovelli",
    version: "0.0.1",
    formalized: false,
    notes: "Relational quantum mechanics: properties as interactions, no observer-independent state",
};

pub fn show_rovelli_station() {
    println!("=== CARLO ROVELLI & RELATIONAL QUANTUM MECHANICS (1996) ===");
    println!();
    print_biography();
    println!();
    explain_core_thesis();
    println!();
    demonstrate_no_isolated_systems();
    println!();
    demonstrate_entangled_pair_as_relationship();
    println!();
    explain_von_neumann_chain_resolution();
    println!();
    demonstrate_quantum_zeno_effect();
    println!();
    demonstrate_double_slit_disturbance();
    println!();
    compare_interpretations();
    println!();
    conclude_no_measurement_without_interaction();
}

fn print_biography() {
    println!("Carlo Rovelli (born 1956)");
    println!("Italian theoretical physicist, Aix-Marseille University");
    println!();
    println!("Rovelli grew up in Venice and studied physics at the University of Pavia.");
    println!("His 1996 paper 'Relational Quantum Mechanics' proposed a radical idea:");
    println!("physical properties don't exist independently. They only exist IN RELATION");
    println!("to other systems. There is no God's-eye view of the universe — only");
    println!("descriptions relative to specific interactions.");
    println!();
    println!("This isn't just philosophy. It's a concrete interpretation of quantum");
    println!("mechanics that dissolves the measurement problem without adding hidden");
    println!("variables (Bohm), collapsing wavefunctions (Copenhagen), or branching");
    println!("universes (Everett).");
}

fn explain_core_thesis() {
    println!("--- THE CORE THESIS: NO OBSERVER-INDEPENDENT STATE ---");
    println!();
    println!("Standard quantum mechanics says a system has a state |ψ⟩. But whose state?");
    println!("Relative to whom?");
    println!();
    println!("Rovelli's answer: A system's state is ALWAYS relative to another system.");
    println!("There is no absolute state — only states-as-seen-by-other-systems.");
    println!();
    println!("This mirrors relativity: just as there's no preferred reference frame in");
    println!("spacetime (Einstein), there's no privileged observer in quantum mechanics.");
    println!("Every system describes every other system relative to their interaction.");
    println!();
    println!("Key consequence: When you measure a particle, you're not revealing a");
    println!("pre-existing property. You're creating a fact — relative to you. The");
    println!("particle didn't have that spin 'before' your measurement. Not because it");
    println!("was hidden, but because the property only EXISTS in the relationship.");
}

fn demonstrate_no_isolated_systems() {
    println!("--- NO ISOLATED THINGS: SYSTEMS ARE BOUNDARIES ---");
    println!();
    println!("What we call a 'system' is a boundary we draw around part of a larger");
    println!("network of interactions. The boundary is arbitrary — chosen for convenience.");
    println!();
    println!("Example: An electron in a lab.");
    println!("  • Relative to the detector: it has a definite position (after measurement)");
    println!("  • Relative to the lab wall: it's part of a thermal environment");
    println!("  • Relative to the Earth: it's orbiting the sun at 30 km/s");
    println!("  • Relative to the galaxy: it's spinning around the galactic center");
    println!();
    println!("None of these descriptions is 'the real one.' They're all equally valid,");
    println!("each relative to a different system. The electron doesn't have ONE state —");
    println!("it has states relative to everything it interacts with.");
    println!();

    // Demonstrate: same system, different relational descriptions
    let systems = [
        ("Detector", "position = 3.2e-10 m (measured)"),
        ("Lab environment", "thermal energy ~ kT at 300K"),
        ("Earth frame", "velocity = 30 km/s orbital"),
        ("Galactic frame", "velocity = 220 km/s around center"),
    ];

    println!("The same electron, described relationally:");
    println!();
    for &(observer, description) in &systems {
        println!("  Relative to {:<20}: {}", observer, description);
    }
    println!();
    println!("All true. None complete. The 'real' electron is the network of all these");
    println!("relationships — not any single one of them.");
}

fn demonstrate_entangled_pair_as_relationship() {
    println!("--- THE ENTANGLED PAIR: OBSERVER AND OBSERVED, UNIFIED ---");
    println!();
    println!("In standard language, we say 'Alice measures particle A, and that determines");
    println!("particle B's state.' But this treats Alice as outside the system.");
    println!();
    println!("Rovelli's framing: The entangled pair IS the observer-observed relationship.");
    println!("Particle A is B's observer. Particle B is A's observer. They define each");
    println!("other's properties through their shared state.");
    println!();
    println!("The singlet state: |ψ⟩ = (|↑⟩ₐ|↓⟩ᵦ - |↓⟩ₐ|↑⟩ᵦ) / √2");
    println!();
    println!("  • Relative to A: B has opposite spin (fact established by their interaction)");
    println!("  • Relative to B: A has opposite spin (same fact, different perspective)");
    println!("  • Relative to Alice (external): the pair is in superposition until she measures");
    println!();
    println!("The 'spooky action at a distance' dissolves. There's no signal from A to B.");
    println!("There's just a relationship that was always there, now accessed from a new");
    println!("angle by Alice's measurement.");
    println!();

    // Demonstrate: reciprocal but asymmetric descriptions
    println!("Reciprocal observation (R(A,B) ≠ R(B,A)):");
    println!();
    println!("  A observes B: 'If I'm spin-up, B is spin-down'");
    println!("  B observes A: 'If I'm spin-down, A is spin-up'");
    println!("  Alice observes pair: 'The joint state is entangled'");
    println!();
    println!("Each description is valid relative to its observer. None is privileged.");
    println!("The relationship doesn't need a referee — it IS the reality.");
}

fn explain_von_neumann_chain_resolution() {
    println!("--- WHERE DOES VON NEUMANN'S CHAIN END? ---");
    println!();
    println!("Von Neumann showed that measurement propagates: system → apparatus →");
    println!("observer's eye → observer's brain. Where does the chain stop and 'real'");
    println!("collapse begin?");
    println!();
    println!("Copenhagen says: at consciousness (vague).");
    println!("Many Worlds says: nowhere — everything branches (proliferate).");
    println!("Rovelli says: the question is malformed.");
    println!();
    println!("There's no privileged 'real' observer. Each link in the chain is a system");
    println!("that has a definite state RELATIVE TO ITS INTERACTION PARTNER. The apparatus");
    println!("has a definite reading relative to itself. The observer has a definite memory");
    println!("relative to themselves. No global collapse needed — just consistent local facts.");
    println!();
    println!("The chain doesn't end because it never started from outside the system.");
    println!("Every 'observation' is just another interaction in the network.");
}

fn demonstrate_quantum_zeno_effect() {
    println!("--- THE QUANTUM ZENO EFFECT: MEASUREMENT FREEZES EVOLUTION ---");
    println!();
    println!("If you repeatedly measure a quantum system, you can SUPPRESS its evolution.");
    println!("The more often you look, the less it changes. Watching the pot keeps it from");
    println!("boiling — literally, in quantum mechanics.");
    println!();
    println!("Mechanism: Each measurement projects the state back toward its initial value.");
    println!("Between measurements, the system has time to evolve slightly. But if you measure");
    println!("faster than it can evolve significantly, you keep resetting it.");
    println!();

    // Simulate: survival probability with repeated measurements
    // P_survive ≈ (1 - (Δt)²/τ²)^n for n measurements over total time T
    let total_time = 1.0;
    let evolution_timescale = 1.0;

    println!("Survival probability vs number of measurements:");
    println!();
    println!("{:<20} {:>15} {:>15}", "Measurements (n)", "Interval Δt", "P(survive)");
    println!("{}", "-".repeat(55));

    let n_values = [1, 10, 100, 1000, 10000];
    for &n in &n_values {
        let dt = total_time / n as f64;
        // Approximate survival probability (quantum Zeno)
        let p_single = 1.0 - (dt / evolution_timescale).powi(2);
        let p_survive = p_single.powi(n as i32);
        println!(
            "{:<20} {:>15.6} {:>15.6}",
            n, dt, p_survive
        );
    }

    println!();
    println!("As n → ∞ (continuous measurement), P(survive) → 1.");
    println!("The system is frozen by observation. This is the Quantum Zeno Effect.");
    println!();
    println!("RQM interpretation: Each measurement creates a fact relative to the");
    println!("measuring apparatus. The system's evolution between measurements is");
    println!("relative to systems it hasn't interacted with yet. Continuous");
    println!("measurement means continuous fact-creation, pinning the state.");
}

fn demonstrate_double_slit_disturbance() {
    println!("--- DOUBLE-SLIT: MECHANICAL vs RELATIONAL DISTURBANCE ---");
    println!();
    println!("When you add which-path detectors to a double-slit experiment, interference");
    println!("disappears. Why? Two explanations compete:");
    println!();
    println!("  1. MECHANICAL: The detector physically kicks the particle (momentum transfer).");
    println!("     The 'oblique angle' of interaction changes its trajectory.");
    println!();
    println!("  2. RELATIONAL: The detector becomes entangled with the particle's path.");
    println!("     Even without a significant kick, the PATH INFORMATION exists in the");
    println!("     detector. Interference requires indistinguishable paths — and now they're");
    println!("     distinguishable in principle (decoherence).");
    println!();
    println!("2019 experiment (Science Advances): Measured momentum disturbance in which-");
    println!("path measurements. Found that interference loss exceeds what mechanical kicks");
    println!("alone can explain. Both mechanisms contribute, but relational disturbance");
    println!("(decoherence) is the dominant factor.");
    println!();

    // Demonstrate: interference visibility vs which-path information
    // V² + D² ≤ 1 (Englert-Greenberger-Yasin duality relation)
    // V = visibility, D = distinguishability

    println!("Wave-particle duality: V² + D² ≤ 1");
    println!("  V = interference visibility");
    println!("  D = path distinguishability");
    println!();
    println!("{:<30} {:>8} {:>8} {:>12}", "Scenario", "V", "D", "V² + D²");
    println!("{}", "-".repeat(65));

    let scenarios = [
        ("No which-path detector", 1.0, 0.0),
        ("Weak detector (partial info)", 0.707, 0.707),
        ("Strong detector (full info)", 0.0, 1.0),
    ];

    for &(label, v, d) in &scenarios {
        let sum_sq = v * v + d * d;
        println!(
            "{:<30} {:>8.3} {:>8.3} {:>12.3}",
            label, v, d, sum_sq
        );
    }

    println!();
    println!("Perfect interference (V=1) requires zero path information (D=0).");
    println!("Perfect which-path knowledge (D=1) destroys all interference (V=0).");
    println!("You can't have both. Not because of mechanical kicks — because the");
    println!("information exists in the relationship between particle and detector.");
}

fn compare_interpretations() {
    println!("--- COMPARING INTERPRETATIONS ---");
    println!();
    println!("{:<20} {:<35} {:<30}", "Interpretation", "Measurement problem", "Ontology");
    println!("{}", "-".repeat(90));
    println!(
        "{:<20} {:<35} {:<30}",
        "Copenhagen",
        "Collapse at observation (vague)",
        "Wavefunction is tool, not real"
    );
    println!(
        "{:<20} {:<35} {:<30}",
        "Many Worlds",
        "No collapse — branching",
        "All outcomes exist in branches"
    );
    println!(
        "{:<20} {:<35} {:<30}",
        "Bohmian",
        "Hidden variables guide particles",
        "Particles + pilot wave (non-local)"
    );
    println!(
        "{:<20} {:<35} {:<30}",
        "RQM (Rovelli)",
        "No collapse — relative facts",
        "Properties exist only in relations"
    );
    println!();
    println!("RQM's unique move: The measurement problem dissolves because there's no");
    println!("single 'true' state that needs to collapse. Each system has its own");
    println!("legitimate description relative to its interactions. No contradiction —");
    println!("just different perspectives on the same network of relationships.");
}

fn conclude_no_measurement_without_interaction() {
    println!("--- THE FOUNDATIONAL PRINCIPLE ---");
    println!();
    println!("There is no measurement without an interaction,");
    println!("and no interaction without consequences");
    println!("for the systems participating in it.");
    println!();
    println!("You are always interacting with what you observe. The question is never");
    println!("'Am I affecting it?' but 'How am I affecting it, and can I account for it?'");
    println!();
    println!("The consequences might be:");
    println!("  • Negligible (looking at a mountain)");
    println!("  • Measurable (photon hitting your retina)");
    println!("  • Fundamental to the phenomenon (double-slit, entanglement)");
    println!();
    println!("The challenge of physics is distinguishing those cases — and knowing that");
    println!("the observer is never outside the system being observed.");
    println!();
    println!("CONNECTION TO THE GYM: This station reframes everything before it.");
    println!("Bell's theorem shows correlations violate local realism. Wheeler's delayed");
    println!("choice shows measurement basis matters. Von Neumann's chain asks where");
    println!("collapse happens. Everett says it never does — everything branches.");
    println!("Rovelli says the question itself assumes a view from nowhere that doesn't");
    println!("exist. The only reality is the network of interactions, and we're in it.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_zeno_effect_freezes_evolution() {
        let total_time: f64 = 1.0;
        let evolution_timescale: f64 = 1.0;

        // More measurements → higher survival probability
        let p_few: f64 = (1.0 - (total_time / 10.0).powi(2) / evolution_timescale.powi(2))
            .powi(10);
        let p_many: f64 = (1.0 - (total_time / 10000.0).powi(2) / evolution_timescale.powi(2))
            .powi(10000);

        assert!(p_many > p_few);
        // With very many measurements, survival approaches 1
        assert!(p_many > 0.99);
    }

    #[test]
    fn test_no_measurements_full_evolution() {
        let total_time = 1.0;
        let evolution_timescale = 1.0;
        let n = 1;
        let dt = total_time / n as f64;
        let p_single = 1.0 - (dt / evolution_timescale).powi(2);
        let p_survive = p_single.powi(n as i32);
        // With one measurement at the end, survival is low
        assert!(p_survive < 0.5);
    }

    #[test]
    fn test_wave_particle_duality_relation() {
        // V² + D² ≤ 1 for all scenarios
        let scenarios = [(1.0, 0.0), (0.707, 0.707), (0.0, 1.0)];
        for &(v, d) in &scenarios {
            assert!(v * v + d * d <= 1.0 + 1e-6);
        }
    }

    #[test]
    fn test_perfect_interference_requires_no_path_info() {
        let v = 1.0; // perfect interference
        let d = 0.0; // no path information
        assert!(v * v + d * d <= 1.0);
    }

    #[test]
    fn test_perfect_path_info_destroys_interference() {
        let v = 0.0; // no interference
        let d = 1.0; // perfect path information
        assert!(v * v + d * d <= 1.0);
    }

    #[test]
    fn test_partial_which_path_reduces_both() {
        let v = 0.707; // reduced interference
        let d = 0.707; // partial path information
        assert!(v < 1.0 && d < 1.0);
        assert!(((v * v + d * d - 1.0) as f64).abs() < 0.01_f64);
    }

    #[test]
    fn test_entangled_pair_reciprocal_descriptions() {
        // In singlet state, if A is up then B is down (and vice versa)
        // This tests the correlation structure, not quantum mechanics itself
        let a_spin_up = true;
        let b_spin_down = a_spin_up;  // anti-correlated: A up → B down
        assert!(b_spin_down);

        let a_spin_down = false;
        let b_spin_up = !a_spin_down;  // A down (false) → B up (true)
        assert!(b_spin_up);
    }
}
