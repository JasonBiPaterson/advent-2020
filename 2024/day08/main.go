package main

import (
	"bufio"
	"fmt"
	"os"
)

type coord struct {
	x int
	y int
}

func (c1 coord) getAntinode(c2 coord) coord {
	return coord{2*c1.x - c2.x, 2*c1.y - c2.y}
}

func (c coord) bounded(bound coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x <= bound.x && c.y <= bound.y
}

func parse() (map[rune][]coord, coord) {
	scanner := bufio.NewScanner(os.Stdin)
	aPos := map[rune][]coord{}
	bound := coord{}
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		for j, r := range line {
			if r != '.' {
				aPos[r] = append(aPos[r], coord{i, j})
			}
			bound.y = i
		}
		bound.x = i
	}
	return aPos, bound
}

func part1(aPos map[rune][]coord, bound coord) int {
	antinodes := map[coord]struct{}{}
	for _, cs := range aPos {
		for _, c1 := range cs {
			for _, c2 := range cs {
				if c1 != c2 {
					if n := c1.getAntinode(c2); n.bounded(bound) {
						antinodes[n] = struct{}{}
					}
					if n := c2.getAntinode(c1); n.bounded(bound) {
						antinodes[n] = struct{}{}
					}
				}
			}
		}
	}

	return len(antinodes)
}

func part2(aPos map[rune][]coord, bound coord) int {
	antinodes := map[coord]struct{}{}
	for _, cs := range aPos {
		for _, c1 := range cs {
			for _, c2 := range cs {
				if c1 != c2 {
					for a, b := c1, c2; b.bounded(bound); a, b = b, b.getAntinode(a) {
						antinodes[b] = struct{}{}
					}
					for a, b := c1, c2; a.bounded(bound); a, b = a.getAntinode(b), a {
						antinodes[b] = struct{}{}
					}
				}
			}
		}
	}
	return len(antinodes)
}

func main() {
	aPos, bound := parse()

	fmt.Println("Part 1:", part1(aPos, bound))
	fmt.Println("Part 2:", part2(aPos, bound))
}
