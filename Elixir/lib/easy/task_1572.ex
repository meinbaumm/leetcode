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

  def diagonal_sum_recursive(mat) do
    {m, n} = {length(mat), Enum.at(mat, 0) |> length()}
    diag_sum(mat, {m, n})
  end

  defp diag_sum(mat, {1, 1}) do
    [[sum]] = mat
    sum
  end

  defp diag_sum(mat, {m, n}) do
    indexes = for x <- 1..m, y <- 1..n, do: {x, y}
    main_diagonal = Enum.filter(indexes, fn {m_ind, n_ind} -> m_ind == n_ind end)
    companion_diagonal = Enum.filter(indexes, fn {m_ind, n_ind} -> m_ind + n_ind == n + 1 end)

    uniq_indexes = Enum.uniq(main_diagonal ++ companion_diagonal)

    diag_sum(mat, {m, n}, uniq_indexes, 0)
  end

  defp diag_sum(_mat, {_m, _n}, [], sum), do: sum

  defp diag_sum(mat, {_m, _n}, [{m_ind, n_ind} | tail], sum) do
    new_sum =
      mat
      |> Enum.at(m_ind - 1)
      |> Enum.at(n_ind - 1)
      |> then(fn el -> el + sum end)

    diag_sum(mat, {m_ind, n_ind}, tail, new_sum)
  end
end
