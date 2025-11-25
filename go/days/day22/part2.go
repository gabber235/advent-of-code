package day22

import (
	"fmt"
	"strconv"
	"strings"
)

func getPrice(secret int) int {
	return secret % 10
}

func getChanges(prices []int) []int {
	changes := make([]int, len(prices)-1)
	for i := 1; i < len(prices); i++ {
		changes[i-1] = prices[i] - prices[i-1]
	}
	return changes
}

func makeKey(changes []int, i int) string {
	if i < 3 {
		return "" // Not enough changes yet
	}
	return fmt.Sprintf("%d,%d,%d,%d",
		changes[i-3], changes[i-2], changes[i-1], changes[i])
}

func Part2(content string) (string, error) {
	lines := strings.Split(strings.TrimSpace(content), "\n")

	sequenceToBananas := make(map[string]int)

	totalRounds := 2000

	for _, line := range lines {
		initialSecret, err := strconv.Atoi(strings.TrimSpace(line))
		if err != nil {
			return "", err
		}

		prices := make([]int, totalRounds+1)
		current := initialSecret
		prices[0] = getPrice(current)

		for i := 1; i < totalRounds; i++ {
			current = calculateNextSecret(current)
			prices[i] = getPrice(current)
		}

		changes := getChanges(prices)

		seenSequences := make(map[string]bool)

		for i := 3; i < len(changes); i++ {
			key := makeKey(changes, i)
			if key == "" {
				continue
			}

			if !seenSequences[key] {
				seenSequences[key] = true
				sequenceToBananas[key] += prices[i+1]
			}
		}
	}

	// Find the sequence that gives the most bananas
	maxBananas := 0
	var bestSequence string
	for sequence, bananas := range sequenceToBananas {
		if bananas > maxBananas {
			maxBananas = bananas
			bestSequence = sequence
		}
	}

	fmt.Println(bestSequence)

	return strconv.Itoa(maxBananas), nil
}
