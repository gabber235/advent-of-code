package day18

import (
	"container/heap"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func Part1(content string) (string, error) {
	positions, err := Parse(content)
	if err != nil {
		return "", err
	}

	corrupted := positions[:1024]
	width := 71
	height := 71

	Visualize(nil, width, height, corrupted)

	node := findPath(corrupted, width, height)
	Visualize(node, width, height, corrupted)
	if node == nil {
		return "not found", nil
	}
	steps := node.Cost
	return fmt.Sprintf("%d", steps), nil
}

const (
	Reset  = "\033[0m"
	Red    = "\033[31m"
	Green  = "\033[32m"
	Yellow = "\033[33m"
	Blue   = "\033[34m"
	Purple = "\033[35m"
	Cyan   = "\033[36m"
	Gray   = "\033[37m"
)

func Visualize(node *Node, width int, height int, corrupted []Position) {
	path := make([]Position, 0)
	for node != nil {
		path = append(path, *node.Position)
		node = node.Parent
	}
	fmt.Print("\033[H\033[2J")

	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			p := Position{X: x, Y: y}
			if slices.Contains(corrupted, p) {
				fmt.Print(Red + "#" + Reset)
				continue
			}
			if slices.Contains(path, p) {
				fmt.Print(Blue + "·" + Reset)
				continue
			}

			fmt.Print(Gray + "." + Reset)
		}
		fmt.Println()
	}
}

func findPath(corrupted []Position, width int, height int) *Node {
	// Run Dijkstra's algorithm from 0,0 to width-1,height-1
	queue := make(PriorityQueue, 0)
	heap.Init(&queue)
	heap.Push(&queue, &Item{node: &Node{Position: &Position{X: 0, Y: 0}, Parent: nil, Cost: 0}, priority: 0})

	visited := make(map[string]*Position)
	costs := make(map[string]int)

	for queue.Len() > 0 {
		item := heap.Pop(&queue).(*Item)
		current := item.node

		if current.Position.X == width-1 && current.Position.Y == height-1 {
			return item.node
		}

		neighbors := current.Neighbors(corrupted, width, height)
		for _, neighbor := range neighbors {
			stateKey := neighbor.Serialize()
			if cost, exists := costs[stateKey]; !exists || neighbor.Cost < cost {
				costs[stateKey] = neighbor.Cost
				visited[stateKey] = neighbor.Position
				heap.Push(&queue, &Item{node: neighbor, priority: neighbor.Cost})
			}
		}
	}

	return nil
}

func (n *Node) Neighbors(corrupted []Position, width int, height int) []*Node {
	var neighbors []*Node
	points := []Position{
		{X: n.Position.X - 1, Y: n.Position.Y},
		{X: n.Position.X + 1, Y: n.Position.Y},
		{X: n.Position.X, Y: n.Position.Y - 1},
		{X: n.Position.X, Y: n.Position.Y + 1},
	}
	for _, point := range points {
		if point.X < 0 || point.X >= width || point.Y < 0 || point.Y >= height {
			continue
		}
		if slices.Contains(corrupted, point) {
			continue
		}
		neighbors = append(neighbors, &Node{Position: &point, Parent: n, Cost: n.Cost + 1})
	}
	return neighbors
}

func (n *Node) Serialize() string {
	return fmt.Sprintf("%d,%d", n.Position.X, n.Position.Y)
}

func Parse(content string) ([]Position, error) {
	positions := make([]Position, 0)
	for _, line := range strings.Split(content, "\n") {
		parts := strings.Split(line, ",")
		if len(parts) != 2 {
			return nil, fmt.Errorf("invalid line: %s", line)
		}
		x, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, fmt.Errorf("invalid x: %s", parts[0])
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, fmt.Errorf("invalid y: %s", parts[1])
		}
		positions = append(positions, Position{x, y})
	}
	return positions, nil
}

type Position struct {
	X int
	Y int
}

type Node struct {
	Position *Position
	Parent   *Node
	Cost     int
}

type Space int

type Item struct {
	node     *Node
	priority int
	index    int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}
