package day24

import (
	"fmt"
	"log"
	"sort"
	"strconv"
	"strings"
)

type Wire struct {
	name  string
	value *bool
}

type Gate struct {
	gateType string // "AND", "OR", "XOR"
	input1   string
	input2   string
	output   string
}

type Circuit struct {
	wires map[string]*Wire
	gates []Gate
}

func parseInput(content string) (*Circuit, error) {
	sections := strings.Split(strings.TrimSpace(content), "\n\n")
	if len(sections) != 2 {
		return nil, fmt.Errorf("expected 2 sections, got %d", len(sections))
	}

	circuit := &Circuit{
		wires: make(map[string]*Wire),
		gates: []Gate{},
	}

	for _, line := range strings.Split(sections[0], "\n") {
		parts := strings.Split(strings.TrimSpace(line), ":")
		if len(parts) != 2 {
			return nil, fmt.Errorf("invalid wire value line: %s", line)
		}
		wireName := strings.TrimSpace(parts[0])
		value, err := strconv.Atoi(strings.TrimSpace(parts[1]))
		if err != nil {
			return nil, fmt.Errorf("invalid wire value: %s", parts[1])
		}
		boolValue := value == 1
		circuit.wires[wireName] = &Wire{name: wireName, value: &boolValue}
	}

	for _, line := range strings.Split(sections[1], "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		parts := strings.Split(line, " -> ")
		if len(parts) != 2 {
			return nil, fmt.Errorf("invalid gate line: %s", line)
		}

		inputs := strings.Split(parts[0], " ")
		output := strings.TrimSpace(parts[1])

		if _, exists := circuit.wires[output]; !exists {
			circuit.wires[output] = &Wire{name: output, value: nil}
		}

		var gateType, input1, input2 string
		if len(inputs) == 3 {
			input1 = inputs[0]
			gateType = inputs[1]
			input2 = inputs[2]
		} else {
			return nil, fmt.Errorf("invalid gate inputs: %s", parts[0])
		}

		if _, exists := circuit.wires[input1]; !exists {
			circuit.wires[input1] = &Wire{name: input1, value: nil}
		}
		if _, exists := circuit.wires[input2]; !exists {
			circuit.wires[input2] = &Wire{name: input2, value: nil}
		}

		circuit.gates = append(circuit.gates, Gate{
			gateType: gateType,
			input1:   input1,
			input2:   input2,
			output:   output,
		})
	}

	return circuit, nil
}

func (c *Circuit) evaluateGate(gate Gate) bool {
	input1Wire := c.wires[gate.input1]
	input2Wire := c.wires[gate.input2]

	if input1Wire.value == nil || input2Wire.value == nil {
		return false
	}

	var result bool
	switch gate.gateType {
	case "AND":
		result = *input1Wire.value && *input2Wire.value
	case "OR":
		result = *input1Wire.value || *input2Wire.value
	case "XOR":
		result = *input1Wire.value != *input2Wire.value
	default:
		log.Printf("Unknown gate type: %s", gate.gateType)
		return false
	}

	c.wires[gate.output].value = &result
	return true
}

func (c *Circuit) simulateCircuit() error {
	for {
		progress := false
		allZWiresSet := true

		for wireName, wire := range c.wires {
			if strings.HasPrefix(wireName, "z") && wire.value == nil {
				allZWiresSet = false
				break
			}
		}

		if allZWiresSet {
			return nil
		}

		for _, gate := range c.gates {
			if c.wires[gate.output].value == nil {
				if c.evaluateGate(gate) {
					progress = true
				}
			}
		}

		if !progress {
			return fmt.Errorf("circuit simulation stuck - no progress made")
		}
	}
}

func (c *Circuit) getResult() int64 {
	var zWires []string
	for wireName := range c.wires {
		if strings.HasPrefix(wireName, "z") {
			zWires = append(zWires, wireName)
		}
	}

	sort.Slice(zWires, func(i, j int) bool {
		return zWires[i] > zWires[j]
	})

	var binaryStr string
	for _, wireName := range zWires {
		wire := c.wires[wireName]
		if *wire.value {
			binaryStr += "1"
		} else {
			binaryStr += "0"
		}
	}

	result, _ := strconv.ParseInt(binaryStr, 2, 64)
	return result
}

func Part1(content string) (string, error) {
	circuit, err := parseInput(content)
	if err != nil {
		return "", fmt.Errorf("error parsing input: %v", err)
	}

	err = circuit.simulateCircuit()
	if err != nil {
		return "", fmt.Errorf("error simulating circuit: %v", err)
	}

	result := circuit.getResult()
	return fmt.Sprintf("%d", result), nil
}
