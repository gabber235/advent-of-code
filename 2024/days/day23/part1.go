package day23

import (
	"fmt"
	"sort"
	"strings"
)

func buildGraph(content string) map[string][]string {
	connections := make(map[string][]string)

	lines := strings.Split(strings.TrimSpace(content), "\n")

	for _, line := range lines {
		parts := strings.Split(line, "-")
		from, to := parts[0], parts[1]

		connections[from] = append(connections[from], to)
		connections[to] = append(connections[to], from)
	}

	return connections
}

func findTriangles(connections map[string][]string) [][]string {
	var triangles [][]string

	for node := range connections {
		neighbors := connections[node]
		for i := 0; i < len(neighbors); i++ {
			for j := i + 1; j < len(neighbors); j++ {
				n1, n2 := neighbors[i], neighbors[j]
				isConnected := false
				for _, n := range connections[n1] {
					if n == n2 {
						isConnected = true
						break
					}
				}

				if isConnected {
					triangle := []string{node, n1, n2}
					sort.Strings(triangle)
					triangles = append(triangles, triangle)
				}
			}
		}
	}

	seen := make(map[string]bool)
	var uniqueTriangles [][]string
	for _, triangle := range triangles {
		key := strings.Join(triangle, ",")
		if !seen[key] {
			seen[key] = true
			uniqueTriangles = append(uniqueTriangles, triangle)
		}
	}

	return uniqueTriangles
}

func Part1(content string) (string, error) {
	connections := buildGraph(content)

	triangles := findTriangles(connections)

	fmt.Printf("Found %d triangles\n\n", len(triangles))

	count := 0
	for _, triangle := range triangles {
		hasT := false
		for _, node := range triangle {
			if strings.HasPrefix(node, "t") {
				hasT = true
				break
			}
		}
		if hasT {
			count++
			fmt.Println(triangle)
		}
	}

	return fmt.Sprintf("%d", count), nil
}
