defmodule Easy.Task389Test do
  use ExUnit.Case
  doctest Easy.Task392

  test "find_the_difference/2" do
    s1 = "abcd"
    t1 = "abcde"
    assert Easy.Task389.find_the_difference(s1, t1) == 101

    s2 = ""
    t2 = "y"
    assert Easy.Task389.find_the_difference(s2, t2) == 121
  end
end
