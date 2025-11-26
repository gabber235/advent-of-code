package day06

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	grid, guard := Parse(content)

	walked := make([]Position, 0)
	walked = append(walked, Position{guard.x, guard.y})

	for InsideGrid(guard.x, guard.y, grid) {
		if Step(&guard, grid) {
			if InsideGrid(guard.x, guard.y, grid) && !HasWalked(Position{guard.x, guard.y}, walked) {
				walked = append(walked, Position{guard.x, guard.y})
			}
		} else {
			fmt.Printf("\n\n")
			Print(guard, walked, grid)
		}
	}

	return strconv.Itoa(len(walked)), nil
}

func InsideGrid(x, y int, grid [][]Block) bool {
	return x >= 0 && x < len(grid) && y >= 0 && y < len(grid[0])
}

func Step(guard *Guard, grid [][]Block) bool {
	if LookingBlock(guard.x, guard.y, guard.direction, grid) == Wall {
		guard.direction = RotateRight(guard.direction)
		return false
	}
	switch guard.direction {
	case North:
		guard.y--
	case South:
		guard.y++
	case East:
		guard.x++
	case West:
		guard.x--
	}

	return true
}

func LookingBlock(x, y int, direction Direction, grid [][]Block) Block {
	nextX := x
	nextY := y

	switch direction {
	case North:
		nextY = y - 1
	case South:
		nextY = y + 1
	case East:
		nextX = x + 1
	case West:
		nextX = x - 1
	}

	if InsideGrid(nextX, nextY, grid) {
		return grid[nextY][nextX]
	}

	return Empty
}

func RotateRight(direction Direction) Direction {
	switch direction {
	case North:
		return East
	case East:
		return South
	case South:
		return West
	case West:
		return North
	}
	return direction
}

func Parse(content string) ([][]Block, Guard) {
	lines := strings.Split(content, "\n")
	grid := make([][]Block, len(lines))
	var guard Guard

	for y, line := range lines {
		grid[y] = make([]Block, len(line))
		for x, char := range line {
			switch char {
			case '#':
				grid[y][x] = Wall
			case '.':
				grid[y][x] = Empty
			case '^':
				guard = Guard{x, y, North}
				grid[y][x] = Empty
			case '>':
				guard = Guard{x, y, East}
				grid[y][x] = Empty
			case 'v':
				guard = Guard{x, y, South}
				grid[y][x] = Empty
			case '<':
				guard = Guard{x, y, West}
				grid[y][x] = Empty
			}
		}
	}

	return grid, guard
}

func Print(guard Guard, walked []Position, grid [][]Block) {
	for y, line := range grid {
		for x, block := range line {
			if x == guard.x && y == guard.y {
				fmt.Printf("%s", guard)
				continue
			}
			if HasWalked(Position{x, y}, walked) {
				fmt.Printf("X")
				continue
			}
			fmt.Printf("%s", block)
		}
		fmt.Println()
	}
}

func HasWalked(pos Position, walked []Position) bool {
	for _, p := range walked {
		if p == pos {
			return true
		}
	}
	return false
}

func (b Block) String() string {
	switch b {
	case Wall:
		return "#"
	case Empty:
		return "."
	}
	return ""
}

func (g Guard) String() string {
	var direction string
	switch g.direction {
	case North:
		direction = "^"
	case South:
		direction = "v"
	case East:
		direction = ">"
	case West:
		direction = "<"
	}
	return direction
}

type Guard struct {
	x, y      int
	direction Direction
}

type Direction int

const (
	North Direction = iota
	South
	East
	West
)

type Block int

const (
	Empty Block = iota
	Wall
)

type Position struct {
	x, y int
}
