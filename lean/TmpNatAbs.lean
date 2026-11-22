import Mathlib

-- Test what we know about Int.natAbs
theorem test_natAbs_neg (s a : ℕ) (h_off_neg : (s : ℤ) - a < 0) :
    Int.natAbs ((s : ℤ) - a) = (a - s : ℕ) := by
  -- For x < 0, |x| = -x. Here x = (s-a), so |(s-a)| = a-s
  -- Try using abs_of_neg which should give us |x| = -x for x < 0
  have h_abs : Int.natAbs ((s : ℤ) - a) = (-(s - a)).toNat := by
    -- Since (s-a) < 0, we have -(s-a) > 0
    -- And for negative x, natAbs x = (-x).toNat
    exact Int.natAbs_of_nonpos (by omega)
  -- Now -(s-a) = a-s as integers
  have h_simplify : (-(s - a)).toNat = (a - s : ℕ) := by
    norm_cast
  rw [h_abs, h_simplify]
