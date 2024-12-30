package main

import (
	"bufio"
	"fmt"
	"os"
)

const width int = 5

type schematic [width]int

func main() {
	var keys []schematic = []schematic{}
	var locks []schematic = []schematic{}

	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		var isKey bool
		switch scanner.Text() {
		case "#####":
			isKey = true
		case ".....":
			isKey = false
		default:
			panic("OH NO")
		}
		scanner.Scan()
		s := schematic{}
		for i := 0; i < 5; i++ {
			line := scanner.Text()
			for j, r := range line {
				if r == '#' {
					s[j]++
				}
			}
			scanner.Scan()
		}
		scanner.Scan()

		if isKey {
			keys = append(keys, s)
		} else {
			locks = append(locks, s)
		}
	}

	ans := 0
	for _, k := range keys {
		for _, l := range locks {
			fits := true
			for i := 0; i < width; i++ {
				if k[i]+l[i] > 5 {
					fits = false
					break
				}
			}
			if fits {
				ans++
			}
		}
	}

	fmt.Println("Part 1:", ans)
}
