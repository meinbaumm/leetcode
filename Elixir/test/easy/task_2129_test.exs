defmodule Easy.Task2129Test do
  use ExUnit.Case
  doctest Easy.Task2129

  test "capitalize the title" do
    title1 = "capiTalIze tHe titLe"
    assert Easy.Task2129.capitalize_title(title1) == "Capitalize The Title"

    title2 = "First leTTeR of EACH Word"
    assert Easy.Task2129.capitalize_title(title2) == "First Letter of Each Word"

    title3 = "i lOve leetcode"
    assert Easy.Task2129.capitalize_title(title3) == "i Love Leetcode"
  end
end
