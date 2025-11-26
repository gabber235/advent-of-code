package day08

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
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
				antinode1, antinode2 := Interference(point1, point2)
				// If the antinodes are inside the grid, add them to the list
				if antinode1.x >= 0 && antinode1.x < width && antinode1.y >= 0 && antinode1.y < height && !slices.Contains(antinodes, antinode1) {
					antinodes = append(antinodes, antinode1)
				}
				if antinode2.x >= 0 && antinode2.x < width && antinode2.y >= 0 && antinode2.y < height && !slices.Contains(antinodes, antinode2) {
					antinodes = append(antinodes, antinode2)
				}
			}
		}
	}

	fmt.Printf("Antinodes: %v\n", antinodes)

	Print(result, width, height, antinodes)

	return strconv.Itoa(len(antinodes)), nil
}

// Interference occurs at the extend of the line where the distance to the
// farthest point is double the distance to the closest point.
func Interference(p1, p2 Point) (Point, Point) {
	// Vector from p1 to p2
	dx := p2.x - p1.x
	dy := p2.y - p1.y

	x1 := p1.x - dx
	y1 := p1.y - dy
	x2 := p2.x + dx
	y2 := p2.y + dy

	return Point{x1, y1}, Point{x2, y2}
}

func Parse(content string) (map[Point]string, int, int) {
	result := make(map[Point]string)
	split := strings.Split(content, "\n")
	for y, row := range split {
		for x, char := range row {
			if char != '.' {
				result[Point{x, y}] = string(char)
			}
		}
	}

	width := len(split[0])
	height := len(split)

	return result, width, height
}

func Print(result map[Point]string, width, height int, antinodes []Point) {
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			if frequency, ok := result[Point{x, y}]; ok {
				fmt.Printf("%s", frequency)
			} else if slices.Contains(antinodes, Point{x, y}) {
				fmt.Printf("#")
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Println()
	}
}

type Point struct {
	x int
	y int
}
