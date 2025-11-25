package day15

import (
	"fmt"
	"strings"
)

type Position struct {
	X, Y int
}

type Move rune

const (
	Up    Move = '^'
	Down  Move = 'v'
	Left  Move = '<'
	Right Move = '>'
)

type Warehouse struct {
	Grid   [][]rune
	Robot  Position
	Boxes  []Position
	Width  int
	Height int
}

func ParseInput(input string) (*Warehouse, []Move, error) {
	parts := strings.SplitN(input, "\n\n", 2)
	if len(parts) != 2 {
		return nil, nil, nil
	}

	warehouse := parseWarehouse(parts[0])
	if warehouse == nil {
		return nil, nil, nil
	}

	moves := parseMoves(parts[1])

	return warehouse, moves, nil
}

func parseWarehouse(mapStr string) *Warehouse {
	lines := strings.Split(strings.TrimSpace(mapStr), "\n")
	height := len(lines)
	width := len(lines[0])

	grid := make([][]rune, height)
	var boxes []Position
	var robot Position

	for y, line := range lines {
		grid[y] = make([]rune, width)
		for x, char := range line {
			grid[y][x] = char

			switch char {
			case 'O':
				boxes = append(boxes, Position{X: x, Y: y})
				// Replace box with empty space in grid for consistency
				grid[y][x] = '.'
			case '@':
				robot = Position{X: x, Y: y}
				// Replace robot with empty space in grid for consistency
				grid[y][x] = '.'
			}
		}
	}

	return &Warehouse{
		Grid:   grid,
		Robot:  robot,
		Boxes:  boxes,
		Width:  width,
		Height: height,
	}
}

func parseMoves(moveStr string) []Move {
	moveStr = strings.ReplaceAll(moveStr, "\n", "")
	moveStr = strings.TrimSpace(moveStr)

	moves := make([]Move, 0, len(moveStr))
	for _, char := range moveStr {
		switch char {
		case '^', 'v', '<', '>':
			moves = append(moves, Move(char))
		}
	}

	return moves
}

func (w *Warehouse) IsWall(pos Position) bool {
	if pos.Y < 0 || pos.Y >= w.Height || pos.X < 0 || pos.X >= w.Width {
		return true
	}
	return w.Grid[pos.Y][pos.X] == '#'
}

func (w *Warehouse) HasBox(pos Position) bool {
	for _, box := range w.Boxes {
		if box == pos {
			return true
		}
	}
	return false
}

func (w *Warehouse) String() string {
	display := make([][]rune, w.Height)
	for i := range w.Grid {
		display[i] = make([]rune, w.Width)
		copy(display[i], w.Grid[i])
	}

	for _, box := range w.Boxes {
		display[box.Y][box.X] = 'O'
	}

	display[w.Robot.Y][w.Robot.X] = '@'

	var sb strings.Builder
	for _, row := range display {
		sb.WriteString(string(row))
		sb.WriteRune('\n')
	}
	return sb.String()
}

func Part1(content string) (string, error) {
	warehouse, moves, err := ParseInput(content)
	if err != nil {
		return "", err
	}

	// fmt.Printf("Initial state:\n%s\n", warehouse.String())

	for _, move := range moves {
		if err := SimulateMovement(warehouse, move); err != nil {
			return "", err
		}
		// fmt.Printf("State after move %c:\n%s\n", move, warehouse)
	}

	// Calculate GPS coordinates sum
	sum := 0
	for _, box := range warehouse.Boxes {
		sum += (box.Y)*100 + (box.X)
	}

	return fmt.Sprintf("%d", sum), nil
}

func SimulateMovement(warehouse *Warehouse, move Move) error {
	nextPos := warehouse.Robot.GetDirection(move)

	if warehouse.CanMove(nextPos, move) {
		if warehouse.HasBox(nextPos) {
			warehouse.ShiftBoxes(nextPos, move)
		}
		warehouse.Robot = nextPos
	}

	return nil
}

func (w *Warehouse) CanMove(pos Position, move Move) bool {
	if w.HasBox(pos) {
		return w.CanMove(pos.GetDirection(move), move)
	}
	if w.IsWall(pos) {
		return false
	}
	return true
}

func (w *Warehouse) ShiftBoxes(pos Position, move Move) {
	var boxesToMove []Position
	currentPos := pos

	for w.HasBox(currentPos) {
		boxesToMove = append(boxesToMove, currentPos)
		currentPos = currentPos.GetDirection(move)
	}

	if !w.IsWall(currentPos) {
		for i, oldPos := range boxesToMove {
			for j, box := range w.Boxes {
				if box == oldPos {
					w.Boxes[j] = boxesToMove[i].GetDirection(move)
					break
				}
			}
		}
	}
}

func (p *Position) GetDirection(move Move) Position {
	switch move {
	case Up:
		return Position{X: p.X, Y: p.Y - 1}
	case Down:
		return Position{X: p.X, Y: p.Y + 1}
	case Left:
		return Position{X: p.X - 1, Y: p.Y}
	case Right:
		return Position{X: p.X + 1, Y: p.Y}
	default:
		return *p
	}
}
