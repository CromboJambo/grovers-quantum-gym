import Mathlib.Tactic
#check @List.take_append_of_le_length
#check @List.drop_append_of_le_length
#check @List.take_append
#check @List.drop_append
example (bs rest : List Bool) : (bs ++ rest).take bs.length = bs := by
  simp [List.take_append_of_le_length]
example (bs rest : List Bool) : (bs ++ rest).drop bs.length = rest := by
  simp [List.drop_append_of_le_length]
