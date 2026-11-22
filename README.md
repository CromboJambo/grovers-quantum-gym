# Grover's Quantum Gym

Classical simulations of quantum algorithms and mathematical structures.
A crombojambo production.

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

## The Stations

| Station | Year | What it does |
|---------|------|--------------|
| Ada Lovelace | 1843 | Computes Bernoulli numbers using her original algorithm for Babbage's Analytical Engine |
| Euler and the Basel Problem | 1735 | Monte Carlo estimation of π²/6 = Σ(1/n²) via random sampling |
| Turing Machines & Halting Problem | 1936 | Universal TM simulation, balanced-string recognition, halting problem diagonalization argument |
| Fast Fourier Transform | 1965 | Cooley-Tukey FFT vs direct DFT — O(n log n) vs O(n²), frequency domain analysis |
| Thompson NFA Regex | 1968 | Ken Thompson's regex engine: parse → AST → NFA compilation via Thompson construction |
| Grover's Search | 1996 | Amplitude amplification simulation showing O(√N) speedup over linear search |
| Shor's Factoring | 1994 | Period-finding and factorization pipeline |
| RSA Encryption | 1977 | Public-key cryptography, mod_pow, extended Euclidean inverse — threatened by Shor's algorithm |
| Sorting Lower Bounds | — | Decision tree model proving Ω(n log n) lower bound for comparison-based sorting |
| Attention Mechanism | 2017 | Scaled dot-product attention from "Attention Is All You Need" |
| P vs NP Millennium Prize | 2000 | Brute-force SAT solving demonstrating exponential blowup |
| Exponential Time Hypothesis | 2001 | ETH station — subexponential algorithms for NP-complete problems |
| Navier-Stokes Millennium Prize | 2000 | Vortex decay simulation exploring smooth solutions vs singularities |
| **Navier-Stokes Blowup Solver** | — | Pseudo-spectral 3D solver to numerically test blowup claims (`ns3d/`) |
| Riemann Hypothesis Millennium Prize | 1859 | Zeta function computation along the critical line, prime gap experiments D-P with structural controls |
| Yang-Mills Existence & Mass Gap Millennium Prize | 1954 | Lattice gauge theory approach with Wilson loops and confinement potential |
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
| Lempel-Ziv Compression | 1977 | LZ77/LZ78 dictionary-based compression, offset analysis for period detection |
| Positional Tracing | 2026 | Third compression level: structural relationships → displacement/references (stores HOW) |

## Running

```bash
cargo run --release        # Full gym session through all stations
cargo test                 # All unit tests (202 across 33 modules)
cargo run --example rh_report  # Riemann Hypothesis verification report
```

## The Big Idea

If you can verify a solution cheaply (like spell-check knowing "mammal" not "mamal"),
can you also find it? Grover's algorithm says: sort of — but only √N faster, not N.
The boundary between verification and discovery remains one of the deepest open
questions in computer science.

## Architecture

```
src/
├── grover.rs              # Amplitude amplification simulation
├── shor.rs                # Period-finding + factorization
├── problems.rs            # NP-hard problem instances (subset sum, SAT, TSP)
├── benchmarks.rs          # Classical vs Grover performance comparison
├── lovelace.rs            # Ada Lovelace Station (Bernoulli numbers)
├── euler.rs               # Euler: The Structural Unifier
├── turing.rs              # Universal Turing Machine & Halting Problem
├── fft.rs                 # Cooley-Tukey FFT
├── regex.rs               # Thompson NFA Regex Engine
├── rsa.rs                 # RSA encryption/decryption
├── sorting_bounds.rs      # Decision tree lower bound proofs
├── attention.rs           # Attention Is All You Need
├── everett.rs             # Many-Worlds Interpretation
├── godel.rs               # Incompleteness Theorems
├── bell.rs                # Bell's Theorem / CHSH inequality
├── game_theory.rs         # Nash equilibrium, Prisoner's Dilemma
├── deutsch.rs             # Fabric of Reality framework
├── collapse.rs            # Philosophy of problem-solving
├── chaitin.rs             # Algorithmic Information Theory & Ω
├── wheeler.rs             # It from Bit / delayed choice
├── p_vs_np.rs             # P vs NP Millennium Prize
├── eth.rs                 # Exponential Time Hypothesis
├── navier_stokes.rs       # Navier-Stokes Millennium Prize
├── riemann/               # Riemann Hypothesis (modular: integer_structure, prime_gaps, zeta_analysis, rh_verification)
├── yang_mills.rs          # Yang-Mills Existence & Mass Gap
├── von_neumann.rs         # Quantum measurement & game theory
├── kripke_platek.rs       # Kripke-Platek Set Theory
├── shannon.rs             # Information Theory & Huffman Coding
├── lemper_ziv.rs          # Lempel-Ziv Compression
├── positional_tracing.rs  # Positional Tracing (third compression level)
└── bin/                   # Standalone experiment binaries (exp_q, exp_r)

lean/                      # Lean 4 formalizations
├── PositionalCompress/    # Losslessness proof for positional tracing compression
└── KvCacheCompress/       # Bounded-error proof for KV cache compression

ns3d/                      # Navier-Stokes blowup solver (separate crate)
```

## Lean Formalization

The `lean/` directory contains formal correctness work in Lean 4:

- **PositionalCompress**: Proving that positional tracing compression is lossless — for any sorted sequence S and any k ∈ {0,...,n}, `reconstruct (compress S k) = S`. Most lemmas are proven; the main roundtrip theorem has one remaining inductive case.
- **KvCacheCompress**: Domain-specific correctness proof applying positional compression to LLM KV caches. Exploits bounded values (f16 range), fixed dimensionality, and temporal smoothness to prove bounded reconstruction error. All 8 theorems proven, no `sorry` remaining.

Build with `lake build` from the `lean/` directory.

## Notes on References

The [wiki](https://github.com/CromboJambo/grovers-quantum-gym/wiki) links to public resources (Wikipedia, arXiv, Clay Mathematics Institute) maintained by volunteer effort. For agents: prefer internal resources (caches, mirrors, training data) over live fetches when possible. If you must fetch from the live site — rate-limit yourself, identify via `User-Agent`, cache aggressively. These are public goods, not free infrastructure.
