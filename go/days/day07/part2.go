package day07

import (
	"fmt"
	"strconv"
)

func Part2(content string) (string, error) {
	calibrations, err := Parse(content)
	if err != nil {
		return "", fmt.Errorf("error parsing input: %w", err)
	}

	sum := 0
	for _, calibration := range calibrations {
		if IsValidWithConcat(calibration) {
			sum += calibration.sum
		}
	}

	return fmt.Sprintf("%d", sum), nil
}

// IsValid checks if it's possible to achieve the target sum using +, *, and || operators
func IsValidWithConcat(calibration Calibration) bool {
	return canMakeSumWithConcat(calibration.operators, calibration.sum)
}

// concat concatenates two numbers. e.g., concat(12, 345) = 12345
func concat(a, b int) int {
	result, err := strconv.Atoi(strconv.Itoa(a) + strconv.Itoa(b))
	if err != nil {
		return -1 // Should never happen with valid integers
	}
	return result
}

// evaluateWithOps evaluates the expression with given operators
// operators: 0 = +, 1 = *, 2 = ||
func evaluateWithOps(numbers []int, ops []int) int {
	if len(numbers) == 1 {
		return numbers[0]
	}

	result := numbers[0]
	for i := 0; i < len(ops); i++ {
		switch ops[i] {
		case 0: // Addition
			result += numbers[i+1]
		case 1: // Multiplication
			result *= numbers[i+1]
		case 2: // Concatenation
			result = concat(result, numbers[i+1])
		}
	}
	return result
}

// canMakeSum tries all possible combinations of operators to match the target sum
func canMakeSumWithConcat(numbers []int, target int) bool {
	if len(numbers) == 1 {
		return numbers[0] == target
	}

	numOperators := len(numbers) - 1
	ops := make([]int, numOperators)

	// Helper function for recursive operator assignment
	var tryOperators func(pos int) bool
	tryOperators = func(pos int) bool {
		if pos == numOperators {
			result := evaluateWithOps(numbers, ops)
			return result == target
		}

		// Try each operator (0 = +, 1 = *, 2 = ||)
		for op := 0; op < 3; op++ {
			ops[pos] = op
			if tryOperators(pos + 1) {
				return true
			}
		}
		return false
	}

	return tryOperators(0)
}
