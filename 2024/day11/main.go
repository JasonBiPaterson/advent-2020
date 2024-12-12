package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse() map[int]int {
	scanner := bufio.NewScanner(os.Stdin)

	var stones []int
	scanner.Scan()
	line := scanner.Text()
	for _, field := range strings.Fields(line) {
		n, _ := strconv.Atoi(field)
		stones = append(stones, n)
	}

	stoneCount := map[int]int{}
	for _, stone := range stones {
		stoneCount[stone]++
	}

	return stoneCount
}

func blink(stoneCount *map[int]int) {
	next := map[int]int{}
	for stone, n := range *stoneCount {
		if stone == 0 {
			next[1] += n
		} else if s := strconv.Itoa(stone); len(s)%2 == 0 {
			i := len(s) / 2
			stone1, _ := strconv.Atoi(s[:i])
			next[stone1] += n
			stone2, _ := strconv.Atoi(s[i:])
			next[stone2] += n
		} else {
			next[stone*2024] += n
		}
	}
	*stoneCount = next
}

func blinkN(stoneCount map[int]int, n int) int {
	ans := 0
	for i := 0; i < n; i++ {
		blink(&stoneCount)
	}
	for _, v := range stoneCount {
		ans += v
	}
	return ans
}

func part1(stoneCount map[int]int) int {
	return blinkN(stoneCount, 25)
}

func part2(stoneCount map[int]int) int {
	return blinkN(stoneCount, 75)
}

func main() {
	stoneCount := parse()

	fmt.Println("Part 1:", part1(stoneCount))
	fmt.Println("Part 2:", part2(stoneCount))
}
