package part1

import (
	"errors"
	"fmt"
	c "aoc2022/q03/common"
)

var (
	aValue        byte = byte("a"[0])
	zValue        byte = byte("z"[0])
	capitalAValue byte = byte("A"[0])
)

func makeMap(chunk string) map[string]bool {
	chunkMap := make(map[string]bool)
	for _, x := range chunk {
		chunkMap[string(x)] = true
	}

	return chunkMap
}

func findCommon(line string) (string, error) {
	mid := len(line) / 2
	part1 := line[:mid]
	part2 := line[mid:]

	part1Map := makeMap(part1)

	for _, x := range part2 {
		letter := string(x)
		_, exists := part1Map[letter]
		if exists {
			return letter, nil
		}
	}

	return "", errors.New("common not found")
}

func Part1(lines []string) {
	var sum uint64 = 0

	for _, line := range lines {
		mid := len(line) / 2
		part1 := line[:mid]
		part2 := line[mid:]

		common := c.FindChunkCommon([]string{part1, part2})
		sum += c.GetValueFromU64(common)
	}

	fmt.Printf("Part 1: %d\n", sum)
}
