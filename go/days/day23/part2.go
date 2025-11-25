package day23

import (
	"sort"
	"strings"
)

func findPivot(vertices map[string]bool, connections map[string][]string) string {
	maxDegree := -1
	var pivot string

	for v := range vertices {
		degree := 0
		for _, neighbor := range connections[v] {
			if vertices[neighbor] {
				degree++
			}
		}
		if degree > maxDegree {
			maxDegree = degree
			pivot = v
		}
	}
	return pivot
}

func isConnected(v1, v2 string, connections map[string][]string) bool {
	for _, neighbor := range connections[v1] {
		if neighbor == v2 {
			return true
		}
	}
	return false
}

func bronKerbosch(r, p, x map[string]bool, connections map[string][]string, maxClique *[]string) {
	if len(p) == 0 && len(x) == 0 {
		// Found a maximal clique
		if len(r) > len(*maxClique) {
			*maxClique = make([]string, 0, len(r))
			for v := range r {
				*maxClique = append(*maxClique, v)
			}
		}
		return
	}

	var pivot string
	if len(p) > 0 {
		combinedSet := make(map[string]bool)
		for v := range p {
			combinedSet[v] = true
		}
		for v := range x {
			combinedSet[v] = true
		}
		pivot = findPivot(combinedSet, connections)
	} else {
		pivot = findPivot(x, connections)
	}

	pivotNeighbors := make(map[string]bool)
	for _, neighbor := range connections[pivot] {
		pivotNeighbors[neighbor] = true
	}

	vertices := make([]string, 0, len(p))
	for v := range p {
		if !pivotNeighbors[v] {
			vertices = append(vertices, v)
		}
	}

	for v := range p {
		if pivotNeighbors[v] {
			vertices = append(vertices, v)
		}
	}

	for _, v := range vertices {
		if len(r)+len(p) <= len(*maxClique) {
			return
		}

		rNew := make(map[string]bool)
		for vertex := range r {
			rNew[vertex] = true
		}
		rNew[v] = true

		pNew := make(map[string]bool)
		xNew := make(map[string]bool)

		for _, neighbor := range connections[v] {
			if p[neighbor] {
				pNew[neighbor] = true
			}
			if x[neighbor] {
				xNew[neighbor] = true
			}
		}

		bronKerbosch(rNew, pNew, xNew, connections, maxClique)

		delete(p, v)
		x[v] = true
	}
}

func Part2(content string) (string, error) {
	connections := buildGraph(content)

	r := make(map[string]bool) // Current clique
	p := make(map[string]bool) // Potential vertices
	x := make(map[string]bool) // Excluded vertices

	for vertex := range connections {
		p[vertex] = true
	}

	maxClique := make([]string, 0)
	bronKerbosch(r, p, x, connections, &maxClique)

	sort.Strings(maxClique)
	password := strings.Join(maxClique, ",")

	return password, nil
}
