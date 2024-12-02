package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse() [][]int {
	scanner := bufio.NewScanner(os.Stdin)

	var reports [][]int
	for scanner.Scan() {
		line := scanner.Text()
		fields := strings.Fields(line)
		var report []int
		for _, field := range fields {
			n, _ := strconv.Atoi(field)
			report = append(report, n)
		}
		reports = append(reports, report)
	}

	return reports
}

func safeReport(report []int) bool {
	increasing := report[0] < report[1]
	for i := 0; i < len(report)-1; i++ {
		var n1, n2 int = report[i], report[i+1]
		var diff int
		if increasing {
			diff = n2 - n1
		} else {
			diff = n1 - n2
		}
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

func part1(reports [][]int) int {
	ans := 0
	for _, report := range reports {
		if safeReport(report) {
			ans++
		}
	}
	return ans
}

func part2(reports [][]int) int {
	ans := 0
	for _, report := range reports {
		safe := safeReport(report)

		for i := 0; !safe && i < len(report); i++ {
			var report2 []int
			report2 = append(report2, report[:i]...)
			report2 = append(report2, report[i+1:]...)

			safe = safeReport(report2)
		}

		if safe {
			ans++
		}
	}
	return ans
}

func main() {
	reports := parse()

	fmt.Println("Part 1:", part1(reports))
	fmt.Println("Part 2:", part2(reports))
}
