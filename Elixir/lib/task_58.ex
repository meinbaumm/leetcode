defmodule Task58 do
  @moduledoc """
  58. Length of Last Word
  https://leetcode.com/problems/length-of-last-word/
  """
  @spec length_of_last_word(s :: String.t) :: integer
  def length_of_last_word(s) do
    s
    |> String.split(" ", trim: true)
    |> Enum.at(-1)
    |> String.length()
  end
end
