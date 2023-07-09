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
