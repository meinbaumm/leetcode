defmodule Easy.Task1816 do
  @moduledoc """
  1816. Truncate Sentence
  https://leetcode.com/problems/truncate-sentence/
  """
  @spec truncate_sentence(s :: String.t, k :: integer) :: String.t
  def truncate_sentence(s, k) do

    truncated =
      String.split(s, " ")
      |> Stream.take(k)
      |> Enum.reduce("", fn word, acc_string ->
        acc_string <> word <> " "
      end)

  String.replace_trailing(truncated, String.at(truncated, -1), "")
  end
end
