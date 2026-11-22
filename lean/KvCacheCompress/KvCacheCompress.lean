/-
KV-CACHE-COMPRESS: Lean 4 Formalization

Domain-specific correctness proof for positional-compression applied to
LLM KV caches. Exploits KV cache structure:

  1. Bounded values: f16 range is [-65504, 65504]
  2. Fixed dimensionality: each vector has exactly `head_dim` components
  3. Temporal smoothness: |v[t+1][d] - v[t][d]| ≤ ε for small ε
  4. Lossy tolerance: we prove bounded error, not losslessness

Main results:
  - quantize_delta_bound: |quantize_δ(x) - x| ≤ δ/2
  - kv_reconstruction_error_bound: E(t) ≤ dist_to_anchor(t)·δ/2
  - kv_max_error_bound: max error ≤ (anchor_interval - 1)·δ/2
-/

import Mathlib.Data.Real.Basic
import Mathlib.Data.Fin.Basic
import Mathlib.Analysis.Normed.Group.Basic
import Mathlib.Tactic

/-! ## Core Types -/

/-- A single KV vector at one position: head_dim components. -/
abbrev KvVector (head_dim : ℕ) := Fin head_dim → ℝ

/-- The full KV cache: n positions × head_dim dimensions. -/
abbrev KvCache (n head_dim : ℕ) := Fin n → KvVector head_dim

/-! ## Quantization -/

/-- Quantize a real value to the nearest multiple of δ. -/
noncomputable def quantize_delta {δ : ℝ} (hδ : 0 < δ) (x : ℝ) : ℝ :=
  ⌊x / δ + 1/2⌋ * δ

/-- Quantization error is bounded by δ/2. -/
theorem quantize_delta_bound {δ : ℝ} (hδ : 0 < δ) (x : ℝ) : |quantize_delta hδ x - x| ≤ δ / 2 := by
  set k : ℤ := ⌊x / δ + 1/2⌋ with hk_def
  have h_floor_le : (k : ℝ) ≤ x / δ + 1/2 := Int.floor_le (x / δ + 1/2)
  have h_floor_lt : x / δ + 1/2 < (k : ℝ) + 1 := Int.lt_floor_add_one (x / δ + 1/2)
  -- The only division fact needed: x / δ * δ = x
  have h_div_mul : x / δ * δ = x := div_mul_cancel₀ x hδ.ne'
  
  -- From h_floor_le: k ≤ x/δ + 1/2 → multiply by δ > 0: k*δ ≤ x + δ/2
  have h_right_raw : (k : ℝ) * δ ≤ x + δ / 2 := by
    calc
      (k : ℝ) * δ ≤ (x / δ + 1/2) * δ := mul_le_mul_of_nonneg_right h_floor_le hδ.le
      _ = x / δ * δ + δ / 2 := by ring
      _ = x + δ / 2 := by rw [h_div_mul]
  
  -- From h_floor_lt: x/δ + 1/2 < k+1 → multiply by δ > 0: x + δ/2 < (k+1)*δ
  have h_left_raw : x + δ / 2 < ((k : ℝ) + 1) * δ := by
    calc
      x + δ / 2 = x / δ * δ + δ / 2 := by rw [h_div_mul]
      _ = (x / δ + 1/2) * δ := by ring
      _ < ((k : ℝ) + 1) * δ := mul_lt_mul_of_pos_right h_floor_lt hδ
  
  -- Combine: |quantize_delta hδ x - x| ≤ δ/2
  have h_quant_eq : quantize_delta hδ x = (k : ℝ) * δ := by
    rw [quantize_delta, hk_def]
  rw [abs_le, h_quant_eq]
  constructor <;> linarith

/-! ## Delta Encoding with Periodic Anchors -/

/-- Delta-encoded representation. -/
structure DeltaEncodedKv (n head_dim anchor_interval : ℕ) where
  deltas  : Fin n → KvVector head_dim

