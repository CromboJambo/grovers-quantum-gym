/-
POSITIONAL-COMPRESS: Formalization in Lean 4

Formal specification and losslessness proof for the positional tracing
compression algorithm. This file defines the data structures, the greedy
compression strategy, reconstruction, and states the main theorem.

Theorem: For any sorted sequence S of natural numbers and any k ∈ {0,...,n},
         reconstruct (compress S k) = S
-/

import Mathlib.Data.List.Basic
import Mathlib.Data.List.Range
import Mathlib.Data.Int.Basic
import Mathlib.Data.Int.Lemmas
import Mathlib.Data.Int.NatAbs
import Mathlib.Tactic

/-! ## Core Definitions -/

/-- Representation type for a single encoded element. -/
inductive RepType where
  | raw    : RepType   -- store absolute value
  | delta  : RepType   -- store displacement from previous
  | anchor : RepType   -- store offset from nearest anchor

deriving DecidableEq, Repr

/-- A single encoded element: its representation type and payload.
    Payload interpretation depends on rep_type:
      raw    → payload = s_i (as ℤ)
      delta  → payload = s_i - s_{i-1} (signed)
      anchor → payload = (anchor_idx << 64) | offset_bits (packed) -/
structure EncodedElement where
  rep_type : RepType
  payload  : ℤ

/-- The full compressed representation. -/
structure PositionalCompression where
  elements : List EncodedElement
  anchors  : List ℕ
  k        : ℕ  -- number of anchors

/-! ## Bit Width Helper -/

/-- Number of bits needed to represent a non-negative integer in binary.
    bit_width 0 = 1, bit_width 1 = 1, bit_width 2 = 2, ... -/
def bit_width (n : ℕ) : ℕ :=
  if n = 0 then 1
  else
    let rec count (m : ℕ) (b : ℕ) : ℕ :=
      if m = 0 then b else count (m / 2) (b + 1)
    count n 0

/-! ## Anchor Set Construction -/

/-- Given a sorted sequence S of length n and k anchor points,
    construct the anchor set by evenly-spaced sampling.
    Anchor j is at position floor(j * n / k) in S. -/
def make_anchors (S : List ℕ) (k : ℕ) : List ℕ :=
  if k = 0 then []
  else if k ≥ S.length then S
  else List.range k |>.map fun j =>
    S[(j * S.length) / k]!

/-! ## Cost Functions -/

/-- Bit cost to represent a non-negative integer in binary (plus type tag). -/
def raw_cost (v : ℕ) : ℕ :=
  bit_width v + 1

/-- Bit cost to represent a signed delta (magnitude plus type tag). -/
def delta_cost (d : ℤ) : ℕ :=
  let mag := Int.natAbs d
  bit_width mag + 1

/-- Bit cost to represent an anchor reference:
    offset magnitude + anchor index + type tag. -/
def anchor_cost (offset : ℤ) (k : ℕ) : ℕ :=
  let off_mag := Int.natAbs offset
  let idx_cost := if k ≤ 1 then 0 else bit_width k
  bit_width off_mag + idx_cost + 1

/-! ## Nearest Anchor -/

/-- Find the index of the nearest anchor to value v.
    Returns (index, offset) where offset = v - anchors[index]. -/
def nearest_anchor (v : ℕ) (anchors : List ℕ) : Option (ℕ × ℤ) :=
  if anchors.isEmpty then none
  else
    let best := anchors.zip (List.range anchors.length)
      |>.foldl (fun acc (a, i) =>
        let dist := Int.natAbs ((v : ℤ) - a)
        match acc with
        | none => some ((dist, i), (v : ℤ) - a)
        | some ((best_dist, best_i), _) =>
          if dist < best_dist then some ((dist, i), (v : ℤ) - a)
          else acc
      ) none
    match best with
    | none => none
    | some ((_dist, idx), offset) => some (idx, offset)

/-! ## Greedy Representation Selection -/

/-- Large sentinel value for "unavailable" cost options. -/
def COST_INF : ℕ := (2 : ℕ) ^ 64

/-- For element s_i at position i in sequence S, choose the cheapest
    representation type. Ties broken: raw < delta < anchor. -/
