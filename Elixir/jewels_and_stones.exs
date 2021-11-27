# https://leetcode.com/problems/jewels-and-stones/
# 771. Jewels and Stones

defmodule Solution do
  def num_jewels_in_stones(jewels, stones) do
    jewel_list = String.graphemes(jewels)
    stone_list = String.graphemes(stones)

    Enum.reduce(stone_list, 0,
      &case &1 in jewel_list do
        true -> &2 + 1
        _ -> &2
      end
      )
  end
end

jewels = "aA"
stones = "aAAbbbb"

Solution.num_jewels_in_stones(jewels, stones)
|> IO.puts()  # 3
