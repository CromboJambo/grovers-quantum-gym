//! Thompson NFA Regular Expression Engine (1968)
//!
//! Builds a nondeterministic finite automaton from a regex pattern,
//! then simulates it using subset construction. Based on Ken Thompson's
//! original algorithm for the ed editor.


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "regex",
    version: "0.0.1",
    formalized: false,
    notes: "Regular expression matching simulation",
};


use std::collections::{HashMap, HashSet};

// AST node types
#[derive(Debug)]
enum AstNode {
    Literal(char),
    Any,
    Concat(Vec<AstNode>),
    Star(Box<AstNode>),
    Plus(Box<AstNode>),
    Opt(Box<AstNode>),
}

// NFA state: id -> list of (input char or '.' for any, target state)
#[derive(Debug)]
struct State {
    transitions: Vec<(char, i32)>,
    epsilon: Vec<i32>,
}

fn new_state() -> State {
    State {
        transitions: vec![],
        epsilon: vec![],
    }
}

// Thompson construction: compile AST to NFA fragment (start, accept) state ids
fn compile(node: &AstNode, states: &mut HashMap<i32, State>, next_id: &mut i32) -> (i32, i32) {
    match node {
        AstNode::Literal(ch) => {
            let start = *next_id;
            let accept = start + 1;
            states.insert(start, new_state());
            states.insert(accept, new_state());
            states
                .get_mut(&start)
                .unwrap()
                .transitions
                .push((*ch, accept));
            *next_id = accept + 1;
            (start, accept)
        }
        AstNode::Any => {
            let start = *next_id;
            let accept = start + 1;
            states.insert(start, new_state());
            states.insert(accept, new_state());
            states
                .get_mut(&start)
                .unwrap()
                .transitions
                .push(('.', accept));
            *next_id = accept + 1;
            (start, accept)
        }
        AstNode::Concat(nodes) => {
            let mut current_start = -1;
            let mut prev_accept = -1;

            for node in nodes {
                let (s, a) = compile(node, states, next_id);
                if current_start == -1 {
                    current_start = s;
                }
                if prev_accept != -1 {
                    // Link previous accept to this start via epsilon
                    states.get_mut(&prev_accept).unwrap().epsilon.push(s);
                }
                prev_accept = a;
            }

            (current_start, prev_accept)
        }
        AstNode::Star(child) => {
            let (cs, ca) = compile(child, states, next_id);
            let start = *next_id;
            let accept = start + 1;
            states.insert(start, new_state());
            states.insert(accept, new_state());

            // Start -> child start (epsilon)
            states.get_mut(&start).unwrap().epsilon.push(cs);
            // Child accept -> child start (loop, epsilon)
            states.get_mut(&ca).unwrap().epsilon.push(cs);
            // Child accept -> final accept (exit loop, epsilon)
            states.get_mut(&ca).unwrap().epsilon.push(accept);
            // Start -> final accept (zero occurrences, epsilon)
            states.get_mut(&start).unwrap().epsilon.push(accept);

            *next_id = accept + 1;
            (start, accept)
        }
        AstNode::Plus(child) => {
            let (cs, ca) = compile(child, states, next_id);
            let start = *next_id;
            let accept = start + 1;
            states.insert(start, new_state());
            states.insert(accept, new_state());

            // Start -> child start (at least one occurrence)
            states.get_mut(&start).unwrap().epsilon.push(cs);
            // Child accept -> child start (loop)
            states.get_mut(&ca).unwrap().epsilon.push(cs);
            // Child accept -> final accept (exit after at least one)
            states.get_mut(&ca).unwrap().epsilon.push(accept);

            *next_id = accept + 1;
            (start, accept)
        }
        AstNode::Opt(child) => {
            let (cs, ca) = compile(child, states, next_id);
            let start = *next_id;
            let accept = start + 1;
            states.insert(start, new_state());
            states.insert(accept, new_state());

            // Start -> child start
            states.get_mut(&start).unwrap().epsilon.push(cs);
            // Child accept -> final accept
            states.get_mut(&ca).unwrap().epsilon.push(accept);
            // Start -> final accept (skip, zero occurrences)
            states.get_mut(&start).unwrap().epsilon.push(accept);

            *next_id = accept + 1;
            (start, accept)
        }
    }
}

// Compute epsilon closure of a set of states
fn epsilon_closure(states: &HashMap<i32, State>, start_states: &[i32]) -> HashSet<i32> {
    let mut result = HashSet::new();
    let mut stack: Vec<i32> = start_states.to_vec();

    while let Some(s) = stack.pop() {
        if result.contains(&s) {
            continue;
        }
        result.insert(s);

        if let Some(state) = states.get(&s) {
            for &eps_target in &state.epsilon {
                if !result.contains(&eps_target) {
                    stack.push(eps_target);
                }
            }
        }
    }

    result
}

