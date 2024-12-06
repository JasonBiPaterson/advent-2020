package main

import (
	"bufio"
	"fmt"
	"maps"
	"os"
	"slices"
)

type coord struct {
	x int
	y int
}

func (c coord) add(v coord) coord {
	return coord{c.x + v.x, c.y + v.y}
}

var (
	up    = coord{-1, 0}
	down  = coord{1, 0}
	left  = coord{0, -1}
	right = coord{0, 1}
)

func (direction coord) turned() coord {
	switch direction {
	case up:
		return right
	case right:
		return down
	case down:
		return left
	case left:
		return up
	}
	panic("Use a direction")
}

func parse() (coord, coord, []coord) {
	scanner := bufio.NewScanner(os.Stdin)

	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	var obstructions []coord
	var position coord
	for i, line := range lines {
		for j, r := range line {
			switch r {
			case '#':
				obstructions = append(obstructions, coord{i, j})
			case '^':
				position = coord{i, j}
			}
		}
	}

	var maxPos = coord{len(lines) - 1, len(lines[0]) - 1}

	return position, maxPos, obstructions
}

type state struct {
	position  coord
	direction coord
}

func (s state) bounded(bound coord) bool {
	c := s.position
	return c.x >= 0 && c.y >= 0 && c.x <= bound.x && c.y <= bound.y
}

func (s *state) step(obstructions []coord) {
	posStep := s.position.add(s.direction)
	if slices.Contains(obstructions, posStep) {
		s.direction = s.direction.turned()
	} else {
		s.position = posStep
	}
}

func part1(position coord, maxPos coord, obstructions []coord) int {
	visited := map[coord]struct{}{}
	for s := (state{position, up}); s.bounded(maxPos); s.step(obstructions) {
		visited[s.position] = struct{}{}
	}
	return len(visited)
}

func part2(position coord, maxPos coord, obstructions []coord) int {
	ans := 0

	states := map[state]bool{}
	for s := (state{position, up}); s.bounded(maxPos); s.step(obstructions) {
		states[s] = true

		newObstruction := s.position.add(s.direction)
		canObstruct := true
		if slices.Contains(obstructions, newObstruction) {
			canObstruct = false
		}
		for x := range maps.Keys(states) {
			if x.position == newObstruction {
				canObstruct = false
				break
			}
		}

		if canObstruct {
			obstructions2 := append(obstructions, newObstruction)
			states2 := maps.Clone(states)
			for s2 := (state{s.position, s.direction.turned()}); s2.bounded(maxPos); s2.step(obstructions2) {
				if states2[s2] {
					ans++
					break
				}
				states2[s2] = true
			}
		}
	}

	return ans
}

func main() {
	position, maxPos, obstructions := parse()

	fmt.Println("Part 1:", part1(position, maxPos, obstructions))
	fmt.Println("Part 2:", part2(position, maxPos, obstructions))
}