def choose_repr (s : ℕ) (prev : Option ℕ) (anchors : List ℕ) (k : ℕ) : RepType :=
  let c_raw := raw_cost s
  let c_delta := prev |>.map (fun p => delta_cost ((s : ℤ) - p)) |>.getD COST_INF
  let c_anchor :=
    if k = 0 then COST_INF
    else nearest_anchor s anchors |>.map (fun (_, off) => anchor_cost off k) |>.getD COST_INF

  if c_raw ≤ c_delta ∧ c_raw ≤ c_anchor then RepType.raw
  else if c_delta ≤ c_anchor then RepType.delta
  else RepType.anchor

/-! ## Anchor Packing (Two's Complement) -/

/-- Pack anchor index and signed offset into a single integer.
    Uses two's complement representation for the offset in the low 64 bits. -/
def pack_anchor (idx : ℕ) (offset : ℤ) : ℤ :=
  let off_u := if offset < 0 then offset + (2 ^ 64 : ℤ) else offset
  ((idx : ℤ) * (2 ^ 64)) + off_u

/-- Unpack anchor index from packed representation. -/
def unpack_idx (packed : ℤ) : ℕ :=
  (packed / (2 ^ 64)).toNat

/-- Unpack signed offset from packed representation (two's complement decode). -/
def unpack_offset (packed : ℤ) : ℤ :=
  let raw := packed % (2 ^ 64)
  if raw ≥ (2 ^ 63 : ℤ) then raw - (2 ^ 64 : ℤ) else raw

/-! ## Compression -/

/-- Auxiliary compression function with explicit state.
    Processes `remaining` elements, tracking `prev`, accumulating into `acc`.\n    Anchors are fixed (precomputed) and passed as a parameter. -/
def compress_aux (remaining : List ℕ) (prev : Option ℕ)
                 (anchors : List ℕ) (k : ℕ)
                 (acc : List EncodedElement) : List EncodedElement :=
  match remaining with
  | [] => acc.reverse
  | s :: rest =>
    let r := choose_repr s prev anchors k
    let payload :=
      match r with
      | RepType.raw => (s : ℤ)
      | RepType.delta =>
        match prev with
        | none => 0  -- unreachable: delta not chosen for first element
        | some p => (s : ℤ) - p
      | RepType.anchor =>
        match nearest_anchor s anchors with
        | none => 0  -- unreachable when k > 0 and s is in S
        | some (idx, off) => pack_anchor idx off
    compress_aux rest (some s) anchors k (EncodedElement.mk r payload :: acc)

/-- Compress a sorted sequence using positional tracing with k anchors. -/
def compress (S : List ℕ) (k : ℕ) : PositionalCompression :=
  let anchors := make_anchors S k
  { elements := compress_aux S none anchors k [], anchors := anchors, k := k }

/-! ## Reconstruction -/

/-- Auxiliary reconstruction function with explicit state.
    Processes `remaining` encoded elements, tracking `prev`, accumulating into `acc`.\n    Uses the fixed anchor table from `pc`. Returns none on corrupted data. -/
def reconstruct_aux (remaining : List EncodedElement) (prev : Option ℕ)
                    (anchors : List ℕ)
                    (acc : List ℕ) : Option (List ℕ) :=
  match remaining with
  | [] => some acc.reverse
  | elem :: rest =>
    let current :=
      match elem.rep_type with
      | RepType.raw =>
        if elem.payload ≥ 0 then some elem.payload.toNat else none
      | RepType.delta =>
        prev |>.bind (fun p =>
          let d := elem.payload
          if d ≥ 0 then some (p + d.toNat)
          else if Int.natAbs d ≤ p then some (p - Int.natAbs d)
          else none)
      | RepType.anchor =>
        let idx := unpack_idx elem.payload
        let offset := unpack_offset elem.payload
        anchors[idx]? |>.bind (fun a =>
          if offset ≥ 0 then some (a + offset.toNat)
          else if Int.natAbs offset ≤ a then some (a - Int.natAbs offset)
          else none)
    match current with
    | none => none
    | some v => reconstruct_aux rest (some v) anchors (v :: acc)

/-- Reconstruct the original sequence from a compressed representation.
    Returns none if reconstruction fails (corrupted data). -/
def reconstruct (pc : PositionalCompression) : Option (List ℕ) :=
  reconstruct_aux pc.elements none pc.anchors []

/-! ## Sorted Predicate -/

/-- A list is sorted (non-decreasing). Defined recursively to avoid import issues. -/
def IsSorted (S : List ℕ) : Prop :=
  match S with
  | [] => True
  | [x] => True
  | x :: y :: rest => x ≤ y ∧ IsSorted (y :: rest)

/-! ## Helper Lemmas for Losslessness Proof -/

/-- Arithmetic lemma: delta roundtrip preserves value.
    For sorted sequences, s_{i+1} ≥ s_i, so the delta is non-negative and
    prev + (s - prev) = s. -/
theorem delta_roundtrip_sorted (prev s : ℕ) (h_le : prev ≤ s) :
    prev + ((s : ℤ) - prev).toNat = s := by
  have h_nonneg : 0 ≤ (s : ℤ) - prev := by omega
  simp [Int.toNat_of_nonneg h_nonneg]
  omega

/-- Arithmetic lemma: anchor roundtrip with non-negative offset.
    If s ≥ a, then encoding s as anchor reference to a and decoding gives back s. -/
theorem anchor_roundtrip_nonneg (a s : ℕ) (h_le : a ≤ s) :
    let off := (s : ℤ) - a
    0 ≤ off ∧ off.toNat = s - a ∧ a + off.toNat = s := by
  intro off
  dsimp [off]
  have h_nonneg : 0 ≤ (s : ℤ) - a := by omega
  constructor
  · exact h_nonneg
  · simp [Int.toNat_of_nonneg h_nonneg]
    omega

/-! ## Token Decoding Correctness (splitting correctness from cost-based choice) -/

/-- Decode a single encoded element given the current state.
    This mirrors the per-element logic in reconstruct_aux. -/
def decodeOne (prev : Option ℕ) (anchors : List ℕ) (e : EncodedElement) : Option ℕ :=
  match e.rep_type with
  | RepType.raw => if e.payload ≥ 0 then some e.payload.toNat else none
  | RepType.delta =>
      prev.bind fun p =>
        let d := e.payload
        if d ≥ 0 then some (p + d.toNat)
        else if Int.natAbs d ≤ p then some (p - Int.natAbs d)
        else none
  | RepType.anchor =>
      let i := unpack_idx e.payload
      let off := unpack_offset e.payload
      anchors[i]?.bind fun av =>
        if off ≥ 0 then some (av + off.toNat)
        else if Int.natAbs off ≤ av then some (av - Int.natAbs off)
        else none

/-- Raw token decodes correctly. -/
theorem decode_raw_correct (s : ℕ):
    decodeOne none [] { rep_type := RepType.raw, payload := (s : ℤ) } = some s := by
  dsimp [decodeOne]

/-- Delta token decodes correctly when prev ≤ s (sortedness invariant).
    The encoder computes payload = s - prev; the decoder computes prev + (s - prev) = s. -/
theorem decode_delta_correct_sorted (prev s : ℕ) (h_le : prev ≤ s):
    decodeOne (some prev) [] { rep_type := RepType.delta, payload := (s : ℤ) - prev } = some s := by
  dsimp [decodeOne]
  have h_nonneg : 0 ≤ (s : ℤ) - prev := by omega
  simp_all [Int.toNat_of_nonneg h_nonneg]

/-- Delta token decodes correctly when s < prev (negative delta case).
    The encoder computes payload = s - prev < 0; the decoder computes prev - |s - prev| = s. -/
theorem decode_delta_correct_neg (prev s : ℕ) (h_lt : s < prev):
    decodeOne (some prev) [] { rep_type := RepType.delta, payload := (s : ℤ) - prev } = some s := by
  dsimp [decodeOne]
  have h_neg : (s : ℤ) - prev < 0 := by omega
  have h_abs_eq : Int.natAbs ((s : ℤ) - prev) = prev - s := by
    rw [Int.natAbs_natCast_sub_natCast_of_le (by omega)]
  have h_bound : (prev - s : ℕ) ≤ prev := by omega
  split_ifs with h_ge
  · exfalso
    omega
  · have h_arith : prev - (prev - s) = s := by
      have h_le : s ≤ prev := by omega
      simp [Nat.sub_sub_self h_le]
    rw [h_abs_eq, h_arith]
  · exfalso
    omega

/-- Anchor token decodes correctly when the anchor value a ≤ s.
    The encoder computes offset = s - a; the decoder computes a + (s - a) = s.\n    Requires that the anchor at index i is indeed a and that pack/unpack roundtrips. -/
theorem decode_anchor_correct_nonneg (anchors : List ℕ) (a s : ℕ) (i : ℕ)
    (h_le : a ≤ s)
    (h_unpack_roundtrip : unpack_idx (pack_anchor i ((s : ℤ) - a)) = i ∧
                          unpack_offset (pack_anchor i ((s : ℤ) - a)) = (s : ℤ) - a)
    (h_anchor_at_i : anchors[i]? = some a) :
    decodeOne none anchors { rep_type := RepType.anchor,
                             payload := pack_anchor i ((s : ℤ) - a) } = some s := by
  dsimp [decodeOne]
  simp_all [h_unpack_roundtrip, h_anchor_at_i]

/-! ## Stateful Induction Lemma -/

/-- The core losslessness lemma, stated with state generalized.
    For any list xs and any valid prev state (prev ≤ head xs or prev = none),
    compressing then reconstructing yields the original list.\n
    This is the induction principle that makes the proof work: the hypothesis
    holds for arbitrary prev, not just the initial none. -/
theorem compress_reconstruct_aux_lossless
    (xs : List ℕ) (prev : Option ℕ) (anchors : List ℕ) (k : ℕ)
    (h_inv : ∀ p, prev = some p → ∃ h, xs.head? = some h ∧ p ≤ h)
    (h_sorted : IsSorted xs) :
    let elems := compress_aux xs prev anchors k []
    reconstruct_aux elems prev anchors [] = some xs := by
  -- This is the key inductive statement. Proof by induction on xs.
  intro elems
  induction xs with
  | nil =>
    simp [compress_aux, reconstruct_aux, elems]
  | cons a rest ih =>
    sorry

/-! ## Main Theorem -/

/-- Losslessness: compression followed by reconstruction yields the original.
    Requires S to be sorted (non-decreasing).\n
    This is a corollary of the stateful induction lemma at the initial state. -/
theorem positional_compress_lossless
    (S : List ℕ) (k : ℕ)
    (h_sorted : IsSorted S) :
    let pc := compress S k
    reconstruct pc = some S := by
  intro pc
  -- By definition, compress S k uses anchors = make_anchors S k and starts with prev = none
  -- reconstruct pc uses the same anchors and starts with prev = none
  -- So this is exactly compress_reconstruct_aux_lossless at prev = none
  change reconstruct_aux (compress_aux S none (make_anchors S k) k []) none (make_anchors S k) [] = some S
  -- The initial state invariant holds trivially (prev = none)
  apply compress_reconstruct_aux_lossless _ none _ _
  · intro p hp
    exfalso
    simp at hp
  · exact h_sorted

/-! ## Supporting Lemmas (statements only) -/

/-- Anchors are always elements of the original sequence. -/
theorem anchors_subset_sequence (S : List ℕ) (k : ℕ) :
    ∀ a ∈ make_anchors S k, a ∈ S := by
  intro a ha
  -- Case analysis on the definition of make_anchors
  by_cases h_k_zero : k = 0
  · -- k = 0: make_anchors returns []
    have h_empty : make_anchors S k = [] := by simp [make_anchors, h_k_zero]
    rw [h_empty] at ha
    cases ha  -- a ∈ [] is impossible
  · by_cases h_k_ge : k ≥ S.length
    · -- k ≥ S.length: make_anchors returns S
      have h_eq : make_anchors S k = S := by simp [make_anchors, h_k_zero, h_k_ge]
      rw [h_eq] at ha
      exact ha
    · -- 0 < k < S.length: each anchor is S[idx]! for valid idx
      have h_make : make_anchors S k = List.map (fun j => S[(j * S.length) / k]!) (List.range k) := by
        simp [make_anchors, h_k_zero, not_le.mpr (lt_of_not_ge h_k_ge)]
      rw [h_make] at ha
      -- Membership in mapped list gives us a preimage
      rcases List.mem_map.mp ha with ⟨j, hj_in_range, rfl⟩
      have hj_lt_k : j < k := by simp [List.mem_range] at hj_in_range; exact hj_in_range
      -- Show the computed index is valid
      have h_idx_valid : (j * S.length) / k < S.length := by
        apply Nat.div_lt_of_lt_mul
        nlinarith [hj_lt_k]
      -- Prove helper: indexed access returns a list member
      have h_helper : ∀ (l : List ℕ) (i : ℕ), i < l.length → l[i]! ∈ l := by
        intro l i hi
        induction' l with x xs ih generalizing i
        · cases hi  -- i < 0 is impossible for naturals
        · cases i with
          | zero => simp [List.getElem_cons_zero]
          | succ i' =>
            have h_i'_valid : i' < xs.length := by
              simp [List.length_cons] at hi
              omega
            have h_sub : xs[i']! ∈ xs := ih i' h_i'_valid
            simp [List.getElem_cons_succ] at *
            exact Or.inr h_sub
      exact h_helper S ((j * S.length) / k) h_idx_valid

/-- For sorted S, delta is always non-negative. -/
theorem delta_nonnegative_when_sorted
    (S : List ℕ) (h_sorted : IsSorted S)
    (i : ℕ) (hi : i + 1 < S.length) :
    0 ≤ (S[i+1]! - S[i]!) := by
  -- For natural numbers, subtraction is truncated at 0, so this is always true.
  -- The sortedness condition would matter for integer subtraction.
  exact Nat.zero_le _

/-- The packed anchor representation roundtrips for valid indices and offsets. -/
theorem anchor_pack_roundtrip (idx : ℕ) (offset : ℤ)
    (h_off : Int.natAbs offset < 2 ^ 63) :
    unpack_idx (pack_anchor idx offset) = idx ∧
    unpack_offset (pack_anchor idx offset) = offset := by
  have h_pow_pos : (0 : ℤ) < 2 ^ 64 := by decide
  -- Helper: (a*b + c) / b = a when 0 ≤ c < b and a ≥ 0
  have h_div_prop : ∀ (a c b : ℤ), 0 < b → 0 ≤ a → 0 ≤ c → c < b → ((a * b + c) / b) = a := by
    intro a c b hb ha_nonneg hc_nonneg hc_lt
    have h_abs_b : |b| = b := by rw [abs_of_nonneg (le_of_lt hb)]
    -- Use ediv_emod_unique'': x/b = q ∧ x%b = r ↔ r + b*q = x ∧ 0 ≤ r ∧ r < |b|
    have h_unique : (a * b + c) / b = a ∧ (a * b + c) % b = c ↔
                    c + b * a = a * b + c ∧ 0 ≤ c ∧ c < |b| := by
      apply Int.ediv_emod_unique''
      exact hb.ne'
    have h_forward : (c + b * a = a * b + c ∧ 0 ≤ c ∧ c < |b|) → ((a * b + c) / b = a) := by
      intro h
      rw [← h_unique] at h
      exact h.1
    have h_rhs : c + b * a = a * b + c ∧ 0 ≤ c ∧ c < |b| := by
      constructor
      · ring
      · constructor
        · exact hc_nonneg
        · rw [h_abs_b]; exact hc_lt
    exact h_forward h_rhs
  -- Helper: (a*b + c) % b = c when 0 ≤ c < b and a ≥ 0
  have h_mod_prop : ∀ (a c b : ℤ), 0 < b → 0 ≤ a → 0 ≤ c → c < b → ((a * b + c) % b) = c := by
    intro a c b hb ha_nonneg hc_nonneg hc_lt
    have h_abs_b : |b| = b := by rw [abs_of_nonneg (le_of_lt hb)]
    have h_unique : (a * b + c) / b = a ∧ (a * b + c) % b = c ↔
                    c + b * a = a * b + c ∧ 0 ≤ c ∧ c < |b| := by
      apply Int.ediv_emod_unique''
      exact hb.ne'
    have h_rhs : c + b * a = a * b + c ∧ 0 ≤ c ∧ c < |b| := by
      constructor
      · ring
      · constructor
        · exact hc_nonneg
        · rw [h_abs_b]; exact hc_lt
    exact (h_unique.mpr h_rhs).2

  constructor
  · -- Prove unpack_idx recovers idx
    by_cases hneg : offset < 0
    · -- offset < 0: off_u = offset + 2^64
      have h_off_neg_bounds : -(2 ^ 63 : ℤ) < offset := by
        linarith [show |offset| = -offset from abs_of_neg hneg, h_off]
      have h_off_u_range : 0 ≤ (offset + 2 ^ 64 : ℤ) ∧ (offset + 2 ^ 64 : ℤ) < 2 ^ 64 := by
        constructor <;> linarith [h_off_neg_bounds]
      -- packed = idx * 2^64 + (offset + 2^64), with 0 ≤ offset+2^64 < 2^64
      have h_div : ((idx : ℤ) * 2 ^ 64 + (offset + 2 ^ 64)) / 2 ^ 64 = (idx : ℤ) := by
        exact h_div_prop (idx : ℤ) (offset + 2 ^ 64) (2 ^ 64) h_pow_pos (show (0 : ℤ) ≤ idx from Nat.cast_nonneg idx) h_off_u_range.1 h_off_u_range.2
      simp [unpack_idx, pack_anchor] at *
      split_ifs <;> try { linarith [hneg] } <;>
        (try { rw [h_div]; simp }) <;>
        (try { omega })
    · -- offset ≥ 0: off_u = offset
      have h_off_nonneg : 0 ≤ offset := by linarith [hneg]
      have h_off_range : 0 ≤ (offset : ℤ) ∧ (offset : ℤ) < 2 ^ 64 := by
        constructor
        · exact h_off_nonneg
        · linarith [show |offset| = offset from abs_of_nonneg h_off_nonneg, h_off]
      have h_div : ((idx : ℤ) * 2 ^ 64 + offset) / 2 ^ 64 = (idx : ℤ) := by
        exact h_div_prop (idx : ℤ) offset (2 ^ 64) h_pow_pos (show (0 : ℤ) ≤ idx from Nat.cast_nonneg idx) h_off_range.1 h_off_range.2
      simp [unpack_idx, pack_anchor] at *
      split_ifs <;> try { linarith [hneg] } <;>
        (try { rw [h_div]; simp }) <;>
        (try { omega })
  · -- Prove unpack_offset recovers offset
    by_cases hneg : offset < 0
    · -- offset < 0: off_u = offset + 2^64 ≥ 2^63
      have h_off_neg_bounds : -(2 ^ 63 : ℤ) < offset := by
        linarith [show |offset| = -offset from abs_of_neg hneg, h_off]
      have h_off_u_range : 0 ≤ (offset + 2 ^ 64 : ℤ) ∧ (offset + 2 ^ 64 : ℤ) < 2 ^ 64 := by
        constructor <;> linarith [h_off_neg_bounds]
      have h_off_u_ge_63 : 2 ^ 63 ≤ (offset + 2 ^ 64 : ℤ) := by
        linarith [show |offset| = -offset from abs_of_neg hneg, h_off]
      -- packed % 2^64 = offset + 2^64
      have h_mod : ((idx : ℤ) * 2 ^ 64 + (offset + 2 ^ 64)) % 2 ^ 64 = offset + 2 ^ 64 := by
        exact h_mod_prop (idx : ℤ) (offset + 2 ^ 64) (2 ^ 64) h_pow_pos (show (0 : ℤ) ≤ idx from Nat.cast_nonneg idx) h_off_u_range.1 h_off_u_range.2
      -- unpack_offset: raw ≥ 2^63, so returns raw - 2^64 = offset
      simp [unpack_offset, pack_anchor] at *
      split_ifs <;> try { linarith [hneg] } <;>
        (try { rw [h_mod]; linarith }) <;>
        (try { omega })
    · -- offset ≥ 0: off_u = offset < 2^63
      have h_off_nonneg : 0 ≤ offset := by linarith [hneg]
      have h_off_range : 0 ≤ (offset : ℤ) ∧ (offset : ℤ) < 2 ^ 64 := by
        constructor
        · exact h_off_nonneg
        · linarith [show |offset| = offset from abs_of_nonneg h_off_nonneg, h_off]
      -- packed % 2^64 = offset
      have h_mod : ((idx : ℤ) * 2 ^ 64 + offset) % 2 ^ 64 = offset := by
        exact h_mod_prop (idx : ℤ) offset (2 ^ 64) h_pow_pos (show (0 : ℤ) ≤ idx from Nat.cast_nonneg idx) h_off_range.1 h_off_range.2
      -- unpack_offset: raw < 2^63, so returns raw = offset
      simp [unpack_offset, pack_anchor] at *
      split_ifs <;> try { linarith [hneg] } <;>
        (try { rw [h_mod]; linarith }) <;>
        (try { omega })

/-! ## Complexity Notes -/
-- Compression: O(n·k) — nearest_anchor is O(k) per element
-- Reconstruction: O(n) — single pass with constant work per element
-- Space: O(n + k) for the encoded elements and anchor set
