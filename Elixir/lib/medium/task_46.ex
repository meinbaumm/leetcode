defmodule Medium.Task46 do
  @moduledoc """
  46. Permutations
  https://leetcode.com/problems/permutations/description/
  """

  def permute(nums), do: shuffle(nums, length(nums))

  defp shuffle([], _), do: [[]]
  defp shuffle(_, 0), do: [[]]

  defp shuffle(list, i) do
    for x <- list, y <- shuffle(list, i - 1), reduce: [] do
      acc ->
        cond do
          x not in y and y not in acc -> [[x | y] | acc]
          x in y or y in acc -> acc
        end
    end
  end
end
