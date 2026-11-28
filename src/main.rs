use clap::{Parser, CommandFactory};
use clap_complete::{generate, Shell};
use shor_grover_exploration::grover::{grover_search, linear_search};
use shor_grover_exploration::shor::shor_factor;
use std::io;
use std::time::Instant;

const STATIONS: &[&str] = &[
    "grover", "shor", "problems", "lovelace", "turing", "fft", "regex", "rsa",
    "sorting_bounds", "attention", "euler", "everett", "godel", "bell",
    "game_theory", "deutsch", "collapse", "chaitin", "wheeler", "p_vs_np",
    "eth", "navier_stokes", "riemann", "yang_mills", "von_neumann",
    "relativity", "rovelli", "kripke_platek", "shannon", "lemper_ziv",
    "positional_tracing", "matmul_exponent", "crypto_economics",
    "quantum_circuit_complexity",
];

#[derive(Parser)]
#[command(name = "shor-grover-exploration", about = "Grover's Quantum Gym")]
struct Cli {
    /// Run only this station (run all if omitted)
    #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(STATIONS))]
    station: Option<String>,

    /// Generate shell completions and exit
    #[arg(long, value_enum)]
    completions: Option<Shell>,
}

fn should_run(selected: &Option<String>, name: &str) -> bool {
    match selected {
        None => true,
        Some(s) => s == name,
    }
}

