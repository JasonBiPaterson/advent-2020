defmodule Day04 do
  @moduledoc "Advent of Code, Day 4 2023"

  require NimbleParsec
  alias NimbleParsec, as: P

  ignore_spaces = P.ignore(P.times(P.string(" "), min: 1))

  parse_id =
    P.ignore(P.string("Card"))
    |> P.concat(ignore_spaces)
    |> P.integer(min: 1)
    |> P.unwrap_and_tag(:id)
    |> P.ignore(P.string(":"))
    |> P.concat(ignore_spaces)

  parse_win_nums =
    P.times(
      P.choice([
        P.integer(min: 1),
        ignore_spaces
      ]),
      min: 1
    )
    |> P.tag(:win_nums)
    |> P.ignore(P.string("|"))
    |> P.concat(ignore_spaces)

  parse_draw_nums =
    P.times(
      P.choice([
        P.integer(min: 1),
        ignore_spaces
      ]),
      min: 1
    )
    |> P.tag(:draw_nums)
    |> P.ignore(
      P.choice([
        P.string("\n"),
        P.eos()
      ])
    )

  P.defparsec(
    :parse,
    parse_id
    |> P.concat(parse_win_nums)
    |> P.concat(parse_draw_nums)
    |> P.tag(:card)
    |> P.times(min: 1)
  )

  @spec part1(String.t()) :: integer()
  def part1(input) do
    {:ok, cards, _rest, _context, _line, _column} = parse(input)

    cards
    |> Enum.map(fn {:card,
                    [
                      id: _id,
                      win_nums: win_nums,
                      draw_nums: draw_nums
                    ]} ->
      win_count =
        MapSet.intersection(MapSet.new(win_nums), MapSet.new(draw_nums))
        |> Enum.count()

      if win_count > 0 do
        Integer.pow(2, win_count - 1)
      else
        0
      end
    end)
    |> Enum.sum()
  end

  defp part2_step([{count, card} | tail], acc) do
    {:card, [id: _id, win_nums: win_nums, draw_nums: draw_nums]} = card

    win_count =
      MapSet.intersection(MapSet.new(win_nums), MapSet.new(draw_nums))
      |> Enum.count()

    part2_step(
      Enum.concat(
        tail
        |> Enum.take(win_count)
        |> Enum.map(fn {n, c} -> {n + count, c} end),
        tail
        |> Enum.drop(win_count)
      ),
      acc + count
    )
  end

  defp part2_step([], acc), do: acc

  @spec part2(String.t()) :: integer()
  def part2(input) do
    {:ok, cards, _rest, _context, _line, _column} = parse(input)
    part2_step(Enum.map(cards, fn card -> {1, card} end), 0)
  end

  def answers() do
    input = File.read!("priv/day04.txt")
    {part1(input), part2(input)}
  end
end
