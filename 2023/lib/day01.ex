defmodule Day01 do
  @moduledoc "Advent of Code, Day 1 2023"

  @spec part1(String.t()) :: integer()
  def part1(input) do
    re = ~r/\d/

    input
    |> String.trim()
    |> String.split()
    |> Enum.map(fn line ->
      captures = Regex.scan(re, line)
      a = captures |> List.first() |> List.first() |> String.to_integer() |> Kernel.*(10)
      b = captures |> List.last() |> List.first() |> String.to_integer()
      a + b
    end)
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    re = ~r/one|two|three|four|five|six|seven|eight|nine|\d/

    parse = fn [capture] ->
      case capture do
        "one" -> 1
        "two" -> 2
        "three" -> 3
        "four" -> 4
        "five" -> 5
        "six" -> 6
        "seven" -> 7
        "eight" -> 8
        "nine" -> 9
        _ -> String.to_integer(capture)
      end
    end

    re_rev = ~r/eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d/

    parse_rev = fn [capture] ->
      case capture do
        "eno" -> 1
        "owt" -> 2
        "eerht" -> 3
        "ruof" -> 4
        "evif" -> 5
        "xis" -> 6
        "neves" -> 7
        "thgie" -> 8
        "enin" -> 9
        _ -> String.to_integer(capture)
      end
    end

    input
    |> String.trim()
    |> String.split()
    |> Enum.map(fn line ->
      a = Regex.run(re, line) |> parse.() |> Kernel.*(10)
      b = Regex.run(re_rev, String.reverse(line)) |> parse_rev.()
      a + b
    end)
    |> Enum.sum()
  end

  def answers() do
    input = File.read!("priv/day01.txt")
    {part1(input), part2(input)}
  end
end
