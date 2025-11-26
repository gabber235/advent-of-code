package day16

import (
	"container/heap"
	"fmt"
	"slices"
	"strconv"
)

func Part2(content string) (string, error) {
	maze, err := ParseInput(content)
	if err != nil {
		return "", err
	}

	startNode := &Node{Point: maze.Start, Parent: nil, Cost: 0, Direction: East}
	nodes := DijkstraAll(maze, startNode)

	points := make([]Point, 0)
	for _, node := range nodes {
		current := node
		for current != nil {
			if !slices.Contains(points, current.Point) {
				points = append(points, current.Point)
			}
			current = current.Parent
		}
	}

	maze.VisualizeEndings(points)

	return strconv.Itoa(len(points)), nil
}

func (m *Maze) VisualizeEndings(points []Point) {
	fmt.Print("\033[H\033[2J")

	for y := 0; y < m.Height; y++ {
		for x := 0; x < m.Width; x++ {
			p := Point{X: x, Y: y}

			if slices.Contains(points, p) {
				fmt.Print(Blue + "O" + Reset)
				continue
			}

			switch m.Grid[y][x] {
			case 'S':
				fmt.Print(Green + "S" + Reset)
			case 'E':
				fmt.Print(Yellow + "E" + Reset)
			case '#':
				fmt.Print(Gray + "#" + Reset)
			default:
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func DijkstraAll(maze *Maze, startNode *Node) []*Node {
	queue := make(PriorityQueue, 0)
	heap.Init(&queue)
	heap.Push(&queue, &Item{node: startNode, priority: 0})

	visited := make(map[string]*Node)
	costs := make(map[string]int)

	endNodes := make([]*Node, 0)
	maxCost := int(1e9)

	for queue.Len() > 0 {
		item := heap.Pop(&queue).(*Item)
		current := item.node

		if current.Point == maze.End {
			if current.Cost == maxCost {
				endNodes = append(endNodes, current)
			}
			if current.Cost < maxCost {
				endNodes = make([]*Node, 0)
				endNodes = append(endNodes, current)
				maxCost = current.Cost
			}
			continue
		}

		neighbors := current.Neighbors(maze)
		for _, neighbor := range neighbors {
			stateKey := neighbor.Serialize()
			if cost, exists := costs[stateKey]; !exists || neighbor.Cost <= cost {
				if neighbor.Cost > maxCost {
					continue
				}
				costs[stateKey] = neighbor.Cost
				visited[stateKey] = neighbor
				heap.Push(&queue, &Item{node: neighbor, priority: neighbor.Cost})
			}
		}
	}

	return endNodes
}
