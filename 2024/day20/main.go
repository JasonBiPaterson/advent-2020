package main

import (
	"bufio"
	"fmt"
	"os"
)

type Coord struct {
	x, y int
}

func (c Coord) plus(v Coord) Coord {
	return Coord{c.x + v.x, c.y + v.y}
}

type Map struct {
	start, end Coord
	track      map[Coord]struct{}
}

func parse() Map {
	scanner := bufio.NewScanner(os.Stdin)
	m := Map{
		track: make(map[Coord]struct{}),
	}
	for i := 0; scanner.Scan(); i++ {
		for j, r := range scanner.Text() {
			switch r {
			case '.':
				m.track[Coord{i, j}] = struct{}{}
			case 'S':
				m.start = Coord{i, j}
			case 'E':
				m.end = Coord{i, j}
			}
		}
	}
	return m
}

func main() {
	m := parse()
	var vectors = [...]Coord{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	dist := make(map[Coord]int, len(m.track)+2)
	for c := m.start; true; {
		dist[c] = len(dist)
		if c == m.end {
			break
		}

		for _, v := range vectors {
			step := c.plus(v)
			if _, ok := dist[step]; ok {
				continue
			}
			if _, ok := m.track[step]; ok || step == m.end {
				c = step
				break
			}
		}
	}

	cheats := map[Coord]int{
		{0, 0}: 0,
	}
	for i := 0; i < 20; i++ {
		for c, n := range cheats {
			if n == i {
				for _, v := range vectors {
					step := c.plus(v)
					if _, ok := cheats[step]; !ok {
						cheats[step] = i + 1
					}
				}
			}
		}
	}
	for c, n := range cheats {
		if n <= 1 {
			delete(cheats, c)
		}
	}

	part1 := 0
	part2 := 0
	for c, n := range dist {
		for cheat, cheatN := range cheats {
			step := c.plus(cheat)
			if m, ok := dist[step]; ok {
				diff := m - n - cheatN
				if diff >= 100 {
					if cheatN == 2 {
						part1++
					}
					part2++
				}
			}
		}
	}

	fmt.Println("Part 1:", part1)
	fmt.Println("Part 2:", part2)
}
