defmodule Day03 do
  @moduledoc "Advent of Code, Day 3 2023"

  @spec part1(String.t()) :: integer()
  def part1(input) do
    lines = input |> String.trim() |> String.split("\n")

    lines
    |> Enum.with_index()
    |> Enum.map(fn {line, index} ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> Enum.map(&List.first(&1))
      |> Enum.filter(fn {start, length} ->
        Enum.slice(lines, max(0, index - 1)..(index + 1))
        |> Enum.any?(
          &Regex.run(~r/[^.\d]/, String.slice(&1, max(0, start - 1)..(start + length)))
        )
      end)
      |> Enum.map(fn {start, length} ->
        line |> String.slice(start, length) |> String.to_integer()
      end)
    end)
    |> Enum.concat()
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    lines = input |> String.trim() |> String.split("\n")

    lines
    |> Enum.with_index()
    |> Enum.map(fn {line, index} ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> Enum.map(&List.first(&1))
      |> Enum.map(fn {start, length} ->
        Enum.slice(lines, max(0, index - 1)..(index + 1))
        |> Enum.with_index()
        |> Enum.map(fn {slice_line, slice_index} ->
          {slice_index,
           Regex.run(~r/[*]/, String.slice(slice_line, max(0, start - 1)..(start + length)),
             return: :index
           )}
        end)
        |> Enum.filter(fn {_, x} -> x end)
        |> Enum.map(fn {x, [{y, _}]} ->
          {
            index + x - if(index == 0, do: 0, else: 1),
            y + max(0, start - 1),
            line |> String.slice(start, length) |> String.to_integer()
          }
        end)
      end)
      |> Enum.concat()
    end)
    |> Enum.concat()
    |> Enum.group_by(fn {x, y, _} -> {x, y} end)
    |> Map.values()
    |> Enum.filter(fn coll -> Enum.count(coll) >= 2 end)
    |> Enum.map(fn coll -> Enum.reduce(coll, 1, fn {_, _, n}, acc -> n * acc end) end)
    |> Enum.sum()
  end

  def answers() do
    input = File.read!("priv/day03.txt")
    {part1(input), part2(input)}
  end
end
