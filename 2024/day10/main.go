package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type coord struct {
	x int
	y int
}

func (c coord) plus(v coord) coord {
	return (coord{c.x + v.x, c.y + v.y})
}

var vectors = []coord{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func parse() map[coord]int {
	scanner := bufio.NewScanner(os.Stdin)

	positions := map[coord]int{}
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		for j, r := range line {
			if r == '.' {
				continue
			}
			n, _ := strconv.Atoi(string(r))
			positions[coord{i, j}] = n
		}
	}

	return positions
}

func part1(positions map[coord]int) int {
	ans := 0
	for pos, nnn := range positions {
		if nnn != 0 {
			continue
		}
		acc := map[coord]struct{}{
			pos: {},
		}
		for i := 1; i <= 9; i++ {
			next := map[coord]struct{}{}
			for c1 := range acc {
				for _, v := range vectors {
					nextC := c1.plus(v)
					if n, ok := positions[nextC]; ok && n == i {
						next[nextC] = struct{}{}
					}
				}
			}
			acc = next
		}
		ans += len(acc)
	}
	return ans
}

func part2(positions map[coord]int) int {
	ans := 0
	for pos, nnn := range positions {
		if nnn != 0 {
			continue
		}
		acc := map[coord]int{
			pos: 1,
		}
		for i := 1; i <= 9; i++ {
			next := map[coord]int{}
			for c1, count := range acc {
				for _, v := range vectors {
					nextC := c1.plus(v)
					if n, ok := positions[nextC]; ok && n == i {
						next[nextC] += count
					}
				}
			}
			acc = next
		}
		for _, n := range acc {
			ans += n
		}
	}
	return ans
}

func main() {
	positions := parse()

	fmt.Println("Part 1:", part1(positions))
	fmt.Println("Part 2:", part2(positions))
}
