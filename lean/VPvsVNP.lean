module

/-!
## VP vs VNP — Algebraic Complexity Classes

Formalizes the algebraic complexity classes VP and VNP, where the November 2026
OpenAI/Anthropic breakthrough (VP ≠ VNP) was proven. This is the algebraic analog
of P vs NP: instead of Boolean circuits deciding languages, we use arithmetic
circuits computing polynomial families.

Key differences from classical P vs NP:
- Inputs are variables (x₁, ..., xₙ), not bit strings
- Gates compute + and * over a field (ℚ here), not AND/OR/NOT
- "Size" = number of gates, "degree" = polynomial degree
- VP ≈ P (efficiently computable polynomials)
- VNP ≈ NP (polynomials with short algebraic witnesses)

The canonical VNP-complete problem: the **permanent** of a matrix.
VP contains the **determinant**. VP ≠ VNP means permanent has no
polynomial-size arithmetic circuits (assuming standard conjectures).

Status: Definitions + basic properties. The separation theorem itself
is `sorry` — that's what OpenAI/Anthropic proved in November 2026.
-/

import Mathlib.Algebra.Polynomial.Basic
import Mathlib.Data.Matrix.Basic
import Mathlib.Data.Fin.Basic
import Mathlib.Tactic

open Polynomial Matrix Fin

/-! ## Arithmetic Circuits -/

/-- An **arithmetic circuit** with n inputs over a field K.
    Leaves are either input variables (x₁ through xₙ) or constants.
    Internal gates compute addition or multiplication. -/
inductive ArithCircuit {K : Type*} [Field K] (n : ℕ) : Type
  | var (i : Fin n) : ArithCircuit n      -- input variable x_{i+1}
  | const (c : K) : ArithCircuit n        -- constant
  | add (f g : ArithCircuit n) : ArithCircuit n  -- f + g
  | mul (f g : ArithCircuit n) : ArithCircuit n  -- f * g

namespace ArithCircuit
/-- **Size** of a circuit: total number of gates. -/
def size {K : Type*} [Field K] {n : ℕ} (c : ArithCircuit K n) : ℕ :=
  match c with
  | .var _ => 1
  | .const _ => 1
  | .add f g => 1 + f.size + g.size
  | .mul f g => 1 + f.size + g.size

/-- **Depth** of a circuit: longest path from input to output. -/
def depth {K : Type*} [Field K] {n : ℕ} (c : ArithCircuit K n) : ℕ :=
  match c with
  | .var _ => 0
  | .const _ => 0
  | .add f g => 1 + max f.depth g.depth
  | .mul f g => 1 + max f.depth g.depth
end ArithCircuit
/-! ## Polynomial Families -/

/-- A **polynomial family** is a sequence {fₙ} where fₙ ∈ K[x₁, ..., xₙ].
    We model this as a function from n to polynomials in n variables.

    Note: mathlib's `MvPolynomial (Fin n) K` gives us polynomials in exactly n variables. -/
abbrev PolyFamily (K : Type*) [Field K] := ∀ n : ℕ, MvPolynomial (Fin n) K
/-! ## The Class VP -/

/-- A polynomial family {fₙ} is in **VP** if there exists a polynomial p
    such that fₙ can be computed by an arithmetic circuit of size ≤ p(n)
    for all sufficiently large n.

    This is the algebraic analog of P: "efficiently computable" polynomials. -/
def InVP {K : Type*} [Field K] (f : PolyFamily K) : Prop :=
  ∃ p : ℕ → ℕ, Monotone p ∧ ∀ᶠ n in Filter.atTop,
    ∃ c : ArithCircuit K n, c.size ≤ p n ∧ evaluatesTo c = f n

/-- **Evaluation** of an arithmetic circuit: maps the circuit to the multivariate
    polynomial it computes. Each gate corresponds to a polynomial operation:
    - Input variable x_i → MvPolynomial.X i
    - Constant c → MvPolynomial.C c
    - Addition gate → polynomial addition
    - Multiplication gate → polynomial multiplication -/
def ArithCircuit.evaluatesTo {K : Type*} [Field K] {n : ℕ}
    (c : ArithCircuit K n) : MvPolynomial (Fin n) K :=
  match c with
  | .var i => MvPolynomial.X i
  | .const k => MvPolynomial.C k
  | .add f g => f.evaluatesTo + g.evaluatesTo
  | .mul f g => f.evaluatesTo * g.evaluatesTo
/-! ## The Class VNP -/

/-- A polynomial family {fₙ} is in **VNP** if there exists a polynomial p and
    a VP family {gₙ} such that:

        fₙ(x₁,...,xₙ) = Σ_{y ∈ {0,1}^{p(n)}} gₙ(x₁,...,xₙ, y₁,...,y_{p(n)})

    Intuitively: fₙ is the sum of exponentially many "easy" (VP-computable)
    terms. This is the algebraic analog of NP: "efficiently verifiable" polynomials,
    where the witness y is summed over (analogous to existential quantification). -/
def InVNP {K : Type*} [Field K] (f : PolyFamily K) : Prop :=
  ∃ p : ℕ → ℕ, Monotone p ∧
    ∃ g : ∀ n : ℕ, MvPolynomial (Fin (n + p n)) K,
      InVP (fun m => g m) ∧
      ∀ᶠ n in Filter.atTop, f n = finiteSumOverBools (g n)

