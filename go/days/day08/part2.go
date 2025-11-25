package day08

import (
	"slices"
	"strconv"
)

func Part2(content string) (string, error) {
	result, width, height := Parse(content)

	antinodes := make([]Point, 0)

	frequencies := make(map[string][]Point)
	for pos, freq := range result {
		frequencies[freq] = append(frequencies[freq], pos)
	}

	for _, points := range frequencies {
		for i := 0; i < len(points); i++ {
			for j := i + 1; j < len(points); j++ {
				point1, point2 := points[i], points[j]
				if point1.x == point2.x && point1.y == point2.y {
					continue
				}

				interference := ResonanceInterference(point1, point2, width, height)

				for _, antinode := range interference {
					if !slices.Contains(antinodes, antinode) {
						antinodes = append(antinodes, antinode)
					}
				}
			}
		}
	}

	Print(result, width, height, antinodes)

	return strconv.Itoa(len(antinodes)), nil
}

func ResonanceInterference(p1, p2 Point, width, height int) []Point {
	// Vector from p1 to p2
	dx := p2.x - p1.x
	dy := p2.y - p1.y

	antinodes := make([]Point, 0)
	antinodes = append(antinodes, p1)

	last := p1

	for {
		x := last.x - dx
		y := last.y - dy

		if x < 0 || x >= width || y < 0 || y >= height {
			break
		}

		last = Point{x, y}
		antinodes = append(antinodes, last)
	}

	for {
		x := last.x + dx
		y := last.y + dy

		if x < 0 || x >= width || y < 0 || y >= height {
			break
		}

		last = Point{x, y}
		antinodes = append(antinodes, last)
	}

	return antinodes
}
