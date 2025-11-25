package day07

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	calibrations, err := Parse(content)
	if err != nil {
		return "", fmt.Errorf("error parsing input: %w", err)
	}

	total := 0
	for _, calibration := range calibrations {
		if IsValid(calibration) {
			total += calibration.sum
		}
	}
	return strconv.Itoa(total), nil
}

func IsValid(calibration Calibration) bool {
	return canMakeSum(calibration.operators, calibration.sum)
}

// canMakeSum tries all possible combinations of operators to match the target sum
func canMakeSum(numbers []int, target int) bool {
	if len(numbers) == 1 {
		return numbers[0] == target
	}

	// Generate all possible combinations of operators
	numOperators := len(numbers) - 1
	combinations := 1 << numOperators // 2^n combinations for n operators

	for i := 0; i < combinations; i++ {
		result := numbers[0]

		// Try each combination of operators
		for j := 0; j < numOperators; j++ {
			// Use bit j of i to determine if we use + or *
			if (i & (1 << j)) != 0 {
				// Multiply
				result *= numbers[j+1]
			} else {
				// Add
				result += numbers[j+1]
			}
		}

		if result == target {
			return true
		}
	}

	return false
}

func Parse(content string) ([]Calibration, error) {
	var calibrations []Calibration
	scanner := bufio.NewScanner(strings.NewReader(content))

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}

		calibration, err := ParseCalibration(line)
		if err != nil {
			return nil, fmt.Errorf("error parsing line '%s': %w", line, err)
		}
		calibrations = append(calibrations, calibration)
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error scanning content: %w", err)
	}

	return calibrations, nil
}

func ParseCalibration(content string) (Calibration, error) {
	parts := strings.Split(content, ":")
	if len(parts) != 2 {
		return Calibration{}, fmt.Errorf("invalid format: missing colon")
	}

	sum, err := strconv.Atoi(strings.TrimSpace(parts[0]))
	if err != nil {
		return Calibration{}, fmt.Errorf("invalid sum value: %w", err)
	}

	var operators []int
	operatorStrs := strings.Fields(strings.TrimSpace(parts[1]))

	for _, opStr := range operatorStrs {
		op, err := strconv.Atoi(opStr)
		if err != nil {
			return Calibration{}, fmt.Errorf("invalid operator value '%s': %w", opStr, err)
		}
		operators = append(operators, op)
	}

	return Calibration{
		sum:       sum,
		operators: operators,
	}, nil
}

type Calibration struct {
	sum       int
	operators []int
}
