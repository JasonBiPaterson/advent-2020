package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

type coord struct {
	x int
	y int
}

type robot struct {
	p coord
	v coord
}

var width = 101
var height = 103

func (r robot) teleport(n int) coord {
	return coord{
		((r.p.x+n*r.v.x)%width + width) % width,
		((r.p.y+n*r.v.y)%height + height) % height,
	}
}

func parse() []robot {
	scanner := bufio.NewScanner(os.Stdin)
	var rs []robot
	for scanner.Scan() {
		var r robot
		line := scanner.Text()
		fmt.Sscanf(line, "p=%d,%d v=%d,%d", &r.p.x, &r.p.y, &r.v.x, &r.v.y)
		rs = append(rs, r)
	}
	return rs
}

func part1(rs []robot) int {
	qs := [4]int{}
	for _, r := range rs {
		c := r.teleport(100)
		switch {
		case c.x < width/2 && c.y < height/2:
			qs[0]++
		case c.x > width/2 && c.y < height/2:
			qs[1]++
		case c.x < width/2 && c.y > height/2:
			qs[2]++
		case c.x > width/2 && c.y > height/2:
			qs[3]++
		}
	}
	ans := 1
	for _, n := range qs {
		ans *= n
	}
	return ans
}

func printGrid(cs map[coord]struct{}) {
	for i := 0; i < height; i += 2 {
		for j := 0; j < width; j++ {
			_, ok1 := cs[coord{j, i}]
			_, ok2 := cs[coord{j, i + 1}]
			switch {
			case ok1 && ok2:
				fmt.Print("█")
			case ok1:
				fmt.Print("▀")
			case ok2:
				fmt.Print("▄")
			default:
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func part2(rs []robot) int {
	for i := 1; true; i++ {
		cs := make(map[coord]struct{}, len(rs))
		neighbours := 0
		for _, r := range rs {
			c := r.teleport(i)
			cs[c] = struct{}{}
		}
		for c := range cs {
			for _, v := range []coord{
				{-1, 0}, {1, 0}, {0, -1}, {0, 1},
			} {
				if _, ok := cs[coord{c.x + v.x, c.y + v.y}]; ok {
					neighbours++
					break
				}
			}
		}

		if neighbours > len(cs)/2 {
			printGrid(cs)
			return i
		}
	}
	return 0
}

func main() {
	rs := parse()

	var isTest = flag.Bool("test", false, "TEST")
	flag.Parse()
	if *isTest {
		width = 11
		height = 7
	}

	fmt.Println("Part 1:", part1(rs))
	if !*isTest {
		fmt.Println("Part 2:", part2(rs))
	}
}
