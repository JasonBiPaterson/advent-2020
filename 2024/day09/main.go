package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
)

func parse() []int {
	scanner := bufio.NewScanner(os.Stdin)

	var a []int
	scanner.Scan()
	line := scanner.Text()
	for _, r := range line {
		n, _ := strconv.Atoi(string(r))
		a = append(a, n)
	}

	return a
}

func part1(diskMap []int) int {
	ans := 0

	disk := []int{}
	for i, v := range diskMap {
		var id int
		if i%2 == 0 {
			id = i / 2
		} else {
			id = -1
		}
		for j := 0; j < v; j++ {
			disk = append(disk, id)
		}
	}

	x := 0
	for i, id1 := range slices.Backward(disk) {
		if id1 >= 0 {
			for j := x; j < i; j++ {
				if id2 := disk[j]; id2 == -1 {
					disk[i], disk[j] = -1, disk[i]
					x = j
					break
				}
			}
		}
	}

	for i, n := range disk {
		if n < 0 {
			break
		}
		ans += i * n
	}

	return ans
}

type block struct {
	id  int
	len int
	pos int
}

func part2(diskMap []int) int {
	ans := 0

	files := []block{}
	spaces := []block{}

	blockPos := 0
	for i, v := range diskMap {
		if i%2 == 0 {
			files = append(files, block{i / 2, v, blockPos})
		} else {
			spaces = append(spaces, block{-1, v, blockPos})
		}
		if v > 0 {
			blockPos += v
		}
	}

	for _, file := range slices.Backward(files) {
		// Find space
		for j, gap := range spaces {
			if gap.pos > file.pos {
				break
			}
			if gap.len >= file.len {
				spaces[j] = block{-1, gap.len - file.len, gap.pos + file.len}
				file.pos = gap.pos
				break
			}
		}
		// Add checksum
		for i := file.pos; i < file.pos+file.len; i++ {
			ans += i * file.id
		}
	}

	return ans
}

func main() {
	diskMap := parse()

	fmt.Println("Part 1:", part1(diskMap))
	fmt.Println("Part 2:", part2(diskMap))
}