// NFA simulation using subset construction (power set algorithm)
fn match_nfa(start: i32, accept: i32, states: &HashMap<i32, State>, input: &str) -> bool {
    // Compute epsilon closure of start state
    let mut current_states = epsilon_closure(states, &[start]);

    for ch in input.chars() {
        let mut next_states = HashSet::new();

        // For each state in current set, check transitions on this character
        for state_id in current_states.iter() {
            if let Some(state) = states.get(state_id) {
                for (trans_char, target) in &state.transitions {
                    if *trans_char == '.' || *trans_char == ch {
                        next_states.insert(*target);
                    }
                }
            }
        }

        // Compute epsilon closure of the resulting states
        let owned: Vec<i32> = next_states.iter().cloned().collect();
        current_states = epsilon_closure(states, &owned);
    }

    // After consuming all input, compute epsilon closure and check for accept state
    let final_closure =
        epsilon_closure(states, &current_states.iter().cloned().collect::<Vec<_>>());
    final_closure.contains(&accept)
}

// Simplified regex parser
fn parse_regex(pattern: &str) -> AstNode {
    let mut nodes = vec![];
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '.' {
            nodes.push(AstNode::Any);
        } else if ch == '*' {
            let last = nodes.pop().unwrap();
            nodes.push(AstNode::Star(Box::new(last)));
        } else if ch == '+' {
            let last = nodes.pop().unwrap();
            nodes.push(AstNode::Plus(Box::new(last)));
        } else if ch == '?' {
            let last = nodes.pop().unwrap();
            nodes.push(AstNode::Opt(Box::new(last)));
        } else {
            nodes.push(AstNode::Literal(ch));
        }

        i += 1;
    }

    AstNode::Concat(nodes)
}

// Test harness
fn test_regex(pattern: &str, input: &str, expected: bool) {
    let ast = parse_regex(pattern);
    let mut states = HashMap::new();
    let mut next_id = 1;
    let (start, accept) = compile(&ast, &mut states, &mut next_id);
    let result = match_nfa(start, accept, &states, input);

    if result == expected {
        println!("✓ \"{}\" matches /{}/: {}", input, pattern, result);
    } else {
        println!(
            "✗ \"{}\" matches /{}/: {} (expected {})",
            input, pattern, result, expected
        );
    }
}

pub fn show_regex_station() {
    println!("=== THOMPSON NFA REGEX ENGINE (1968) ===");
    println!("Based on Ken Thompson's algorithm for the ed editor\n");

    // Basic literal matching
    test_regex("hello", "hello", true);
    test_regex("hello", "world", false);

    // Wildcard
    test_regex("h.llo", "hello", true);
    test_regex("h.llo", "hallo", true);
    test_regex("h.llo", "hllo", false);

    // Kleene star
    test_regex("ab*c", "ac", true); // Zero b's
    test_regex("ab*c", "abc", true); // One b
    test_regex("ab*c", "abbbc", true); // Three b's
    test_regex("ab*c", "abcd", false);

    // Plus quantifier
    test_regex("ab+c", "ac", false); // Need at least one b
    test_regex("ab+c", "abc", true);
    test_regex("ab+c", "abbbc", true);

    // Optional
    test_regex("colou?r", "color", true);
    test_regex("colou?r", "colour", true);
    test_regex("colou?r", "colur", false);

    println!("\n=== REGEX ENGINE COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_match() {
        let ast = parse_regex("hello");
        let mut states = HashMap::new();
        let mut next_id = 1;
        let (start, accept) = compile(&ast, &mut states, &mut next_id);
        assert!(match_nfa(start, accept, &states, "hello"));
        assert!(!match_nfa(start, accept, &states, "world"));
    }

    #[test]
    fn test_wildcard() {
        let ast = parse_regex("h.llo");
        let mut states = HashMap::new();
        let mut next_id = 1;
        let (start, accept) = compile(&ast, &mut states, &mut next_id);
        assert!(match_nfa(start, accept, &states, "hello"));
        assert!(match_nfa(start, accept, &states, "hallo"));
        assert!(!match_nfa(start, accept, &states, "hllo"));
    }

    #[test]
    fn test_kleene_star() {
        let ast = parse_regex("ab*c");
        let mut states = HashMap::new();
        let mut next_id = 1;
        let (start, accept) = compile(&ast, &mut states, &mut next_id);
        assert!(match_nfa(start, accept, &states, "ac"));
        assert!(match_nfa(start, accept, &states, "abc"));
        assert!(match_nfa(start, accept, &states, "abbbc"));
        assert!(!match_nfa(start, accept, &states, "abcd"));
    }

    #[test]
    fn test_plus_quantifier() {
        let ast = parse_regex("ab+c");
        let mut states = HashMap::new();
        let mut next_id = 1;
        let (start, accept) = compile(&ast, &mut states, &mut next_id);
        assert!(!match_nfa(start, accept, &states, "ac"));
        assert!(match_nfa(start, accept, &states, "abc"));
    }

    #[test]
    fn test_optional() {
        let ast = parse_regex("colou?r");
        let mut states = HashMap::new();
        let mut next_id = 1;
        let (start, accept) = compile(&ast, &mut states, &mut next_id);
        assert!(match_nfa(start, accept, &states, "color"));
        assert!(match_nfa(start, accept, &states, "colour"));
    }
}
