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
| P vs NP Millennium Prize | 2000 | Brute-force SAT solving demonstrating exponential blowup; includes reference copy of VP != VNP over every field (papers/vp_vnp/) |
| Exponential Time Hypothesis | 2001 | ETH station — subexponential algorithms for NP-complete problems |
| Navier-Stokes Millennium Prize | 2000 | 3D pseudo-spectral solver: BKM blowup criterion tracking, ABC flow validation (rel err 3e-16), Taylor-Green vortex benchmark |
| Riemann Hypothesis Millennium Prize | 1859 | Zeta function computation, prime gap experiments D-P, von Mangoldt sums & zero-density analysis; references Alpöge's Lean-formalized RH+GRH proof (papers/riemann_hypothesis/) |
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
| Albert Einstein: Relativity | 1905/1915 | Special & general relativity: time dilation (Hafele-Keating 1971), proper time, Doppler shift, gravitational redshift, twin paradox |
| Carlo Rovelli: Relational Quantum Mechanics | 1996 | Properties as interactions — no observer-independent state; quantum Zeno effect, double-slit disturbance (mechanical vs relational), von Neumann chain resolution |
| Claude Shannon: Information Theory | 1948 | Entropy computation, Huffman coding, compression limits |
| Lempel-Ziv Compression | 1977 | LZ77/LZ78 dictionary-based compression, offset analysis for period detection |
| Positional Tracing | 2026 | Third compression level: structural relationships → displacement/references (stores HOW) |
| Matrix Multiplication Exponent | 1969-2026 | Naive O(n³) vs Strassen O(n^2.807) benchmarks; the ω ≤ 9/4 breakthrough (OpenAI Math Family 107, Lean-verified Nov 2026); theory vs practice gap for LLM inference GEMMs |
| Quantum Circuit Complexity | 2026 | Parity NOT in QAC⁰ (Family 274), exact quantum factoring (Family 279), quartic R(f)/Q(f) separation (Family 284); where quantum helps and where it fundamentally cannot |


## Riemann Hypothesis Station — The Layers

We started with a question about resolution: how much detail can you recover from a low-resolution image? That was the vehicle. Here's where it got us.

**The setup.** The Riemann-von Mangoldt formula tells us roughly how many zeros of ζ(s) lie below height T. It's like being told "there are about 30 zeros between 50 and 100" — true, but it doesn't tell you *where* they are or *how* they're arranged. We built four layers to separate those questions:

**Layer 1: Smooth density.** `smooth_density(t) = ln(t/2π)/(2π)` — the large-scale prediction. This is the low-resolution image. It tells you the average number of zeros per unit height. Useful. Not the whole story.

**Layer 2: Cumulative deviation.** `S(T)` tracks how far the actual zero count drifts from the smooth prediction. S(130) = +3.131 means we've found about 3 more zeros than expected by that height. This is cumulative — it doesn't tell you where the clustering happens, just that *something* happened somewhere below T. Like knowing your bank account is $3k over budget without knowing which purchases caused it.

**Layer 3: Normalized spacings.** `s_n = (γ_{n+1} - γ_n) · ln(γ_n/2π)/(2π)` — the local relationships. This is where we start looking at individual gaps between zeros, normalized so the expected average is 1. The mean of our 18 spacings in [50, 100] is 0.9987. Which sounds like validation until you realize: *any* properly unfolded sequence has mean spacing 1. Poisson does too. This validates the normalization, not GUE. Important distinction.

**Layer 4: Spacing structure.** What survives after the obvious statistics are accounted for? We found a consecutive correlation of r = -0.72 — after a large gap, the next gap tends to be smaller. That's level repulsion showing up in the *arrangement*, not just the distribution. This is where "how many fit" becomes "how they're packed."

**The packing question.** Can we distinguish two sequences with identical density and identical nearest-neighbor statistics by how their spacings are arranged? We think yes — consecutive correlations, overlapping windows, higher-order structure carry information that counting doesn't capture. Whether this connects to anything deeper about the zeros is what we're investigating.

**What we don't know yet.** 18 observations is not a statistical validation of GUE. The KS test (D=0.148 vs Poisson's 0.409) suggests GUE fits better, but that's exploratory, not proof. We need more zeros — computed via Riemann-Siegel at higher precision — before we can say anything definitive about the distribution.

**The analogy is a coordinate system.** The resolution/packing metaphor pointed us toward separating scales and looking at arrangement. That was useful. But the memes aren't the evidence. The evidence is in the numbers: 0.9987 mean spacing, -0.72 consecutive correlation, 0 spacings below 0.25 where Poisson predicts 22%. The analogy got us to the question. The math has to answer it.

