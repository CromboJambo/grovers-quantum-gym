import Mathlib.Data.Real.Basic
import Mathlib.Analysis.SpecialFunctions.Pow.Real
import Mathlib.Data.Fin.Basic
import Mathlib.Analysis.Normed.Group.Basic
import Mathlib.Tactic

noncomputable def quantize_delta {δ : ℝ} (_hδ : 0 < δ) (x : ℝ) : ℝ :=
  ⌊x / δ + 1/2⌋ * δ

noncomputable def quantize_clipped_int {δ : ℝ} (_hδ : 0 < δ) (d : ℝ) : ℤ :=
  max (-127) (min 127 ⌊d / δ + 1/2⌋)

noncomputable def quantize_clipped {δ : ℝ} (hδ : 0 < δ) (d : ℝ) : ℝ :=
  (quantize_clipped_int hδ d : ℝ) * δ

theorem quantize_clipped_no_clip {δ : ℝ} (hδ : 0 < δ) (d : ℝ) (hd : |d| ≤ 126 * δ) :
    quantize_clipped_int hδ d = ⌊d / δ + 1/2⌋ := by
  rw [abs_le] at hd
  -- Show floor(d/δ + 1/2) is in [-126, 126]
  have h_floor_left : (-126 : ℤ) ≤ ⌊d / δ + 1/2⌋ := by
    have h : (-126 : ℝ) ≤ d / δ + 1/2 := by
      calc
        (-126 : ℝ) = (-(126 * δ)) / δ := by field_simp [hδ.ne']
        _ ≤ d / δ := div_le_div_of_nonneg_right hd.1 hδ.le
        _ ≤ d / δ + 1/2 := le_add_of_nonneg_right (by norm_num)
    exact Int.le_floor.mpr h
  have h_floor_right : ⌊d / δ + 1/2⌋ ≤ 126 := by
    have h_lt : (⌊d / δ + 1/2⌋ : ℝ) < 127 := by
      have h_floor_le_x : (⌊d / δ + 1/2⌋ : ℝ) ≤ d / δ + 1/2 := Int.floor_le _
      have h_div_bound : d / δ ≤ (126 * δ) / δ := div_le_div_of_nonneg_right hd.2 hδ.le
      calc
        (⌊d / δ + 1/2⌋ : ℝ) ≤ d / δ + 1/2 := h_floor_le_x
        _ ≤ (126 * δ) / δ + 1/2 := by linarith [h_div_bound]
        _ = 126 + 1/2 := by field_simp [hδ.ne']
        _ < 127 := by norm_num
    have h_int_lt : ⌊d / δ + 1/2⌋ < 127 := by exact_mod_cast h_lt
    omega
  -- So max(-127, min(127, k)) = k for k ∈ [-126, 126]
  have h_min : min 127 ⌊d / δ + 1/2⌋ = ⌊d / δ + 1/2⌋ := by
    rw [min_eq_right (by linarith [h_floor_right])]
  have h_max : max (-127) (min 127 ⌊d / δ + 1/2⌋) = min 127 ⌊d / δ + 1/2⌋ := by
    rw [max_eq_right]; linarith
  rw [quantize_clipped_int, h_max, h_min]

theorem quantize_clipped_error_bound {δ : ℝ} (hδ : 0 < δ) (d : ℝ) (hd : |d| ≤ 126 * δ) :
    -(δ / 2) ≤ d - quantize_clipped hδ d ∧ d - quantize_clipped hδ d < δ / 2 := by
  have h_no_clip : quantize_clipped_int hδ d = ⌊d / δ + 1/2⌋ := quantize_clipped_no_clip hδ d hd
  have h_eq : quantize_clipped hδ d = quantize_delta hδ d := by
    rw [quantize_clipped, quantize_delta]
    simp [h_no_clip]
  rw [h_eq]
  set k : ℤ := ⌊d / δ + 1/2⌋ with hk_def
  have h_floor_le : (k : ℝ) ≤ d / δ + 1/2 := Int.floor_le (d / δ + 1/2)
  have h_floor_lt : d / δ + 1/2 < (k : ℝ) + 1 := Int.lt_floor_add_one (d / δ + 1/2)
  have h_div_mul : d / δ * δ = d := div_mul_cancel₀ d hδ.ne'
  have h_left : -(δ / 2) ≤ d - (k : ℝ) * δ := by
    have h_raw : (k : ℝ) * δ ≤ d + δ / 2 := by
      calc
        (k : ℝ) * δ ≤ (d / δ + 1/2) * δ := mul_le_mul_of_nonneg_right h_floor_le hδ.le
        _ = d / δ * δ + δ / 2 := by ring
        _ = d + δ / 2 := by rw [h_div_mul]
    linarith
  have h_right : d - (k : ℝ) * δ < δ / 2 := by
    have h_raw : d + δ / 2 < ((k : ℝ) + 1) * δ := by
      calc
        d + δ / 2 = d / δ * δ + δ / 2 := by rw [h_div_mul]
        _ = (d / δ + 1/2) * δ := by ring
        _ < ((k : ℝ) + 1) * δ := mul_lt_mul_of_pos_right h_floor_lt hδ
    have h_expand : ((k : ℝ) + 1) * δ = (k : ℝ) * δ + δ := by ring
    rw [h_expand] at h_raw
    linarith
  -- Replace ⌊d/δ + 1/2⌋ with k in the goal
  have h_goal_eq : d - quantize_delta hδ d = d - (k : ℝ) * δ := by
    rw [quantize_delta, hk_def.symm]
  rw [h_goal_eq]
  constructor <;> assumption

noncomputable def feedback_reconstruct_clipped {δ : ℝ} (hδ : 0 < δ) (_x₀ : ℝ)
    (deltas : ℕ → ℝ) : ℕ → ℝ
  | 0 => _x₀
  | k + 1 => feedback_reconstruct_clipped hδ _x₀ deltas k + quantize_clipped hδ (deltas (k + 1))

theorem delta_error_bound_feedback_clipped
  {n : ℕ} {δ : ℝ} (hδ : 0 < δ)
  (_x₀ : ℝ) (deltas : ℕ → ℝ)
  (h_range : ∀ k ≤ n, |deltas (k + 1)| ≤ 126 * δ)
  :
    -(δ / 2) ≤ deltas (n + 1) - quantize_clipped hδ (deltas (n + 1)) ∧
    deltas (n + 1) - quantize_clipped hδ (deltas (n + 1)) < δ / 2 := by
  have h_n : |deltas (n + 1)| ≤ 126 * δ := h_range n (le_refl n)
  exact quantize_clipped_error_bound hδ (deltas (n + 1)) h_n

theorem delta_error_bound_no_clipping_clipped
  {n : ℕ} {δ : ℝ} (hδ : 0 < δ)
  (delta_i : Fin n → ℝ)
  (h_range : ∀ i : Fin n, |delta_i i| ≤ 126 * δ)
  :
    |∑ i : Fin n, (quantize_clipped hδ (delta_i i) - delta_i i)| ≤ (n : ℝ) * δ / 2 := by
  have h_each : ∀ i : Fin n, |quantize_clipped hδ (delta_i i) - delta_i i| ≤ δ / 2 := by
    intro i
    have h_bound : -(δ / 2) ≤ delta_i i - quantize_clipped hδ (delta_i i) ∧
                   delta_i i - quantize_clipped hδ (delta_i i) < δ / 2 :=
      quantize_clipped_error_bound hδ (delta_i i) (h_range i)
    rw [abs_le]
    constructor
    · linarith [h_bound.1, h_bound.2]
    · linarith [h_bound.1, h_bound.2]
  calc
    |∑ i : Fin n, (quantize_clipped hδ (delta_i i) - delta_i i)| ≤
        ∑ i : Fin n, |quantize_clipped hδ (delta_i i) - delta_i i| := by
      have h := norm_sum_le (Finset.univ : Finset (Fin n)) (fun i => quantize_clipped hδ (delta_i i) - delta_i i)
      simpa [Real.norm_eq_abs] using h
    _ ≤ ∑ i : Fin n, δ / 2 := by
      apply Finset.sum_le_sum
      intro i _
      exact h_each i
    _ = (n : ℝ) * δ / 2 := by
      simp [Finset.sum_const]
      ring
