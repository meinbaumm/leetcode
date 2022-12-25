defmodule Easy.Task415Test do
  use ExUnit.Case
  doctest Easy.Task415

  test "415. Add Strings add_strings_simple/2" do
    num1 = "11"
    num2 = "123"
    assert Easy.Task415.add_strings_simple(num1, num2) == "134"

    num3 = "456"
    num4 = "77"
    assert Easy.Task415.add_strings_simple(num3, num4) == "533"

    num5 = "0"
    num6 = "0"
    assert Easy.Task415.add_strings_simple(num5, num6) == "0"
  end

  test "415. Add Strings add_strings/2" do
    num1 = "11"
    num2 = "123"
    assert Easy.Task415.add_strings(num1, num2) == "134"

    num3 = "456"
    num4 = "77"
    assert Easy.Task415.add_strings(num3, num4) == "533"

    num5 = "0"
    num6 = "0"
    assert Easy.Task415.add_strings(num5, num6) == "0"
  end

  test "415. Add Strings add_strings_recursive/2" do
    num1 = "11"
    num2 = "123"
    assert Easy.Task415.add_strings_recursive(num1, num2) == "134"

    num3 = "456"
    num4 = "77"
    assert Easy.Task415.add_strings_recursive(num3, num4) == "533"

    num5 = "0"
    num6 = "0"
    assert Easy.Task415.add_strings_recursive(num5, num6) == "0"
  end
end
