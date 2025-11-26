package day11

import (
	"strconv"
)

func Part2(content string) (string, error) {
	numbers, err := Parse(content)
	if err != nil {
		return "", err
	}

	count := 0
	cache := make(map[Rock]int)

	for _, number := range numbers {
		count += Blink(cache, Rock{Number: number, BlinksLeft: 75})
	}

	return strconv.Itoa(count), nil
}
