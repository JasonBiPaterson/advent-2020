package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
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

func f(design string, p *[]string) bool {
	for _, pattern := range *p {
		if len(design) < len(pattern) {
			continue
		}
		if design[:len(pattern)] == pattern {
			if len(design) == len(pattern) {
				return true
			} else if f(design[len(pattern):], p) {
				return true
			}
		}
	}
	return false
}

func part1(patterns []string, designs []string) int {
	slices.SortFunc(patterns, func(a, b string) int { return len(a) - len(b) })

	reducedPatterns := []string{}
	for i, p := range slices.Backward(patterns) {
		a := patterns[:i]
		if !f(p, &a) {
			reducedPatterns = append(reducedPatterns, p)
		}
	}

	ans := 0
	for _, design := range designs {
		if f(design, &reducedPatterns) {
			ans++
		}
	}
	return ans
}

func part2(patterns []string, designs []string) int {
	ans := 0
	return ans
}

func main() {
	patterns, designs := parse()

	fmt.Println("Part 1:", part1(patterns, designs))
	fmt.Println("Part 2:", part2(patterns, designs))
}