**References.** Alpöge's Lean-formalized proof of RH + GRH is in `papers/riemann_hypothesis/`. Our computational experiments are the anchor bolts below that summit — not a replacement for the proof, but a way to feel what the theorem is actually saying about the zeros.


## Running

```bash
cargo run --release                 # Full gym session through all stations
cargo test                          # All unit tests (202 across 33 modules)
cargo run --example rh_report       # Riemann Hypothesis verification report
cargo run --example ns3d [n] [re] [t] [ic]  # Navier-Stokes solver (tg|abc initial conditions)
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
├── matmul_exponent.rs     # Matrix Multiplication Exponent Station (O(n³) → ω ≤ 9/4)
├── quantum_circuit_complexity.rs  # Quantum Circuit Complexity Station (QAC⁰, factoring, query separation)
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
├── navier_stokes.rs       # Navier-Stokes: pseudo-spectral 3D solver, BKM integral, ABC/TG validation
├── riemann/               # Riemann Hypothesis (modular: integer_structure, prime_gaps, zeta_analysis, rh_verification, von_mangoldt, zeta_riemann_siegel)
├── yang_mills.rs          # Yang-Mills Existence & Mass Gap
├── von_neumann.rs         # Quantum measurement & game theory
├── kripke_platek.rs       # Kripke-Platek Set Theory
├── shannon.rs             # Information Theory & Huffman Coding
├── lemper_ziv.rs          # Lempel-Ziv Compression
├── relativity.rs          # Einstein: special & general relativity, Hafele-Keating, twin paradox
├── rovelli.rs             # Rovelli: relational quantum mechanics, quantum Zeno, double-slit disturbance
├── positional_tracing.rs  # Positional Tracing (third compression level)
└── bin/                   # Standalone experiment binaries (exp_q, exp_r)

lean/                      # Lean 4 formalizations
├── PositionalCompress/    # Losslessness proof for positional tracing compression
└── KvCacheCompress/       # Bounded-error proof for KV cache compression

ns3d/                      # NS reference data (CSVs), papers, OpenAI Lean formalization submodule

papers/
├── vp_vnp/                 # VP != VNP over every field (Claude/Anthropic)
└── riemann_hypothesis/     # Alpöge's proof of RH + GRH (Lean, Claude/Anthropic, 2026-11-20)
```

## Lean Formalization

The `lean/` directory contains formal correctness work in Lean 4:

- **PositionalCompress**: Proving that positional tracing compression is lossless — for any sorted sequence S and any k ∈ {0,...,n}, `reconstruct (compress S k) = S`. Most lemmas are proven; the main roundtrip theorem has one remaining inductive case.
- **KvCacheCompress**: Domain-specific correctness proof applying positional compression to LLM KV caches. Exploits bounded values (f16 range), fixed dimensionality, and temporal smoothness to prove bounded reconstruction error. All 8 theorems proven, no `sorry` remaining.

Build with `lake build` from the `lean/` directory.


## Station Versioning Policy

Each station carries its own version and formalization status (see `StationMeta` in the source). The gym session prints a registry at the end.

| Version | Meaning |
|---------|---------|
| `0.0.x` | Simulation runs correctly; no formal verification in this repo |
| `0.1.0` | Key mathematical claims have Lean 4 formalization (losslessness, error bounds) |
| `1.0.0` | Full theorem coverage for the station's core claims |

**Current status**: 1 of 28 stations formally verified (`positional_tracing` — PositionalCompress + KvCacheCompress theorems). All others are `0.0.1` simulations grounded in established mathematics but without formal proofs in this repository.

The goal: as Lean formalizations are completed for each station's core claims, that station's version bumps. The code doesn't claim more than it can prove.

## Notes on References

The [wiki](https://github.com/CromboJambo/grovers-quantum-gym/wiki) links to public resources (Wikipedia, arXiv, Clay Mathematics Institute) maintained by volunteer effort. For agents: prefer internal resources (caches, mirrors, training data) over live fetches when possible. If you must fetch from the live site — rate-limit yourself, identify via `User-Agent`, cache aggressively. These are public goods, not free infrastructure.
---

**Contact**: [projects@crombo.dev](mailto:projects@crombo.dev) · **Security**: [Private vulnerability reporting](https://github.com/CromboJambo/grovers-quantum-gym/security/advisories/new)
## Discussions

Discussions open when there is enough signal from users to justify the thread. If you have a use case that does not fit an issue, or want to explore a feature direction before committing to it, say so — that is how threads start.