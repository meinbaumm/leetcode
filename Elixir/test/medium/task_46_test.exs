defmodule Medium.Task46Test do
  use ExUnit.Case

  doctest Medium.Task46

  test "permute/1" do
    n1 = [1, 2, 3]

    sorted_answer_1 =
      Enum.sort([[3, 1, 2], [3, 2, 1], [2, 1, 3], [2, 3, 1], [1, 2, 3], [1, 3, 2]])

    assert Enum.sort(Medium.Task46.permute(n1)) == sorted_answer_1

    n2 = [0, 1]
    sorted_answer_2 = Enum.sort([[0, 1], [1, 0]])

    assert Enum.sort(Medium.Task46.permute(n2)) == sorted_answer_2

    n3 = [1]
    assert Medium.Task46.permute(n3) == [[1]]
  end
end
