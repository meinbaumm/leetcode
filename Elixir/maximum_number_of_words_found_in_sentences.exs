# https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
# 2114. Maximum Number of Words Found in Sentences

defmodule Solution do
  defp count_spaces(sentences) do
    Enum.map(sentences, fn s ->
      freq = String.graphemes(s) |> Enum.frequencies()
      case Map.get(freq, " ") do
        nil -> 0
        _ -> Map.get(freq, " ")
      end
    end)
    |> Enum.max()
  end

  @spec most_words_found(sentences :: [String.t]) :: integer
  def most_words_found(sentences) do
    count_spaces(sentences) + 1
  end
end
