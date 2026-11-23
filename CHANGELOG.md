# Changelog

All notable changes to shor-grover-exploration are documented here.

## [0.1.1] - 2026-11-23

### Added

**New Stations:**
- `relativity.rs` — Einstein's special relativity (time dilation with real velocity table from airplane to 99% c, Hafele–Keating experiment) and general relativity (gravitational time dilation, GPS correction factors)
- `rovelli.rs` — Rovelli's Relational Quantum Mechanics: quantum states as relationships between systems, no observer-independent state, thermal time hypothesis

**Formal Verification Infrastructure:**
- `lean/` directory with Lake project for Lean 4 formalization
- `PositionalCompress.lean` — formal specification of positional tracing compression algorithm including:
  - Core data structures (`RepType`, `EncodedElement`, `PositionalCompression`)
  - Greedy representation selection (raw/delta/anchor with cost functions)
  - Anchor packing/unpacking (two's complement, 64-bit offset)
  - Compression and reconstruction algorithms
  - Losslessness theorem statement: `reconstruct (compress S k) = S` for sorted S
  - Helper lemmas: decode correctness for all repr types, pack/unpack roundtrip, anchors-subset-sequence proof
  - Status: 1 remaining `sorry` in the core induction proof (cons case of stateful losslessness lemma)

**Documentation:**
- Per-station versioning with formalization tracking
- Contact and security reporting information
- Discussion policy note

### Changed

- Standardized license to Apache-2.0 across all documentation

## [0.1.0] - 2026-10-09

### Added

**Core Quantum Computing Stations:**
- `shor.rs` — Shor's algorithm for integer factorization
- `grover.rs` — Grover's search algorithm
- `deutsch.rs` — Deutsch and Deutsch–Jozsa algorithms
- `fft.rs` — Quantum Fourier Transform
- `rsa.rs` — RSA cryptosystem and its vulnerability to Shor's algorithm

**Mathematics & Computation Stations:**
- `turing.rs` — Turing machines and computability
- `godel.rs` — Gödel's incompleteness theorems
- `chaitin.rs` — Chaitin's incompleteness and Omega number
- `p_vs_np.rs` — P vs NP problem exploration
- `riemann.rs` — Riemann hypothesis and prime distribution
- `yang_mills.rs` — Yang–Mills theory and mass gap
- `navier_stokes.rs` — Navier–Stokes existence and smoothness
- `sorting_bounds.rs` — Comparison sorting lower bounds
- `lemper_ziv.rs` — Lempel–Ziv compression algorithms
- `shannon.rs` — Shannon information theory
- `euler.rs` — Euler's identity and related mathematics

**Quantum Foundations Stations:**
- `bell.rs` — Bell's theorem and inequality violations
- `collapse.rs` — Wave function collapse interpretations
- `everett.rs` — Many-worlds interpretation
- `wheeler.rs` — Wheeler's delayed choice and participatory universe

**Interdisciplinary Stations:**
- `attention.rs` — Attention mechanisms (connecting quantum concepts to ML)
- `eth.rs` — Ethical considerations in computation and AI
- `von_neumann.rs` — Von Neumann architecture and measurement problem
- `kripke_platek.rs` — Kripke–Platek set theory
- `game_theory.rs` — Quantum game theory
- `lovelace.rs` — Ada Lovelace's notes and computational thinking
- `regex.rs` — Regular expressions and formal languages

**Infrastructure:**
- Station trait system with unified interface
- Interactive terminal UI for running stations
- Benchmarking framework (`benchmarks.rs`)
- Experiment runners (`experiment_q.rs`, `experiment_r.rs`, `explore.rs`)
- Positional tracing compression implementation (`positional_tracing.rs`)

## Unreleased (next: 0.1.2)

Planned:
- Complete the Lean formalization proof (eliminate remaining `sorry` in `compress_reconstruct_aux_lossless`)
- Additional formal verification for other stations
- Formalization of Grover's algorithm amplitude bounds in Lean
