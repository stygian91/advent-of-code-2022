package main

import (
	"aoc2022/utils"
	"fmt"
	"strconv"
	"strings"
)

type Range struct {
	begin uint
	end   uint
}

func New(input string) Range {
	parts := strings.Split(input, "-")
	begin, _ := strconv.Atoi(parts[0])
	end, _ := strconv.Atoi(parts[1])

	return Range{
		begin: uint(begin),
		end:   uint(end),
	}
}

func (this Range) contains(other Range) bool {
	return this.begin <= other.begin && this.end >= other.end
}

// func (this Range) overlaps(other Range) bool {
//   return this.begin
// }

func part1(lines []string) {
	count := 0

	for _, line := range lines {
		inputs := strings.Split(line, ",")
		rangeA := New(inputs[0])
		rangeB := New(inputs[1])

		if rangeA.contains(rangeB) || rangeB.contains(rangeA) {
			count++
		}
	}

	fmt.Println("Part 1:", count)
}

func main() {
	lines, _ := utils.ReadLines("./data/input.txt")

  part1(lines)
}

