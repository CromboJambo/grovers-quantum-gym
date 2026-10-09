//! Classical simulations of quantum algorithms for exploring P vs NP boundaries.
//!
//! This crate implements:
//! - Grover's search algorithm (classical simulation of amplitude amplification)
//! - Shor's factoring algorithm (classical period-finding components)
//! - Applications to NP-hard problems: subset sum, SAT, traveling salesman

pub mod grover;
pub mod shor;
pub mod problems;
pub mod benchmarks;
pub mod lovelace;
pub mod regex;
pub mod attention;
pub mod p_vs_np;
pub mod navier_stokes;
pub mod riemann;
pub mod yang_mills;

// Re-export main types for convenience
pub use grover::{grover_search, GroverOracle, linear_search};
pub use shor::{shor_factor, classical_period_finding};