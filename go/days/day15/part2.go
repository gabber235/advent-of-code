package day15

import (
	"fmt"
	"strings"
	"time"
)

type WideBox struct {
	Left, Y int
}

type WideWarehouse struct {
	Grid   [][]rune
	Robot  Position
	Boxes  []WideBox
	Width  int
	Height int
}

func ParseWideInput(input string) (*WideWarehouse, []Move, error) {
	parts := strings.SplitN(input, "\n\n", 2)
	if len(parts) != 2 {
		return nil, nil, fmt.Errorf("invalid input format")
	}

	warehouse := parseWideWarehouse(scaleMap(parts[0]))
	moves := parseMoves(parts[1])

	return warehouse, moves, nil
}

func scaleMap(mapStr string) string {
	var scaled strings.Builder
	lines := strings.Split(strings.TrimSpace(mapStr), "\n")

	for _, line := range lines {
		for _, char := range line {
			switch char {
			case '#':
				scaled.WriteString("##")
			case 'O':
				scaled.WriteString("[]")
			case '.':
				scaled.WriteString("..")
			case '@':
				scaled.WriteString("@.")
			}
		}
		scaled.WriteRune('\n')
	}
	return scaled.String()
}

func parseWideWarehouse(mapStr string) *WideWarehouse {
	lines := strings.Split(strings.TrimSpace(mapStr), "\n")
	height := len(lines)
	width := len(lines[0])

	grid := make([][]rune, height)
	var boxes []WideBox
	var robot Position

	for y, line := range lines {
		grid[y] = make([]rune, width)
		for x, char := range line {
			grid[y][x] = char

			switch char {
			case '[':
				boxes = append(boxes, WideBox{Left: x, Y: y})
				grid[y][x] = '.'
			case ']':
				grid[y][x] = '.'
			case '@':
				robot = Position{X: x, Y: y}
				grid[y][x] = '.'
			}
		}
	}

	return &WideWarehouse{
		Grid:   grid,
		Robot:  robot,
		Boxes:  boxes,
		Width:  width,
		Height: height,
	}
}

func (w *WideWarehouse) String() string {
	display := make([][]rune, w.Height)
	for i := range w.Grid {
		display[i] = make([]rune, w.Width)
		copy(display[i], w.Grid[i])
	}

	// Place boxes
	for _, box := range w.Boxes {
		display[box.Y][box.Left] = '['
		display[box.Y][box.Left+1] = ']'
	}

	// Place robot
	display[w.Robot.Y][w.Robot.X] = '@'

	var sb strings.Builder
	for _, row := range display {
		sb.WriteString(string(row))
		sb.WriteRune('\n')
	}
	return sb.String()
}

func (w *WideWarehouse) GetBox(pos Position) *WideBox {
	for _, box := range w.Boxes {
		if pos.Y == box.Y && (pos.X == box.Left || pos.X == box.Left+1) {
			return &box
		}
	}
	return nil
}

func (w *WideWarehouse) HasBox(pos Position) bool {
	return w.GetBox(pos) != nil
}

func (w *WideWarehouse) IsWall(pos Position) bool {
	if pos.Y < 0 || pos.Y >= w.Height || pos.X < 0 || pos.X >= w.Width {
		return true
	}
	return w.Grid[pos.Y][pos.X] == '#'
}

func (m Move) VerticalOffset() int {
	switch m {
	case Up:
		return -1
	case Down:
		return 1
	default:
		return 0
	}
}

func (b WideBox) DirectedNeighbors(direction Move) []Position {
	switch direction {
	case Up:
		return []Position{
			{X: b.Left, Y: b.Y - 1},
			{X: b.Left + 1, Y: b.Y - 1},
		}

	case Down:
		return []Position{
			{X: b.Left, Y: b.Y + 1},
			{X: b.Left + 1, Y: b.Y + 1},
		}

	case Left:
		return []Position{
			{X: b.Left - 1, Y: b.Y},
		}

	case Right:
		return []Position{
			{X: b.Left + 2, Y: b.Y},
		}
	}
	return []Position{}
}

