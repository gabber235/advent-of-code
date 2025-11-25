package day19

import (
	"strconv"
	"strings"
)

type MultiSolver struct {
	patterns []string
	memo     map[string]int
}

func NewMultiSolver(patterns []string) *MultiSolver {
	return &MultiSolver{
		patterns: patterns,
		memo:     make(map[string]int),
	}
}

func (s *MultiSolver) matches(target string) int {
	if result, exists := s.memo[target]; exists {
		return result
	}

	if len(target) == 0 {
		return 1
	}

	matches := 0
	for _, pattern := range s.patterns {
		if len(pattern) > len(target) {
			continue
		}

		if strings.HasPrefix(target, pattern) {
			remaining := target[len(pattern):]
			matches += s.matches(remaining)
		}
	}

	s.memo[target] = matches
	return matches
}

func Part2(content string) (string, error) {
	patterns, designs := parseInput(content)
	solver := NewMultiSolver(patterns)

	possibleCount := 0
	for _, design := range designs {
		possibleCount += solver.matches(design)
	}

	return strconv.Itoa(possibleCount), nil
}
