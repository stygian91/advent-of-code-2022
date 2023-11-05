package part2

import (
	c "aoc2022/q03/common"
	"fmt"
)

func Part2(lines []string) {
	chunks := chunkify(lines, 3)
	var sum uint64 = 0

	for _, chunk := range chunks {
		common := c.FindChunkCommon(chunk)
		sum += c.GetValueFromU64(common)
	}

	fmt.Printf("Part 2: %d\n", sum)
}

func chunkify(lines []string, chunkSize int) [][]string {
	var (
		chunks [][]string
		chunk  []string
	)

	for i, line := range lines {
		chunk = append(chunk, line)

		if (i+1)%chunkSize == 0 || i == len(lines)-1 {
			chunks = append(chunks, chunk)
			chunk = make([]string, 0)
		}
	}

	return chunks
}
