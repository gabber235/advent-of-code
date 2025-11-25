package day18

import "fmt"

func Part2(content string) (string, error) {
	positions, err := Parse(content)
	if err != nil {
		return "", err
	}

	// We need to find the first position that when included blocks any path.
	// So we do a binary search on the index to see if we can or cannot find it.
	// The first index that does block the path is the one we want.
	// Then were report the position of that index.

	start := 0
	end := len(positions)
	for start < end {
		mid := (start + end) / 2

		corrupted := positions[:mid+1]
		width := 71
		height := 71

		position := positions[mid+1]
		fmt.Printf("Searching with %d corrupted positions, start: %d, end: %d, mid: %d, position: %v\n", len(corrupted), start, end, mid, position)
		node := findPath(corrupted, width, height)
		fmt.Printf("Found node: %v\n", node)
		if node == nil {
			end = mid
		} else {
			start = mid + 1
		}
	}

	fmt.Printf("Found start: %d, end: %d\n", start, end)

	position := positions[start]

	return fmt.Sprintf("%d,%d", position.X, position.Y), nil
}
