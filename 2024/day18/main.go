package main

import (
	"bufio"
	"fmt"
	"os"
)

type Coord struct {
	x int
	y int
}

func (c Coord) plus(v Coord) Coord {
	return Coord{c.x + v.x, c.y + v.y}
}

func bfs(corrupted map[Coord]struct{}, maxC Coord) map[Coord]Coord {
	q := []Coord{{0, 0}}
	explored := map[Coord]struct{}{{0, 0}: {}}
	parents := map[Coord]Coord{}
	for len(q) > 0 {
		v := q[0]
		q = q[1:]
		if v == maxC {
			break
		}
		for _, vector := range []Coord{
			{-1, 0}, {1, 0}, {0, -1}, {0, 1},
		} {
			w := v.plus(vector)
			if w.x < 0 || w.y < 0 || w.x > maxC.x || w.y > maxC.y {
				continue
			}
			if _, ok := corrupted[w]; ok {
				continue
			}
			if _, ok := explored[w]; ok {
				continue
			}
			explored[w] = struct{}{}
			parents[w] = v
			q = append(q, w)
		}
	}
	return parents
}

func parse() ([]Coord, Coord) {
	scanner := bufio.NewScanner(os.Stdin)
	var cs []Coord
	var maxC Coord
	for scanner.Scan() {
		var c Coord
		fmt.Sscanf(scanner.Text(), "%d,%d", &c.x, &c.y)
		maxC = Coord{max(c.x, maxC.x), max(c.y, maxC.y)}
		cs = append(cs, c)
	}
	return cs, maxC
}

func part1(cs []Coord, maxC Coord) int {
	corrupted := map[Coord]struct{}{}
	var bs int
	switch maxC.x {
	case 6:
		bs = 12
	case 70:
		bs = 1024
	}
	for i := 0; i < bs; i++ {
		corrupted[cs[i]] = struct{}{}
	}

	parents := bfs(corrupted, maxC)

	ans := 0
	for c := maxC; ; {
		if parent, ok := parents[c]; ok {
			ans++
			c = parent
		} else {
			break
		}
	}
	return ans
}

func part2(cs []Coord, maxC Coord) string {
	corrupted := map[Coord]struct{}{}
	pathCs := map[Coord]struct{}{}
	for _, c := range cs {
		if len(pathCs) != 0 {
			if _, ok := pathCs[c]; !ok {
				continue
			}
		}

		corrupted[c] = struct{}{}
		parents := bfs(corrupted, maxC)
		if _, ok := parents[maxC]; !ok {
			return fmt.Sprintf("%d,%d", c.x, c.y)
		}
	}
	return ""
}

func main() {
	cs, maxC := parse()

	fmt.Println("Part 1:", part1(cs, maxC))
	fmt.Println("Part 2:", part2(cs, maxC))
}
