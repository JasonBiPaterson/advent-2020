package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
)

func parse() map[string][]string {
	scanner := bufio.NewScanner(os.Stdin)
	m := map[string][]string{}
	for scanner.Scan() {
		line := scanner.Text()
		a := line[:2]
		b := line[3:]
		m[a] = append(m[a], b)
		m[b] = append(m[b], a)
	}
	return m
}

func part1(m map[string][]string) int {
	asdf := [][3]string{}
	for k1, v1 := range m {
		if k1[0] != 't' {
			continue
		}
		for _, k2 := range v1 {
			if k2 == k1 {
				continue
			}
			v2 := m[k2]
			for _, k3 := range v2 {
				if k3 == k2 {
					continue
				}
				v3 := m[k3]
				if slices.Contains(v3, k1) {
					good := true
					for _, cycle := range asdf {
						if slices.Contains(cycle[:], k1) && slices.Contains(cycle[:], k2) && slices.Contains(cycle[:], k3) {
							good = false
							break
						}
					}
					if good {
						asdf = append(asdf, [3]string{k1, k2, k3})
					}
				}
			}
		}
	}
	return len(asdf)
}

func union(lists [][]string) []string {
	tmp := lists[0]
	for _, list := range lists[1:] {
		tmp1 := []string{}
		for _, s := range list {
			if slices.Contains(tmp, s) {
				tmp1 = append(tmp1, s)
			}
		}
		tmp = tmp1
	}
	return tmp
}

func part2(m map[string][]string) string {
	regions := [][]string{}
	for k1, v1 := range m {
		for _, k2 := range v1 {
			if k1 == k2 {
				continue
			}
			coll := []string{k1, k2}
			slices.Sort(coll)
			if !slices.ContainsFunc(regions, func(e []string) bool {
				return slices.Equal(e, coll)
			}) {
				regions = append(regions, coll)
			}
		}
	}

	var found []string
	for len(regions) > 0 {
		newRegions := [][]string{}
		for _, region := range regions {
			vs := [][]string{}
			for _, k := range region {
				v := m[k]
				vs = append(vs, v)
			}
			newRegion := union(vs)
			if len(newRegion) == 0 {
				continue
			}
			newRegion = append(region, newRegion[0])
			slices.Sort(newRegion)

			if !slices.ContainsFunc(regions, func(e []string) bool {
				return slices.Equal(e, newRegion)
			}) {
				newRegions = append(newRegions, newRegion)
				if len(newRegion) > len(found) {
					found = newRegion
				}
			}
		}
		regions = newRegions
	}
	return strings.Join(found, ",")
}

func main() {
	ms := parse()

	fmt.Println("Part 1:", part1(ms))
	fmt.Println("Part 2:", part2(ms))
}
