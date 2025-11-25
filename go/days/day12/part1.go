package day12

import (
	"slices"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	grid := Parse(content)

	visited := make([]Point, 0)
	price := 0

	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[y]); x++ {
			if slices.Contains(visited, Point{x, y}) {
				continue
			}

			area := 0
			paremeter := 0

			todo := []Point{{x, y}}
			for len(todo) > 0 {
				p := todo[0]
				todo = todo[1:]
				visited = append(visited, p)
				area++

				letter := Letter(grid, p)

				neighbors := p.Neighbors()
				for _, n := range neighbors {
					if Letter(grid, n) == letter {
						if !slices.Contains(visited, n) && !slices.Contains(todo, n) {
							todo = append(todo, n)
						}
					} else {
						paremeter++
					}
				}
			}

			price += area * paremeter
		}
	}

	return strconv.Itoa(price), nil
}

func Letter(grid [][]string, p Point) string {
	if p.x < 0 || p.y < 0 || p.x >= len(grid[0]) || p.y >= len(grid) {
		return ""
	}
	return grid[p.y][p.x]
}

func (p Point) Neighbors() []Point {
	return []Point{
		{p.x - 1, p.y},
		{p.x + 1, p.y},
		{p.x, p.y - 1},
		{p.x, p.y + 1},
	}
}

func Parse(content string) [][]string {
	lines := strings.Split(content, "\n")
	result := make([][]string, len(lines))
	for y, line := range lines {
		result[y] = strings.Split(line, "")
	}

	return result
}

type Point struct {
	x int
	y int
}
