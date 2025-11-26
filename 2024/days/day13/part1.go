package day13

import (
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	machines, err := parseMachines(content)
	if err != nil {
		return "", err
	}

	var tokens int
	for _, machine := range machines {
		tokens += CalculateTokens(machine)
	}

	return strconv.Itoa(tokens), nil
}

func CalculateTokens(machine Machine) int {
	// We need to solve:
	// a * ButtonA.x + b * ButtonB.x = Prize.x
	// a * ButtonA.y + b * ButtonB.y = Prize.y

	// This can be rewritten as:
	// (ButtonA.x * ButtonB.y - ButtonA.y * ButtonB.x) * k =
	//     Prize.x * ButtonB.y - Prize.y * ButtonB.x

	det := machine.ButtonA.x*machine.ButtonB.y - machine.ButtonA.y*machine.ButtonB.x
	if det == 0 {
		return 0
	}

	rhs := machine.Prize.x*machine.ButtonB.y - machine.Prize.y*machine.ButtonB.x

	if rhs%det != 0 {
		return 0
	}

	k := rhs / det
	a := k
	b := (machine.Prize.x - machine.ButtonA.x*a) / machine.ButtonB.x

	if a < 0 || b < 0 {
		return 0
	}

	if a*machine.ButtonA.x+b*machine.ButtonB.x != machine.Prize.x ||
		a*machine.ButtonA.y+b*machine.ButtonB.y != machine.Prize.y {
		return 0
	}

	return a*3 + b
}

type Position struct {
	x int
	y int
}

type Machine struct {
	ButtonA Position
	ButtonB Position
	Prize   Position
}

func parseMachine(lines []string) (Machine, error) {
	// Parse Button A
	aFields := strings.Split(strings.TrimPrefix(lines[0], "Button A: "), ", ")
	ax, err := strconv.Atoi(strings.TrimPrefix(aFields[0], "X+"))
	if err != nil {
		return Machine{}, err
	}
	ay, err := strconv.Atoi(strings.TrimPrefix(aFields[1], "Y+"))
	if err != nil {
		return Machine{}, err
	}

	// Parse Button B
	bFields := strings.Split(strings.TrimPrefix(lines[1], "Button B: "), ", ")
	bx, err := strconv.Atoi(strings.TrimPrefix(bFields[0], "X+"))
	if err != nil {
		return Machine{}, err
	}
	by, err := strconv.Atoi(strings.TrimPrefix(bFields[1], "Y+"))
	if err != nil {
		return Machine{}, err
	}

	// Parse Prize
	pFields := strings.Split(strings.TrimPrefix(lines[2], "Prize: "), ", ")
	px, err := strconv.Atoi(strings.TrimPrefix(pFields[0], "X="))
	if err != nil {
		return Machine{}, err
	}
	py, err := strconv.Atoi(strings.TrimPrefix(pFields[1], "Y="))
	if err != nil {
		return Machine{}, err
	}

	return Machine{
		ButtonA: Position{x: ax, y: ay},
		ButtonB: Position{x: bx, y: by},
		Prize:   Position{x: px, y: py},
	}, nil
}

func parseMachines(content string) ([]Machine, error) {
	// Split content into groups of three lines
	lines := strings.Split(strings.TrimSpace(content), "\n")
	var machines []Machine

	for i := 0; i < len(lines); i += 4 { // Skip empty lines between machines
		if i+2 >= len(lines) {
			break
		}
		machine, err := parseMachine(lines[i : i+3])
		if err != nil {
			return nil, err
		}
		machines = append(machines, machine)
	}

	return machines, nil
}
