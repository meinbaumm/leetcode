defmodule Easy.Task392 do
  @moduledoc """
  392. Is Subsequence
  https://leetcode.com/problems/is-subsequence/
  """

  @spec is_subsequence(s :: String.t(), t :: String.t()) :: boolean
  def is_subsequence(s, t) do
    t_graphemes = String.graphemes(t)
    s_graphemes = String.graphemes(s)

    maybe_subs(s_graphemes, t_graphemes, "")
  end

  defp maybe_subs([], _t, _acc), do: true
  defp maybe_subs(s, [], acc), do: acc == s

  defp maybe_subs([s_head | s_tail], [t_head | t_tail], acc) do
    case s_head == t_head do
      true -> maybe_subs(s_tail, t_tail, acc <> s_head)
      false -> maybe_subs([s_head | s_tail], t_tail, acc)
    end
  end
end
