/-
POSITIONAL-COMPRESS: Formalization in Lean 4

Formal specification and losslessness proof for the positional tracing
compression algorithm. This file defines the data structures, the greedy
compression strategy, reconstruction, and states the main theorem.

Theorem: For any sorted sequence S of natural numbers and any k ∈ {0,...,n},
         reconstruct (compress S k) = S
-/

import Mathlib.Data.List.Basic
import Mathlib.Data.Nat.Basic
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

/-! ## Anchor Set Construction -/

/-- Given a sorted sequence S of length n and k anchor points,
    construct the anchor set by evenly-spaced sampling.
    Anchor j is at position floor(j * n / k) in S. -/
def make_anchors (S : List ℕ) (k : ℕ) : List ℕ :=
  if h : k = 0 then []
  else if k ≥ S.length then S
  else Finset.range k |>.toList.map fun j =>
    S.get! ((j * S.length) / k)

/-! ## Cost Functions -/

/-- Bit cost to represent a non-negative integer in binary (plus type tag).
    Uses bitWidth as a discrete proxy for ceil(log2(x+1)). -/
def raw_cost (v : ℕ) : ℕ :=
  if v = 0 then 1 else v.bitWidth + 1

/-- Bit cost to represent a signed delta (magnitude plus type tag). -/
def delta_cost (d : ℤ) : ℕ :=
  let mag := d.natAbs
  if mag = 0 then 1 else mag.bitWidth + 1

/-- Bit cost to represent an anchor reference:
    offset magnitude + anchor index + type tag. -/
def anchor_cost (offset : ℤ) (k : ℕ) : ℕ :=
  let off_mag := offset.natAbs
  let idx_cost := if k ≤ 1 then 0 else k.bitWidth
  if off_mag = 0 then 1 + idx_cost + 1
  else off_mag.bitWidth + idx_cost + 1

/-! ## Nearest Anchor -/

/-- Find the index of the nearest anchor to value v.
    Returns (index, offset) where offset = v - anchors[index]. -/
def nearest_anchor (v : ℕ) (anchors : List ℕ) : Option (ℕ × ℤ) :=
  if h : anchors.isEmpty then none
  else
    let best := anchors.zip (List.range anchors.length)
      |>.foldl (fun acc (a, i) =>
        let dist := (v - a).natAbs
        match acc with
        | none => some ((dist, i), v - a)
        | some ((best_dist, best_i), _) =>
          if dist < best_dist then some ((dist, i), v - a)
          else acc
      ) none
    match best with
    | none => none
    | some ((_dist, idx), offset) => some (idx, offset)

/-! ## Greedy Representation Selection -/

/-- For element s_i at position i in sequence S, choose the cheapest
    representation type. Ties broken: raw < delta < anchor. -/
def choose_repr (s : ℕ) (prev : Option ℕ) (anchors : List ℕ) (k : ℕ) : RepType :=
  let c_raw := raw_cost s
  let c_delta := prev |>.map (fun p => delta_cost ((s : ℤ) - p)) |>.getD Nat.maxValue
  let c_anchor :=
    if k = 0 then Nat.maxValue
    else nearest_anchor s anchors |>.map (fun (_, off) => anchor_cost off k) |>.getD Nat.maxValue

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
          | some (idx, off) => ((idx : ℤ) << 64) + (off & 0xFFFFFFFFFFFFFFFF)
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
          if elem.payload ≥ 0 then some (elem.payload.toNat) else none
        | RepType.delta =>
          prev |>.andThen (fun p =>
            let d := elem.payload
            if d ≥ 0 then some (p + d.toNat)
            else if d.natAbs ≤ p then some (p - d.natAbs)
            else none)
        | RepType.anchor =>
          let idx := ((elem.payload >> 64).toNat : ℕ)
          let off_raw := elem.payload & 0xFFFFFFFFFFFFFFFF
          let offset := if (off_raw >>> 63) = 1 then -(off_raw - (1 << 63)) else off_raw
          pc.anchors.get? idx |>.andThen (fun a =>
            if offset ≥ 0 then some (a + offset.toNat)
            else if offset.natAbs ≤ a then some (a - offset.natAbs)
            else none)
      match current with
      | none => none
      | some v => go rest (some v) (v :: acc)
  go pc.elements none []

/-! ## Main Theorem -/

/-- Losslessness: compression followed by reconstruction yields the original.
    Requires S to be sorted (non-decreasing). -/
theorem positional_compress_lossless
    (S : List ℕ) (k : ℕ)
    (h_sorted : S.Sorted (· ≤ ·)) :
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
    (S : List ℕ) (h_sorted : S.Sorted (· ≤ ·))
    (i : ℕ) (hi : i + 1 < S.length) :
    0 ≤ (S.get! (i + 1) - S.get! i) := by
  sorry

/-- The packed anchor representation roundtrips for valid indices and offsets. -/
theorem anchor_pack_roundtrip (idx : ℕ) (offset : ℤ)
    (h_idx : idx < 65536)  -- practical bound on k
    (h_off : offset.natAbs < 2^63) :
    let packed := ((idx : ℤ) << 64) + (offset & 0xFFFFFFFFFFFFFFFF)
    ((packed >> 64).toNat = idx) ∧
    (let off_raw := packed & 0xFFFFFFFFFFFFFFFF
     let decoded := if (off_raw >>> 63) = 1 then -(off_raw - (1 << 63)) else off_raw
     decoded = offset) := by
  sorry

/-! ## Complexity Notes -/
-- Compression: O(n·k) — nearest_anchor is O(k) per element
-- Reconstruction: O(n) — single pass with constant work per element
-- Space: O(n + k) for the encoded elements and anchor set
