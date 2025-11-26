package day25

import (
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	schematics := Parse(content)

	fit := 0
	for _, schematic := range schematics {
		if schematic.Type != Lock {
			continue
		}
		for _, other := range schematics {
			if other.Type != Key {
				continue
			}
			if schematic.FithWith(other) {
				fit++
			}
		}
	}

	return strconv.Itoa(fit), nil
}

func (s Schematic) FithWith(other Schematic) bool {
	if s.Type == other.Type {
		return false
	}

	if len(s.Heights) != len(other.Heights) {
		return false
	}

	for i := 0; i < len(s.Heights); i++ {
		if s.Heights[i]+other.Heights[i] > 5 {
			return false
		}
	}
	return true
}

func Parse(content string) []Schematic {
	schematics := []Schematic{}

	// Split by double newline to separate schematics
	blocks := strings.Split(strings.TrimSpace(content), "\n\n")

	// Parse each schematic
	for _, block := range blocks {
		lines := strings.Split(block, "\n")
		schematic := parseSchematic(lines)
		schematics = append(schematics, schematic)
	}
	return schematics
}

type SchematicType int

const (
	Lock SchematicType = iota
	Key
)

type Schematic struct {
	Heights []int
	Type    SchematicType
}

func parseSchematic(lines []string) Schematic {
	if len(lines) == 0 || len(lines[0]) == 0 {
		return Schematic{}
	}

	isLock := strings.Contains(lines[0], "#")
	var Type SchematicType
	if !isLock {
		Type = Key
	} else {
		Type = Lock
	}

	width := len(lines[0])
	heights := make([]int, width)

	for col := 0; col < width; col++ {
		height := 0
		for row := 1; row < len(lines)-1; row++ {
			if lines[row][col] == '#' {
				height++
			}
		}
		heights[col] = height
	}

	return Schematic{
		Heights: heights,
		Type:    Type,
	}
}
