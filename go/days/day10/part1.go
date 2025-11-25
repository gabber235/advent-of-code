package day10

import (
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	grid, err := Parse(content)
	if err != nil {
		return "", err
	}

	finishedTrails := make([]Trail, 0)

	trailQueue := make([]Trail, 0)

	// Fill the trail queue with all 0 points
	for y, row := range grid {
		for x, cell := range row {
			if cell == 0 {
				point := Point{x, y}
				trailQueue = append(trailQueue, Trail{
					start:   point,
					current: point,
				})
			}
		}
	}

	for len(trailQueue) > 0 {
		nextTrail := trailQueue[0]
		trailQueue = trailQueue[1:]

		neighbours := ValidNeighbours(grid, nextTrail)
		for _, neighbour := range neighbours {
			if valueAt(grid, neighbour.current) == 9 {
				finishedTrails = append(finishedTrails, neighbour)
			}
			trailQueue = append(trailQueue, neighbour)
		}
	}

	// Filter out the trails that are duplicates
	finishedTrails = removeDuplicates(finishedTrails)

	return strconv.Itoa(len(finishedTrails)), nil
}

func ValidNeighbours(grid [][]int, trail Trail) []Trail {
	x := trail.current.x
	y := trail.current.y

	neighbours := make([]Trail, 0)

	points := []Point{
		{x - 1, y},
		{x + 1, y},
		{x, y - 1},
		{x, y + 1},
	}

	currentValue := valueAt(grid, trail.current)
	for _, point := range points {
		if valueAt(grid, point) == currentValue+1 {
			neighbours = append(neighbours, Trail{
				start:   trail.start,
				current: point,
			})
		}
	}

	return neighbours
}

func valueAt(grid [][]int, point Point) int {
	y := point.y
	x := point.x

	if x < 0 || x >= len(grid[0]) {
		return -1
	}
	if y < 0 || y >= len(grid) {
		return -1
	}
	return grid[point.y][point.x]
}

func removeDuplicates(trails []Trail) []Trail {
	seen := make(map[Trail]bool)
	uniqueTrails := make([]Trail, 0)
	for _, trail := range trails {
		if !seen[trail] {
			uniqueTrails = append(uniqueTrails, trail)
			seen[trail] = true
		}
	}
	return uniqueTrails
}

func Parse(content string) ([][]int, error) {
	lines := strings.Split(content, "\n")
	result := make([][]int, len(lines))
	for i, line := range lines {
		result[i] = make([]int, len(line))
		for j, char := range line {
			result[i][j], _ = strconv.Atoi(string(char))
		}
	}
	return result, nil
}

type Point struct {
	x int
	y int
}

type Trail struct {
	start   Point
	current Point
}
