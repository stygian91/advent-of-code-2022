package main

import (
	"aoc/utils"
	"fmt"
	"sort"
	"strconv"
)

func parse_calories(lines []string) ([][]uint, error) {
	groups := [][]uint{}
	group := []uint{}

	for _, line := range lines {
		if len(line) == 0 {
			groups = append(groups, group)
			group = []uint{}
			continue
		}

		calories, err := strconv.ParseUint(line, 10, 0)
		if err != nil {
			return nil, err
		}

		group = append(group, uint(calories))
	}

	return groups, nil
}

func run(lines []string) {
	groups, err := parse_calories(lines)
	if err != nil {
		fmt.Printf("Error while parsing lines. Error: %s\n", err)
		return
	}

	calories := []uint{}
	for _, group := range groups {
		sum := uint(0)

		for _, cal := range group {
			sum = cal + sum
		}

		calories = append(calories, sum)
	}

	sort.Slice(calories, func(i, j int) bool {
		return calories[i] > calories[j]
	})

	fmt.Println("Part 1: ", calories[0])
	fmt.Println("Part 2: ", calories[0]+calories[1]+calories[2])
}

func main() {
	lines, err := utils.ReadLines("./data/input.txt")

	if err != nil {
		fmt.Println("Error while reading file: ", err)
	}

	run(lines)
}
