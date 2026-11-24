# VP ≠ VNP over every field — reference copy

**Paper:** *VP ̸= VNP OVER EVERY FIELD* (70 pp)
**Authors:** CLAUDE / ANTHROPIC
**Model:** Claude (Anthropic LLM family) — found and wrote the proofs
**Provider:** Anthropic
**Date:** 2026-11-23, merged into `anthropics/formal-math` as PR #51
**Source:** <https://github.com/anthropics/formal-math/blob/main/vp-vnp/paper.pdf>
**Lean formalization:** <https://github.com/anthropics/formal-math/tree/main/vp-vnp>

Files here: `paper.pdf` (original), `paper.txt` (`pdftotext -layout` extraction).

## What it claims

A uniform constant c > 0 and, for each characteristic p (zero or prime), a threshold m₀ such that over **every** field of characteristic p, any arithmetic circuit computing

- the permanent PERMₘ (when p ≠ 2), and
- the unsigned Hamiltonian-cycle polynomial HCₘ (all fields),

needs more than m^(c·(log₂ m)^(1/3)) = 2^(c·(log m)^(4/3)) gates.

Consequences: VP ⊊ VNP over every field; and the same bound for approximating circuits gives VNP ⊄ VP̄ (border complexity).

## Verification status (audited 2026-11-24)

- Lean 4.33.1 + pinned Mathlib; ~72,900 lines across `Paper/`, `Nc/`, `Ov/`, `NS1/`.
- Local `lake build`: **Build completed successfully (9032 jobs)**; only linter warnings; the only `sorry`s are the five declarations in `Challenge.lean`.
- `#print axioms` on the five headline theorems: exactly `[propext, Classical.choice, Quot.sound]`.
- No `axiom`/`opaque`/`unsafe`/`native_decide`/`@[extern]` anywhere; nothing imports `Challenge` except `scripts/PrintAxioms.lean`.
- Upstream CI `verify (vp-vnp)` = success (Palomar comparator + nanoda kernel + landrun sandbox).
- Statements match the standard Valiant definitions and the paper's §3 printout; proof spine audited (dichotomy Flat ∨ Contracting, transfer to algebraically closed fields, Ryser witness for HC ∈ VNP).

## Caveats

- No human mathematician has checked the development; the paper states this explicitly (§3, Appendix Q).
- The bound is quasi-polynomial, not exponential; it does **not** settle P vs NP (algebraic circuits work over all field elements, not just {0,1}).
- Remark N.6 (determinantal-complexity consequence) is not formalized; some appendix steps exist only in Lean.
- This station is about Boolean complexity; the paper is algebraic complexity. They share lower-bound machinery, not conclusions.
