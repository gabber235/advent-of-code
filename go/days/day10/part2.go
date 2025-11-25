package day10

import (
	"strconv"
)

func Part2(content string) (string, error) {
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

	return strconv.Itoa(len(finishedTrails)), nil
}
