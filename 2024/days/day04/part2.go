package day04

import (
	"strconv"
)

func Part2(content string) (string, error) {
	grid := Parse(content)

	xmases := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			mas := 0

			// Backwards \
			if ReadMas(grid, pos{x, y}, pos{-1, -1}, pos{1, 1}) {
				mas++
			}
			if ReadMas(grid, pos{x, y}, pos{1, 1}, pos{-1, -1}) {
				mas++
			}

			// Forwards /
			if ReadMas(grid, pos{x, y}, pos{-1, 1}, pos{1, -1}) {
				mas++
			}
			if ReadMas(grid, pos{x, y}, pos{1, -1}, pos{-1, 1}) {
				mas++
			}

			if mas == 2 {
				xmases++
			}
		}
	}

	return strconv.Itoa(xmases), nil
}

func ReadMas(grid [][]string, position pos, relativeM pos, relativeS pos) bool {
	if letterAt(grid, position) != "A" {
		return false
	}
	if letterAt(grid, add(position, relativeM)) != "M" {
		return false
	}
	if letterAt(grid, add(position, relativeS)) != "S" {
		return false
	}
	return true
}
