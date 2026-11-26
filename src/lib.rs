//! Grover's Quantum Gym — classical simulations of quantum algorithms and mathematical structures.
//!
//! This crate implements:
//! - Grover's search algorithm (classical simulation of amplitude amplification)
//! - Shor's factoring algorithm (classical period-finding components)
//! - Applications to NP-hard problems: subset sum, SAT, traveling salesman


/// Per-station version and formalization metadata.
///
/// Versioning policy:
/// - 0.0.x — simulation runs, no formal verification in this repo
/// - 0.1.0 — key claims have Lean formalization (e.g., losslessness, error bounds)
/// - 1.0.0 — full theorem coverage for the station's core mathematical claims
pub struct StationMeta {
    pub name: &'static str,
    pub version: &'static str,
    pub formalized: bool,
    pub notes: &'static str,
}

impl StationMeta {
    /// Format for display in the gym session.
    pub fn display(&self) -> String {
        let status = if self.formalized { "✓ formalized" } else { "simulation" };
        format!("{} v{} [{}]", self.name, self.version, status)
    }
}

pub mod attention;
pub mod bell;
pub mod benchmarks;
pub mod chaitin;
pub mod collapse;
pub mod deutsch;
pub mod euler;
pub mod everett;
pub mod experiment_q;
pub use experiment_q::run_experiment_q;
pub mod fft;
pub mod game_theory;
pub mod godel;
pub mod kripke_platek;
pub mod lemper_ziv;
pub mod matmul_exponent;
pub mod shannon;
pub mod grover;
pub mod lovelace;
pub mod navier_stokes;
pub mod positional_tracing;
pub mod p_vs_np;
pub mod relativity;
pub mod eth;
pub mod problems;
pub mod regex;
pub mod rovelli;
pub mod riemann;
pub mod rsa;
pub mod shor;
pub mod sorting_bounds;
pub mod turing;
pub mod von_neumann;
pub mod wheeler;
pub mod yang_mills;

// Re-export main types for convenience
pub use grover::{GroverOracle, grover_search, linear_search};
pub use shor::{classical_period_finding, shor_factor};
pub mod experiment_r;
pub use experiment_r::run_experiment_r;
