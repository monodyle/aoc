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
	err := helpers.ScanLines("./input", func(line string) error {
		row := make([]bool, 0, len(line))
		for _, block := range line {
			row = append(row, block == tree)
		}
		patterns = append(patterns, row)
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	var partOne int = mapFinder(patterns, Move{3, 1})
	fmt.Println("Part One:", partOne)
	fmt.Println("Part Two:", partOne*mapFinder(patterns, Move{3, 1})*mapFinder(patterns, Move{5, 1})*mapFinder(patterns, Move{7, 1})*mapFinder(patterns, Move{1, 2}))
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
