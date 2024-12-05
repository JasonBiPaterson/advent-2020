package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func parse() (map[int][]int, [][]int) {
	scanner := bufio.NewScanner(os.Stdin)

	rules2 := make(map[int][]int)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		split := strings.Split(line, "|")
		a, _ := strconv.Atoi(split[0])
		b, _ := strconv.Atoi(split[1])
		m := rules2[a]
		rules2[a] = append(m, b)
	}
	var pages [][]int
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Split(line, ",")
		var update []int
		for _, s := range split {
			n, _ := strconv.Atoi(s)
			update = append(update, n)
		}
		pages = append(pages, update)
	}

	return rules2, pages
}

func main() {
	rules, pages := parse()

	part1, part2 := 0, 0
	for _, update := range pages {
		valid := true
		for i := 0; i < len(update)-1; i++ {
			n := update[i]
			m := update[i+1]
			if !slices.Contains(rules[n], m) {
				valid = false
				break
			}
		}
		if valid {
			part1 += update[len(update)/2]
		} else {
			slices.SortFunc(update, func(n, m int) int {
				if !slices.Contains(rules[n], m) {
					return 1
				}
				return -1
			})
			part2 += update[len(update)/2]
		}
	}

	fmt.Println("Part 1:", part1)
	fmt.Println("Part 2:", part2)
}
