package day22

import (
	"strconv"
	"strings"
)

// mix performs a bitwise XOR operation between two numbers
func mix(secret, value int) int {
	return secret ^ value
}

// prune calculates the value modulo 16777216
func prune(secret int) int {
	return secret % 16777216
}

// calculateNextSecret generates the next secret number based on the rules
func calculateNextSecret(secret int) int {
	// Step 1: multiply by 64
	result := mix(secret, secret*64)
	result = prune(result)

	// Step 2: divide by 32
	result = mix(result, result/32)
	result = prune(result)

	// Step 3: multiply by 2048
	result = mix(result, result*2048)
	result = prune(result)

	return result
}

// generateNthSecret generates the nth new secret number in the sequence
func generateNthSecret(initialSecret, n int) int {
	current := initialSecret
	for i := 0; i < n; i++ {
		current = calculateNextSecret(current)
	}
	return current
}

func Part1(content string) (string, error) {
	// Parse input
	lines := strings.Split(strings.TrimSpace(content), "\n")
	sum := 0

	// Process each initial secret
	for _, line := range lines {
		initialSecret, err := strconv.Atoi(strings.TrimSpace(line))
		if err != nil {
			return "", err
		}

		// Generate the 2000th new secret and add to sum
		secret := generateNthSecret(initialSecret, 2000)
		sum += secret
	}

	return strconv.Itoa(sum), nil
}
