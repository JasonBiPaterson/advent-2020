package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func parse() string {
	scanner := bufio.NewScanner(os.Stdin)
	s := ""
	for scanner.Scan() {
		s += scanner.Text()
	}
	return s
}

func part1(s string) int {
	ans := 0
	r := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	matches := r.FindAllStringSubmatch(s, -1)
	for _, match := range matches {
		n, _ := strconv.Atoi(match[1])
		m, _ := strconv.Atoi(match[2])
		ans += n * m
	}
	return ans
}

func part2(s string) int {
	ans := 0
	r := regexp.MustCompile(`mul\((\d+),(\d+)\)|do\(\)|don't\(\)`)
	matches := r.FindAllStringSubmatch(s, -1)
	enabled := true
	for _, match := range matches {
		if match[0] == "do()" {
			enabled = true
		} else if match[0] == "don't()" {
			enabled = false
		} else if enabled {
			n, _ := strconv.Atoi(match[1])
			m, _ := strconv.Atoi(match[2])
			ans += n * m
		}
	}
	return ans
}

func main() {
	s := parse()
	fmt.Println("Part 1:", part1(s))
	fmt.Println("Part 2:", part2(s))
}
