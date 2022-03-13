# https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
# 2114. Maximum Number of Words Found in Sentences

defmodule Solution do
  @spec most_words_found(sentences :: [String.t]) :: integer
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

    def most_words_found(sentences) do
      count_spaces(sentences)
    end
end
