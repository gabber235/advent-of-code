package day20

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type Point struct {
	x, y int
}

func (p Point) Add(dir Point) Point {
	return Point{p.x + dir.x, p.y + dir.y}
}

type Maze struct {
	grid   [][]byte
	start  Point
	end    Point
	width  int
	height int
}

func parseInput(content string) Maze {
	lines := strings.Split(strings.TrimSpace(content), "\n")
	height := len(lines)
	width := len(lines[0])
	grid := make([][]byte, height)

	var start, end Point

	for y, line := range lines {
		grid[y] = make([]byte, width)
		for x, ch := range line {
			grid[y][x] = byte(ch)
			switch ch {
			case 'S':
				start = Point{x, y}
			case 'E':
				end = Point{x, y}
			}
		}
	}

	return Maze{
		grid:   grid,
		start:  start,
		end:    end,
		width:  width,
		height: height,
	}
}

func (m *Maze) isInBounds(p Point) bool {
	return p.x >= 0 && p.y >= 0 && p.x < m.width && p.y < m.height
}

func (m *Maze) isWall(p Point) bool {
	return m.grid[p.y][p.x] == '#'
}

func Part1(content string) (string, error) {
	maze := parseInput(content)

	cache := make(map[Point]int)

	maze.findPath(maze.start, maze.end, cache)

	cheatTimes := maze.findPathWithCheats(maze.start, maze.end, 1, cache)

	betterPaths := make(map[int]int, 0)
	for _, diff := range cheatTimes {
		betterPaths[diff] += 1
	}

	diffs := make([]int, 0, len(betterPaths))
	for diff := range betterPaths {
		diffs = append(diffs, diff)
	}

	slices.Sort(diffs)

	for _, diff := range diffs {
		fmt.Printf("There are %d cheats that save %d picoseconds\n", betterPaths[diff], diff)
	}

	atLeast100 := 0
	for diff, count := range betterPaths {
		if diff > 100 {
			atLeast100 += count
		}
	}

	return strconv.Itoa(atLeast100), nil
}

func (m *Maze) visualizeWithCache(cache map[Point]int) {
	// ANSI codes
	const (
		reset    = "\033[0m"
		red      = "\033[31m" // walls
		green    = "\033[32m" // shortest cached path
		yellow   = "\033[33m" // alternate cached paths
		cyan     = "\033[36m" // start/end
		clear    = "\033[H\033[2J"
		moveHome = "\033[H"
	)

	// Clear screen
	fmt.Print(clear)
	fmt.Print(moveHome)

	fmt.Println("\nMaze with Cache Visualization:")
	fmt.Println("Numbers show shortest path length to end from that point")

	// Print the maze with cache information
	for y := 0; y < m.height; y++ {
		// First line: maze representation
		for x := 0; x < m.width; x++ {
			p := Point{x, y}
			char := string(m.grid[y][x])

			switch {
			case p == m.start:
				fmt.Printf("%sS%s", cyan, reset)
			case p == m.end:
				fmt.Printf("%sE%s", cyan, reset)
			case char == "#":
				fmt.Printf("%s#%s", red, reset)
			case cache[p] > 0:
				fmt.Printf("%s•%s", green, reset) // single path
			default:
				fmt.Printf(".")
			}
		}
		fmt.Printf("   ") // spacing between maze and numbers

		// Second line: path lengths
		for x := 0; x < m.width; x++ {
			p := Point{x, y}
			if shortest, exists := cache[p]; exists {
				fmt.Printf("%3d", shortest)
			} else {
				fmt.Printf("   ")
			}
		}
		fmt.Println()
	}

	// Print cache details
	fmt.Println("\nDetailed Cache Information:")
	fmt.Printf("Total cached points: %d\n", len(cache))

	fmt.Println("\nLegend:")
	fmt.Printf("%sS%s/%sE%s - Start/End\n", cyan, reset, cyan, reset)
	fmt.Printf("%s•%s - Cached path\n", green, reset)
	fmt.Printf("%s#%s - Walls\n", red, reset)
	fmt.Println()
}
