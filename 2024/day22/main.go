package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
)

type coord struct {
	x int
	y int
}

func parse() []int {
	scanner := bufio.NewScanner(os.Stdin)
	var ns []int
	for scanner.Scan() {
		n, _ := strconv.Atoi(scanner.Text())
		ns = append(ns, n)
	}
	return ns
}

func nextSecret(n int) int {
	div := 1<<24 - 1

	next := n << 6
	next ^= n
	next &= div
	n = next
	next >>= 5
	next ^= n
	next &= div
	n = next
	next <<= 11
	next ^= n
	next &= div
	n = next
	return next
}

func part1(ns []int) int {
	ans := 0
	for _, n := range ns {
		for i := 0; i < 2000; i++ {
			n = nextSecret(n)
		}
		ans += n
	}
	return ans
}

// Slowwwwww
func part2(ns []int) int {
	allPrices := [][]int{}
	allChanges := [][]int{}
	for _, n := range ns {
		prices := []int{n % 10}
		changes := []int{}
		for i := 0; i < 2000; i++ {
			next := nextSecret(n)
			prices = append(prices, next%10)
			changes = append(changes, next%10-n%10)
			n = next
		}
		allPrices = append(allPrices, prices)
		allChanges = append(allChanges, changes)
	}

	asdf := [][]int{}
	for a := -8; a < 9; a++ {
		for b := -8; b < 9; b++ {
			if a+b <= -10 || a+b >= 9 {
				continue
			}
			for c := -8; c < 9; c++ {
				if a+b+c <= -10 || a+b+c >= 9 || b+c <= -10 || b+c >= 9 {
					continue
				}
				for d := 1; d < 10; d++ {
					if a+b+c+d > 0 && b+c+d > 0 && c+d > 0 &&
						a+b+c+d < 10 && b+c+d < 10 && c+d < 10 {
						asdf = append(asdf, []int{a, b, c, d})
					}
				}
			}
		}
	}

	ans := 0
	for _, sequence1 := range asdf {
		tmp := 0
		for i := range allChanges {
			changes := allChanges[i]
			for j := 0; j < len(changes)-3; j++ {
				sequence2 := changes[j : j+4]
				if slices.Equal(sequence1, sequence2) {
					tmp += allPrices[i][j+4]
					break
				}
			}
		}
		ans = max(ans, tmp)
	}
	return ans
}

func main() {
	ms := parse()

	fmt.Println("Part 1:", part1(ms))
	fmt.Println("Part 2:", part2(ms))
}
