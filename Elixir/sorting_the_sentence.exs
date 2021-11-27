# https://leetcode.com/problems/sorting-the-sentence/
# 1859. Sorting the Sentence

defmodule Solution do
  def sorting_sentence(sentence) do
    String.split(sentence)
    |> Enum.map(fn(word_and_num) ->
        %{
          word: String.slice(word_and_num, 0..-2),
          num: String.slice(word_and_num, -1..-1) |> String.to_integer()
        }
    end
    )
    |> Enum.map(fn(m) -> m[:num] end)
    |> Enum.sort()
  end
end

s = "is2 sentence4 This1 a3"

Solution.sorting_sentence(s) |> IO.inspect()
