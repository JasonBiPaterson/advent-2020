package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func parse() ([]int, []int) {
	scanner := bufio.NewScanner(os.Stdin)

	var a, b []int
	for scanner.Scan() {
		line := scanner.Text()
		fields := strings.Fields(line)
		field_a, _ := strconv.Atoi(fields[0])
		a = append(a, field_a)
		field_b, _ := strconv.Atoi(fields[1])
		b = append(b, field_b)
	}

	return a, b
}

func part1(a []int, b []int) int {
	a = slices.Clone(a)
	b = slices.Clone(b)
	slices.Sort(a)
	slices.Sort(b)

	ans := 0
	for i := 0; i < len(a); i++ {
		n, m := a[i], b[i]
		if n > m {
			ans += n - m
		} else {
			ans += m - n
		}
	}
	return ans
}

func part2(a []int, b []int) int {
	ans := 0
	for _, n := range a {
		count := 0
		for _, m := range b {
			if n == m {
				count++
			}
		}
		ans += n * count
	}
	return ans
}

func main() {
	a, b := parse()

	fmt.Println("Part 1:", part1(a, b))
	fmt.Println("Part 2:", part2(a, b))
}
