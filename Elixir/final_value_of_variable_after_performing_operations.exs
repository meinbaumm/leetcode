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

  # def final_value_after_operations(operations) do
  #   hidden_final_value_after_operations(operations, 0)
  # end
end

# operations = ["--X","X++","X++"]
operations = ["++X","++X","X++"]
IO.inspect(Solution.final_value_after_operations(operations))
