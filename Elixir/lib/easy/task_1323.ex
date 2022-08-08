defmodule Easy.Task1323 do
  @moduledoc """
  1323. Maximum 69 Number
  https://leetcode.com/problems/maximum-69-number/
  """

  def maximum_69_number(num = 9999), do: num

  def maximum_69_number(num) do
    graphemes =
      num
      |> Integer.to_string()
      |> String.graphemes()

    six_ind =
      graphemes
      |> Enum.with_index(fn element, index -> {element, index} end)
      |> six_indexes()

    calculate_max_sum(graphemes, six_ind, [])
  end

  defp six_indexes(graphemes_indexes) do
    graphemes_indexes
    |> Enum.reduce([], fn {el, ind}, acc ->
      case el do
        "6" ->
          [ind | acc]

        "9" ->
          acc
      end
    end)
  end

  defp calculate_max_sum(_graphemes, [], num_list), do: Enum.max(num_list)

  defp calculate_max_sum(graphemes, [head | tail], num_list) do
    new_num =
      graphemes
      |> List.replace_at(head, "9")
      |> Enum.reduce("", fn x, acc -> acc <> x end)
      |> String.to_integer()

    calculate_max_sum(graphemes, tail, [new_num | num_list])
  end
end
