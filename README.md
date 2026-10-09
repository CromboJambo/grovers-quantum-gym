# Shor-Grover Exploration

Classical simulations of quantum algorithms for exploring P vs NP boundaries.
A crombojambo production.

## What's Here

- **Grover's Search**: Classical simulation of amplitude amplification showing O(√N) speedup over linear search
- **Shor's Factoring**: Classical period-finding components demonstrating the factorization pipeline
- **NP-Hard Problems**: Subset sum, SAT, and traveling salesman as test cases

## The Quantum Gym

This project has evolved into a "quantum gym" — a collection of themed stations exploring major milestones in computation:

| Station | Year | What it does |
|---------|------|--------------|
| Ada Lovelace | 1843 | Computes Bernoulli numbers using her original algorithm for Babbage's Analytical Engine |
| Thompson NFA Regex | 1968 | Ken Thompson's regex engine: parse → AST → NFA compilation via Thompson construction |
| Grover's Search | 1996 | Amplitude amplification simulation showing O(√N) speedup |
| Shor's Factoring | 1994 | Period-finding and factorization pipeline |
| Attention Mechanism | 2017 | Scaled dot-product attention from "Attention Is All You Need" |
| P vs NP Millennium Prize | 2000 | Brute-force SAT solving demonstrating exponential blowup |
| Navier-Stokes Millennium Prize | 2000 | Vortex decay simulation exploring smooth solutions vs singularities |
| Riemann Hypothesis Millennium Prize | 1859 | Zeta function computation along the critical line searching for zeros |
| Yang-Mills Existence & Mass Gap Millennium Prize | 1954 | Lattice gauge theory approach with Wilson loops and confinement potential |

## Running

```bash
cargo run --release        # Full gym session through all stations
cargo test                 # All unit tests (37 across 11 modules)
```

## The Big Idea

If you can verify a solution cheaply (like spell-check knowing "mammal" not "mamal"),
can you also find it? Grover's algorithm says: sort of — but only √N faster, not N.
The boundary between verification and discovery remains one of the deepest open
questions in computer science.

## Architecture

```
src/
├── grover.rs       # Amplitude amplification simulation
├── shor.rs         # Period-finding + factorization
├── problems.rs     # NP-hard problem instances
├── benchmarks.rs   # Classical vs Grover performance comparison
├── lovelace.rs     # Ada Lovelace Station (Bernoulli numbers)
├── regex.rs        # Thompson NFA Regex Engine
├── attention.rs    # Attention Is All You Need
├── p_vs_np.rs      # P vs NP Millennium Prize Problem
├── navier_stokes.rs # Navier-Stokes Millennium Prize Problem
├── riemann.rs      # Riemann Hypothesis Millennium Prize Problem
└── yang_mills.rs   # Yang-Mills Existence & Mass Gap Problem
```
