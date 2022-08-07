defmodule Easy.Task709Test do
  use ExUnit.Case
  doctest Easy.Task709

  test "To Lower Case" do
    s1 = "Hello"
    assert Easy.Task709.to_lower_case(s1) == "hello"

    s2 = "here"
    assert Easy.Task709.to_lower_case(s2) == "here"

    s3 = "LOVELY"
    assert Easy.Task709.to_lower_case(s3) == "lovely"
  end
end
