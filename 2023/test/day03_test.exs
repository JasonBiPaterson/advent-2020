defmodule Day03Test do
  use ExUnit.Case
  doctest Day03

  test "Examples" do
    input = """
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    """

    assert Day03.part1(input) == 4361
    assert Day03.part2(input) == 467_835
  end
end
