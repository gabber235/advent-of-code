package day20

import (
	"fmt"
	"slices"
	"strconv"
	"time"
)

func (m *Maze) findPath(start, end Point, cache map[Point]int) {
	point := start
	cache[start] = 0
	currentPath := make([]Point, 0)
	currentPath = append(currentPath, start)

	for point != end {
		point = m.findNextPathPoint(point, currentPath)

		for point, value := range cache {
			cache[point] = value + 1
		}

		cache[point] = 0
		currentPath = append(currentPath, point)
	}
}

func (m *Maze) findNextPathPoint(start Point, currentPath []Point) Point {
	dirs := []Point{
		{0, -1},
		{0, 1},
		{-1, 0},
		{1, 0},
	}

	for _, dir := range dirs {
		nextPoint := start.Add(dir)
		if !m.isInBounds(nextPoint) {
			continue
		}

		if slices.Contains(currentPath, nextPoint) {
			continue
		}

		if m.isWall(nextPoint) {
			continue
		}

		return nextPoint
	}
	panic("no path found")
}

func (m *Maze) visualizeCheatSearch(start Point, discovered map[Point]bool, queue []Point, endPoints map[Point]int, cache map[Point]int, iteration int) {
	const (
		reset    = "\033[0m"
		red      = "\033[31m" // worse/same points
		green    = "\033[32m" // better points
		blue     = "\033[34m" // current position
		cyan     = "\033[36m" // maze start/end
		gray     = "\033[90m" // walls
		yellow   = "\033[33m" // discovered walls
		clear    = "\033[H\033[2J"
		moveHome = "\033[H"
	)

	fmt.Print(clear, moveHome)
	fmt.Printf("\nCheat Search Visualization - Wave %d\n", iteration)
	fmt.Printf("Current time to end: %d\n", cache[start])

	for y := 0; y < m.height; y++ {
		// Main visualization
		for x := 0; x < m.width; x++ {
			p := Point{x, y}
			char := string(m.grid[y][x])

			switch {
			case p == m.start:
				fmt.Printf("%sS%s", cyan, reset)
			case p == m.end:
				fmt.Printf("%sE%s", cyan, reset)
			case p == start:
				fmt.Printf("%s@%s", blue, reset)
			case slices.Contains(queue, p):
				fmt.Printf("%s+%s", green, reset)
			case endPoints[p] > 0:
				fmt.Printf("%s*%s", green, reset)
			case discovered[p]:
				fmt.Printf("%s•%s", yellow, reset)
			case char == "#":
				fmt.Printf("%s#%s", gray, reset)
			default:
				fmt.Printf(".")
			}
		}

		fmt.Printf("   ")

		// Time values with maze structure and color-coded times
		for x := 0; x < m.width; x++ {
			p := Point{x, y}
			char := string(m.grid[y][x])

			switch {
			case p == m.start:
				fmt.Printf("%s S %s", cyan, reset)
			case p == m.end:
				fmt.Printf("%s E %s", cyan, reset)
			case p == start:
				fmt.Printf("%s @ %s", blue, reset)
			case char == "#":
				fmt.Printf("%s###%s", gray, reset)
			case endPoints[p] > 0:
				timeVal := endPoints[p]
				timeToEnd := cache[p]
				totalTime := timeToEnd + timeVal
				startTime := cache[start]

				if totalTime < startTime {
					// Better path - show in green
					fmt.Printf("%s%3d%s", green, timeVal, reset)
				} else {
					// Worse or same path - show in red
					fmt.Printf("%s%3d%s", red, timeVal, reset)
				}
			default:
				fmt.Printf("   ")
			}
		}

		// Add a third column showing the time savings
		fmt.Printf("   ")
		for x := 0; x < m.width; x++ {
			p := Point{x, y}
			if timeVal := endPoints[p]; timeVal > 0 {
				timeToEnd := cache[p]
				totalTime := timeToEnd + timeVal
				timeSaved := cache[start] - totalTime
				if timeSaved > 0 {
					fmt.Printf("%s%3d%s", green, timeSaved, reset)
				} else {
					fmt.Printf("%s%3d%s", red, timeSaved, reset)
				}
			} else {
				fmt.Printf("   ")
			}
		}
		fmt.Println()
	}

	betterPaths := 0
	worsePaths := 0
	for p, timeVal := range endPoints {
		timeToEnd := cache[p]
		totalTime := timeToEnd + timeVal
		if totalTime < cache[start] {
			betterPaths++
		} else {
			worsePaths++
		}
	}

	fmt.Println("\nWave Statistics:")
	fmt.Printf("Current wave: %d\n", iteration)
	fmt.Printf("Points in wave front: %d\n", len(queue))
	fmt.Printf("Total discovered walls: %d\n", len(discovered))
	fmt.Printf("Better paths found: %d\n", betterPaths)
	fmt.Printf("Worse/same paths found: %d\n", worsePaths)

	fmt.Println("\nLegend:")
	fmt.Printf("%sS%s/%sE%s - Maze start/end\n", cyan, reset, cyan, reset)
	fmt.Printf("%s@%s - Search start point\n", blue, reset)
	fmt.Printf("%s+%s - Current wave front\n", green, reset)
	fmt.Printf("%s•%s - Discovered walls\n", yellow, reset)
	fmt.Printf("%s*%s - Found exit points\n", green, reset)
	fmt.Printf("%s#%s - Walls\n", gray, reset)
	fmt.Printf("Numbers: %sgreen%s = faster path, %sred%s = slower path\n", green, reset, red, reset)
	fmt.Println("Right columns: 1) Time to traverse wall, 2) Total time saved")
	fmt.Println()

	time.Sleep(100 * time.Millisecond)
}

