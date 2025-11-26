package day02

import (
	"strconv"
	"strings"
)

func Part2(content string) (string, error) {
	lines := strings.Split(strings.TrimSpace(content), "\n")

	totalValid := 0
	for _, line := range lines {
		numbers, err := ParseLine(line)
		if err != nil {
			return "", err
		}
		if ValidatePart2(numbers) {
			totalValid++
		}
	}

	return strconv.Itoa(totalValid), nil
}

func ValidatePart2(numbers []int) bool {
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
	if ValidateWithMask(numbers) {
		return true
	}
	return false
}

func ValidateWithMask(numbers []int) bool {
	for index := range numbers {
		if ValidatePart1(Mask(numbers, index)) {
			return true
		}
	}
	return false
}

func Mask(numbers []int, index int) []int {
	if index < 0 || index >= len(numbers) {
		return numbers
	}
	result := make([]int, len(numbers)-1)
	copy(result, numbers[:index])
	copy(result[index:], numbers[index+1:])
	return result
}
