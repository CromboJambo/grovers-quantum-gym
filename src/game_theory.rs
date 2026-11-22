/// Game Theory Station: Nash Equilibrium and Cooperation Emergence
/// Demonstrates the Prisoner's Dilemma, iterated strategies, and how
/// cooperation emerges from selfish rationality — connecting to the
/// philosophical theme that problems propagate with solutions.


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "game_theory",
    version: "0.0.1",
    formalized: false,
    notes: "Quantum game theory simulations",
};


pub fn show_game_theory_station() {
    println!("=== GAME THEORY: NASH EQUILIBRIUM AND COOPERATION ===");
    println!();
    print_biography();
    println!();
    demonstrate_prisoners_dilemma();
    println!();
    demonstrate_iterated_cooperation();
}

fn print_biography() {
    println!("John Nash (1928-2015)");
    println!("American mathematician, Fields Medalist (1994)");
    println!();
    println!("At age 21, Nash proved the existence of equilibrium in non-");
    println!("cooperative games. His insight: when rational agents compete,");
    println!("the outcome is not necessarily optimal for anyone — cooperation");
    println!("can emerge only through repeated interaction and trust.");
    println!();
    println!("His life story (depicted in 'A Beautiful Mind') reflects the");
    println!("theorem itself: even brilliant rational minds struggle to escape");
    println!("the traps of self-referential thinking and isolation.");
}

fn demonstrate_prisoners_dilemma() {
    println!("--- THE PRISONER'S DILEMMA ---");
    println!();
    println!("Two suspects are arrested. Each can either COOPERATE (stay silent)");
    println!("or DEFECT (betray the other). Payoffs (years in prison):");
    println!();
    println!("             Bob Cooperates    Bob Defects");
    println!("Alice Cooperates     -1, -1          -5, 0");
    println!("Alice Defects        0, -5           -3, -3");
    println!();
    println!("Rational analysis for Alice:");
    println!("  If Bob cooperates: I should defect (0 < -1)");
    println!("  If Bob defects:   I should defect (-3 < -5)");
    println!("  Therefore, rational choice is to DEFECT.");
    println!();
    println!("Same logic applies to Bob. Both defect → both get -3 years.");
    println!("But if both had cooperated, they'd each get only -1 year!");
    println!();
    println!("Nash Equilibrium: (Defect, Defect) — stable but suboptimal.");
    println!("The rational choice leads to a collectively worse outcome.");
    println!();
    println!("This is the 'problem propagating with solutions' in action:");
    println!("each agent's optimal strategy creates a worse situation for all.");

    // Simulate single-shot game
    let alice_choice = "DEFECT";
    let bob_choice = "DEFECT";
    let outcome = -3;

    println!();
    println!("Simulation (single-shot):");
    println!("  Alice chooses: {}", alice_choice);
    println!("  Bob chooses:   {}", bob_choice);
    println!("  Outcome: Both get {} years (Nash equilibrium)", outcome);
    println!("  Could have gotten: -1 year each if they cooperated");
}

fn demonstrate_iterated_cooperation() {
    println!();
    println!("--- ITERATED PRISONER'S DILEMMA ---");
    println!();
    println!("Now imagine the game is played many times. Can cooperation emerge?");
    println!("Robert Axelrod (1984) ran computer tournaments to find out.");
    println!();

    // Simulate iterated game with different strategies
    let rounds = 50;

    // Tit-for-Tat vs Always Defect
    let mut tft_score = 0;
    let mut ad_score = 0;
    let mut prev_tft = "COOPERATE";

    println!("Strategy: Tit-for-Tat (cooperate first, then mirror opponent)");
    println!("vs Strategy: Always Defect");
    println!();

    for round in 1..=rounds {
        let tft_move = prev_tft;
        let ad_move = "DEFECT";

        // Payoff matrix
        if tft_move == "COOPERATE" && ad_move == "DEFECT" {
            tft_score -= 5;
            ad_score += 0;
        } else if tft_move == "DEFECT" && ad_move == "DEFECT" {
            tft_score -= 3;
            ad_score -= 3;
        }

        prev_tft = ad_move; // TFT mirrors opponent's last move
    }

    println!("After {} rounds:", rounds);
    println!("  Tit-for-Tat score: {}", tft_score);
    println!("  Always Defect score: {}", ad_score);
    println!();

    // Now simulate two Tit-for-Tats cooperating
    let mut tft1_score = 0;
    let mut tft2_score = 0;
    let mut prev_tft1 = "COOPERATE";
    let mut prev_tft2 = "COOPERATE";

    println!("Strategy: Tit-for-Tat vs Tit-for-Tat");
    println!();

    for round in 1..=rounds {
        let move1 = prev_tft1;
        let move2 = prev_tft2;

        if move1 == "COOPERATE" && move2 == "COOPERATE" {
            tft1_score -= 1;
            tft2_score -= 1;
        } else if move1 == "DEFECT" && move2 == "DEFECT" {
            tft1_score -= 3;
            tft2_score -= 3;
        }

        prev_tft1 = move2;
        prev_tft2 = move1;
    }

    println!("After {} rounds:", rounds);
    println!("  Tit-for-Tat 1 score: {}", tft1_score);
    println!("  Tit-for-Tat 2 score: {}", tft2_score);
    println!();

    println!("KEY INSIGHT:");
    println!("  In repeated games, cooperation emerges as the rational strategy.");
    println!("  The 'shadow of the future' changes behavior — you can punish");
    println!("  defection by defecting in return next time.");
    println!();
    println!("  This connects to our earlier discussion: problems propagate");
    println!("  with solutions, but REPEATED INTERACTION and MEMORY allow");
    println!("  better outcomes. The universe isn't a single-shot game; it's");
    println!("  an iterated one where cooperation can evolve.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prisoners_dilemma_payoffs() {
        // Verify the payoff structure is correct (negative = prison years, higher is better)
        let cooperate_cooperate = (-1, -1);
        let defect_defect = (-3, -3);
        let betray_cooperate = (0, -5);

        assert!(cooperate_cooperate.0 > defect_defect.0); // Cooperation better than mutual defection
        assert!(betray_cooperate.0 > cooperate_cooperate.0); // Betrayal best for betrayer
        assert!(betray_cooperate.1 < defect_defect.1); // Betrayed worst off
    }

    #[test]
    fn test_iterated_tit_for_tat_cooperates() {
        // Two TFT strategies should cooperate most of the time
        let rounds = 100;
        let mut tft1_score = 0;
        let mut tft2_score = 0;
        let mut prev_tft1 = "COOPERATE";
        let mut prev_tft2 = "COOPERATE";

        for _ in 1..=rounds {
            let move1 = prev_tft1;
            let move2 = prev_tft2;

            if move1 == "COOPERATE" && move2 == "COOPERATE" {
                tft1_score -= 1;
                tft2_score -= 1;
            } else if move1 == "DEFECT" && move2 == "DEFECT" {
                tft1_score -= 3;
                tft2_score -= 3;
            }

            prev_tft1 = move2;
            prev_tft2 = move1;
        }

        // With perfect TFT, they should always cooperate → -1 per round each
        assert_eq!(tft1_score, -rounds);
        assert_eq!(tft2_score, -rounds);
    }
}
