package day01

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
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

	// Sort the columns
	sort.Ints(firstColumn)
	sort.Ints(secondColumn)

	totalDistance := 0

	for i := 0; i < len(firstColumn); i++ {
		// Get the difference between the two columns. But both can be bigger or smaller than the other
		if firstColumn[i] > secondColumn[i] {
			totalDistance += firstColumn[i] - secondColumn[i]
		} else {
			totalDistance += secondColumn[i] - firstColumn[i]
		}
	}

	return fmt.Sprintf("%d", totalDistance), nil
}

// All lines are in the form of "1324   3412" we want to return the two numbers
func ProcessLine(line string) (int, int, error) {
	numbers := strings.Split(line, "   ")
	if len(numbers) != 2 {
		return 0, 0, fmt.Errorf("invalid line: '%s'", line)
	}
	first, err := strconv.Atoi(numbers[0])
	if err != nil {
		return 0, 0, err
	}
	second, err := strconv.Atoi(numbers[1])
	if err != nil {
		return 0, 0, err
	}
	return first, second, nil
}

func Assert(condition bool, message string) {
	if !condition {
		panic(message)
	}
}