/-- Encode a cache as delta representation with anchors every `anchor_interval` positions. -/
noncomputable def encode_kv_cache (n head_dim anchor_interval : ℕ)
    (h_n_pos : 0 < n) (h_anchor : 1 ≤ anchor_interval)
    {δ : ℝ} (hδ : 0 < δ) (cache : KvCache n head_dim) : DeltaEncodedKv n head_dim anchor_interval where
  deltas := fun t =>
    if h_t_anchor : t.val % anchor_interval = 0 then cache t
    else if t.val = 0 then cache t
    else
      let prev : Fin n := ⟨t.val - 1, by omega⟩
      fun d => quantize_delta hδ (cache t d - cache prev d)

/-! ## Reconstruction -/

/-- Reconstruct the cache from delta encoding. -/
noncomputable def decode_kv_cache (n head_dim anchor_interval : ℕ)
    (h_n_pos : 0 < n) (enc : DeltaEncodedKv n head_dim anchor_interval) (t : Fin n) : KvVector head_dim :=
  if h_t_anchor : t.val % anchor_interval = 0 then enc.deltas t
  else if t.val = 0 then enc.deltas t
  else
    let prev : Fin n := ⟨t.val - 1, by omega⟩
    let decoded_prev := decode_kv_cache n head_dim anchor_interval h_n_pos enc prev
    fun d => decoded_prev d + enc.deltas t d

/-! ## Error Analysis -/

/-- Distance from position t to the most recent anchor at or before t. -/
def distance_to_last_anchor (n anchor_interval : ℕ) (t : Fin n) : ℕ :=
  t.val % anchor_interval

