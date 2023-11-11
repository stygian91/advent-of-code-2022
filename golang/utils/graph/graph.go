package graph

import (
	"aoc2022/utils"
	"fmt"
	"math"
	"slices"
)

type Edge[KeyType comparable] struct {
	To   KeyType
	Cost int64
}

type Graph[KeyType comparable, ValueType interface{}] struct {
	Vertices map[KeyType]ValueType
	Edges    map[KeyType][]Edge[KeyType]
}

func (this Graph[KeyType, ValueType]) Dijkstra(start KeyType, finish KeyType, maxIterations int) (map[KeyType]int64, error) {
	_, startFound := this.Vertices[start]
	if !startFound {
		return nil, fmt.Errorf("Start vertex(%+v) not found", start)
	}

	_, finishFound := this.Vertices[start]
	if !finishFound {
		return nil, fmt.Errorf("Finish vertex(%+v) not found", finish)
	}

	var currentIndex int
	current := start
	unvisited := utils.GetMapKeys[KeyType, ValueType](this.Vertices)
	distances := initDistances[KeyType, ValueType](this, start)

	for i := 0; i < maxIterations; i++ {
		var next KeyType
		children := this.Edges[current]
		hasNext := false

		for _, child := range children {
			currDist := distances[current]
			if currDist == math.MaxInt64 {
				return nil, fmt.Errorf("Current distance was math.max")
			}

			childDist := currDist + child.Cost
			if childDist < distances[child.To] {
				distances[child.To] = childDist
			}

			if !hasNext && slices.Index(unvisited, child.To) != -1 {
				hasNext = true
				next = child.To
			}
		}

		currentIndex = slices.Index(unvisited, current)
		if currentIndex != -1 {
			unvisited = slices.Delete(unvisited, currentIndex, currentIndex)
		}

		if !hasNext {
			return distances, nil
		}

		current = next
	}

	return nil, fmt.Errorf("Reached max iteration count")
}

func initDistances[K comparable, V interface{}](graph Graph[K, V], start K) map[K]int64 {
	costs := make(map[K]int64, len(graph.Vertices))

	for k := range graph.Vertices {
		costs[k] = math.MaxInt64
	}

	costs[start] = 0

	return costs
}
