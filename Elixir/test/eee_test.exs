defmodule EeeTest do
  use ExUnit.Case
  doctest Eee

  test "greets the world" do
    assert Eee.hello() == :world
  end
end
