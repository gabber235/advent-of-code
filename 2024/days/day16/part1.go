package day16

import (
	"container/heap"
	"fmt"
	"strconv"
	"strings"
	"time"
)

type Direction int

const (
	East Direction = iota
	South
	West
	North
)

type Point struct {
	X, Y int
}

type Maze struct {
	Grid   [][]rune
	Start  Point
	End    Point
	Height int
	Width  int
}

func ParseInput(input string) (*Maze, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	maze := &Maze{
		Grid:   make([][]rune, len(lines)),
		Height: len(lines),
	}

	if len(lines) > 0 {
		maze.Width = len(lines[0])
	}

	for y, line := range lines {
		maze.Grid[y] = make([]rune, len(line))
		for x, char := range line {
			maze.Grid[y][x] = char

			switch char {
			case 'S':
				maze.Start = Point{X: x, Y: y}
			case 'E':
				maze.End = Point{X: x, Y: y}
			}
		}
	}

	return maze, nil
}

func (m *Maze) IsWall(p Point) bool {
	if p.Y < 0 || p.Y >= m.Height || p.X < 0 || p.X >= m.Width {
		return true
	}
	return m.Grid[p.Y][p.X] == '#'
}

func (m *Maze) IsInBounds(p Point) bool {
	return p.Y >= 0 && p.Y < m.Height && p.X >= 0 && p.X < m.Width
}

type Node struct {
	Point     Point
	Parent    *Node
	Cost      int
	Direction Direction
}

func (n *Node) Neighbors(maze *Maze) []*Node {
	var neighbors []*Node
	switch n.Direction {
	case East:
		neighbors = addNeighbor(neighbors, *n, n.Cost+1, maze, East)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, South)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, North)
		neighbors = addNeighbor(neighbors, *n, n.Cost+2001, maze, West)
	case South:
		neighbors = addNeighbor(neighbors, *n, n.Cost+1, maze, South)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, East)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, West)
		neighbors = addNeighbor(neighbors, *n, n.Cost+2001, maze, North)
	case West:
		neighbors = addNeighbor(neighbors, *n, n.Cost+1, maze, West)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, North)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, South)
		neighbors = addNeighbor(neighbors, *n, n.Cost+2001, maze, East)
	case North:
		neighbors = addNeighbor(neighbors, *n, n.Cost+1, maze, North)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, East)
		neighbors = addNeighbor(neighbors, *n, n.Cost+1001, maze, West)
		neighbors = addNeighbor(neighbors, *n, n.Cost+2001, maze, South)
	}
	return neighbors
}

func addNeighbor(neighbors []*Node, parent Node, cost int, maze *Maze, direction Direction) []*Node {
	var node *Node
	switch direction {
	case East:
		p := Point{X: parent.Point.X + 1, Y: parent.Point.Y}
		node = &Node{Point: p, Parent: &parent, Cost: cost, Direction: East}
	case South:
		p := Point{X: parent.Point.X, Y: parent.Point.Y + 1}
		node = &Node{Point: p, Parent: &parent, Cost: cost, Direction: South}
	case West:
		p := Point{X: parent.Point.X - 1, Y: parent.Point.Y}
		node = &Node{Point: p, Parent: &parent, Cost: cost, Direction: West}
	case North:
		p := Point{X: parent.Point.X, Y: parent.Point.Y - 1}
		node = &Node{Point: p, Parent: &parent, Cost: cost, Direction: North}
	}
	if maze.IsWall(node.Point) {
		return neighbors
	}
	return append(neighbors, node)
}

func Part1(content string) (string, error) {
	maze, err := ParseInput(content)
	if err != nil {
		return "", err
	}

	startNode := &Node{Point: maze.Start, Parent: nil, Cost: 0, Direction: East}
	result := Dijkstra(maze, startNode)

	return strconv.Itoa(result), nil
}

// ANSI color codes
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

var DirectionSymbol = map[Direction]string{
	East:  "→",
	South: "↓",
	West:  "←",
	North: "↑",
}

func (m *Maze) Visualize(visited map[string]*Node, current *Node) {
	fmt.Print("\033[H\033[2J")

	for y := 0; y < m.Height; y++ {
		for x := 0; x < m.Width; x++ {
			p := Point{X: x, Y: y}

			if current != nil && current.Point == p {
				fmt.Print(Red + DirectionSymbol[current.Direction] + Reset)
				continue
			}

			found := false
			for stateKey := range visited {
				var px, py int
				fmt.Sscanf(stateKey, "%d,%d", &px, &py)
				if px == x && py == y {
					fmt.Print(Blue + "·" + Reset)
					found = true
					break
				}
			}
			if found {
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
	time.Sleep(10 * time.Millisecond)
}

func (n *Node) Serialize() string {
	return fmt.Sprintf("%d,%d,%d", n.Point.X, n.Point.Y, n.Direction)
}

func Dijkstra(maze *Maze, startNode *Node) int {
	queue := make(PriorityQueue, 0)
	heap.Init(&queue)
	heap.Push(&queue, &Item{node: startNode, priority: 0})

	visited := make(map[string]*Node)
	costs := make(map[string]int)

	for queue.Len() > 0 {
		item := heap.Pop(&queue).(*Item)
		current := item.node

		maze.Visualize(visited, current)

		if current.Point == maze.End {
			maze.Visualize(visited, current)
			return current.Cost
		}

		neighbors := current.Neighbors(maze)
		for _, neighbor := range neighbors {
			stateKey := neighbor.Serialize()
			if cost, exists := costs[stateKey]; !exists || neighbor.Cost < cost {
				costs[stateKey] = neighbor.Cost
				visited[stateKey] = neighbor
				heap.Push(&queue, &Item{node: neighbor, priority: neighbor.Cost})
			}
		}
	}

	return -1
}

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
