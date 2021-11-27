# https://leetcode.com/problems/longest-common-prefix/
# 14. Longest Common Prefix

defmodule Solution do
  def longest_common_prefix(strs) do
    fr_one = freqs(strs, 1)
    values_one = Map.values(fr_one)

    check_prefix(fr_one, values_one)
  end

  defp freqs(strs, end_string) do
    Enum.map(strs, fn(prefix) -> String.slice(prefix, 0..end_string) end)
    |> Enum.frequencies()
  end

  defp check_prefix(fr, values) do
    case Enum.sort(values) == values do
      true when length(values) > 1 -> ""
      true when length(values) == 1 ->
        Enum.find(fr, fn {_key, val} -> val == Enum.max(values) end)
        |> elem(0)
      _ -> ""
    end
  end
end

strs = ["ab", "a"]

IO.inspect(Solution.longest_common_prefix(strs))
