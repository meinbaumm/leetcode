defmodule Task771Test do
  use ExUnit.Case
  doctest Task771

  test "Jewels and Stones" do
    jewels1 = "aA"
    stones1 = "aAAbbbb"
    assert Task771.num_jewels_in_stones(jewels1, stones1) == 3

    jewels2 = "z"
    stones2 = "ZZ"
    assert Task771.num_jewels_in_stones(jewels2, stones2) == 0
  end
end
