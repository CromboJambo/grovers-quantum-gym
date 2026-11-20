# Shor-Grover Exploration

Classical simulations of quantum algorithms for exploring P vs NP boundaries.
A crombojambo production.

## What's Here

- **Grover's Search**: Classical simulation of amplitude amplification showing O(√N) speedup over linear search
- **Shor's Factoring**: Classical period-finding components demonstrating the factorization pipeline
- **NP-Hard Problems**: Subset sum, SAT, and traveling salesman as test cases

## What a Station Is

Each station runs real computations and prints real numbers. The Riemann station
doesn't tell you about prime gaps — it computes them up to N=10000, compares
prime-bounded intervals against random controls across 15 experiments (D-P),
and shows you the TV distance between distributions. Change the limit and watch
what happens.

The Shannon station doesn't explain entropy — it builds Huffman codes and shows
you where compression actually works and where it doesn't. The Grover station
doesn't claim O(√N) speedup — it runs both linear and amplitude-amplified search
and prints the comparison table.

No slides. No "in this section we will learn." Just code that runs and numbers
that are either interesting or not.

## The Quantum Gym

This project has evolved into a "quantum gym" — a collection of themed stations exploring major milestones in computation:

| Station | Year | What it does |
|---------|------|--------------|
| Ada Lovelace | 1843 | Computes Bernoulli numbers using her original algorithm for Babbage's Analytical Engine |
| Euler and the Basel Problem | 1735 | Monte Carlo estimation of π²/6 = Σ(1/n²) via random sampling |
| Turing Machines & Halting Problem | 1936 | Universal TM simulation, balanced-string recognition, halting problem diagonalization argument |
| Fast Fourier Transform | 1965 | Cooley-Tukey FFT vs direct DFT — O(n log n) vs O(n²), frequency domain analysis |
| Thompson NFA Regex | 1968 | Ken Thompson's regex engine: parse → AST → NFA compilation via Thompson construction |
| Grover's Search | 1996 | Amplitude amplification simulation showing O(√N) speedup |
| Shor's Factoring | 1994 | Period-finding and factorization pipeline |
| RSA Encryption | 1977 | Public-key cryptography, mod_pow, extended Euclidean inverse — threatened by Shor's algorithm |
| Attention Mechanism | 2017 | Scaled dot-product attention from "Attention Is All You Need" |
| P vs NP Millennium Prize | 2000 | Brute-force SAT solving demonstrating exponential blowup |
| Navier-Stokes Millennium Prize | 2000 | Vortex decay simulation exploring smooth solutions vs singularities |
| **Navier-Stokes Blowup Solver** | — | Pseudo-spectral 3D solver to numerically test blowup claims (ns3d/) |
| Riemann Hypothesis Millennium Prize | 1859 | Zeta function computation along the critical line searching for zeros |
| Yang-Mills Existence & Mass Gap Millennium Prize | 1954 | Lattice gauge theory approach with Wilson loops and confinement potential |
| Sorting Lower Bounds | — | Decision tree model proving Ω(n log n) lower bound for comparison-based sorting |
| Hugh Everett III: Many-Worlds Interpretation | 1957 | Branching universe simulation, measure problem, decoherence |
| Kurt Gödel: Incompleteness Theorems | 1931 | Gödel numbering, First and Second Incompleteness Theorems demonstrated |
| John Stewart Bell's Theorem | 1964 | CHSH inequality, local hidden variables vs quantum mechanics simulation |
| John Nash: Game Theory and Cooperation | 1950 | Prisoner's Dilemma, iterated cooperation strategies |
| David Deutsch: The Fabric of Reality | 1997 | Four Strands framework (Many-Worlds, Popperian epistemology, computation, evolution) |
| Philosophy of Problem-Solving | — | Celebrating collapse, learning from wrong hypotheses, epistemic vs psychological closure |
| John von Neumann: Measurement and Strategy | 1932/1944 | Quantum measurement theory, projection postulate, game theory foundations |
| Kripke-Platek Set Theory | 1960s | Type-theoretic grounding of sets, hereditarily finite sets, PEG grammars as KP objects |
| Gregory Chaitin: Algorithmic Information Theory | 1970s | Kolmogorov complexity, halting probability Ω, Gödel's theorems for programs |
| John Archibald Wheeler: It from Bit | 1967-2008 | Delayed-choice experiments, participation theory, reality emerges from information |
| Claude Shannon: Information Theory | 1948 | Entropy computation, Huffman coding, compression limits |
| Lempel-Ziv Compression | 1977 | LZ77/LZ78 dictionary-based compression, offset analysis |
| Exponential Time Hypothesis | 2001 | ETH station — subexponential algorithms for NP-complete problems |

## Running

```bash
cargo run --release        # Full gym session through all stations
cargo test                 # All unit tests (137 across 20+ modules)
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
