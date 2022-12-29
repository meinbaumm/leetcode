defmodule Easy.Task389 do
  @moduledoc """
  389. Find the Difference
  https://leetcode.com/problems/find-the-difference/
  """

  @spec find_the_difference(s :: String.t(), t :: String.t()) :: char
  def find_the_difference(s, t) do
    letter = to_string(String.to_charlist(t) -- String.to_charlist(s))
    <<difference, _>> = letter <> <<0>>
    difference
  end
end
