package main

import (
	"aoc2022/utils"
	"aoc2022/q03/part1"
	"aoc2022/q03/part2"
	"fmt"
)

func main() {
	fmt.Println("Main")
	lines, _ := utils.ReadLines("./data/input.txt")
	part1.Part1(lines)
	part2.Part2(lines)
}
