defmodule Easy.Task392Test do
  use ExUnit.Case
  doctest Easy.Task392

  test "is_subsequence/2" do
    s1 = "abc"
    t1 = "ahbgdc"
    assert Easy.Task392.is_subsequence(s1, t1) == true

    s2 = "axc"
    t2 = "ahbgdc"
    assert Easy.Task392.is_subsequence(s2, t2) == false

    s3 = "acb"
    t3 = "ahbgdc"
    assert Easy.Task392.is_subsequence(s3, t3) == false
  end
end
