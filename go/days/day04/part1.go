package day04

import (
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	grid := Parse(content)

	xmases := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			// Horizontal
			if ReadXMas(grid, pos{x, y}, pos{1, 0}, pos{2, 0}, pos{3, 0}) {
				xmases++
			}
			if ReadXMas(grid, pos{x, y}, pos{-1, 0}, pos{-2, 0}, pos{-3, 0}) {
				xmases++
			}

			// Vertical
			if ReadXMas(grid, pos{x, y}, pos{0, 1}, pos{0, 2}, pos{0, 3}) {
				xmases++
			}
			if ReadXMas(grid, pos{x, y}, pos{0, -1}, pos{0, -2}, pos{0, -3}) {
				xmases++
			}

			// Diagonal
			if ReadXMas(grid, pos{x, y}, pos{1, 1}, pos{2, 2}, pos{3, 3}) {
				xmases++
			}
			if ReadXMas(grid, pos{x, y}, pos{-1, -1}, pos{-2, -2}, pos{-3, -3}) {
				xmases++
			}
			if ReadXMas(grid, pos{x, y}, pos{-1, 1}, pos{-2, 2}, pos{-3, 3}) {
				xmases++
			}
			if ReadXMas(grid, pos{x, y}, pos{1, -1}, pos{2, -2}, pos{3, -3}) {
				xmases++
			}
		}
	}

	return strconv.Itoa(xmases), nil
}

func Parse(content string) [][]string {
	lines := strings.Split(content, "\n")
	grid := make([][]string, len(lines))
	for i, line := range lines {
		grid[i] = strings.Split(line, "")
	}
	return grid
}

type pos struct {
	x, y int
}

func add(a, b pos) pos {
	return pos{a.x + b.x, a.y + b.y}
}

// Returns the letter at the given position
// If the position is out of bounds, it returns an empty string
func letterAt(grid [][]string, position pos) string {
	if position.x < 0 || position.x >= len(grid[0]) {
		return ""
	}
	if position.y < 0 || position.y >= len(grid) {
		return ""
	}
	return grid[position.y][position.x]
}

// Checks if the given positions read out XMas
// The position is the absolute position of the X
// Then the position of MAS relative to the X
// So X could be at position (5,4) and M at (1, 1), A at (2, 2) and S at (3, 3)
func ReadXMas(grid [][]string, position pos, ralativeM pos, relativeA pos, relativeS pos) bool {
	if letterAt(grid, position) != "X" {
		return false
	}
	if letterAt(grid, add(position, ralativeM)) != "M" {
		return false
	}
	if letterAt(grid, add(position, relativeA)) != "A" {
		return false
	}
	if letterAt(grid, add(position, relativeS)) != "S" {
		return false
	}
	return true
}
