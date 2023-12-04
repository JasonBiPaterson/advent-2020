defmodule Day02 do
  @moduledoc "Advent of Code, Day 2 2023"

  require NimbleParsec
  alias NimbleParsec, as: P

  parse_cubes =
    P.integer(min: 1)
    |> P.ignore(P.string(" "))
    |> P.ascii_string([?a..?z], min: 1)
    |> P.tag(:cubes)

  parse_hand =
    P.times(
      P.choice([
        parse_cubes,
        P.ignore(P.string(", "))
      ]),
      min: 1
    )
    |> P.tag(:hand)

  parse_game =
    P.ignore(P.string("Game "))
    |> P.integer(min: 1)
    |> P.ignore(P.string(": "))
    |> P.times(
      parse_hand
      |> P.choice([
        P.ignore(P.string("; ")),
        P.ignore(P.string("\n")),
        P.eos()
      ]),
      min: 1
    )
    |> P.tag(:game)

  P.defparsec(:parse, P.times(parse_game, min: 1))

  @spec part1(String.t()) :: integer()
  def part1(input) do
    {:ok, games, _rest, _context, _line, _column} = parse(input)

    games
    |> Enum.map(fn {:game, game} ->
      id = List.first(game)

      is_valid =
        Enum.drop(game, 1)
        |> Enum.all?(fn {:hand, hand} ->
          Enum.all?(hand, fn {:cubes, cubes} ->
            case cubes do
              [n, "red"] -> n <= 12
              [n, "green"] -> n <= 13
              [n, "blue"] -> n <= 14
            end
          end)
        end)

      if(is_valid, do: id, else: 0)
    end)
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    {:ok, games, _rest, _context, _line, _column} = parse(input)

    games
    |> Enum.map(fn {:game, game} ->
      Enum.drop(game, 1)
      |> Enum.reduce(
        %{"red" => 0, "green" => 0, "blue" => 0},
        fn {:hand, hand}, hand_acc ->
          Enum.reduce(
            hand,
            hand_acc,
            fn {:cubes, [n, color]}, cube_acc ->
              %{cube_acc | color => max(cube_acc[color], n)}
            end
          )
        end
      )
      |> Map.values()
      |> Enum.product()
    end)
    |> Enum.sum()
  end

  def answers() do
    input = File.read!("priv/day02.txt")
    {part1(input), part2(input)}
  end
end
