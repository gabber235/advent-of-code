package day21

import (
	"fmt"
	"strconv"
	"strings"
)

func Part2(content string) (string, error) {
	codes := parseInput(content)

	numericKeypad := NewKeypad(NumericKeypad)
	directionalKeypad := NewKeypad(DirectionalKeypad)

	lastIndex := NewPathIndex(directionalKeypad)

	for i := 0; i < 24; i++ {
		fmt.Printf("Building index for layer %d\n", i)
		lastIndex = BuildLayerIndex(directionalKeypad, lastIndex)
		fmt.Printf("Index for layer %d: %v\n\n", i, lastIndex)
	}

	finalIndex := BuildLayerIndex(numericKeypad, lastIndex)

	totalComplexity := 0

	// Process each code
	for _, code := range codes {
		// Get numeric value for complexity calculation
		numStr := strings.TrimLeft(code[:len(code)-1], "0")
		numVal, _ := strconv.Atoi(numStr)

		// Find shortest path length for this code using final index
		pathLength := findShortestPathLengthForCode(code, finalIndex)
		fmt.Printf("Path length for %s: %d\n", code, pathLength)

		// Calculate and add complexity
		complexity := pathLength * numVal
		totalComplexity += complexity
	}

	return strconv.Itoa(totalComplexity), nil
}
