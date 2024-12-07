package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type equation struct {
	testValue int
	nums      []int
}

func parse() []equation {
	scanner := bufio.NewScanner(os.Stdin)

	es := []equation{}
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.SplitN(line, ": ", 2)
		testValue, _ := strconv.Atoi(split[0])
		nums := []int{}
		for _, s := range strings.Fields(split[1]) {
			num, _ := strconv.Atoi(s)
			nums = append(nums, num)
		}
		es = append(es, equation{testValue, nums})
	}

	return es
}

func (e equation) hasOps1() bool {
	if len(e.nums) == 1 {
		if e.testValue == e.nums[0] {
			return true
		}
		return false
	}
	nums := e.nums
	testValue := e.testValue
	last := nums[len(nums)-1]
	butLast := nums[:len(nums)-1]
	if testValue < last {
		return false
	}
	// Multiply
	if testValue%last == 0 && (equation{testValue / last, butLast}).hasOps1() {
		return true
	}
	// Add
	if (equation{testValue - last, butLast}).hasOps1() {
		return true
	}
	return false
}

func part1(es []equation) int {
	ans := 0
	for _, e := range es {
		if e.hasOps1() {
			ans += e.testValue
		}
	}
	return ans
}

func (e equation) hasOps2() bool {
	if len(e.nums) == 1 {
		if e.testValue == e.nums[0] {
			return true
		}
		return false
	}
	nums := e.nums
	testValue := e.testValue
	last := nums[len(nums)-1]
	butLast := nums[:len(nums)-1]
	if testValue < last {
		return false
	}
	// Multiply
	if testValue%last == 0 && (equation{testValue / last, butLast}).hasOps2() {
		return true
	}
	// Add
	if (equation{testValue - last, butLast}).hasOps2() {
		return true
	}
	// Concat
	{
		testValueStr := strconv.Itoa(testValue)
		lastStr := strconv.Itoa(last)
		idx := len(testValueStr) - len(lastStr)
		if idx > 0 && testValueStr[idx:] == lastStr {
			n, _ := strconv.Atoi(testValueStr[:idx])
			if (equation{n, butLast}).hasOps2() {
				return true
			}
		}
	}

	return false
}

func part2(es []equation) int {
	ans := 0
	for _, e := range es {
		if e.hasOps2() {
			ans += e.testValue
		}
	}
	return ans
}

func main() {
	es := parse()

	fmt.Println("Part 1:", part1(es))
	fmt.Println("Part 2:", part2(es))
}
