# https://leetcode.com/problems/number-of-good-pairs/
# 1512. Number of Good Pairs

# Given an array of integers nums, return the number of good pairs.
# A pair (i, j) is called good if nums[i] == nums[j] and i < j.

defmodule Solution do
  @spec num_identical_pairs(nums :: [integer]) :: integer
  def num_identical_pairs([_n]), do: 0
  def num_identical_pairs(nums) do
    enumerate = Enum.with_index(nums)
    pairs = for x <- enumerate, y <- enumerate, do: [x, y]

    Enum.map(
      pairs,
      fn variant ->
        el_i = Enum.at(variant, 0) |> elem(0)
        el_j = Enum.at(variant, 1) |> elem(0)
        i = Enum.at(variant, 0) |> elem(1)
        j = Enum.at(variant, 1) |> elem(1)
        if el_i == el_j and i < j do
          true
        end
      end
    )
    |> Enum.filter(fn x -> x == true end)
    |> length()
  end
end
