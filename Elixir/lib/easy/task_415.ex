defmodule Easy.Task415 do
  @moduledoc """
  415. Add Strings
  https://leetcode.com/problems/add-strings/
  """

  require Integer

  def add_strings_simple(num1, num2) do
    (String.to_integer(num1) + String.to_integer(num2))
    |> Integer.to_string()
  end

  def add_strings_recursive(num1, num2) do
    num1 = {String.graphemes(num1), String.length(num1), 0}
    num2 = {String.graphemes(num2), String.length(num2), 0}

    num1_as_integer = add(num1)
    num2_as_integer = add(num2)

    Integer.to_string(num1_as_integer + num2_as_integer)
  end

  defp add({_graphemes, 0, acc}) do
    acc
  end

  defp add({[head | tail], len, acc}) do
    updated_acc = compare_and_replace(head) * Integer.pow(10, len - 1) + acc
    add({tail, len - 1, updated_acc})
  end

  defp compare_and_replace(number) do
    case number do
      "0" -> 0
      "1" -> 1
      "2" -> 2
      "3" -> 3
      "4" -> 4
      "5" -> 5
      "6" -> 6
      "7" -> 7
      "8" -> 8
      "9" -> 9
    end
  end

  def add_strings(num1, num2) do
    to_string(parse_int(num1) + parse_int(num2))
  end

  defp ascii_to_digit(ascii) when ascii >= 48 and ascii < 58 do
    ascii - 48
  end

  defp ascii_to_digit(_), do: raise(ArgumentError)

  def parse_int(str) do
    str
    |> String.reverse()
    |> do_parse_int(0, [])
  end

  defp do_parse_int(<<char::utf8>> <> rest, index, cache) do
    new_part = ascii_to_digit(char) * round(:math.pow(10, index))

    do_parse_int(
      rest,
      index + 1,
      [new_part | cache]
    )
  end

  defp do_parse_int("", _, cache) do
    Enum.reduce(cache, 0, &Kernel.+/2)
  end
end
