package day17

import (
	"strconv"
	"strings"
)

type OpCode int

const (
	ADV OpCode = iota // 0: Division result to A register
	BXL               // 1: XOR B register with literal
	BST               // 2: Set B register to combo operand mod 8
	JNZ               // 3: Jump if A != 0
	BXC               // 4: XOR B register with C register
	OUT               // 5: Output combo operand mod 8
	BDV               // 6: Division result to B register
	CDV               // 7: Division result to C register
)

type Computer struct {
	A       int
	B       int
	C       int
	IP      int
	Program []int
	Output  []int
}

func ParseInput(content string) (*Computer, error) {
	lines := strings.Split(strings.TrimSpace(content), "\n")
	comp := &Computer{
		Output: make([]int, 0),
	}

	// Parse registers
	for _, line := range lines {
		if strings.HasPrefix(line, "Register A:") {
			val, err := strconv.Atoi(strings.TrimPrefix(line, "Register A: "))
			if err != nil {
				return nil, err
			}
			comp.A = val
		} else if strings.HasPrefix(line, "Register B:") {
			val, err := strconv.Atoi(strings.TrimPrefix(line, "Register B: "))
			if err != nil {
				return nil, err
			}
			comp.B = val
		} else if strings.HasPrefix(line, "Register C:") {
			val, err := strconv.Atoi(strings.TrimPrefix(line, "Register C: "))
			if err != nil {
				return nil, err
			}
			comp.C = val
		} else if strings.HasPrefix(line, "Program:") {
			progStr := strings.TrimPrefix(line, "Program: ")
			numbers := strings.Split(strings.TrimSpace(progStr), ",")
			comp.Program = make([]int, len(numbers))
			for i, num := range numbers {
				val, err := strconv.Atoi(strings.TrimSpace(num))
				if err != nil {
					return nil, err
				}
				comp.Program[i] = val
			}
		}
	}

	return comp, nil
}

func (c *Computer) GetComboValue(operand int) int {
	switch operand {
	case 0, 1, 2, 3:
		return operand
	case 4:
		return c.A
	case 5:
		return c.B
	case 6:
		return c.C
	default: // 7 is reserved
		return 0
	}
}

func (c *Computer) Step() bool {
	if c.IP >= len(c.Program) {
		return false
	}

	opcode := OpCode(c.Program[c.IP])
	operand := c.Program[c.IP+1]

	switch opcode {
	case ADV:
		denominator := 1 << c.GetComboValue(operand)
		c.A = c.A / denominator
	case BXL:
		c.B ^= operand
	case BST:
		c.B = c.GetComboValue(operand) % 8
	case JNZ:
		if c.A != 0 {
			c.IP = operand
			return true
		}
	case BXC:
		c.B ^= c.C
	case OUT:
		c.Output = append(c.Output, c.GetComboValue(operand)%8)
	case BDV:
		denominator := 1 << c.GetComboValue(operand)
		c.B = c.A / denominator
	case CDV:
		denominator := 1 << c.GetComboValue(operand)
		c.C = c.A / denominator
	}

	if opcode != JNZ || c.A == 0 {
		c.IP += 2
	}
	return true
}

func (c *Computer) Run() {
	for c.Step() {
	}
}

func (c *Computer) GetOutput() string {
	var output []string
	for _, val := range c.Output {
		output = append(output, strconv.Itoa(val))
	}
	return strings.Join(output, ",")
}

func Part1(content string) (string, error) {
	computer, err := ParseInput(content)
	if err != nil {
		return "", err
	}

	computer.Run()
	return computer.GetOutput(), nil
}
