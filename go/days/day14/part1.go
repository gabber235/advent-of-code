package day14

import (
	"strconv"
	"strings"
)

type Point struct {
	x, y int
}

type Robot struct {
	position Point
	velocity Point
}

func parseInput(input string) []Robot {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	robots := make([]Robot, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, " ")
		pos := strings.TrimPrefix(parts[0], "p=")
		vel := strings.TrimPrefix(parts[1], "v=")

		posCoords := strings.Split(pos, ",")
		velCoords := strings.Split(vel, ",")

		x, _ := strconv.Atoi(posCoords[0])
		y, _ := strconv.Atoi(posCoords[1])
		dx, _ := strconv.Atoi(velCoords[0])
		dy, _ := strconv.Atoi(velCoords[1])

		robots[i] = Robot{
			position: Point{x: x, y: y},
			velocity: Point{x: dx, y: dy},
		}
	}
	return robots
}

func moveRobot(robot *Robot, width, height int) {
	// Update position
	robot.position.x += robot.velocity.x
	robot.position.y += robot.velocity.y

	// Handle wrapping around edges
	robot.position.x = ((robot.position.x % width) + width) % width
	robot.position.y = ((robot.position.y % height) + height) % height
}

func countRobotsInQuadrants(robots []Robot, width, height int) int {
	// Initialize counters for each quadrant
	quadrants := make([]int, 4)
	midX := width / 2
	midY := height / 2

	for _, robot := range robots {
		x, y := robot.position.x, robot.position.y

		// Skip robots on middle lines
		if x == midX || y == midY {
			continue
		}

		// Determine quadrant (0: top-left, 1: top-right, 2: bottom-left, 3: bottom-right)
		quadIndex := 0
		if x > midX {
			quadIndex += 1
		}
		if y > midY {
			quadIndex += 2
		}

		quadrants[quadIndex]++
	}

	// Calculate safety factor (multiply all quadrant counts)
	safety := 1
	for _, count := range quadrants {
		safety *= count
	}

	return safety
}

func Part1(content string) (string, error) {
	width, height := 101, 103
	// width, height := 11, 7
	robots := parseInput(content)

	// Simulate 100 seconds
	for i := 0; i < 100; i++ {
		for j := range robots {
			moveRobot(&robots[j], width, height)
		}
	}

	// Calculate safety factor
	safety := countRobotsInQuadrants(robots, width, height)

	return strconv.Itoa(safety), nil
}
