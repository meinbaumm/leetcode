defmodule Task58Test do
  use ExUnit.Case
  doctest Task58

  test "Length of Last Word" do
    s1 = "Hello World"
    assert Task58.length_of_last_word(s1) == 5

    s2 = "   fly me   to   the moon  "
    assert Task58.length_of_last_word(s2) == 4

    s3 = "luffy is still joyboy"
    assert Task58.length_of_last_word(s3) == 6
  end
end
