package day06

import (
	"fmt"
	"strconv"
)

func Part2(content string) (string, error) {
	grid, guard := Parse(content)

	startGuard := Guard{guard.x, guard.y, guard.direction}

	possibleBlocks := make([]Position, 0)

	steps := 0
	for InsideGrid(guard.x, guard.y, grid) {
		steps++
		fmt.Printf("Step %d\n", steps)
		Step(&guard, grid)

		if LookingBlock(guard.x, guard.y, guard.direction, grid) == Wall {
			continue
		}

		obsticlePosition := MoveDirection(Position{guard.x, guard.y}, guard.direction)
		if !InsideGrid(obsticlePosition.x, obsticlePosition.y, grid) {
			continue
		}

		if AlreadyTried(obsticlePosition, possibleBlocks) {
			continue
		}

		if startGuard.x == guard.x && startGuard.y == guard.y {
			continue
		}

		gridClone := CloneGrid(grid)

		gridClone[obsticlePosition.y][obsticlePosition.x] = Wall

		if GetsStuckInLoop(startGuard, gridClone) {
			possibleBlocks = append(possibleBlocks, obsticlePosition)
		}
	}

	return strconv.Itoa(len(possibleBlocks)), nil
}

func AlreadyTried(pos Position, possibleBlocks []Position) bool {
	for _, p := range possibleBlocks {
		if p == pos {
			return true
		}
	}
	return false
}

func CloneGrid(grid [][]Block) [][]Block {
	gridCopy := make([][]Block, len(grid))

	for y, line := range grid {
		gridCopy[y] = make([]Block, len(line))
		copy(gridCopy[y], line)
	}

	return gridCopy
}

func GetsStuckInLoop(guard Guard, grid [][]Block) bool {
	walked := make([]Guard, 0)
	walked = append(walked, guard)
	for InsideGrid(guard.x, guard.y, grid) {
		Step(&guard, grid)
		if HasGuardWalked(guard, walked) {
			return true
		}
		walked = append(walked, guard)
	}
	return false
}

func HasGuardWalked(guard Guard, walked []Guard) bool {
	for _, g := range walked {
		if g == guard {
			return true
		}
	}
	return false
}

const (
	resetColor  = "\033[0m"
	darkGrayFg  = "\033[38;5;240m" // Dark gray text
	lightGrayFg = "\033[38;5;250m" // Light gray text
	whiteFg     = "\033[97m"       // White text
	greenFg     = "\033[32m"       // Green text
	redFg       = "\033[31m"       // Red text
)

func PrintFake(guard Guard, walked []Guard, obsticlePosition Position, grid [][]Block) {
	for y, line := range grid {
		for x, block := range line {
			if x == guard.x && y == guard.y {
				// Guard in green
				fmt.Printf("%s%s%s", greenFg, guard, resetColor)
				continue
			}

			directions := WalkedDirections(Position{x, y}, walked)
			if len(directions) > 0 {
				// Fake path in white
				fmt.Printf("%s%s%s", whiteFg, FakeWalkDirection(directions), resetColor)
				continue
			}

			if x == obsticlePosition.x && y == obsticlePosition.y {
				// Obstacle in red
				fmt.Printf("%sO%s", redFg, resetColor)
				continue
			}

			// Grid blocks in dark gray
			fmt.Printf("%s%s%s", darkGrayFg, block, resetColor)
		}
		fmt.Println()
	}
}

func WalkedDirections(pos Position, walked []Guard) []Direction {
	var directions []Direction
	for _, g := range walked {
		if g.x == pos.x && g.y == pos.y {
			directions = append(directions, g.direction)
		}
	}
	return directions
}

func (d Direction) String() string {
	switch d {
	case North:
		return "|"
	case South:
		return "|"
	case East:
		return "-"
	case West:
		return "-"
	}
	return ""
}

func FakeWalkDirection(directions []Direction) string {
	if len(directions) == 0 {
		return ""
	}

	// Single direction - use simple arrows
	if len(directions) == 1 {
		switch directions[0] {
		case North:
			return "↑" // or "󰁝" for nerd font
		case South:
			return "↓" // or "󰁅" for nerd font
		case East:
			return "→" // or "󰁔" for nerd font
		case West:
			return "←" // or "󰁍" for nerd font
		}
	}

	// Create a map to track which directions are present
	dirMap := make(map[Direction]bool)
	for _, d := range directions {
		dirMap[d] = true
	}

	// Handle different combinations
	switch len(dirMap) {
	case 2:
		// Straight lines
		if dirMap[North] && dirMap[South] {
			return "│" // vertical line
		}
		if dirMap[East] && dirMap[West] {
			return "─" // horizontal line
		}
		// Corners
		if dirMap[North] && dirMap[East] {
			return "└"
		}
		if dirMap[North] && dirMap[West] {
			return "┘"
		}
		if dirMap[South] && dirMap[East] {
			return "┌"
		}
		if dirMap[South] && dirMap[West] {
			return "┐"
		}
	case 3:
		// T-junctions
		if !dirMap[North] {
			return "┬"
		}
		if !dirMap[South] {
			return "┴"
		}
		if !dirMap[East] {
			return "┤"
		}
		if !dirMap[West] {
			return "├"
		}
	case 4:
		return "┼" // crossroads
	}

	return "•" // fallback for unexpected cases
}

func MoveDirection(pos Position, dir Direction) Position {
	switch dir {
	case North:
		return Position{x: pos.x, y: pos.y - 1}
	case South:
		return Position{x: pos.x, y: pos.y + 1}
	case East:
		return Position{x: pos.x + 1, y: pos.y}
	case West:
		return Position{x: pos.x - 1, y: pos.y}
	}
	return pos
}
