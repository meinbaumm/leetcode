# https://leetcode.com/problems/counting-words-with-a-given-prefix/
# 2185. Counting Words With a Given Prefix

defmodule Solution do
  def prefix_count(words, pref) do
    words
    |> Enum.filter(&String.starts_with?(&1, pref))
    |> Enum.count()
  end
end
