defmodule Task1572Test do
  use ExUnit.Case
  doctest Easy.Task1572

  test "Matrix Diagonal Sum" do
    m1 = [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]
    ]

    assert Easy.Task1572.diagonal_sum(m1) == 25

    m2 = [
      [1, 1, 1, 1],
      [1, 1, 1, 1],
      [1, 1, 1, 1],
      [1, 1, 1, 1]
    ]

    assert Easy.Task1572.diagonal_sum(m2) == 8

    m3 = [[5]]
    assert Easy.Task1572.diagonal_sum(m3) == 5

    m4 = [
      [7, 3, 1, 9],
      [3, 4, 6, 9],
      [6, 9, 6, 6],
      [9, 5, 8, 5]
    ]

    assert Easy.Task1572.diagonal_sum(m4) == 55
  end
end
