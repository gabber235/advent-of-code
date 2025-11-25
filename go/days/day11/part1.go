package day11

import (
	"math"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	numbers, err := Parse(content)
	if err != nil {
		return "", err
	}

	count := 0
	cache := make(map[Rock]int)

	for _, number := range numbers {
		count += Blink(cache, Rock{Number: number, BlinksLeft: 25})
	}

	return strconv.Itoa(count), nil
}

func Blink(cache map[Rock]int, rock Rock) int {
	if rock.BlinksLeft == 0 {
		return 1
	}
	if val, ok := cache[rock]; ok {
		return val
	}

	if rock.Number == 0 {
		rocks := Blink(cache, Rock{Number: 1, BlinksLeft: rock.BlinksLeft - 1})
		cache[rock] = rocks
		return rocks
	}

	digits := int(math.Log10(float64(rock.Number))) + 1
	if digits%2 == 0 {
		divisor := int(math.Pow10(digits / 2))
		rocks := Blink(cache, Rock{Number: rock.Number / divisor, BlinksLeft: rock.BlinksLeft - 1})
		rocks += Blink(cache, Rock{Number: rock.Number % divisor, BlinksLeft: rock.BlinksLeft - 1})
		cache[rock] = rocks
		return rocks
	}

	rocks := Blink(cache, Rock{Number: rock.Number * 2024, BlinksLeft: rock.BlinksLeft - 1})
	cache[rock] = rocks
	return rocks
}

func Parse(content string) ([]int, error) {
	words := strings.Split(content, " ")
	result := make([]int, len(words))
	for i, word := range words {
		result[i], _ = strconv.Atoi(word)
	}
	return result, nil
}

type Rock struct {
	Number     int
	BlinksLeft int
}
