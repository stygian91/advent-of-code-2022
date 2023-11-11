package main

import (
	e "aoc2022/q12/entry"
	"aoc2022/utils"
)

func main() {
	lines, _ := utils.ReadLines("./data/demo.txt")
	var heightmap [][]e.Entry
	var row []e.Entry

	for _, line := range lines {
		row = make([]e.Entry, 0)

		for _, rune := range line {
			row = append(row, e.FromRune(rune))
		}

		heightmap = append(heightmap, row)
	}
}
