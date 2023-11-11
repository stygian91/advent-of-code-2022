package utils

import (
	"bufio"
	"os"
)

func ReadLines(path string) ([]string, error) {
	readFile, err := os.Open(path)

	if err != nil {
		return nil, err
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var result []string

	for fileScanner.Scan() {
		result = append(result, fileScanner.Text())
	}

	return result, nil
}

func GetMapKeys[K comparable, V interface{}](m map[K]V) []K {
	keys := make([]K, len(m))
	i := 0

	for k := range m {
		keys[i] = k
		i++
	}

	return keys
}