fn main() {
    let cli = Cli::parse();

    // Handle completions generation
    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        generate(shell, &mut cmd, "shor-grover-exploration", &mut io::stdout());
        return;
    }

    let run_all = cli.station.is_none();

    if run_all {
        println!("=== QUANTUM GYM: EXPLORATION SESSION ===\n");
    }

    // Grover's Search - scaling study + multiple targets
    if should_run(&cli.station, "grover") {
        if !run_all {
            println!("=== QUANTUM GYM: GROVER STATION ===\n");
        } else {
            println!("--- GROVER'S SEARCH: SCALING STUDY ---");
        }
        let sizes = [8, 64, 256, 1024];
        for &n in &sizes {
            let target = n / 2;

            // Classical
            let start = Instant::now();
            let classical_result = linear_search(n, |x| x == target);
            let classical_time = start.elapsed();

            // Grover's
            let start = Instant::now();
            let grover_result = grover_search(n, |x| x == target);
            let grover_time = start.elapsed();

            println!(
                "N={:5}: classical {:6.3}ms (found={}) vs Grover {:6.3}ms (found={})",
                n,
                classical_time.as_secs_f64() * 1000.0,
                classical_result.is_some(),
                grover_time.as_secs_f64() * 1000.0,
                grover_result.is_some()
            );
        }

        // Grover's with multiple marked states
        println!("\n--- GROVER'S SEARCH: MULTIPLE TARGETS ---");
        let result = grover_search(100, |x| x % 7 == 0);
        println!(
            "Find multiples of 7 in [0,100): found={}",
            result.map(|r| r.to_string()).unwrap_or_default()
        );
    }

    // Shor's Factoring - stress test with larger numbers
    if should_run(&cli.station, "shor") {
        println!("\n--- SHOR'S FACTORING: STRESS TEST ---");
        let numbers = [15u64, 21, 35, 55, 77, 91, 143, 221, 323, 391];
        for &n in &numbers {
            let result = shor_factor(n);
            match result {
                Some((p, q)) => println!("{:4} = {} \u{00d7} {}", n, p, q),
                None => println!("{:4} = (failed to factor)", n),
            }
        }
    }

    // NP-Hard Problems - Grover vs Classical comparison
    if should_run(&cli.station, "problems") {
        println!("\n--- NP-HARD PROBLEMS ---");

        // Subset Sum
        let numbers = vec![3, 34, 4, 5, 6, 12, 9, 8];
        let target = 27;
        let result = shor_grover_exploration::problems::subset_sum_classical(&numbers, target);
        println!("Subset sum (target={})", target);
        if let Some(indices) = result {
            let mut sum = 0;
            let mut parts = Vec::new();
            for &i in &indices {
                parts.push(numbers[i].to_string());
                sum += numbers[i];
            }
            println!("  Found: {} = {}", parts.join(" + "), sum);
        }

        // SAT Problem
        println!("\nSAT problem (3-CNF with 4 variables):");
        // (x1 ∨ x2) ∧ (¬x1 ∨ x3) ∧ (x2 ∨ ¬x3 ∨ x4)
        let clauses = vec![
            vec![1, 2],     // x1 ∨ x2
            vec![-1, 3],    // ¬x1 ∨ x3
            vec![2, -3, 4], // x2 ∨ ¬x3 ∨ x4
        ];
        let result = shor_grover_exploration::problems::sat_classical(&clauses, 4);
        match result {
            Some(assignment) => {
                println!("  Satisfiable! Assignment:");
                for (i, &val) in assignment.iter().enumerate() {
                    println!("    x{} = {}", i + 1, if val { "true" } else { "false" });
                }
            }
            None => println!("  Unsatisfiable"),
        }

        // TSP - Nearest Neighbor vs Brute Force Comparison
        println!("\nTSP (Nearest Neighbor Heuristic):");
        let distances = vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0],
        ];
        let (tour, total) = shor_grover_exploration::problems::tsp_nearest_neighbor(&distances);
        println!(
            "  Tour: {}",
            tour.iter()
                .map(|&c| c.to_string())
                .collect::<Vec<_>>()
                .join(" \u{2192} ")
        );
        println!("  Total distance: {}", total);
    }

    if should_run(&cli.station, "lovelace") {
        println!("\n--- ADA LOVELACE STATION ---");
        shor_grover_exploration::lovelace::show_lovelace_station();
    }

    if should_run(&cli.station, "turing") {
        println!("\n--- TURING MACHINES & HALTING PROBLEM (1936) ---");
        shor_grover_exploration::turing::demonstrate_universal_turing_machine();
        shor_grover_exploration::turing::demonstrate_halting_problem();
        shor_grover_exploration::turing::demonstrate_church_turing_thesis();
    }

    if should_run(&cli.station, "fft") {
        println!("\n--- FAST FOURIER TRANSFORM (1965) ---");
        shor_grover_exploration::fft::demonstrate_fft();
    }

    if should_run(&cli.station, "regex") {
        println!("\n--- THOMPSON REGEX ENGINE (1968) ---");
        shor_grover_exploration::regex::show_regex_station();
    }

    if should_run(&cli.station, "rsa") {
        println!("\n--- RSA ENCRYPTION (1977) ---");
        shor_grover_exploration::rsa::demonstrate_rsa();
        shor_grover_exploration::rsa::demonstrate_prime_factorization_difficulty();
    }

    if should_run(&cli.station, "sorting_bounds") {
        println!("\n--- SORTING LOWER BOUNDS ---");
        shor_grover_exploration::sorting_bounds::demonstrate_decision_tree(4);
        shor_grover_exploration::sorting_bounds::demonstrate_information_theoretic_argument();
        shor_grover_exploration::sorting_bounds::demonstrate_linear_time_sorts();
    }

    if should_run(&cli.station, "attention") {
        println!("\n--- ATTENTION IS ALL YOU NEED (2017) ---");
        shor_grover_exploration::attention::show_attention_station();
    }

    if should_run(&cli.station, "euler") {
        println!("\n--- EULER: THE STRUCTURAL UNIFIER (1707-1783) ---");
        shor_grover_exploration::euler::show_euler_station();
    }

    if should_run(&cli.station, "everett") {
        println!("\n--- HUGH EVERETT III: MANY-WORLDS INTERPRETATION (1957) ---");
        shor_grover_exploration::everett::show_everett_station();
        println!();
    }

    if should_run(&cli.station, "godel") {
        println!("\n--- KURT G\u{00d6}DEL: INCOMPLETENESS THEOREMS (1931) ---");
        shor_grover_exploration::godel::show_godel_station();
    }

    if should_run(&cli.station, "bell") {
        println!("\n--- JOHN STEWART BELL'S THEOREM (1964) ---");
        shor_grover_exploration::bell::show_bell_station();
    }

    if should_run(&cli.station, "game_theory") {
        println!("\n--- JOHN NASH: GAME THEORY AND COOPERATION (1950) ---");
        shor_grover_exploration::game_theory::show_game_theory_station();
    }

    if should_run(&cli.station, "deutsch") {
        println!("\n--- DAVID DEUTSCH: THE FABRIC OF REALITY (1997) ---");
        shor_grover_exploration::deutsch::show_deutsch_station();
    }

    if should_run(&cli.station, "collapse") {
        println!("\n--- THE PHILOSOPHY OF PROBLEM-SOLVING: CELEBRATING COLLAPSE ---");
        shor_grover_exploration::collapse::show_collapse_station();
    }

    if should_run(&cli.station, "chaitin") {
        println!("\n--- GREGORY CHAITIN: ALGORITHMIC INFORMATION THEORY & \u{03a9} (1963-2024) ---");
        shor_grover_exploration::chaitin::show_chaitin_station();
    }

    if should_run(&cli.station, "wheeler") {
        println!("\n--- JOHN ARCHIBALD WHEELER: DELAYED CHOICE & IT FROM BIT (1967-2008) ---");
        shor_grover_exploration::wheeler::show_wheeler_station();
    }

    if should_run(&cli.station, "p_vs_np") {
        println!("\n--- P VS NP PROBLEM ---");
        shor_grover_exploration::p_vs_np::show_p_vs_np_station();
    }

    if should_run(&cli.station, "eth") {
        println!("\n--- EXPONENTIAL TIME HYPOTHESIS (ETH) STATION ---");
        shor_grover_exploration::eth::show_eth_station();
    }

    if should_run(&cli.station, "navier_stokes") {
        println!("\n--- NAVIER-STOKES MILLENNIUM PRIZE PROBLEM ---");
        shor_grover_exploration::navier_stokes::show_navier_stokes_station();
    }

    if should_run(&cli.station, "riemann") {
        println!("\n--- RIEMANN HYPOTHESIS MILLENNIUM PRIZE PROBLEM ---");
        shor_grover_exploration::riemann::show_riemann_station();
    }

    if should_run(&cli.station, "yang_mills") {
        println!("\n--- YANG-MILLS EXISTENCE AND MASS GAP PROBLEM ---");
        shor_grover_exploration::yang_mills::show_yang_mills_station();
    }

    if should_run(&cli.station, "von_neumann") {
        println!("\n--- JOHN VON NEUMANN: QUANTUM MEASUREMENT & GAME THEORY (1932, 1944) ---");
        shor_grover_exploration::von_neumann::show_von_neumann_station();
    }

    if should_run(&cli.station, "relativity") {
        println!("\n--- ALBERT EINSTEIN & RELATIVITY (1905/1915) ---");
        shor_grover_exploration::relativity::show_relativity_station();
    }

    if should_run(&cli.station, "rovelli") {
        println!("\n--- CARLO ROVELLI & RELATIONAL QUANTUM MECHANICS (1996) ---");
        shor_grover_exploration::rovelli::show_rovelli_station();
    }

    if should_run(&cli.station, "kripke_platek") {
        println!("\n--- KRIPKE-PLATEK SET THEORY: TYPE-THEORETIC GROUNDING (1960s) ---");
        shor_grover_exploration::kripke_platek::show_kripke_platek_station();
    }

    if should_run(&cli.station, "shannon") {
        println!("\n--- SHANNON: INFORMATION THEORY & HUFFMAN CODING (1948) ---");
        shor_grover_exploration::shannon::show_shannon_station();
    }

    if should_run(&cli.station, "lemper_ziv") {
        println!("\n--- LEMPEL-ZIV: STRUCTURAL COMPRESSION VIA DICTIONARIES (1977, 1978) ---");
        shor_grover_exploration::lemper_ziv::show_lz_station();
        println!();
    }

    if should_run(&cli.station, "positional_tracing") {
        shor_grover_exploration::positional_tracing::show_positional_tracing_station();
    }

    if should_run(&cli.station, "matmul_exponent") {
        println!("\n--- MATRIX MULTIPLICATION EXPONENT STATION ---");
        shor_grover_exploration::matmul_exponent::show_matmul_station();
    }

    // Cryptographic Economics (Nov 2026 AI math cluster)
    if should_run(&cli.station, "crypto_economics") {
        shor_grover_exploration::crypto_economics::show_crypto_economics_station();
    }

    if should_run(&cli.station, "quantum_circuit_complexity") {
        println!("\n--- QUANTUM CIRCUIT COMPLEXITY STATION ---");
        shor_grover_exploration::quantum_circuit_complexity::show_quantum_circuit_complexity_station();
    }

    if run_all {
        println!("\n=== EXPLORATION COMPLETE ===");

        // Station version summary
        println!("\n{}", "=".repeat(47));
        println!("  STATION REGISTRY \u{2014} versions & formalization status");
        println!("{}", "=".repeat(47));
        let stations = [
            (&shor_grover_exploration::grover::STATION),
            (&shor_grover_exploration::shor::STATION),
            (&shor_grover_exploration::lovelace::STATION),
            (&shor_grover_exploration::euler::STATION),
            (&shor_grover_exploration::attention::STATION),
            (&shor_grover_exploration::regex::STATION),
            (&shor_grover_exploration::deutsch::STATION),
            (&shor_grover_exploration::bell::STATION),
            (&shor_grover_exploration::game_theory::STATION),
            (&shor_grover_exploration::everett::STATION),
            (&shor_grover_exploration::collapse::STATION),
            (&shor_grover_exploration::godel::STATION),
            (&shor_grover_exploration::chaitin::STATION),
            (&shor_grover_exploration::wheeler::STATION),
            (&shor_grover_exploration::p_vs_np::STATION),
            (&shor_grover_exploration::eth::STATION),
            (&shor_grover_exploration::von_neumann::STATION),
            (&shor_grover_exploration::kripke_platek::STATION),
            (&shor_grover_exploration::shannon::STATION),
            (&shor_grover_exploration::lemper_ziv::STATION),
            (&shor_grover_exploration::positional_tracing::STATION),
            (&shor_grover_exploration::navier_stokes::STATION),
            (&shor_grover_exploration::riemann::STATION),
            (&shor_grover_exploration::yang_mills::STATION),
            (&shor_grover_exploration::relativity::STATION),
            (&shor_grover_exploration::rovelli::STATION),
            (&shor_grover_exploration::matmul_exponent::STATION),
            (&shor_grover_exploration::crypto_economics::STATION),
            (&shor_grover_exploration::quantum_circuit_complexity::STATION),
        ];
        for meta in stations {
            println!("  {}", meta.display());
        }
        let formalized_count = stations.iter().filter(|s| s.formalized).count();
        println!("{}", "=".repeat(47));
        println!("  {} of {} stations formally verified in Lean", formalized_count, stations.len());
        println!("  Version policy: 0.0.x=simulation \u{00b7} 0.1.0=Lean-backed \u{00b7} 1.0.0=full coverage");
    }
}
