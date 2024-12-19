package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func parse() ([]string, []string) {
	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()
	line := scanner.Text()
	patterns := strings.Split(line, ", ")

	scanner.Scan()
	designs := []string{}
	for scanner.Scan() {
		designs = append(designs, scanner.Text())
	}
	return patterns, designs
}

var cache = map[string]int{}
func getWays(s string, ps []string) int {
	if n, ok := cache[s]; ok {
		return n
	}
	for _, p := range ps {
		if len(s) < len(p) {
			continue
		}
		if s[:len(p)] == p {
			if len(s) == len(p) {
				cache[s] += 1
			} else {
				cache[s] += getWays(s[len(p):], ps)
			}
		}
	}
	return cache[s]
}

func main() {
	patterns, designs := parse()

	part1 := 0
	part2 := 0
	for _, design := range designs {
		n := getWays(design, patterns)
		part1 += min(n, 1)
		part2 += n
	}

	fmt.Println("Part 1:", part1)
	fmt.Println("Part 2:", part2)
}
