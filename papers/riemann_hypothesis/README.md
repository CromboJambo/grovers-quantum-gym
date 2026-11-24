# Riemann Hypothesis — Alpöge's Proof (Lean formalization)

**Paper:** *The Riemann Hypothesis is the classic problem in number theory...* ("remark", 4 pp)
**Author:** Levent Alpöge (Harvard Society of Fellows / Anthropic)
**AI Collaborator:** Claude (Anthropic LLM family, unreleased model) — developed proof strategy and formalization
**Provider:** Anthropic
**Date:** Announced 2026-11-20
**Source:** <https://alpo.ge/remark.pdf>
**Lean source (original):** https://alpo.ge/lean.tar.gz.part01 ... part14
**Community Lean repo:** <https://github.com/leanprover-community/riemann-hypothesis>
**Wikipedia:** <https://en.wikipedia.org/wiki/Alp%C3%B6ge%27s_proof_of_the_Riemann_hypothesis>

Files here: `paper.pdf` (original remark), `paper.txt` (`pdftotext -layout` extraction).

## What it claims

A proof of the Riemann Hypothesis and the Generalized Riemann Hypothesis for Dirichlet L-functions, formalized in Lean 4.33.1 without assumptions beyond standard Mathlib axioms. The method uses iterated theta lifts between totally definite unitary groups to achieve multiplicity control over the integers — a Fourier-analytic descendant of Stepanov's argument for RH over function fields.

Key technique: For the von Mangoldt sum of length X, use √X-length chains of theta lifts, each mimicking multiplicity (which is hard to find over ℤ). The formalization was developed with an unreleased Anthropic model and verified by Lean's kernel plus independent comparators (nanoda, con-leche, leanchecker).

## Verification status (community audit)

The [leanprover-community/riemann-hypothesis](https://github.com/leanprover-community/riemann-hypothesis) repo (created 2026-11-20, 234 commits) maintains the formalization with:

- `src/Main.lean` proves:
  - `riemannHypothesis` — Mathlib's RH statement
  - `LFunction_ne_zero_of_half_lt_re` — L(χ, s) ≠ 0 for Re s > 1/2 (all moduli/characters except trivial at s=1)
  - `LFunction_eq_zero_re` — every zero of L(χ, s) with 0 < Re s < 1 has Re s = 1/2
- Comparator checks that proofs use no axioms other than `propext`, `Quot.sound`, `Classical.choice`
- Both Lean's kernel and nanoda accept the proof
- Published source written for Lean 4.33.1 (no `lake comparator` or `leanexport`)

## Caveats

- No independent human mathematician has fully understood the proof yet (per Wikipedia, 2026-11-20)
- The original Lean source is ~17.4M lines across 14 tar.gz parts
- Proof strategy involves advanced automorphic forms machinery (GGP periods, Bessel periods, Rallis seesaw, Iwasawa towers via Behrend) that the author acknowledges needs simplification
- GRH for Dirichlet L-functions proven; Extended RH expected but not yet formalized

## References

- Alpöge's announcement: <https://alpo.ge/>
- Lean community repo: <https://github.com/leanprover-community/riemann-hypothesis>
- Wikipedia (Alpöge): <https://en.wikipedia.org/wiki/Levent_Alp%C3%B6ge>
- Wikipedia (RH proof): <https://en.wikipedia.org/wiki/Alp%C3%B6ge%27s_proof_of_the_Riemann_hypothesis>
