package main

import (
	"fmt"
	"log"

	"github.co/monodyle/aoc/helpers"
)

// Move of toboggan on map
type Move struct {
	right int
	down  int
}

func main() {
	const tree = '#'
	var patterns [][]bool
	err := helpers.ScanFile("./input", func(s string) error {
		row := make([]bool, 0, len(s))
		for _, block := range s {
			row = append(row, block == tree)
		}
		patterns = append(patterns, row)
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", mapFinder(patterns, Move{3, 1}))
}

func mapFinder(p [][]bool, m Move) int {
	var x, y, count int = 0, 0, 0
	for y < len(p) {
		if p[y][x] {
			count++
		}
		x += m.right
		x %= len(p[0])
		y += m.down
	}
	return count
}
