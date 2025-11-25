package day03

import (
	"regexp"
	"strconv"
)

func Part2(content string) (string, error) {
	re := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)`)

	matches := re.FindAllStringSubmatch(content, -1)
	active := true
	total := 0
	for _, match := range matches {
		if match[0] == "do()" {
			active = true
		} else if match[0] == "don't()" {
			active = false
		} else if active {
			total += multiply(match[1], match[2])
		}
	}
	return strconv.Itoa(total), nil
}
