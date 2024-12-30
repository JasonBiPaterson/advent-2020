package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func parse() (map[string]int, [][4]string) {
	scanner := bufio.NewScanner(os.Stdin)
	gates := map[string]int{}
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		var v int
		k := line[:3]
		v, _ = strconv.Atoi(line[5:])
		gates[k] = v
	}

	conns := [][4]string{}
	for scanner.Scan() {
		line := scanner.Text()
		fs := strings.Fields(line)
		conns = append(conns, [4]string{fs[0], fs[1], fs[2], fs[4]})
	}
	return gates, conns
}

func part1(gates map[string]int, conns [][4]string) int {
	ans := 0
	for len(conns) > 0 {
		next := [][4]string{}
		for _, c := range conns {
			a, ok := gates[c[0]]
			if !ok {
				next = append(next, c)
				continue
			}
			b, ok := gates[c[2]]
			if !ok {
				next = append(next, c)
				continue
			}
			switch c[1] {
			case "AND":
				gates[c[3]] = a & b
			case "XOR":
				gates[c[3]] = a ^ b
			case "OR":
				gates[c[3]] = a | b
			}
		}
		conns = next
	}

	for s, n := range gates {
		if s[0] == 'z' {
			m, _ := strconv.Atoi(s[1:])
			ans += n << m
		}
	}
	return ans
}

func f(s string, m map[string][2]string, acc [][]string) [][]string {
	parents, ok := m[s]
	coll := [][]string{}
	for _, path := range acc {
		coll = append(coll, append(path, s))
	}
	if !ok {
		return coll
	}
	collCopy := make([][]string, len(coll))
	for i, v := range coll {
		collCopy[i] = slices.Clone(v)
	}
	return append(f(parents[0], m, coll), f(parents[1], m, collCopy)...)
}

func part2(conns [][4]string) int {
	ans := 0
	return ans
}

func main() {
	gates, conns := parse()

	fmt.Println("Part 1:", part1(gates, conns))
	fmt.Println("Part 2:", part2(conns))
}
