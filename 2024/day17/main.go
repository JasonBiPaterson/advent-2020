package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Device struct {
	a, b, c int
	pointer int
	program []int
}

func (d Device) combo(op int) int {
	switch op {
	case 0, 1, 2, 3:
		return op
	case 4:
		return d.a
	case 5:
		return d.b
	case 6:
		return d.c
	}
	panic("INVALID PROGRAM")
}

func (d *Device) step(output *[]int) {
	opcode := d.program[d.pointer]
	op := d.program[d.pointer+1]
	switch opcode {
	case 0:
		d.a >>= d.combo(op)
	case 1:
		d.b ^= op
	case 2:
		d.b = d.combo(op) & 7
	case 3:
		if d.a != 0 {
			d.pointer = op
			return
		}
	case 4:
		d.b ^= d.c
	case 5:
		*output = append(*output, d.combo(op)&7)
	case 6:
		d.b = d.a >> d.combo(op)
	case 7:
		d.c = d.a >> d.combo(op)
	}
	d.pointer += 2
}

func (d Device) run() []int {
	output := []int{}
	for d.pointer < len(d.program) {
		d.step(&output)
	}
	return output
}

func parse() Device {
	scanner := bufio.NewScanner(os.Stdin)
	d := Device{}
	scanner.Scan()
	fmt.Sscanf(scanner.Text(), "Register A: %d", &d.a)
	scanner.Scan()
	fmt.Sscanf(scanner.Text(), "Register B: %d", &d.b)
	scanner.Scan()
	fmt.Sscanf(scanner.Text(), "Register C: %d", &d.c)
	scanner.Scan()
	scanner.Scan()
	{
		var s string
		fmt.Sscanf(scanner.Text(), "Program: %s", &s)
		for _, v := range strings.Split(s, ",") {
			n, _ := strconv.Atoi(v)
			d.program = append(d.program, n)
		}
	}
	return d
}

func part1(d Device) string {
	output := d.run()
	strN := make([]string, len(output))
	for i, n := range output {
		strN[i] = strconv.Itoa(n)
	}
	return strings.Join(strN, ",")
}

// Don't ask me to prove this
// Saw a pattern, assumed it worked
func part2(d Device) int {
	ns := []int{0}
	for i, _ := range slices.Backward(d.program) {
		ms := []int{}
		for _, n := range ns {
			for j := 0; j <= 0b111; j++ {
				m := n<<3 + j
				dj := d
				dj.a = m
				output := dj.run()
				if slices.Equal(d.program[i:], output) {
					ms = append(ms, m)
				}
			}
		}
		ns = ms
	}
	return slices.Min(ns)
}

func main() {
	ms := parse()

	fmt.Println("Part 1:", part1(ms))
	fmt.Println("Part 2:", part2(ms))
}
