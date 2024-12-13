package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
)

type coord struct {
	x int
	y int
}

type machine struct {
	a     coord
	b     coord
	prize coord
}

func parse() []machine {
	scanner := bufio.NewScanner(os.Stdin)
	var ms []machine
	for scanner.Scan() {
		var m machine
		fmt.Sscanf(scanner.Text(), "Button A: X+%d, Y+%d", &m.a.x, &m.a.y)
		scanner.Scan()
		fmt.Sscanf(scanner.Text(), "Button B: X+%d, Y+%d", &m.b.x, &m.b.y)
		scanner.Scan()
		fmt.Sscanf(scanner.Text(), "Prize: X=%d, Y=%d", &m.prize.x, &m.prize.y)
		ms = append(ms, m)
		scanner.Scan()
	}
	return ms
}

// |ax bx| |n| = |px|
// |ay by| |m|   |py|
//
// |n| = 1/det | by -bx| |px|
// |m|         |-ay  ax| |py|
//
// det = ax by - bx ay

func (m machine) tokens() (int, error) {
	det := m.a.x*m.b.y - m.b.x*m.a.y
	x1 := (m.b.y*m.prize.x - m.b.x*m.prize.y)
	x2 := (-m.a.y*m.prize.x + m.a.x*m.prize.y)
	switch {
	case det == 0:
		panic(fmt.Sprintf("Non invertible matrix for %v\n", m))
	case x1%det != 0 || x2%det != 0:
		return 0, errors.New("Non integer solution")
	default:
		return 3*(x1/det) + x2/det, nil
	}
}

func part1(ms []machine) int {
	ans := 0
	for _, m := range ms {
		n, err := m.tokens()
		if err == nil {
			ans += n
		}
	}
	return ans
}

func part2(ms []machine) int {
	ans := 0
	for _, m := range ms {
		m.prize.x += 10000000000000
		m.prize.y += 10000000000000
		n, err := m.tokens()
		if err == nil {
			ans += n
		}
	}
	return ans
}

func main() {
	ms := parse()

	fmt.Println("Part 1:", part1(ms))
	fmt.Println("Part 2:", part2(ms))
}
