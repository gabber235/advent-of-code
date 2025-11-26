package day21

import (
	"fmt"
	"strconv"
	"strings"
)

type Position struct {
	Row, Col int
}

type KeypadType int

const (
	NumericKeypad KeypadType = iota
	DirectionalKeypad
)

type Keypad struct {
	Layout     [][]string
	ValidPos   map[Position]bool
	MaxRow     int
	MaxCol     int
	KeypadType KeypadType
}

type Direction struct {
	Move     string // "<", ">", "^", "v"
	DeltaRow int
	DeltaCol int
}

var Directions = []Direction{
	{"<", 0, -1},
	{">", 0, 1},
	{"^", -1, 0},
	{"v", 1, 0},
}

// NewKeypad creates a keypad of the specified type
func NewKeypad(kType KeypadType) *Keypad {
	var layout [][]string
	validPos := make(map[Position]bool)

	if kType == NumericKeypad {
		layout = [][]string{
			{"7", "8", "9"},
			{"4", "5", "6"},
			{"1", "2", "3"},
			{"", "0", "A"},
		}
	} else {
		layout = [][]string{
			{"", "^", "A"},
			{"<", "v", ">"},
		}
	}

	for row := range layout {
		for col := range layout[row] {
			if layout[row][col] != "" {
				validPos[Position{row, col}] = true
			}
		}
	}

	return &Keypad{
		Layout:     layout,
		ValidPos:   validPos,
		MaxRow:     len(layout),
		MaxCol:     len(layout[0]),
		KeypadType: kType,
	}
}

type Path struct {
	Moves []string
}

// FindAllPaths finds all possible shortest paths between start and end positions
func (k *Keypad) FindAllPaths(start, end Position) []Path {
	if !k.ValidPos[start] || !k.ValidPos[end] {
		return nil
	}

	visited := make(map[Position]bool)
	paths := make([]Path, 0)
	currentPath := Path{Moves: make([]string, 0)}

	k.findPathsDFS(start, end, visited, &currentPath, &paths)

	// Find minimum length
	minLen := -1
	for _, p := range paths {
		if minLen == -1 || len(p.Moves) < minLen {
			minLen = len(p.Moves)
		}
	}

	// Filter only shortest paths
	shortestPaths := make([]Path, 0)
	for _, p := range paths {
		if len(p.Moves) == minLen {
			shortestPaths = append(shortestPaths, p)
		}
	}

	return shortestPaths
}

// findPathsDFS performs depth-first search to find all possible paths
func (k *Keypad) findPathsDFS(current, end Position, visited map[Position]bool, currentPath *Path, paths *[]Path) {
	if current == end {
		// Add "A" at the end if we reached the target
		newPath := Path{Moves: make([]string, len(currentPath.Moves))}
		copy(newPath.Moves, currentPath.Moves)
		newPath.Moves = append(newPath.Moves, "A")
		*paths = append(*paths, newPath)
		return
	}

	visited[current] = true
	defer delete(visited, current)

	for _, dir := range Directions {
		nextPos := Position{
			Row: current.Row + dir.DeltaRow,
			Col: current.Col + dir.DeltaCol,
		}

		if k.isValidMove(nextPos) && !visited[nextPos] {
			currentPath.Moves = append(currentPath.Moves, dir.Move)
			k.findPathsDFS(nextPos, end, visited, currentPath, paths)
			currentPath.Moves = currentPath.Moves[:len(currentPath.Moves)-1]
		}
	}
}

func (k *Keypad) isValidMove(pos Position) bool {
	if pos.Row < 0 || pos.Row >= k.MaxRow || pos.Col < 0 || pos.Col >= k.MaxCol {
		return false
	}
	return k.ValidPos[pos]
}

func (k *Keypad) GetPositionForKey(key string) (Position, bool) {
	for row := range k.Layout {
		for col := range k.Layout[row] {
			if k.Layout[row][col] == key {
				return Position{row, col}, true
			}
		}
	}
	return Position{}, false
}

func parseInput(content string) []string {
	var codes []string
	lines := strings.Split(content, "\n")
	for _, line := range lines {
		if len(line) == 4 && strings.HasSuffix(line, "A") {
			codes = append(codes, line)
		}
	}
	return codes
}

type PathIndex struct {
	Paths map[string]int // key format: "fromKey-toKey"
}

