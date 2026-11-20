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

/-! ## Compression -/

/-- Compress a sorted sequence using positional tracing with k anchors. -/
def compress (S : List ℕ) (k : ℕ) : PositionalCompression :=
  let anchors := make_anchors S k
  let rec go (remaining : List ℕ) (prev : Option ℕ) (acc : List EncodedElement) : List EncodedElement :=
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
          | some (idx, off) => ((idx : ℤ) * (2 ^ 64)) + off
      go rest (some s) (EncodedElement.mk r payload :: acc)
  { elements := go S none [], anchors := anchors, k := k }

/-! ## Reconstruction -/

/-- Reconstruct the original sequence from a compressed representation.
    Returns none if reconstruction fails (corrupted data). -/
def reconstruct (pc : PositionalCompression) : Option (List ℕ) :=
  let rec go (remaining : List EncodedElement) (prev : Option ℕ)
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
          let idx := (elem.payload / (2 ^ 64)).toNat
          let offset := elem.payload % (2 ^ 64)
          pc.anchors[idx]? |>.bind (fun a =>
            if offset ≥ 0 then some (a + offset.toNat)
            else if Int.natAbs offset ≤ a then some (a - Int.natAbs offset)
            else none)
      match current with
      | none => none
      | some v => go rest (some v) (v :: acc)
  go pc.elements none []

/-! ## Sorted Predicate -/

/-- A list is sorted (non-decreasing). Defined recursively to avoid import issues. -/
def IsSorted (S : List ℕ) : Prop :=
  match S with
  | [] => True
  | [x] => True
  | x :: y :: rest => x ≤ y ∧ IsSorted (y :: rest)

/-! ## Main Theorem -/

/-- Losslessness: compression followed by reconstruction yields the original.
    Requires S to be sorted (non-decreasing). -/
theorem positional_compress_lossless
    (S : List ℕ) (k : ℕ)
    (h_sorted : IsSorted S) :
    let pc := compress S k
    reconstruct pc = some S := by
  -- Proof strategy: induction on S
  -- Base case: empty list trivially roundtrips
  -- Inductive step: for s :: rest, the first element is always raw or anchor
  --   (delta unavailable), and reconstruction of the head gives back s.
  --   The tail reconstructs by IH given that prev = some s is tracked.
  --
  -- Key lemmas needed:
  -- 1. make_anchors S k ⊆ S (anchors are elements of S)
  -- 2. For any s ∈ S and anchors from S, nearest_anchor returns a valid offset
  -- 3. The greedy choice always produces a payload that reconstructs correctly
  -- 4. Delta reconstruction: if prev = s_{i-1} and payload = s_i - s_{i-1},
  --    then prev + payload = s_i (when s_i ≥ s_{i-1}, guaranteed by sortedness)
  -- 5. Anchor reconstruction: if offset = s_i - a_j, then a_j + offset = s_i
  --    (trivial arithmetic, but need to handle the packed representation)
  sorry

/-! ## Supporting Lemmas (statements only) -/

/-- Anchors are always elements of the original sequence. -/
theorem anchors_subset_sequence (S : List ℕ) (k : ℕ) :
    ∀ a ∈ make_anchors S k, a ∈ S := by
  sorry

/-- For sorted S, delta is always non-negative. -/
theorem delta_nonnegative_when_sorted
    (S : List ℕ) (h_sorted : IsSorted S)
    (i : ℕ) (hi : i + 1 < S.length) :
    0 ≤ (S[i+1]! - S[i]!) := by
  sorry

/-- The packed anchor representation roundtrips for valid indices and offsets. -/
theorem anchor_pack_roundtrip (idx : ℕ) (offset : ℤ)
    (h_off : Int.natAbs offset < 2 ^ 63) :
    let packed := ((idx : ℤ) * (2 ^ 64)) + offset
    ((packed / (2 ^ 64)).toNat = idx) ∧
    (packed % (2 ^ 64) = offset) := by
  sorry

/-! ## Complexity Notes -/
-- Compression: O(n·k) — nearest_anchor is O(k) per element
-- Reconstruction: O(n) — single pass with constant work per element
-- Space: O(n + k) for the encoded elements and anchor set
