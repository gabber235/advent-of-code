package day13

import (
	"strconv"
)

func Part2(content string) (string, error) {
	machines, err := parseMachines(content)
	if err != nil {
		return "", err
	}

	var tokens int
	for _, machine := range machines {
		tokens += CalculateTokens(Machine{
			ButtonA: machine.ButtonA,
			ButtonB: machine.ButtonB,
			Prize: Position{
				x: 10000000000000 + machine.Prize.x,
				y: 10000000000000 + machine.Prize.y,
			},
		})
	}

	return strconv.Itoa(tokens), nil
}
