package day05

import (
	"strconv"
)

func Part2(content string) (string, error) {
	rules, updates, err := Parse(content)
	if err != nil {
		return "", err
	}

	total := 0
	for _, update := range updates {
		if !ValidateUpdate(update, rules) {
			CorrectUpdate(update, rules)
			total += MiddleNumber(update)
		}
	}
	return strconv.Itoa(total), nil
}

// Corrects the update by swapping the before and after numbers in the array
func CorrectUpdate(numbers []int, rules []rule) {
	changed := true
	for changed {
		changed = false
		for _, rule := range rules {
			if CorrectUpdateWithRule(numbers, rule) {
				changed = true
				continue
			}
		}
	}
}

func CorrectUpdateWithRule(numbers []int, rule rule) bool {
	indexBefore := -1
	indexAfter := -1
	for i, number := range numbers {
		if number == rule.before {
			indexBefore = i
			if indexAfter != -1 {
				numbers[indexBefore], numbers[indexAfter] = numbers[indexAfter], numbers[indexBefore]
				return true
			}
		}
		if number == rule.after {
			if indexBefore != -1 {
				return false
			}
			indexAfter = i
		}
	}
	return false
}
