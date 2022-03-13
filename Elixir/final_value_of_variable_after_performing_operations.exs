# https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
# 2011. Final Value of Variable After Performing Operations

defmodule Solution do
  @spec final_value_after_operations(operations :: [String.t]) :: integer
  def final_value_after_operations(operations) do
    operations
    |> Enum.reduce(0,
      fn val, acc ->
        case val do
          "++X" -> acc + 1
          "X++" -> acc + 1
          "--X" -> acc - 1
          "X--" -> acc - 1
        end
      end
    )
  end
end
