defmodule Task2129 do
  @moduledoc """
  2129. Capitalize the Title
  https://leetcode.com/problems/capitalize-the-title/
  """
  @spec capitalize_title(title :: String.t) :: String.t
  def capitalize_title(title) do
    capitalized =
      title
      |> String.split(" ")
      |> Stream.map(fn word ->
        case String.length(word) do
          l when l in 1..2 -> String.downcase(word)
          _ -> String.downcase(word) |> String.capitalize()
        end
      end)
      |> Enum.reduce("", fn word, acc_string ->
        acc_string <> word <> " "
      end)

    String.replace_trailing(capitalized, String.at(capitalized, -1), "")
  end
end
