import Mathlib

open Int

-- Find what proves |x| = n for x < 0
example (s a : ℕ) (h_off_neg : (s : ℤ) - a < 0) : Int.natAbs ((s : ℤ) - a) = (a - s : ℕ) := by
  exact?
