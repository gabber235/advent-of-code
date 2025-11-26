package day19

import (
	"strconv"
	"strings"
)

type BinarySolver struct {
	patterns []string
	memo     map[string]bool
}

func NewBinarySolver(patterns []string) *BinarySolver {
	return &BinarySolver{
		patterns: patterns,
		memo:     make(map[string]bool),
	}
}

func (s *BinarySolver) canMatch(target string) bool {
	if result, exists := s.memo[target]; exists {
		return result
	}

	if len(target) == 0 {
		return true
	}

	for _, pattern := range s.patterns {
		if len(pattern) > len(target) {
			continue
		}

		if strings.HasPrefix(target, pattern) {
			remaining := target[len(pattern):]
			if s.canMatch(remaining) {
				s.memo[target] = true
				return true
			}
		}
	}

	s.memo[target] = false
	return false
}

func parseInput(content string) ([]string, []string) {
	lines := strings.Split(content, "\n")
	var patterns, designs []string

	if len(lines) > 0 {
		patterns = strings.Split(strings.TrimSpace(lines[0]), ", ")
	}

	foundEmpty := false
	for _, line := range lines[1:] {
		line = strings.TrimSpace(line)
		if line == "" {
			foundEmpty = true
			continue
		}
		if foundEmpty && line != "" {
			designs = append(designs, line)
		}
	}

	return patterns, designs
}

func Part1(content string) (string, error) {
	patterns, designs := parseInput(content)
	solver := NewBinarySolver(patterns)

	possibleCount := 0
	for _, design := range designs {
		if solver.canMatch(design) {
			possibleCount++
		}
	}

	return strconv.Itoa(possibleCount), nil
}
