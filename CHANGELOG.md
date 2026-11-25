# Changelog

All notable changes to shor-grover-exploration are documented here.

## [Unreleased]

### Added

**Riemann Hypothesis Station — Shape Tests 8-11:**
- SHAPE TEST 8 (`shape_test_depth_constraint`) — Rigorous depth-dependent W/H constraint analysis. Model B (W/H depends on N_smooth) wins with slope -0.70, 67% variance explained by depth. Phase transition at peak 10-12 where variance collapses from ~2.3 to 0.94.
- SHAPE TEST 9 (`shape_test_edge_partition_and_mapping`) — Edge partition and distance mapping. Reveals layered dimension discovery: correlation connects at peak 20, edge structure at T=33, W/H stabilizes at peak 47. Strong level repulsion near peaks (all spacings < 0.5).
- SHAPE TEST 10 (`shape_test_perpendicular_grade`) — Perpendicular-to-grade protocol with fixed 100ft drill (half_window=5). Walks entire sequence in connected order, no near-peak cherry-picking. Finds perpendicular residual is almost entirely curvature measurement (corr=-0.83), orthogonal to slope (corr=-0.08).
- SHAPE TEST 11 (`shape_test_gradient_zero_finding`) — Gradient-based zero finding with Newton-like extrapolation and curvature validation. Found all 10 known zeros from diverse starting points using only local gradient + perpendicular drill. Distance-to-peak estimation functional (mean error 0.57 units).
- `two_sample_ks(a, b)` helper — Kolmogorov-Smirnov two-sample test for distribution comparison.

**Documentation:**
- `docs/riemann/shape-tests.md` — Complete documentation of all 11 shape tests, the 100-foot drill constraint protocol, drilling directions framework, and key results summary.

**Reference material:**
- `papers/vp_vnp/` — reference copy of *VP != VNP over every field* (Claude / Anthropic, 2026): paper PDF + extracted text + attribution/verification note. Proofs written by Claude (model), provider Anthropic; source `anthropics/formal-math` (PR #51).
- `src/p_vs_np.rs` station output now cites the paper with model/provider credit.
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
