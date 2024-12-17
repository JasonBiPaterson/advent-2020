package main

import (
	"bufio"
	"fmt"
	"os"
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
	output := []int{}
	for d.pointer < len(d.program) {
		d.step(&output)
	}

	strN := make([]string, len(output))
	for i, n := range output {
		strN[i] = strconv.Itoa(n)
	}
	return strings.Join(strN, ",")
}

func part2(d Device) int {
	return 0
}

func main() {
	ms := parse()

	fmt.Println("Part 1:", part1(ms))
	fmt.Println("Part 2:", part2(ms))
}
