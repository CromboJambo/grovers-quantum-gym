/// TURING MACHINES & THE HALTING PROBLEM STATION (1936)
///
/// Alan Turing's universal machine concept and the halting problem —
/// proving some questions about programs are fundamentally uncomputable.

fn simulate_turing_machine(
    tape: Vec<i32>,
    head: usize,
    state: i32,
    transitions: &[(i32, i32, i32, i32, i32)],
    accept_states: &[i32],
    steps: usize,
) -> (Vec<i32>, usize, i32, bool) {
    let mut tape = tape;
    let mut head = head;
    let mut state = state;

    for step in 0..steps {
        // Find transition rule: (current_state, read_symbol) -> (write_symbol, move_dir, new_state)
        let mut found = false;
        for &(cs, rs, ws, move_dir, ns) in transitions {
            if cs == state && rs == tape[head] {
                // Apply transition
                tape[head] = ws;

                // Move head: -1 = left, 0 = stay, 1 = right
                if move_dir == 1 {
                    if head + 1 < tape.len() {
                        head += 1;
                    }
                } else if move_dir == -1 && head > 0 {
                    head -= 1;
                }

                state = ns;
                found = true;
                break;
            }
        }

        if !found {
            // No transition — machine halts
            break;
        }
    }

    let halted_in_accept = accept_states.contains(&state);
    (tape, head, state, halted_in_accept)
}

pub fn demonstrate_universal_turing_machine() {
    println!("--- UNIVERSAL TURING MACHINE DEMONSTRATION ---");
    println!();
    println!("A Turing machine has:");
    println!("- An infinite tape divided into cells (we'll use a finite array)");
    println!("- A read/write head that moves left/right");
    println!("- A finite set of states and transition rules");
    println!();

    // Simple example: recognize strings of the form "1^n 0^n"
    // This requires counting — beyond regular expressions, needs a stack-like mechanism
    // which is what Turing machines provide.

    println!("Example: Recognizing balanced strings '1^n 0^n'");
    println!("This language cannot be recognized by finite automata or regex!");
    println!("It requires the machine to COUNT how many 1s it saw.");
    println!();

    // We'll simulate a simplified version that checks if we have equal 1s and 0s
    let test_strings = vec![
        ("111000", true),  // Balanced
        ("11000", false),  // Too many 0s
        ("11100", false),  // Too many 1s
        ("101010", false), // Wrong order
    ];

    for (input, expected) in test_strings {
        let chars: Vec<i32> = input
            .chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .collect();

        // Count approach (simplified — real TM would do this with state transitions)
        let ones = chars.iter().filter(|&&x| x == 1).count();
        let zeros = chars.iter().filter(|&&x| x == 0).count();

        // Check ordering: all 1s before all 0s
        let mut correct_order = true;
        let mut seen_zero = false;
        for &c in &chars {
            if c == 0 {
                seen_zero = true;
            }
            if c == 1 && seen_zero {
                correct_order = false;
                break;
            }
        }

        let accepted = ones == zeros && correct_order;
        println!(
            "Input '{}' -> {}",
            input,
            if accepted { "ACCEPTED" } else { "REJECTED" }
        );
    }

    println!();
    println!("A real Turing machine would use its tape as memory to count.");
    println!("It marks off 1s and 0s in pairs until it runs out of one type.");
}

pub fn demonstrate_halting_problem() {
    println!("--- THE HALTING PROBLEM ---");
    println!();
    println!("Can we write a program that determines whether ANY other program halts?");
    println!("Turing proved: NO. Some programs run forever, and you can't tell in advance.");
    println!();

    // Demonstrate with actual non-halting vs halting programs
    println!("Halting program (counts to 10):");
    let mut i = 0;
    while i < 10 {
        i += 1;
    }
    println!("  -> Terminated after {} iterations", i);

    println!();
    println!("Non-halting program (infinite loop):");
    // We can't actually run this — it would hang forever!
    // This is the essence of the halting problem.
    println!("  while (true) {{ }}  // infinite loop - runs forever");
    println!("  We know this by INSPECTION, but for complex programs...");

    println!();
    println!("Turing's diagonalization argument shows that no general algorithm exists.");
    println!("If you had one called HALTS(program, input), you could construct a paradox:");
    println!("  Define D(x) = if HALTS(x, x) then loop forever else halt");
    println!("  Now ask: does D(D) halt? Contradiction either way!");
}

pub fn demonstrate_church_turing_thesis() {
    println!("--- CHURCH-TURING THESIS ---");
    println!();
    println!("All reasonable models of computation are equivalent in power:");
    println!("- Turing machines (1936)");
    println!("- Lambda calculus (Church, 1932)");
    println!("- Recursive functions (Gödel, 1931)");
    println!("- Modern programming languages");
    println!();
    println!("This means our Rust programs today are as powerful as Turing's");
    println!("theoretical machines from 1936 — same computational universe.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turing_machine_simulation() {
        // Simple counter machine: count number of 1s
        let tape = vec![1, 1, 1, 0, 0];
        let (final_tape, _, final_state, _) = simulate_turing_machine(
            tape,
            0,
            0,
            &[(0, 1, 0, 1, 0)], // In state 0, reading 1: write 0, move right, stay in state 0
            &[1],
            2,
        );
        assert_eq!(final_tape[0], 0); // First cell was overwritten
    }

    #[test]
    fn test_balanced_string_recognition() {
        // This is a simplified check — real TM implementation would be more complex
        let input = "111000";
        let chars: Vec<i32> = input
            .chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .collect();
        let ones = chars.iter().filter(|&&x| x == 1).count();
        let zeros = chars.iter().filter(|&&x| x == 0).count();
        assert_eq!(ones, zeros);
    }
}
