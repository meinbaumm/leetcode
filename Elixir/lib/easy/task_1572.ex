defmodule Easy.Task1572 do
  @moduledoc """
  1572. Matrix Diagonal Sum
  https://leetcode.com/problems/matrix-diagonal-sum/
  """
  @spec diagonal_sum(mat :: [[integer]]) :: integer
  def diagonal_sum(mat) do
    get_indexes = fn m_size, n_size ->
      for x <- 1..m_size, y <- 1..n_size, do: {x, y}
    end

    sum_of_diagonal = fn matrix, diagonal ->
      diagonal
      |> Enum.map(fn {m, n} ->
        matrix
        |> Enum.at(m - 1)
        |> Enum.at(n - 1)
      end)
      |> Enum.sum()
    end

    case {length(mat), Enum.at(mat, 0) |> length()} do
      {1, 1} ->
        mat
        |> Enum.at(0)
        |> Enum.at(0)

      {m_size, n_size} ->
        indexes = get_indexes.(m_size, n_size)
        main_diagonal = Enum.filter(indexes, fn {m, n} -> m == n end)
        companion_diagonal = Enum.filter(indexes, fn {m, n} -> m + n == n_size + 1 end)

        sum_of_diagonal.(mat, Enum.uniq(main_diagonal ++ companion_diagonal))
    end
  end
end
