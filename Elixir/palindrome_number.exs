# https://leetcode.com/problems/palindrome-number/
# 9. Palindrome Number

defmodule Solution do
  def is_palindrome(x) do
    with reversed <- to_string(x) |> String.reverse() do
      case reversed == to_string(x) do
        true -> true
        _ -> false
      end
    end
  end
end

s = -1221

IO.inspect(Solution.is_palindrome(s))  # false