func (w *WideWarehouse) ConnectedBoxes(pos Position, direction Move) ([]WideBox, error) {
	var result []WideBox
	var queue []Position
	queue = append(queue, pos)

	for len(queue) > 0 {
		currentPos := queue[0]
		queue = queue[1:]

		if w.IsWall(currentPos) {
			return nil, fmt.Errorf("invalid position %v", currentPos)
		}

		box := w.GetBox(currentPos)
		if box == nil {
			continue
		}

		result = append(result, *box)
		for _, neighbor := range box.DirectedNeighbors(direction) {
			queue = append(queue, neighbor)
		}
	}
	return result, nil
}

func (b WideBox) Move(direction Move) WideBox {
	switch direction {
	case Up:
		return WideBox{Left: b.Left, Y: b.Y - 1}
	case Down:
		return WideBox{Left: b.Left, Y: b.Y + 1}
	case Left:
		return WideBox{Left: b.Left - 1, Y: b.Y}
	case Right:
		return WideBox{Left: b.Left + 1, Y: b.Y}
	}
	return b
}

func PrintMoveState(w *WideWarehouse, nextPos Position, movedBoxes []WideBox, hitWall bool, move Move) {
	const (
		reset  = "\033[0m"
		red    = "\033[31m" // Robot
		yellow = "\033[33m" // Next position or wall hit
		blue   = "\033[34m" // Boxes that have moved
	)

	display := make([][]string, w.Height)
	for i := range w.Grid {
		display[i] = make([]string, w.Width)
		for j := range w.Grid[i] {
			display[i][j] = string(w.Grid[i][j])
		}
	}

	// Place all boxes
	for _, box := range w.Boxes {
		if inMovedBoxes(box, movedBoxes) {
			display[box.Y][box.Left] = blue + "[" + reset
			display[box.Y][box.Left+1] = blue + "]" + reset
		} else {
			display[box.Y][box.Left] = "["
			display[box.Y][box.Left+1] = "]"
		}
	}

	// Highlight wall or next position if applicable
	if hitWall {
		display[nextPos.Y][nextPos.X] = yellow + "#" + reset
	}

	// Always color the robot
	display[w.Robot.Y][w.Robot.X] = red + "@" + reset

	fmt.Println("\033[2J\033[H") // Clear screen and move cursor to top
	fmt.Printf("Move: %c\n", move)
	if hitWall {
		fmt.Println("Hit wall!")
	} else if len(movedBoxes) > 0 {
		fmt.Println("Boxes moved!")
	} else {
		fmt.Println("Moving to empty space...")
	}
	fmt.Println()

	for _, row := range display {
		fmt.Println(strings.Join(row, ""))
	}
	fmt.Println()

	time.Sleep(500 * time.Millisecond)
}

func inMovedBoxes(box WideBox, movedBoxes []WideBox) bool {
	for _, moved := range movedBoxes {
		if box == moved {
			return true
		}
	}
	return false
}

func SimulateWideMovement(warehouse *WideWarehouse, move Move) {
	nextPos := warehouse.Robot.GetDirection(move)

	// Check if hitting wall
	if warehouse.IsWall(nextPos) {
		// PrintMoveState(warehouse, nextPos, nil, true, move)
		return
	}

	// Moving to empty space
	if !warehouse.HasBox(nextPos) {
		warehouse.Robot = nextPos
		// PrintMoveState(warehouse, nextPos, nil, false, move)
		return
	}

	// Try to move boxes
	connectedBoxes, err := warehouse.ConnectedBoxes(nextPos, move)
	if err != nil {
		// PrintMoveState(warehouse, nextPos, nil, true, move)
		return
	}

	// Update box positions and track moved boxes
	var movedBoxes []WideBox
	for j, originalBox := range warehouse.Boxes {
		for _, connectedBox := range connectedBoxes {
			if originalBox == connectedBox {
				warehouse.Boxes[j] = connectedBox.Move(move)
				movedBoxes = append(movedBoxes, warehouse.Boxes[j])
			}
		}
	}

	warehouse.Robot = nextPos
	// PrintMoveState(warehouse, nextPos, movedBoxes, false, move)
}

func Part2(content string) (string, error) {
	warehouse, moves, err := ParseWideInput(content)
	if err != nil {
		return "", err
	}

	for _, move := range moves {
		SimulateWideMovement(warehouse, move)
	}
	fmt.Printf("Final state:\n%s\n", warehouse.String())

	sum := 0
	for _, box := range warehouse.Boxes {
		sum += (box.Y)*100 + (box.Left)
	}

	return fmt.Sprintf("%d", sum), nil
}
