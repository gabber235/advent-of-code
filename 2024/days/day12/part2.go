package day12

import (
	"slices"
	"strconv"
)

func Part2(content string) (string, error) {
	grid := Parse(content)
	visited := make([]Point, 0)
	price := 0

	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[y]); x++ {
			if slices.Contains(visited, Point{x, y}) {
				continue
			}

			// Collect all points in the region
			region := make([]Point, 0)
			todo := []Point{{x, y}}
			letter := Letter(grid, Point{x, y})

			for len(todo) > 0 {
				p := todo[0]
				todo = todo[1:]

				if !slices.Contains(visited, p) {
					visited = append(visited, p)
					region = append(region, p)

					for _, n := range p.Neighbors() {
						if Letter(grid, n) == letter && !slices.Contains(visited, n) && !slices.Contains(todo, n) && !slices.Contains(region, n) {
							todo = append(todo, n)
						}
					}
				}
			}

			// Calculate area and number of sides
			area := len(region)
			sides := countSides(region, grid)
			price += area * sides
		}
	}

	return strconv.Itoa(price), nil
}

func countSides(region []Point, grid [][]string) int {
	regionPoints := make(map[Point]bool)
	for _, p := range region {
		regionPoints[p] = true
	}

	northSides := make(map[Point]bool)
	southSides := make(map[Point]bool)
	eastSides := make(map[Point]bool)
	westSides := make(map[Point]bool)

	// Generate side maps
	for _, p := range region {
		north := Point{p.x, p.y - 1}
		if !regionPoints[north] {
			northSides[p] = true
		}

		south := Point{p.x, p.y + 1}
		if !regionPoints[south] {
			southSides[p] = true
		}

		east := Point{p.x + 1, p.y}
		if !regionPoints[east] {
			eastSides[p] = true
		}

		west := Point{p.x - 1, p.y}
		if !regionPoints[west] {
			westSides[p] = true
		}
	}

	sides := 0

	// Count horizontal sides
	for y := 0; y < len(grid); y++ {
		inNorthSide := false
		inSouthSide := false

		for x := 0; x < len(grid[0]); x++ {
			p := Point{x, y}

			if northSides[p] {
				if !inNorthSide {
					sides++
					inNorthSide = true
				}
			} else {
				inNorthSide = false
			}

			if southSides[p] {
				if !inSouthSide {
					sides++
					inSouthSide = true
				}
			} else {
				inSouthSide = false
			}
		}
	}

	// Count vertical sides
	for x := 0; x < len(grid[0]); x++ {
		inEastSide := false
		inWestSide := false

		for y := 0; y < len(grid); y++ {
			p := Point{x, y}

			if eastSides[p] {
				if !inEastSide {
					sides++
					inEastSide = true
				}
			} else {
				inEastSide = false
			}

			if westSides[p] {
				if !inWestSide {
					sides++
					inWestSide = true
				}
			} else {
				inWestSide = false
			}
		}
	}

	return sides
}
