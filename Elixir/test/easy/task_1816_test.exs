defmodule Easy.Task1816Test do
  use ExUnit.Case
  doctest Easy.Task1816

  test "Truncate Sentence" do
    s1 = "Hello how are you Contestant"
    k1 = 4
    assert Easy.Task1816.truncate_sentence(s1, k1) == "Hello how are you"

    s2 = "What is the solution to this problem"
    k2 = 4
    assert Easy.Task1816.truncate_sentence(s2, k2) == "What is the solution"

    s3 = "chopper is not a tanuki"
    k3 = 5
    assert Easy.Task1816.truncate_sentence(s3, k3) == "chopper is not a tanuki"
  end
end
