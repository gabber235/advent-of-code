package day02

import (
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	lines := strings.Split(strings.TrimSpace(content), "\n")

	totalValid := 0
	for _, line := range lines {
		numbers, err := ParseLine(line)
		if err != nil {
			return "", err
		}
		if ValidatePart1(numbers) {
			totalValid++
		}
	}

	return strconv.Itoa(totalValid), nil
}

func ValidatePart1(numbers []int) bool {
	// Either all numbers are increasing or all are decreasing
	// All numbers are differ by at least 1 and at most 3
	diff := make([]int, len(numbers)-1)
	for i := 0; i < len(numbers)-1; i++ {
		diff[i] = numbers[i+1] - numbers[i]
	}
	if AllWithinRange(diff, 1, 3) {
		return true
	}
	if AllWithinRange(diff, -3, -1) {
		return true
	}
	return false
}

func AllWithinRange(numbers []int, min int, max int) bool {
	for _, number := range numbers {
		if number < min || number > max {
			return false
		}
	}
	return true
}

func ParseLine(line string) ([]int, error) {
	numbers := strings.Split(line, " ")
	result := make([]int, len(numbers))
	for i, number := range numbers {
		parsed, err := strconv.Atoi(number)
		if err != nil {
			return nil, err
		}
		result[i] = parsed
	}
	return result, nil
}