func (m *Maze) findCheatPoints(start Point, cheatTime int, cache map[Point]int) map[Point]int {
	endPoints := make(map[Point]int, 0)
	dirs := []Point{
		{0, -1},
		{0, 1},
		{-1, 0},
		{1, 0},
	}

	queue := make([]Point, 0)
	queue = append(queue, start)

	discovered := make(map[Point]bool)
	discovered[start] = true

	for time := 0; time < cheatTime; time++ {
		// m.visualizeCheatSearch(start, discovered, queue, endPoints, cache, time)
		newQueue := make([]Point, 0)
		for _, point := range queue {
			for _, dir := range dirs {
				nextPoint := point.Add(dir)
				if !m.isInBounds(nextPoint) {
					continue
				}
				if discovered[nextPoint] {
					continue
				}
				newQueue = append(newQueue, nextPoint)
				discovered[nextPoint] = true
				if !m.isWall(nextPoint) && endPoints[nextPoint] == 0 {
					endPoints[nextPoint] = time + 1
				}
			}
		}
		queue = newQueue
	}

	// m.visualizeCheatSearch(start, discovered, queue, endPoints, cache, cheatTime)
	// time.Sleep(2000 * time.Millisecond)

	return endPoints
}

func (m *Maze) findPathWithCheats(start, end Point, cheatTime int, cache map[Point]int) []int {
	currentPath := make([]Point, 0)
	currentPath = append(currentPath, start)
	point := start

	cheatTimes := make([]int, 0)

	for point != end {
		cheats := m.findCheatPoints(point, cheatTime, cache)
		startTime := cache[point]

		// m.visualizeCheatResults(point, cheats, cache, startTime)

		for endPoint, time := range cheats {
			timeLeft := cache[endPoint]
			totalTime := timeLeft + time
			diff := startTime - totalTime
			if diff <= 0 {
				continue
			}

			cheatTimes = append(cheatTimes, diff)
		}

		point = m.findNextPathPoint(point, currentPath)
		currentPath = append(currentPath, point)
	}
	return cheatTimes
}

func Part2(content string) (string, error) {
	maze := parseInput(content)

	cache := make(map[Point]int)

	maze.findPath(maze.start, maze.end, cache)

	// maze.visualizeWithCache(cache)

	cheatTimes := maze.findPathWithCheats(maze.start, maze.end, 20, cache)

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
		if diff >= 100 {
			atLeast100 += count
		}
	}

	return strconv.Itoa(atLeast100), nil
}

func (m *Maze) visualizeCheatResults(start Point, cheats map[Point]int, cache map[Point]int, startTime int) {
	const (
		reset    = "\033[0m"
		red      = "\033[31m" // worse/same cheat points
		green    = "\033[32m" // better cheat points
		blue     = "\033[34m" // current position
		cyan     = "\033[36m" // maze start/end
		gray     = "\033[90m" // walls
		clear    = "\033[H\033[2J"
		moveHome = "\033[H"
	)

	fmt.Print(clear, moveHome)
	fmt.Printf("\nCheat Points Analysis from position (%d,%d)\n", start.x, start.y)
	fmt.Printf("Current time to end: %d\n", startTime)

	betterCheats := 0
	worseCheats := 0

	// Draw the maze with walls and relevant points
	for y := 0; y < m.height; y++ {
		for x := 0; x < m.width; x++ {
			p := Point{x, y}
			char := string(m.grid[y][x])

			switch {
			case p == m.start:
				fmt.Printf("%sS%s", cyan, reset)
			case p == m.end:
				fmt.Printf("%sE%s", cyan, reset)
			case p == start:
				fmt.Printf("%s@%s", blue, reset)
			case cheats[p] > 0:
				timeToEnd := cache[p]
				timeTaken := cheats[p]
				totalTime := timeToEnd + timeTaken
				timeSaved := startTime - totalTime

				if timeSaved > 0 {
					if timeSaved < 10 {
						fmt.Printf("%s%d%s", green, timeSaved, reset)
					} else {
						fmt.Printf("%s*%s", green, reset)
					}
					betterCheats++
				} else {
					fmt.Printf("%sX%s", red, reset)
					worseCheats++
				}
			case char == "#":
				fmt.Printf("%s#%s", gray, reset)
			default:
				fmt.Printf(".")
			}
		}

		fmt.Println()
	}

	fmt.Println("\nStatistics:")
	fmt.Printf("Total cheat points found: %d\n", len(cheats))
	fmt.Printf("Better paths: %d\n", betterCheats)
	fmt.Printf("Worse/same paths: %d\n", worseCheats)

	fmt.Println("\nLegend:")
	fmt.Printf("%sS%s/%sE%s - Maze start/end\n", cyan, reset, cyan, reset)
	fmt.Printf("%s@%s - Current position\n", blue, reset)
	fmt.Printf("%s*%s - Better cheat point (saves >9 picoseconds)\n", green, reset)
	fmt.Printf("%s1-9%s - Better cheat point (saves 1-9 picoseconds)\n", green, reset)
	fmt.Printf("%sX%s - Worse/same cheat point\n", red, reset)
	fmt.Printf("%s#%s - Walls\n", gray, reset)
	fmt.Println("Numbers on right show exact time saved for each point")
	fmt.Println()

	time.Sleep(1 * time.Second)
}
