/// John Archibald Wheeler Station (1967-2008)
///
/// Demonstrates the delayed-choice experiment and "it from bit" philosophy.
/// Shows how the choice of measurement determines what actually happened in
/// the past, and how reality may emerge from fundamental binary information.
///
/// Historical context:
/// - Born 1911 in Jacksonville, Florida; died 2008 at age 96
/// - PhD from Johns Hopkins (1933) under Karl Herzfeld
/// - Princeton professor for decades; advisor to Hugh Everett III
/// - Coined "black hole" (1967), "wormhole," and "quantum foam"
/// - Proposed the delayed-choice experiment in 1978
/// - Developed "it from bit" philosophy in his later years


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "wheeler",
    version: "0.0.1",
    formalized: false,
    notes: "Wheeler's delayed choice experiment",
};


pub fn show_wheeler_station() {
    println!("=== JOHN ARCHIBALD WHEELER STATION (1967-2008) ===");
    println!();
    print_biography();
    println!();
    explain_delayed_choice();
    println!();
    demonstrate_delayed_choice();
    println!();
    explain_it_from_bit();
    println!();
    demonstrate_participatory_universe();
}

fn print_biography() {
    println!("John Archibald Wheeler (1911-2008)");
    println!("American theoretical physicist, Princeton professor");
    println!("Advisor to Hugh Everett III (Many-Worlds Interpretation)");
    println!();
    println!("Wheeler was one of the most creative physicists of the 20th century.");
    println!("He coined the terms 'black hole,' 'wormhole,' and 'quantum foam.'");
    println!("But his deepest insight came late: the universe may be fundamentally");
    println!("informational — built from bits, binary choices that create reality.");
}

fn explain_delayed_choice() {
    println!("--- THE DELAYED-CHOICE EXPERIMENT (1978) ---");
    println!();
    println!("Wheeler proposed a version of the double-slit experiment where you");
    println!("decide HOW to measure AFTER the photon has already passed through.");
    println!();
    println!("Setup: A photon is sent toward two slits. After it passes through,");
    println!("you choose whether to recombine the paths (wave measurement) or");
    println!("detect which slit it went through (particle measurement).");
    println!();
    println!("The bizarre result: Your LATER choice determines what the photon");
    println!("did EARLIER. If you measure wave behavior, it must have gone through");
    println!("both slits as a wave. If you measure particle behavior, it must have");
    println!("gone through one slit as a particle.");
    println!();
    println!("Wheeler's conclusion: No phenomenon is a phenomenon until it is");
    println!("an observed phenomenon. The past is not fixed — it depends on how");
    println!("we choose to look at it now.");
}

fn demonstrate_delayed_choice() {
    println!("--- DEMONSTRATION: Delayed Choice ---");
    println!();

    // Simulate the experiment with deterministic outcomes
    let photon_path = "photon emitted → travels toward slits";
    println!("{}", photon_path);
    println!();

    // Scenario 1: Choose to measure as wave (recombine paths)
    println!("SCENARIO A: After passing through slits, we recombine the paths");
    println!("          and look for interference pattern.");
    println!();
    let interference = simulate_interference();
    println!("Result: Interference pattern detected? {}", interference);
    println!("Implication: The photon must have traveled as a WAVE through BOTH slits.");
    println!("           We chose to measure wave behavior AFTER it passed the slits,");
    println!("           and that choice determined it was a wave all along.");
    println!();

    // Scenario 2: Choose to measure as particle (which-path)
    println!("SCENARIO B: After passing through slits, we place detectors at");
    println!("          each slit to see which one the photon used.");
    println!();
    let which_slit = simulate_which_path();
    println!("Result: Photon detected at slit {}", which_slit);
    println!("Implication: The photon traveled as a PARTICLE through ONE slit.");
    println!("           We chose to measure particle behavior AFTER it passed the");
    println!("           slits, and that choice determined it was a particle all along.");
    println!();

    println!("THE PARADOX: In both scenarios, the photon had already passed through");
    println!("the slits before we made our measurement choice. Yet our later choice");
    println!("determined what actually happened earlier. The past is not determined");
    println!("until we measure it.");
}

