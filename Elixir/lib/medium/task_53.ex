defmodule Medium.Task53 do
  @moduledoc """
  53. Maximum Subarray
  https://leetcode.com/problems/maximum-subarray/
  """
  @spec max_sub_array(nums :: [integer]) :: integer
  def max_sub_array(nums) do
    Enum.reduce(nums, {0, -10000}, fn n, {cur, max} ->
      {Enum.max([n] ++ [cur + n]), Enum.max([max] ++ [Enum.max([n] ++ [cur + n])])}
    end)
    |> elem(1)
  end
end
