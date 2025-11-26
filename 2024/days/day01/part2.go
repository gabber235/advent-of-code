package day01

import (
	"fmt"
	"strings"
)

func Part2(content string) (string, error) {
	lines := strings.Split(strings.TrimSpace(content), "\n")

	firstColumn := make([]int, 0, len(lines))
	secondColumn := make([]int, 0, len(lines))
	for _, line := range lines {
		first, second, err := ProcessLine(line)
		if err != nil {
			return "", err
		}
		firstColumn = append(firstColumn, first)
		secondColumn = append(secondColumn, second)
	}

	Assert(len(firstColumn) == len(secondColumn), "length of first and second column should be equal")

	totalFrequency := 0
	frequencies := countFrequencies(secondColumn)
	for _, number := range firstColumn {
		if frequency, ok := frequencies[number]; ok {
			totalFrequency += number * frequency
		}
	}

	return fmt.Sprintf("%d", totalFrequency), nil
}

func countFrequencies(numbers []int) map[int]int {
	frequencies := make(map[int]int)
	for _, number := range numbers {
		frequencies[number]++
	}
	return frequencies
}