fn simulate_interference() -> bool {
    // Deterministic: wave measurement always shows interference
    true
}

fn simulate_which_path() -> u8 {
    // Deterministic: particle measurement finds one specific slit
    1
}

fn explain_it_from_bit() {
    println!("--- 'IT FROM BIT' PHILOSOPHY ---");
    println!();
    println!("Wheeler's deepest idea, developed in his later years:");
    println!();
    println!("Every physical entity — every particle, every field of force, even the");
    println!("spacetime continuum itself — derives its function, its meaning, its very");
    println!("being entirely from binary choices, yes-or-no responses, bits.");
    println!();
    println!("'It' (matter, energy, spacetime) arises from 'bit' (information).");
    println!("Reality is not made of stuff — it's made of information. The universe");
    println!("is a self-organizing system that generates itself through questions and");
    println!("answers, each answer being a binary choice.");
}

fn demonstrate_participatory_universe() {
    println!("--- DEMONSTRATION: Participatory Universe ---");
    println!();

    // Show how information builds reality through binary choices
    let mut universe = 0u8;
    let questions = [
        "Is there gravity? (yes=1, no=0)",
        "Do particles have spin? (yes=1, no=0)",
        "Is spacetime curved? (yes=1, no=0)",
        "Does light bend around stars? (yes=1, no=0)",
    ];

    let answers = [1u8, 1, 1, 1]; // All yes — our universe

    println!("Building reality through binary questions:");
    println!();

    for (i, question) in questions.iter().enumerate() {
        let answer = answers[i];
        universe = (universe << 1) | answer;
        println!("Q{}: {}", i + 1, question);
        println!("    A: {} — reality bit set", answer);
        println!("    Universe state so far: {:04b}", universe);
        println!();
    }

    println!(
        "Final universe configuration: {:04b} (binary) = {} (decimal)",
        universe, universe
    );
    println!();
    println!("Each 'it' in the universe emerged from a sequence of 'bits'.");
    println!("The observer participates by asking questions and receiving answers.");
    println!("Without questions being asked, there are no answers — no reality.");
    println!();

    // Connect to Everett
    println!("CONNECTION TO EVERETT: Wheeler was Hugh Everett's PhD advisor at");
    println!("Princeton. He supervised Everett's 1957 thesis that proposed the");
    println!("Many-Worlds Interpretation. Wheeler initially resisted it, but later");
    println!("came to appreciate its implications for his participatory universe.");
    println!("Both ideas share a core insight: reality is not fixed until observed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delayed_choice_interference() {
        let result = simulate_interference();
        assert!(result); // Wave measurement always shows interference
    }

    #[test]
    fn test_delayed_choice_which_path() {
        let result = simulate_which_path();
        assert!(result == 1 || result == 2); // Must be one of the two slits
    }

    #[test]
    fn test_participatory_universe_bits() {
        let mut universe = 0u8;
        let answers = [1u8, 1, 1, 1];

        for answer in &answers {
            universe = (universe << 1) | *answer;
        }

        // Four yes-answers should give binary 1111 = 15
        assert_eq!(universe, 15);
    }

    #[test]
    fn test_universe_state_progression() {
        let mut universe = 0u8;
        let answers = [1u8, 0, 1, 1];

        // After first bit: 1
        universe = (universe << 1) | answers[0];
        assert_eq!(universe, 1);

        // After second bit: 10 = 2
        universe = (universe << 1) | answers[1];
        assert_eq!(universe, 2);

        // After third bit: 101 = 5
        universe = (universe << 1) | answers[2];
        assert_eq!(universe, 5);

        // After fourth bit: 1011 = 11
        universe = (universe << 1) | answers[3];
        assert_eq!(universe, 11);
    }
}
