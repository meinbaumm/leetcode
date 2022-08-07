defmodule Task709Test do
  use ExUnit.Case
  doctest Task709

  test "To Lower Case" do
    s1 = "Hello"
    assert Task709.to_lower_case(s1) == "hello"

    s2 = "here"
    assert Task709.to_lower_case(s2) == "here"

    s3 = "LOVELY"
    assert Task709.to_lower_case(s3) == "lovely"
  end
end
