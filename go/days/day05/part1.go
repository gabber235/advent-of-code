package day05

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	rules, updates, err := Parse(content)
	if err != nil {
		return "", err
	}

	total := 0

	for _, update := range updates {
		if ValidateUpdate(update, rules) {
			total += MiddleNumber(update)
		}
	}

	return strconv.Itoa(total), nil
}

// Returns the middle number of the given numbers
func MiddleNumber(numbers []int) int {
	middle := len(numbers) / 2
	if len(numbers)%2 == 0 {
		return (numbers[middle] + numbers[middle-1]) / 2
	}
	return numbers[middle]
}

func ValidateUpdate(numbers []int, rules []rule) bool {
	for _, rule := range rules {
		if !ValidateUpdateWithRule(numbers, rule) {
			return false
		}
	}
	return true
}

func ValidateUpdateWithRule(numbers []int, rule rule) bool {
	seenBefore := false
	seenAfter := false
	for _, number := range numbers {
		if number == rule.before {
			if seenAfter {
				return false
			}
			seenBefore = true
		}
		if number == rule.after {
			if seenBefore {
				return true
			}
			seenAfter = true
		}
	}
	return true
}

func Parse(content string) ([]rule, [][]int, error) {
	split := strings.Split(content, "\n\n")
	if len(split) != 2 {
		return nil, nil, fmt.Errorf("Expected 2 parts, got %d", len(split))
	}
	rules, err := parseRules(split[0])
	if err != nil {
		return nil, nil, err
	}
	updates, err := parseUpdates(split[1])
	if err != nil {
		return nil, nil, err
	}
	return rules, updates, nil
}

func parseRules(content string) ([]rule, error) {
	lines := strings.Split(content, "\n")
	result := make([]rule, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, "|")
		if len(parts) != 2 {
			return nil, fmt.Errorf("Expected 2 parts, got %d", len(parts))
		}
		before, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, err
		}
		after, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, err
		}
		result[i] = rule{before, after}
	}
	return result, nil
}

func parseUpdates(content string) ([][]int, error) {
	lines := strings.Split(content, "\n")
	result := make([][]int, len(lines))
	for i, line := range lines {
		numbers, err := ParseUpdate(line)
		if err != nil {
			return nil, err
		}
		result[i] = numbers
	}
	return result, nil
}

func ParseUpdate(line string) ([]int, error) {
	numbers := strings.Split(line, ",")
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

type rule struct {
	before, after int
}
