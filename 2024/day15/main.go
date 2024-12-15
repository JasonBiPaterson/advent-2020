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

type Grid struct {
	walls map[Coord]struct{}
	boxes map[Coord]struct{}
	robot Coord
}

func (state Grid) print() {
	maxPos := Coord{0, 0}
	for c := range state.walls {
		if c.x >= maxPos.x && c.y >= maxPos.y {
			maxPos = c
		}
	}

	for i := 0; i <= maxPos.x; i++ {
		for j := 0; j <= maxPos.y; j++ {
			c := Coord{i, j}
			if _, ok := state.walls[c]; ok {
				fmt.Print("#")
			} else if _, ok := state.boxes[c]; ok {
				fmt.Print("O")
			} else if c == state.robot {
				fmt.Print("@")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func (state *Grid) move(v Coord) bool {
	rPosNew := state.robot.plus(v)
	boxesMoved := []Coord{}
	for c := rPosNew; true; c = c.plus(v) {
		if _, ok := state.walls[c]; ok {
			return false
		} else if _, ok := state.boxes[c]; ok {
			boxesMoved = append(boxesMoved, c)
		} else {
			break
		}
	}

	state.robot = rPosNew
	if len(boxesMoved) > 0 {
		delete(state.boxes, boxesMoved[0])
		state.boxes[boxesMoved[len(boxesMoved)-1].plus(v)] = struct{}{}
	}

	return true
}

func parse() (Grid, []Coord) {
	scanner := bufio.NewScanner(os.Stdin)
	var state = Grid{
		map[Coord]struct{}{},
		map[Coord]struct{}{},
		Coord{},
	}
	moves := []Coord{}

	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		if line == "" {
			break
		}
		for j, r := range line {
			switch r {
			case '#':
				state.walls[Coord{i, j}] = struct{}{}
			case 'O':
				state.boxes[Coord{i, j}] = struct{}{}
			case '@':
				state.robot = Coord{i, j}
			}
		}
	}
	for scanner.Scan() {
		line := scanner.Text()
		for _, r := range line {
			var v Coord
			switch r {
			case '<':
				v = Coord{0, -1}
			case '>':
				v = Coord{0, 1}
			case '^':
				v = Coord{-1, 0}
			case 'v':
				v = Coord{1, 0}
			}
			moves = append(moves, v)
		}
	}
	return state, moves
}

func part1(state Grid, moves []Coord) int {
	for _, v := range moves {
		state.move(v)
	}

	ans := 0
	for c := range state.boxes {
		ans += 100*c.x + c.y
	}
	return ans
}

type ExpandedGrid struct {
	walls map[Coord]struct{}
	boxes map[Coord]struct{}
	robot Coord
}

func (grid Grid) expand() ExpandedGrid {
	state := ExpandedGrid{
		map[Coord]struct{}{},
		map[Coord]struct{}{},
		Coord{grid.robot.x, 2 * grid.robot.y},
	}
	for c := range grid.walls {
		state.walls[Coord{c.x, 2 * c.y}] = struct{}{}
		state.walls[Coord{c.x, 2*c.y + 1}] = struct{}{}
	}
	for c := range grid.boxes {
		state.boxes[Coord{c.x, 2 * c.y}] = struct{}{}
	}
	return state
}

func (state ExpandedGrid) print() {
	maxPos := Coord{0, 0}
	for c := range state.walls {
		if c.x >= maxPos.x && c.y >= maxPos.y {
			maxPos = c
		}
	}

	for i := 0; i <= maxPos.x; i++ {
		for j := 0; j <= maxPos.y; j++ {
			c := Coord{i, j}
			if _, ok := state.walls[c]; ok {
				fmt.Print("#")
			} else if _, ok := state.boxes[c]; ok {
				fmt.Print("[]")
				j++
			} else if c == state.robot {
				fmt.Print("@")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func getVBoxes(state ExpandedGrid, c Coord, v Coord) ([]Coord, bool) {
	step := c.plus(v)
	if _, ok := state.walls[step]; ok {
		return nil, false
	} else if _, ok := state.boxes[step]; ok {
		boxes1, ok1 := getVBoxes(state, step, v)
		boxes2, ok2 := getVBoxes(state, step.plus(Coord{0, 1}), v)
		if ok1 && ok2 {
			newBoxes := append(boxes1, boxes2...)
			newBoxes = append(newBoxes, step)
			return newBoxes, true
		} else {
			return nil, false
		}
	} else if _, ok := state.boxes[step.plus(Coord{0, -1})]; ok {
		boxes1, ok1 := getVBoxes(state, step.plus(Coord{0, -1}), v)
		boxes2, ok2 := getVBoxes(state, step, v)
		if ok1 && ok2 {
			newBoxes := append(boxes1, boxes2...)
			newBoxes = append(newBoxes, step.plus(Coord{0, -1}))
			return newBoxes, true
		} else {
			return nil, false
		}
	}
	return []Coord{}, true
}

func (state *ExpandedGrid) move(v Coord) bool {
	rPosNew := state.robot.plus(v)
	if _, ok := state.walls[rPosNew]; ok {
		return false
	}
	boxesMoved := []Coord{}
	switch v {
	case Coord{0, -1}:
		for c := rPosNew; true; c = c.plus(v) {
			if _, ok := state.walls[c]; ok {
				return false
			} else if _, ok := state.boxes[c.plus(v)]; ok {
				c = c.plus(v)
				boxesMoved = append(boxesMoved, c)
			} else {
				break
			}
		}
	case Coord{0, 1}:
		for c := rPosNew; true; c = c.plus(v) {
			if _, ok := state.walls[c]; ok {
				return false
			} else if _, ok := state.boxes[c]; ok {
				boxesMoved = append(boxesMoved, c)
				c = c.plus(v)
			} else {
				break
			}
		}
	default:
		boxes, ok := getVBoxes(*state, state.robot, v)
		if ok {
			boxesMoved = boxes
		} else {
			return false
		}
	}
	for _, c := range boxesMoved {
		delete(state.boxes, c)
	}
	for _, c := range boxesMoved {
		state.boxes[c.plus(v)] = struct{}{}
	}

	state.robot = rPosNew
	return true
}

func part2(state ExpandedGrid, moves []Coord) int {
	for _, v := range moves {
		state.move(v)
	}

	ans := 0
	for c := range state.boxes {
		ans += 100*c.x + c.y
	}
	return ans
}

func main() {
	state1, moves := parse()
	state2 := state1.expand()

	fmt.Println("Part 1:", part1(state1, moves))
	fmt.Println("Part 2:", part2(state2, moves))
}
