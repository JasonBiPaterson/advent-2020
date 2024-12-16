package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

type Coord struct {
	x int
	y int
}

func (c Coord) plus(v Coord) Coord {
	return Coord{c.x + v.x, c.y + v.y}
}

type Maze struct {
	start Coord
	end   Coord
	path  map[Coord]struct{}
}

type State struct {
	pos    Coord
	facing Coord
}

var (
	north = Coord{-1, 0}
	east  = Coord{0, 1}
	south = Coord{1, 0}
	west  = Coord{0, -1}
)

func (c Coord) getStates() []State {
	s := []State{}
	for _, v := range []Coord{north, east, south, west} {
		s = append(s, State{c, v})
	}
	return s
}

type Edge struct {
	next State
	dist int
}

func (s State) getEdges(m *Maze) []Edge {
	es := []Edge{}
	f := func(v Coord, n int) {
		step := s.pos.plus(v)
		if step == m.end {
			es = append(es, Edge{State{step, Coord{}}, n})
		}
		if _, ok := m.path[step]; ok {
			es = append(es, Edge{State{step, v}, n})
		}
	}

	switch s.facing {
	case Coord{}:
		return []Edge{}
	case north, south:
		f(east, 1001)
		f(west, 1001)
	case east, west:
		f(north, 1001)
		f(south, 1001)
	}
	f(s.facing, 1)
	return es
}

func parse() Maze {
	scanner := bufio.NewScanner(os.Stdin)
	m := Maze{
		path: map[Coord]struct{}{},
	}
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		for j, r := range line {
			switch r {
			case 'S':
				m.start = Coord{i, j}
			case 'E':
				m.end = Coord{i, j}
			case '.':
				m.path[Coord{i, j}] = struct{}{}
			}
		}
	}
	return m
}

func countPrev(s State, positions *map[Coord]struct{}, prev map[State][]State) {
	(*positions)[s.pos] = struct{}{}
	for _, v := range prev[s] {
		countPrev(v, positions, prev)
	}
}

func main() {
	m := parse()

	startState := State{m.start, east}
	endState := State{m.end, Coord{}}

	dist := map[State]int{}
	dist[startState] = 0
	prev := map[State][]State{}
	visited := map[State]struct{}{}

	// World's worst Dijkstra implementation. Do not read
	for {
		var u State
		uDist := math.MaxInt
		for v, n := range dist {
			if _, ok := visited[v]; !ok && n < uDist {
				u = v
				uDist = n
			}
		}
		if uDist == math.MaxInt {
			break
		}
		visited[u] = struct{}{}

		for _, e := range u.getEdges(&m) {
			v := e.next
			if _, ok := visited[v]; ok {
				continue
			}

			alt := uDist + e.dist
			if distV, ok := dist[v]; !ok || alt <= distV {
				dist[v] = alt
				if alt == distV {
					prev[v] = append(prev[v], u)
				} else {
					prev[v] = []State{u}
				}
			}
		}
	}

	positions := map[Coord]struct{}{}
	countPrev(endState, &positions, prev)

	fmt.Println("Part 1:", dist[endState])
	fmt.Println("Part 2:", len(positions))
}
