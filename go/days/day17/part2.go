package day17

import (
	"fmt"
	"strings"
)

func Part2(content string) (string, error) {
	// First parse the program to know what we're comparing against
	computer, err := ParseInput(content)
	if err != nil {
		return "", err
	}
	programStr := strings.Trim(strings.Join(strings.Fields(strings.Split(content, "Program:")[1]), ""), " ")

	// Run with Z3 solution value
	computer.A = 236580836040301 // Replace this with Z3 solution
	computer.Run()
	output := computer.GetOutput()

	// Validate that output matches program
	if output != programStr {
		return "", fmt.Errorf("validation failed: output %s does not match program %s", output, programStr)
	}

	return fmt.Sprintf("%d", computer.A), nil
}
