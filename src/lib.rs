//! Classical simulations of quantum algorithms for exploring P vs NP boundaries.
//!
//! This crate implements:
//! - Grover's search algorithm (classical simulation of amplitude amplification)
//! - Shor's factoring algorithm (classical period-finding components)
//! - Applications to NP-hard problems: subset sum, SAT, traveling salesman

pub mod attention;
pub mod benchmarks;
pub mod bell;
pub mod chaitin;
pub mod game_theory;
pub mod collapse;
pub mod deutsch;
pub mod everett;
pub mod euler;
pub mod godel;
pub mod fft;
pub mod grover;
pub mod lovelace;
pub mod navier_stokes;
pub mod p_vs_np;
pub mod problems;
pub mod regex;
pub mod riemann;
pub mod rsa;
pub mod shor;
pub mod sorting_bounds;
pub mod turing;
pub mod wheeler;
pub mod yang_mills;
pub mod von_neumann;

// Re-export main types for convenience
pub use grover::{GroverOracle, grover_search, linear_search};
pub use shor::{classical_period_finding, shor_factor};