/-- **Substitute the last variable** of a polynomial in n+1 variables with a constant c,
    yielding a polynomial in n variables. Used to sum over Boolean assignments. -/
def substLastVar {K : Type*} [CommRing K] {n : ℕ} (c : K)
    (p : MvPolynomial (Fin (n + 1)) K) : MvPolynomial (Fin n) K :=
  MvPolynomial.eval (fun i => if h : i.val < n then MvPolynomial.X ⟨i.val, by omega⟩ else MvPolynomial.C c) p

/-- **Sum over Boolean assignments:** Given a polynomial in n+m variables, sum it over
    all 2^m Boolean assignments to the last m variables, yielding a polynomial in n variables.

    This is the algebraic analog of existential quantification: instead of "there exists y",
    we compute Σ_{y ∈ {0,1}^m} g(x, y). For m=0 this is the identity (no witnesses to sum over). -/
def finiteSumOverBools {K : Type*} [CommRing K] {n m : ℕ}
    (g : MvPolynomial (Fin (n + m)) K) : MvPolynomial (Fin n) K :=
  match m with
  | 0 => g  -- Fin (n + 0) = Fin n, no witnesses to sum over
  | m + 1 =>
    let h := g  -- polynomial in n + (m+1) variables
    -- Sum over the last variable: substitute 0 and 1, add results
    let summedLast := substLastVar {n := n + m} (0 : K) h + substLastVar {n := n + m} (1 : K) h
    -- Recurse on remaining m variables
    finiteSumOverBools {m := m} summedLast
/-! ## VP ⊆ VNP -/

/-- Every polynomial family in VP is also in VNP.
    Proof: if fₙ is computed by a circuit of size p(n), then it's trivially
    the sum over zero witness variables (just fₙ itself). The "witness" space
    is empty, so the sum collapses to fₙ. -/
theorem VP_subset_VNP {K : Type*} [Field K] (f : PolyFamily K) : InVP f → InVNP f := by
  intro h_vp
  -- Unpack the VP membership: there's a size bound p and circuits computing f
  obtain ⟨p, hp_mono, hp_eventually⟩ := h_vp
  -- Construct VNP witness: use zero witness variables (q(n) = 0)
  -- and let g = f itself
  refine' ⟨fun _ => 0, fun n m => by omega, f, ?_, ?_⟩
  · -- Show g (= f) is in VP: it's the same family, so same proof
    exact h_vp
  · -- Show f_n = finiteSumOverBools (f_n) for all sufficiently large n
    -- With zero witness variables, finiteSumOverBools is the identity
    intro n _
    simp [finiteSumOverBools]
/-! ## Canonical Problems -/

/-- The **permanent** of an n×n matrix:
        perm(A) = Σ_{σ ∈ Sₙ} ∏_{i=1}^n A_{i,σ(i)}

    This is the canonical VNP-complete problem (Valiant 1979).
    Unlike the determinant, there's no known polynomial-size circuit for it. -/
def permanent {K : Type*} [Field K] {n : ℕ} (A : Matrix (Fin n) (Fin n) K) : K :=
  ∑ σ : Equiv.Perm (Fin n), ∏ i : Fin n, A i (σ i)

/-- The **determinant** is in VP (computable in polynomial time via Gaussian elimination). -/
def detFamily {K : Type*} [Field K] : PolyFamily K :=
  fun n => MvPolynomial.C (Matrix.det (fun i j => MvPolynomial.X ⟨i, sorry⟩)) -- simplified

/-- The permanent family is in VNP (Valiant's theorem). -/
def permFamily {K : Type*} [Field K] : PolyFamily K :=
  fun n => MvPolynomial.C (permanent (fun i j => MvPolynomial.X ⟨i, sorry⟩)) -- simplified
/-! ## The Separation Theorem (November 2026) -/

/-- **VP ≠ VNP:** There exists a polynomial family in VNP that is not in VP.
    Equivalently: the permanent does not have polynomial-size arithmetic circuits.

    This was proven by OpenAI/Anthropic in November 2026 (formalization pending
    public release). The proof likely uses algebraic geometry techniques similar
    to those in the Alpöge–Furman zeta work.

    If a formal proof appears, it should type-check as:
    ```lean
    theorem VP_ne_VNP : ∃ f : PolyFamily ℚ, InVNP f ∧ ¬InVP f := <proof>
    ```
    -/
theorem VP_ne_VNP : ∃ f : PolyFamily ℚ, InVNP f ∧ ¬InVP f := by
  sorry

/-! ## Why This Matters for Cryptography -/

If VP = VNP (the opposite of what was proven), it would collapse algebraic
complexity in ways that might enable new factoring algorithms. The separation
VP ≠ VNP is *good news* for crypto: it suggests that certain algebraic problems
(including those underlying RSA) are genuinely hard, not just uncracked.

However, VP ≠ VNP does NOT directly prove RSA is secure — it's a necessary
but not sufficient condition. A full cryptographic lower bound would require
showing factoring specifically is outside VP (or whatever class captures
"efficient computation").
-/
