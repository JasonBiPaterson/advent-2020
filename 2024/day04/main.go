package main

import (
	"bufio"
	"fmt"
	"os"
)

func parse() [][]rune {
	scanner := bufio.NewScanner(os.Stdin)

	var grid [][]rune
	for scanner.Scan() {
		line := scanner.Text()
		x := []rune(line)
		grid = append(grid, x)
	}

	return grid
}

func part1(grid [][]rune) int {
	var vectors [][]int
	for k := -1; k <= 1; k++ {
		for l := -1; l <= 1; l++ {
			if k != 0 || l != 0 {
				vectors = append(vectors, []int{k, l})
			}
		}
	}

	ans := 0
	iMax := len(grid) - 1
	jMax := len(grid[0]) - 1
	for i := 0; i <= iMax; i++ {
		for j := 0; j <= jMax; j++ {
			for _, vector := range vectors {
				k, l := vector[0], vector[1]
				if i+3*k < 0 || i+3*k > iMax || j+3*l < 0 || j+3*l > jMax {
					continue
				}
				r1, r2, r3, r4 := grid[i][j], grid[i+k][j+l], grid[i+(2*k)][j+(2*l)], grid[i+(3*k)][j+(3*l)]
				if r1 == 'X' && r2 == 'M' && r3 == 'A' && r4 == 'S' {
					ans++
				}
			}
		}
	}
	return ans
}

func part2(grid [][]rune) int {
	ans := 0
	for i := 1; i < len(grid)-1; i++ {
		for j := 1; j < len(grid[0])-1; j++ {
			c, tl, tr, bl, br := grid[i][j], grid[i-1][j-1], grid[i-1][j+1], grid[i+1][j-1], grid[i+1][j+1]
			if c == 'A' &&
				((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')) &&
				((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M')) {
				ans++
			}
		}
	}
	return ans
}

func main() {
	grid := parse()

	fmt.Println("Part 1:", part1(grid))
	fmt.Println("Part 2:", part2(grid))
}
