/// The Philosophy of Problem-Solving: Celebrating Collapse
/// Based on the user's "Math Comeback Theory" — a reflection on learning,
/// being wrong, and what it means to consider something "solved."


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "collapse",
    version: "0.0.1",
    formalized: false,
    notes: "Wave function collapse simulation",
};


pub fn show_collapse_station() {
    println!("=== THE PHILOSOPHY OF PROBLEM-SOLVING: CELEBRATING COLLAPSE ===");
    println!();
    print_introduction();
    println!();
    demonstrate_learning_from_wrong();
    println!();
    demonstrate_cognitive_closure();
    println!();
    conclude_perfect_participation();
}

fn print_introduction() {
    println!("This station explores a simple question: why do we stop looking?");
    println!();
    println!("When you find something that works, you tend to stop searching.");
    println!("Is that laziness? Or is it something else entirely?");
    println!();
    println!("Maybe we're not lazy. Maybe we're just celebrating the successful");
    println!("collapse of a state we couldn't actually witness.");
    println!();
    println!("Consider this: before you solve a problem, there are many possible");
    println!("solutions superposed in your mind. You can't see them all at once.");
    println!("When you finally find one that works — boom. The state collapses.");
    println!("One outcome becomes real to you. The rest disappear from view.");
    println!();
    println!("Your relaxation isn't a failure. It's the natural rest after");
    println!("measurement. You witnessed the collapse.");
}

fn demonstrate_learning_from_wrong() {
    println!("--- WE LEARN MORE FROM WHAT'S WRONG ---");
    println!();
    println!("Think about it: when you're wrong, you discover something new.");
    println!("When you're right... you might just stop.");
    println!();
    println!("Imagine testing ten possible explanations for why a machine isn't");
    println!("working. Each time one fails, you eliminate a possibility. You learn.");
    println!();
    println!("But when one finally works — the machine starts — you stop investigating.");
    println!("Success closes the investigation. Failure tends to reopen it.");
    println!();
    println!("This is why science focuses on falsifiability: can we specify what");
    println!("would count as evidence AGAINST our theory? A theory that can't be");
    println!("wrong isn't really testable at all.");
    println!();
    println!("But here's the thing: failure isn't automatically informative either.");
    println!("A badly designed experiment can fail without telling you much. And a");
    println!("successful experiment can teach you a tremendous amount when it");
    println!("distinguishes between competing explanations.");
    println!();
    println!("The key insight: an outcome becomes especially informative when it");
    println!("eliminates possibilities we previously considered plausible.");

    // Simulate hypothesis testing
    let hypotheses = vec![
        "Bulb is burnt out",
        "Wire is disconnected",
        "Switch is broken",
        "Power supply failed",
        "Circuit breaker tripped",
    ];

    println!();
    println!("Example: debugging a broken light");
    println!("Testing hypotheses one by one:");

    let mut tested = 0;
    for (i, hyp) in hypotheses.iter().enumerate() {
        println!("  Test {}: {} ...", i + 1, hyp);
        if i == 3 {
            println!("    → CORRECT! Power supply failed. Light works.");
            break;
        }
        println!("    → Wrong. Eliminated.");
        tested += 1;
    }

    println!();
    println!(
        "We learned from {} wrong hypotheses before finding the right one.",
        tested
    );
    println!("Each 'no' brought us closer to understanding.");
}

fn demonstrate_cognitive_closure() {
    println!("--- TWO KINDS OF CLOSURE ---");
    println!();
    println!("There's a subtle but important distinction:");
    println!();
    println!("EPISTEMIC CLOSURE: You've gathered enough evidence to justify");
    println!("a conclusion within a defined scope. This is rational.");
    println!();
    println!("PSYCHOLOGICAL CLOSURE: You FEEL that the question has been answered,");
    println!("so you stop searching. This can happen even without sufficient evidence.");
    println!();
    println!("The danger is mistaking the second for the first.");
    println!();
    println!("Science tries to manage this through replication, criticism, and the");
    println!("possibility of revisiting conclusions. Those practices don't eliminate");
    println!("premature closure, but they help prevent one person's confidence from");
    println!("becoming the final authority.");
    println!();
    println!("And this connects to an earlier insight: who defines the game?");
    println!("A system's definition of 'solved' determines which questions remain");
    println!("open and which become invisible. The person in power gets to decide");
    println!("when we've reached closure — even if we haven't really learned anything.");

    // Demonstrate premature vs earned closure
    println!();
    println!("Example: Is this coin fair?");
    println!("Flip 3 times: H, H, T → 'Looks fair!' (premature closure)");
    println!("Flip 100 times: 98H, 2T → 'Definitely biased.' (earned closure)");
    println!();
    println!("Both feel like closure. Only one is justified.");
}

fn conclude_perfect_participation() {
    println!("--- PERFECT PARTICIPATION ---");
    println!();
    println!("Here's a radical thought: maybe the problem isn't that we learn too");
    println!("slowly. Maybe it's that not enough people are learning.");
    println!();
    println!("If everyone participated perfectly in the search for truth — if every");
    println!("person was actively testing hypotheses, eliminating wrong answers,");
    println!("contributing their perspective — growth would be exponential.");
    println!();
    println!("But we don't have perfect participation. Most people accept what they're");
    println!("told. They reach psychological closure without earning epistemic closure.");
    println!("They stop seeking not because they found the answer, but because they");
    println!("gave up the search.");
    println!();
    println!("Until we can figure out another way to achieve perfect participation,");
    println!("that's the only way we get sub-exponential growth. We're limited by how");
    println!("many people are actually seeking — not just consuming.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_runs() {
        // Just verify it doesn't panic
        show_collapse_station();
    }
}
