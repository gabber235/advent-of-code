package day24

import (
	"fmt"
	"os"
	"strings"
)

// generateDOT generates a DOT language representation of the circuit
func generateDOT(c *Circuit) string {
	var sb strings.Builder
	sb.WriteString("digraph circuit {\n")
	sb.WriteString("  rankdir=LR;\n") // Left to right layout

	// Node styles
	sb.WriteString("  node [shape=box];\n")

	// Add input nodes (x and y wires)
	for name, wire := range c.wires {
		if strings.HasPrefix(name, "x") || strings.HasPrefix(name, "y") {
			value := "undefined"
			if wire.value != nil {
				if *wire.value {
					value = "1"
				} else {
					value = "0"
				}
			}
			sb.WriteString(fmt.Sprintf("  %s [label=\"%s\\n%s\", style=filled, fillcolor=lightblue];\n",
				name, name, value))
		}
	}

	// Add output nodes (z wires)
	for name, wire := range c.wires {
		if strings.HasPrefix(name, "z") {
			value := "undefined"
			if wire.value != nil {
				if *wire.value {
					value = "1"
				} else {
					value = "0"
				}
			}
			sb.WriteString(fmt.Sprintf("  %s [label=\"%s\\n%s\", style=filled, fillcolor=lightgreen];\n",
				name, name, value))
		}
	}

	// Add gates and connections
	for i, gate := range c.gates {
		gateName := fmt.Sprintf("gate_%d", i)
		sb.WriteString(fmt.Sprintf("  %s [label=\"%s\", shape=circle];\n", gateName, gate.gateType))

		// Input connections
		sb.WriteString(fmt.Sprintf("  %s -> %s;\n", gate.input1, gateName))
		sb.WriteString(fmt.Sprintf("  %s -> %s;\n", gate.input2, gateName))

		// Output connection
		sb.WriteString(fmt.Sprintf("  %s -> %s;\n", gateName, gate.output))
	}

	sb.WriteString("}\n")
	return sb.String()
}

// analyzeBinaryAdder analyzes the circuit structure against an ideal ripple carry adder
func analyzeBinaryAdder(c *Circuit) []PotentialSwap {
	potentialSwaps := []PotentialSwap{}

	// Get the number of bits by counting x inputs
	numBits := 0
	for wire := range c.wires {
		if strings.HasPrefix(wire, "x") {
			numBits++
		}
	}

	// Map expected structure
	expectedStructure := mapRippleCarryStructure(numBits)
	actualStructure := mapActualStructure(c)

	// Compare structures and identify potential swaps
	for gateID, expectedOutputs := range expectedStructure {
		actualOutputs, exists := actualStructure[gateID]
		if !exists {
			continue
		}

		if expectedOutputs != actualOutputs {
			potentialSwaps = append(potentialSwaps, PotentialSwap{
				ExpectedOutput: expectedOutputs,
				ActualOutput:   actualOutputs,
				GateID:         gateID,
			})
		}
	}

	return potentialSwaps
}

type PotentialSwap struct {
	ExpectedOutput string
	ActualOutput   string
	GateID         string
}

// mapRippleCarryStructure creates a map of expected connections in a ripple carry adder
func mapRippleCarryStructure(numBits int) map[string]string {
	structure := make(map[string]string)

	// For each bit position
	for i := 0; i < numBits; i++ {
		bitPos := fmt.Sprintf("%02d", i)

		// XOR gate for sum
		sumGateID := fmt.Sprintf("sum_%s", bitPos)
		structure[sumGateID] = fmt.Sprintf("z%s", bitPos)

		// AND gate for carry
		if i < numBits-1 {
			carryGateID := fmt.Sprintf("carry_%s", bitPos)
			structure[carryGateID] = fmt.Sprintf("carry_%s", fmt.Sprintf("%02d", i+1))
		}
	}

	return structure
}

// mapActualStructure creates a map of actual connections in the circuit
func mapActualStructure(c *Circuit) map[string]string {
	structure := make(map[string]string)

	for _, gate := range c.gates {
		gateID := fmt.Sprintf("%s_%s_%s", gate.gateType, gate.input1, gate.input2)
		structure[gateID] = gate.output
	}

	return structure
}

func Part2(content string) (string, error) {
	// Parse circuit
	circuit, err := parseInput(content)
	if err != nil {
		return "", fmt.Errorf("error parsing input: %v", err)
	}

	// Generate DOT representation for visualization
	dot := generateDOT(circuit)
	fmt.Println("Circuit DOT representation:")
	// Write to file
	err = os.WriteFile("circuit.dot", []byte(dot), 0o644)
	if err != nil {
		return "", fmt.Errorf("error writing circuit.dot: %v", err)
	}

	// You can save this to a file and use graphviz to render it:
	// dot -Tpng circuit.dot -o circuit.png

	// Analyze circuit structure
	potentialSwaps := analyzeBinaryAdder(circuit)

	// Print potential swaps for analysis
	fmt.Println("\nPotential swaps identified:")
	for _, swap := range potentialSwaps {
		fmt.Printf("Gate %s: Expected output %s, got %s\n",
			swap.GateID, swap.ExpectedOutput, swap.ActualOutput)
	}

	return "", nil
}
