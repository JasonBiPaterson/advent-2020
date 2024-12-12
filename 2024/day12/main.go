package main

import (
	"bufio"
	"fmt"
	"maps"
	"os"
)

type coord struct {
	x int
	y int
}

type coordMap map[coord]struct{}

func (c coord) plus(v coord) coord {
	return (coord{c.x + v.x, c.y + v.y})
}

var edgeVectors = []coord{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func parse() map[coord]rune {
	scanner := bufio.NewScanner(os.Stdin)

	plots := map[coord]rune{}
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		for j, r := range line {
			plots[coord{i, j}] = r
		}
	}

	return plots
}

type region struct {
	plant rune
	areas coordMap
}

func (re region) area() int {
	return len(re.areas)
}

func (re region) perimeter() int {
	internalEdges := 0
	for c := range re.areas {
		for _, v := range edgeVectors {
			if _, ok := re.areas[c.plus(v)]; ok {
				internalEdges++
			}
		}
	}
	return 4*re.area() - internalEdges
}

func (re region) sides() int {
	corners := coordMap{}
	for c := range re.areas {
		for _, v := range []coord{
			{0, 0}, {1, 0}, {0, 1}, {1, 1},
		} {
			corners[c.plus(v)] = struct{}{}
		}
	}
	ans := 0
	for corner := range corners {
		count := 0
		for _, v := range []coord{
			{0, 0}, {-1, 0}, {0, -1}, {-1, -1},
		} {
			if _, ok := re.areas[corner.plus(v)]; ok {
				count++
			}
		}
		if count == 1 || count == 3 {
			ans++
		} else if count == 2 {
			_, ok1 := re.areas[corner.plus(coord{0, 0})]
			_, ok2 := re.areas[corner.plus(coord{-1, -1})]
			if ok1 == ok2 {
				ans += 2
			}
		}
	}
	return ans
}

func (re *region) expand(c coord, plots *map[coord]rune) {
	re.areas[c] = struct{}{}
	delete(*plots, c)
	for _, v := range edgeVectors {
		newC := c.plus(v)
		if _, ok := re.areas[newC]; ok {
			continue
		}
		if newR, ok := (*plots)[newC]; ok && re.plant == newR {
			re.expand(newC, plots)
		}
	}
}

func getRegions(plots map[coord]rune) []region {
	plots = maps.Clone(plots)
	res := []region{}
	for start := range plots {
		re := region{
			plots[start],
			coordMap{},
		}
		re.expand(start, &plots)
		res = append(res, re)
	}
	return res
}

func part1(res []region) int {
	ans := 0
	for _, re := range res {
		ans += re.area() * re.perimeter()
	}

	return ans
}

func part2(res []region) int {
	ans := 0
	for _, re := range res {
		ans += re.area() * re.sides()
	}
	return ans
}

func main() {
	plots := parse()
	res := getRegions(plots)

	fmt.Println("Part 1:", part1(res))
	fmt.Println("Part 2:", part2(res))
}