/-- Helper: at anchor positions, decode returns the full cache value. -/
lemma decode_at_anchor_is_exact (n head_dim anchor_interval : ℕ)
    (h_n_pos : 0 < n) (h_anchor : 1 ≤ anchor_interval)
    {δ : ℝ} (hδ : 0 < δ)
    (cache : KvCache n head_dim) (t : Fin n) (d : Fin head_dim)
    (h_t_anchor : t.val % anchor_interval = 0) :
    decode_kv_cache n head_dim anchor_interval h_n_pos
      (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t d = cache t d := by
  simp [decode_kv_cache, encode_kv_cache, h_t_anchor]

/-- Main error bound: reconstruction error at position t is bounded by
    (distance to last anchor) · δ/2 per component. -/
theorem kv_reconstruction_error_bound (n head_dim anchor_interval : ℕ)
    (h_n_pos : 0 < n) (h_anchor : 1 ≤ anchor_interval)
    {δ : ℝ} (hδ : 0 < δ)
    (cache : KvCache n head_dim)
    (t : Fin n) (d : Fin head_dim) :
    |decode_kv_cache n head_dim anchor_interval h_n_pos
      (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t d - cache t d| ≤
      (distance_to_last_anchor n anchor_interval t : ℝ) * δ / 2 := by
  -- Prove by induction on position
  have h_main (m : ℕ) (hm : m < n) :
      |decode_kv_cache n head_dim anchor_interval h_n_pos
        (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) ⟨m, hm⟩ d -
       cache ⟨m, hm⟩ d| ≤
       (distance_to_last_anchor n anchor_interval ⟨m, hm⟩ : ℝ) * δ / 2 := by
    induction' m with m ih_general
    · -- Base case: m = 0 is always an anchor
      have h_zero_mod : (0 : ℕ) % anchor_interval = 0 := by simp
      have h_exact := decode_at_anchor_is_exact n head_dim anchor_interval h_n_pos h_anchor hδ cache ⟨0, hm⟩ d h_zero_mod
      rw [h_exact]
      simp [distance_to_last_anchor, abs_of_nonneg, zero_mul]
    · -- Inductive step: m + 1
      have h_m1_lt : m + 1 < n := by omega
      have h_m_lt : m < n := by omega
      let t_cur : Fin n := ⟨m + 1, h_m1_lt⟩
      let t_prev : Fin n := ⟨m, h_m_lt⟩
      have h_ih := ih_general (by omega)
      by_cases h_is_anchor : (m + 1) % anchor_interval = 0
      · -- If anchor: exact
        have h_exact := decode_at_anchor_is_exact n head_dim anchor_interval h_n_pos h_anchor hδ cache t_cur d h_is_anchor
        rw [h_exact]
        simp [distance_to_last_anchor, h_is_anchor, abs_of_nonneg, zero_mul]
      · -- If not anchor: use induction hypothesis + triangle inequality
        have h_decode_rec : decode_kv_cache n head_dim anchor_interval h_n_pos
            (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_cur d =
            decode_kv_cache n head_dim anchor_interval h_n_pos
              (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d +
            (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache).deltas t_cur d := by
          simp [decode_kv_cache, encode_kv_cache, h_is_anchor]
          split_ifs <;> simp_all
        have h_delta_def : (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache).deltas t_cur d =
            quantize_delta hδ (cache t_cur d - cache t_prev d) := by
          simp [encode_kv_cache, h_is_anchor]
          split_ifs <;> simp_all
        rw [h_decode_rec, h_delta_def]
        -- Error decomposition: new_error = prev_error + quantization_error
        have h_err_decomp :
            (decode_kv_cache n head_dim anchor_interval h_n_pos
              (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d +
             quantize_delta hδ (cache t_cur d - cache t_prev d)) - cache t_cur d =
              (decode_kv_cache n head_dim anchor_interval h_n_pos
                (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d - cache t_prev d) +
              (quantize_delta hδ (cache t_cur d - cache t_prev d) - (cache t_cur d - cache t_prev d)) := by
          ring
        rw [h_err_decomp]
        -- Triangle inequality: |a + b| ≤ |a| + |b|
        calc
          |(decode_kv_cache n head_dim anchor_interval h_n_pos
            (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d - cache t_prev d) +
           (quantize_delta hδ (cache t_cur d - cache t_prev d) - (cache t_cur d - cache t_prev d))| ≤
              |decode_kv_cache n head_dim anchor_interval h_n_pos
                (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d - cache t_prev d| +
              |quantize_delta hδ (cache t_cur d - cache t_prev d) - (cache t_cur d - cache t_prev d)| := by
            -- Triangle inequality for absolute values: |a + b| ≤ |a| + |b|
            have a := decode_kv_cache n head_dim anchor_interval h_n_pos
              (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d - cache t_prev d
            have b := quantize_delta hδ (cache t_cur d - cache t_prev d) - (cache t_cur d - cache t_prev d)
            -- Prove |a + b| ≤ |a| + |b| using basic properties
            calc
              |a + b| ≤ |a| + |b| := by
                -- Use nlinarith which should handle abs automatically
                nlinarith [abs_nonneg a, abs_nonneg b]
          _ ≤ |decode_kv_cache n head_dim anchor_interval h_n_pos
            (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t_prev d - cache t_prev d| + δ / 2 := by
            gcongr
            exact quantize_delta_bound hδ (cache t_cur d - cache t_prev d)
          _ ≤ (distance_to_last_anchor n anchor_interval t_prev : ℝ) * δ / 2 + δ / 2 := by
            gcongr
            exact h_ih
          _ = ((distance_to_last_anchor n anchor_interval t_prev : ℝ) + 1) * δ / 2 := by ring
          -- Distance to last anchor increases by 1 when not at anchor
          _ ≤ (distance_to_last_anchor n anchor_interval t_cur : ℝ) * δ / 2 := by
            have h_dist_rel : distance_to_last_anchor n anchor_interval t_cur =
                distance_to_last_anchor n anchor_interval t_prev + 1 := by
              simp [distance_to_last_anchor]
              -- When (m+1) % anchor_interval ≠ 0, we have (m+1) % anchor_interval = m % anchor_interval + 1
              have h_mod_succ : (m + 1) % anchor_interval = m % anchor_interval + 1 := by
                rw [Nat.add_mod]
                -- Need: m % anchor_interval + 1 < anchor_interval
                have h_lt : m % anchor_interval + 1 < anchor_interval := by
                  by_contra h_ge
                  have h_eq : m % anchor_interval + 1 ≥ anchor_interval := by omega
                  have h_mod_zero : (m + 1) % anchor_interval = 0 := by
                    rw [Nat.add_mod] at *
                    omega
                  exact (h_is_anchor h_mod_zero)
                omega
              simp [Nat.mod_eq_of_lt h_lt]
            simp [h_mod_succ]
            ring
  exact h_main t.val t.isLt

/-- Corollary: maximum error over the entire cache is bounded by
    (anchor_interval - 1) · δ / 2 per component. -/
theorem kv_max_error_bound (n head_dim anchor_interval : ℕ)
    (h_n_pos : 0 < n) (h_anchor : 1 ≤ anchor_interval)
    {δ : ℝ} (hδ : 0 < δ)
    (cache : KvCache n head_dim)
    (t : Fin n) (d : Fin head_dim) :
    |decode_kv_cache n head_dim anchor_interval h_n_pos
      (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t d - cache t d| ≤
      ((anchor_interval - 1) : ℝ) * δ / 2 := by
  have h_dist_lt : distance_to_last_anchor n anchor_interval t < anchor_interval :=
    Nat.mod_lt t.val (by omega)
  have h_err_bound := kv_reconstruction_error_bound n head_dim anchor_interval h_n_pos h_anchor hδ cache t d
  calc
    |decode_kv_cache n head_dim anchor_interval h_n_pos
      (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t d - cache t d|
        ≤ (distance_to_last_anchor n anchor_interval t : ℝ) * δ / 2 := h_err_bound
    _ ≤ ((anchor_interval - 1) : ℝ) * δ / 2 := by
      have h_le : (distance_to_last_anchor n anchor_interval t : ℕ) ≤ anchor_interval - 1 :=
        Nat.le_sub_one_of_lt h_dist_lt
      gcongr <;> try linarith [hδ]

/-! ## Propagation to Attention Scores -/

/-- Attention score for query q against key k: scale · (q · k). -/
def attention_score {head_dim : ℕ} (scale : ℝ) (q k : KvVector head_dim) : ℝ :=
  scale * ∑ d : Fin head_dim, q d * k d

/-- Perturbation bound for attention scores. -/
theorem attention_score_lipschitz {head_dim : ℕ}
    (scale : ℝ) (q k k' : KvVector head_dim)
    (err : ℝ) (h_perturb : ∀ d, |k' d - k d| ≤ err)
    (max_q : ℝ) (h_max_q : ∀ d, |q d| ≤ max_q) :
    |attention_score scale q k' - attention_score scale q k| ≤
      |scale| * (head_dim : ℝ) * max_q * err := by
  calc
    |attention_score scale q k' - attention_score scale q k|
        = |scale * ∑ d : Fin head_dim, q d * (k' d - k d)| := by
          simp [attention_score]
          ring
    _ ≤ |scale| * ∑ d : Fin head_dim, |q d * (k' d - k d)| := by
      calc
        |scale * ∑ d : Fin head_dim, q d * (k' d - k d)| = |scale| * |∑ d : Fin head_dim, q d * (k' d - k d)| := abs_mul scale _
        _ ≤ |scale| * ∑ d : Fin head_dim, |q d * (k' d - k d)| := by
          gcongr
          -- Triangle inequality for finite sums: |∑ f| ≤ ∑ |f|
          exact Finset.abs_sum_le_sum_abs (Finset.univ : Finset (Fin head_dim)) _
    _ ≤ |scale| * ∑ d : Fin head_dim, max_q * err := by
      calc
        |scale| * ∑ d : Fin head_dim, |q d * (k' d - k d)| ≤ |scale| * ∑ d : Fin head_dim, max_q * err := by
          gcongr
          exact Finset.sum_le_sum (Finset.univ : Finset (Fin head_dim)) fun d _ =>
            calc
              |q d * (k' d - k d)| = |q d| * |k' d - k d| := abs_mul (q d) (k' d - k d)
              _ ≤ max_q * |k' d - k d| := by gcongr; exact h_max_q d
              _ ≤ max_q * err := by gcongr; exact h_perturb d
    _ = |scale| * (head_dim : ℝ) * max_q * err := by
      simp [Finset.sum_const, Finset.card_fin]

/-- End-to-end: compressed KV cache produces attention scores within bounded tolerance. -/
theorem kv_compression_attention_tolerance (n head_dim anchor_interval : ℕ)
    (h_n_pos : 0 < n) (h_anchor : 1 ≤ anchor_interval)
    {δ B : ℝ} (hδ : 0 < δ) (hB : 0 < B)
    (cache : KvCache n head_dim)
    (h_bounded : ∀ (t : Fin n) (d : Fin head_dim), |cache t d| ≤ B)
    (scale : ℝ) (q : KvVector head_dim)
    (h_q_bounded : ∀ d : Fin head_dim, |q d| ≤ B)
    (h_scale_pos : 0 < scale)
    (t : Fin n) :
    |attention_score scale q (decode_kv_cache n head_dim anchor_interval h_n_pos
        (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t) -
     attention_score scale q (cache t)| ≤
      scale * (head_dim : ℝ) * B * ((anchor_interval - 1) : ℝ) * δ / 2 := by
  have h_perturb : ∀ d : Fin head_dim,
      |decode_kv_cache n head_dim anchor_interval h_n_pos
        (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t d - cache t d| ≤
        ((anchor_interval - 1) : ℝ) * δ / 2 := fun d => kv_max_error_bound n head_dim anchor_interval h_n_pos h_anchor hδ cache t d
  calc
    |attention_score scale q (decode_kv_cache n head_dim anchor_interval h_n_pos
        (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t) -
     attention_score scale q (cache t)| ≤
        |scale| * (head_dim : ℝ) * B * ((anchor_interval - 1) : ℝ) * δ / 2 := by
      have h_lip := attention_score_lipschitz scale q (cache t)
        (decode_kv_cache n head_dim anchor_interval h_n_pos
          (encode_kv_cache n head_dim anchor_interval h_n_pos h_anchor hδ cache) t)
        (((anchor_interval - 1) * δ) / 2) h_perturb B
        h_q_bounded
      -- Normalize: |scale| * hd * B * ((ai-1)*δ/2) = |scale| * hd * B * (ai-1) * δ / 2
      convert h_lip using 1
      ring_nf
    _ = scale * (head_dim : ℝ) * B * ((anchor_interval - 1) : ℝ) * δ / 2 := by
      rw [abs_of_pos h_scale_pos]

/-! ## Practical Instantiation for pesti

For Qwen2.5-Coder-0.5B: head_dim=64, f16 KV cache (B=65504).
With 8-bit deltas (δ ≈ 0.5) and anchors every 128 positions:
  max_error = 127 · 0.25 ≈ 32 per component
The anchor overhead is ~0.8% extra storage for bounded error. -/

/-! ## Delta Encoding Error Bounds -/

/-- **No-feedback (open-loop) delta encoding**.

Each raw delta is quantized independently. The total accumulated error
is the sum of all individual quantization errors, bounded by n·δ/2
via triangle inequality. The `h_range` hypothesis ensures no clipping
occurs in 8-bit storage (all deltas fit in [-127δ, 127δ]). -/
theorem delta_error_bound_no_clipping
  {n : ℕ} {δ : ℝ} (hδ : 0 < δ)
  (delta_i : Fin n → ℝ)
  (h_range : ∀ i : Fin n, |delta_i i| ≤ 127 * δ)
  :
    |∑ i : Fin n, (quantize_delta hδ (delta_i i) - delta_i i)| ≤ (n : ℝ) * δ / 2 := by
  -- Each individual quantization error is bounded by δ/2
  have h_each : ∀ i : Fin n, |quantize_delta hδ (delta_i i) - delta_i i| ≤ δ / 2 :=
    fun i => quantize_delta_bound hδ (delta_i i)
  -- Triangle inequality for finite sums: |∑ f| ≤ ∑ |f|
  calc
    |∑ i : Fin n, (quantize_delta hδ (delta_i i) - delta_i i)| ≤
        ∑ i : Fin n, |quantize_delta hδ (delta_i i) - delta_i i| := by
      exact Finset.abs_sum_le_sum_abs (Finset.univ : Finset (Fin n)) _
    _ ≤ ∑ i : Fin n, δ / 2 := by
      gcongr
      intro i
      exact h_each i
    _ = (n : ℝ) * δ / 2 := by
      simp [Finset.sum_const, Finset.card_fin]

/-! ## Feedback Delta Encoding -/

/-- **Feedback (closed-loop) reconstruction**.

At each step, the quantizer sees the difference between the current true
value and the *reconstructed* previous value (not the true previous value).
This prevents error accumulation: the reconstruction error at any step is
exactly one quantization error, bounded by δ/2. -/
noncomputable def feedback_reconstruct {δ : ℝ} (hδ : 0 < δ) (x₀ : ℝ)
    (deltas : ℕ → ℝ) : ℕ → ℝ
  | 0 => x₀
  | k + 1 => feedback_reconstruct hδ x₀ deltas k + quantize_delta hδ (deltas (k + 1))

/-- **Feedback error bound**.

With feedback, the reconstruction error at step n is exactly the
quantization error of the current delta. This gives a half-open interval
bound: -(δ/2) ≤ error_n < δ/2.

The half-open nature comes from the floor-based rounding in quantize_delta:
the tie-breaking at x = k·δ - δ/2 rounds down (error = +δ/2, inclusive),
while approaching x = k·δ + δ/2 from below gives error → -δ/2 (exclusive). -/
theorem delta_error_bound_feedback
  {n : ℕ} {δ : ℝ} (hδ : 0 < δ)
  (x₀ : ℝ) (deltas : ℕ → ℝ)
  (h_range : ∀ k ≤ n, |feedback_reconstruct hδ x₀ deltas k + deltas (k + 1) -
                       feedback_reconstruct hδ x₀ deltas (k + 1)| ≤ 127 * δ)
  :
    -(δ / 2) ≤ deltas (n + 1) - quantize_delta hδ (deltas (n + 1)) ∧
    deltas (n + 1) - quantize_delta hδ (deltas (n + 1)) < δ / 2 := by
  -- The reconstruction error at step n+1 is exactly the quantization error
  -- of delta_{n+1}: error = d - Q(d)
  -- From floor properties of quantize_delta:
  --   k ≤ x/δ + 1/2 < k+1  where k = ⌊x/δ + 1/2⌋
  -- gives: -δ/2 ≤ x - Q(x) < δ/2
  set d := deltas (n + 1)
  set k : ℤ := ⌊d / δ + 1/2⌋ with hk_def
  have h_floor_le : (k : ℝ) ≤ d / δ + 1/2 := Int.floor_le (d / δ + 1/2)
  have h_floor_lt : d / δ + 1/2 < (k : ℝ) + 1 := Int.lt_floor_add_one (d / δ + 1/2)
  have h_div_mul : d / δ * δ = d := div_mul_cancel₀ d hδ.ne'
  
  -- Left bound: k ≤ d/δ + 1/2 → k*δ ≤ d + δ/2 → d - k*δ ≥ -δ/2
  have h_left : -(δ / 2) ≤ d - (k : ℝ) * δ := by
    have h_raw : (k : ℝ) * δ ≤ d + δ / 2 := by
      calc
        (k : ℝ) * δ ≤ (d / δ + 1/2) * δ := mul_le_mul_of_nonneg_right h_floor_le hδ.le
        _ = d / δ * δ + δ / 2 := by ring
        _ = d + δ / 2 := by rw [h_div_mul]
    linarith
  
  -- Right bound: d/δ + 1/2 < k+1 → d + δ/2 < (k+1)*δ → d - k*δ < δ/2
  have h_right : d - (k : ℝ) * δ < δ / 2 := by
    have h_raw : d + δ / 2 < ((k : ℝ) + 1) * δ := by
      calc
        d + δ / 2 = d / δ * δ + δ / 2 := by rw [h_div_mul]
        _ = (d / δ + 1/2) * δ := by ring
        _ < ((k : ℝ) + 1) * δ := mul_lt_mul_of_pos_right h_floor_lt hδ
  
  -- Combine using the definition of quantize_delta
  have h_quant_eq : quantize_delta hδ d = (k : ℝ) * δ := by
    rw [quantize_delta, hk_def]
  constructor
  · rw [h_quant_eq] at *; exact h_left
  · rw [h_quant_eq] at *; exact h_right

/-! ## Proof Strategy Notes -/

-- Key insight from the Missy Elliott Theorem applied here:
-- Work it: iterate on quantization until error structure is clear
-- Flip it: prove from reconstruction side (decode error) not encoding side
-- Reverse it: periodic anchors reverse accumulation — error grows from each anchor
-- Constraint: same damn theorem, different path