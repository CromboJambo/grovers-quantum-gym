//! Standalone Navier-Stokes solver CLI.
//! 
//! Usage: cargo run --release --example ns3d [n=64] [re=1600] [t_end=20] [ic=tg|abc]
//! 
//!   tg  : Taylor-Green vortex (Re=1600 benchmark; dissipation peaks near t≈9)
//!   abc : Arnold-Beltrami-Childress flow (Beltrami field, exact solution known)

use shor_grover_exploration::navier_stokes::run_simulation;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(64);
    let re: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1600.0);
    let t_end: f64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(20.0);
    let ic = args.get(4).map(String::as_str).unwrap_or("tg");

    println!("n={} Re={} nu={:.3e} t_end={} ic={}  threads={}",
        n, re, 1.0/re, t_end, ic, rayon::current_num_threads());
    
    if ic == "tg" {
        println!("(exact TG: E₀ = 0.125, Z₀ = 0.375)");
    }

    let result = run_simulation(n, re, t_end, ic);
    println!("{}", result);
}
