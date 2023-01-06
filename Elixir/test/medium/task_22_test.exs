defmodule Medium.Task22Test do
  use ExUnit.Case
  doctest Medium.Task22

  test "generate_parenthesis/1" do
    n1 = 3

    assert Medium.Task22.generate_parenthesis(n1) == [
             "((()))",
             "(()())",
             "(())()",
             "()(())",
             "()()()"
           ]

    n2 = 1
    assert Medium.Task22.generate_parenthesis(n2) == ["()"]
  end
end
