package day03

import (
	"regexp"
	"strconv"
)

func Part1(content string) (string, error) {
	// Parse the input with the following regex:
	// mul\((\d{1,3}),(\d{1,3})\)
	// Then loop over the matches and multiply the first number with the second number
	// and add the result to the total
	re := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)

	matches := re.FindAllStringSubmatch(content, -1)
	total := 0
	for _, match := range matches {
		total += multiply(match[1], match[2])
	}

	return strconv.Itoa(total), nil
}

func multiply(a, b string) int {
	aInt, _ := strconv.Atoi(a)
	bInt, _ := strconv.Atoi(b)
	return aInt * bInt
}
