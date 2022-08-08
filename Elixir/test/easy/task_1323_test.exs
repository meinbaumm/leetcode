defmodule Easy.Task1323Test do
  use ExUnit.Case
  doctest Easy.Task1323

  test "Maximum 69 Number" do
    num1 = 9669
    assert Easy.Task1323.maximum_69_number(num1) == 9969

    num2 = 9996
    assert Easy.Task1323.maximum_69_number(num2) == 9999

    num3 = 9999
    assert Easy.Task1323.maximum_69_number(num3) == 9999
  end
end