func NewPathIndex(keypad *Keypad) *PathIndex {
	index := &PathIndex{
		Paths: make(map[string]int),
	}

	// For each valid position
	for fromPos := range keypad.ValidPos {
		fromKey := keypad.Layout[fromPos.Row][fromPos.Col]
		// Find paths to all other valid positions
		for toPos := range keypad.ValidPos {
			toKey := keypad.Layout[toPos.Row][toPos.Col]
			if fromKey != "" && toKey != "" {
				paths := keypad.FindAllPaths(fromPos, toPos)
				if len(paths) > 0 {
					// Only store the first shortest path
					index.Paths[fmt.Sprintf("%s-%s", fromKey, toKey)] = len(paths[0].Moves)
				}
			}
		}
	}

	return index
}

// TranslatedPath represents a path that's been translated through a layer
type TranslatedPath struct {
	Length int
}

// BuildLayerIndex builds a path index for a layer using the previous layer's index
func BuildLayerIndex(currentKeypad *Keypad, previousIndex *PathIndex) *PathIndex {
	newIndex := &PathIndex{
		Paths: make(map[string]int),
	}

	// For each possible key combination in current keypad
	for fromPos := range currentKeypad.ValidPos {
		fromKey := currentKeypad.Layout[fromPos.Row][fromPos.Col]
		for toPos := range currentKeypad.ValidPos {
			toKey := currentKeypad.Layout[toPos.Row][toPos.Col]
			if fromKey == "" || toKey == "" {
				continue
			}

			// Find all possible paths between these positions
			paths := currentKeypad.FindAllPaths(fromPos, toPos)
			// fmt.Printf("\nPaths from %s to %s: %v\n", fromKey, toKey, paths)
			if len(paths) > 0 {
				// Translate all paths and find the one that results in shortest sequence
				shortestLength := -1

				for _, path := range paths {
					translatedPath := translatePathThroughLayer(path, previousIndex)
					if shortestLength == -1 || translatedPath.Length < shortestLength {
						shortestLength = translatedPath.Length
					}
				}

				newIndex.Paths[fmt.Sprintf("%s-%s", fromKey, toKey)] = shortestLength
			} else {
				panic(fmt.Sprintf("No paths found from %s to %s", fromKey, toKey))
			}
		}
	}

	return newIndex
}

// translatePathThroughLayer translates a single path through a previous layer
func translatePathThroughLayer(path Path, previousIndex *PathIndex) TranslatedPath {
	if previousIndex == nil {
		// If no previous index, just return the direct path
		return TranslatedPath{
			Length: len(path.Moves),
		}
	}

	translatedPath := TranslatedPath{
		Length: 0,
	}

	moves := path.Moves
	if len(moves) == 0 {
		return translatedPath
	}

	// We stay on the A key
	if len(moves) == 1 {
		translatedPath.Length = 1
		return translatedPath
	}

	// As we always finish on the A key, it means we also always start on the A key
	previousMove := "A"

	for _, move := range moves {
		// Look up the path between current and next move
		if previousPath, exists := previousIndex.Paths[fmt.Sprintf("%s-%s", previousMove, move)]; exists {
			// fmt.Printf("Found path between %s and %s: %v\n", previousMove, move, previousPath)
			translatedPath.Length += previousPath
		} else {
			panic(fmt.Sprintf("No path found between %s and %s", previousMove, move))
		}
		previousMove = move
	}

	return translatedPath
}

func Part1(content string) (string, error) {
	// Parse the input codes
	codes := parseInput(content)

	// Create the keypads
	numericKeypad := NewKeypad(NumericKeypad)
	directionalKeypad := NewKeypad(DirectionalKeypad)

	// Build indices for each layer
	layer1Index := NewPathIndex(directionalKeypad)
	layer2Index := BuildLayerIndex(directionalKeypad, layer1Index)
	finalIndex := BuildLayerIndex(numericKeypad, layer2Index)

	totalComplexity := 0

	// Process each code
	for _, code := range codes {
		// Get numeric value for complexity calculation
		numStr := strings.TrimLeft(code[:len(code)-1], "0")
		numVal, _ := strconv.Atoi(numStr)

		// Find shortest path length for this code using final index
		pathLength := findShortestPathLengthForCode(code, finalIndex)
		fmt.Printf("Path length for %s: %d\n", code, pathLength)

		// Calculate and add complexity
		complexity := pathLength * numVal
		totalComplexity += complexity
	}

	return strconv.Itoa(totalComplexity), nil
}

func findShortestPathLengthForCode(code string, finalIndex *PathIndex) int {
	shortestLength := 0
	currentKey := "A" // Start from A

	// For each character in the code
	for _, ch := range code {
		targetKey := string(ch)
		if path, exists := finalIndex.Paths[fmt.Sprintf("%s-%s", currentKey, targetKey)]; exists {
			shortestLength += path
			currentKey = targetKey
		} else {
			panic(fmt.Sprintf("No path found between %s and %s", currentKey, targetKey))
		}
	}

	return shortestLength
}
