defmodule ListNode do
  @type t :: %__MODULE__{
    val: integer,
    next: ListNode.t() | nil
  }
  defstruct val: 0, next: nil
end

defmodule Task2 do
  @moduledoc """
  2. Add Two Numbers
  https://leetcode.com/problems/add-two-numbers/
  """
  def add_two_numbers(l1, l2) do
    get_value(l1, l2, 0)
  end

  defp get_value(nil, nil, 0) do
  end

  defp get_value(nil, nil, 1) do
    %ListNode{val: 1, next: nil}
  end

  defp get_value(%{val: x, next: y}, nil, carry) do
    if x + carry > 9 do
      %ListNode{val: x + carry - 10, next: get_value(y, nil, 1)}
    else
      %ListNode{val: x + carry, next: get_value(y, nil, 0)}
    end
  end

  defp get_value(nil, %{val: x, next: y}, carry) do
    if x + carry > 9 do
      %ListNode{val: x + carry - 10, next: get_value(y, nil, 1)}
    else
      %ListNode{val: x + carry, next: get_value(y, nil, 0)}
    end
  end

  defp get_value(%{val: x1, next: y1}, %{val: x2, next: y2}, carry) do
    if x1 + x2 + carry > 9 do
      %ListNode{val: x1 + x2 + carry - 10, next: get_value(y1, y2, 1)}
    else
      %ListNode{val: x1 + x2 + carry, next: get_value(y1, y2, 0)}
    end
  end
end
