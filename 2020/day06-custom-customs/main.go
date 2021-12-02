package main

import (
	"fmt"
	"log"

	"github.co/monodyle/aoc/2020/helpers"
)

func main() {
	var partOne int = 0
	var partTwo int = 0

	err := helpers.ScanGroups("./input", func(group []string) error {
		checker := make(map[rune]int, 26)

		// Part 1
		for _, decl := range group {
			for _, c := range decl {
				checker[c]++
			}
		}
		partOne += len(checker)

		// Part 2
		for _, c := range checker {
			if c == len(group) {
				partTwo++
			}
		}

		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", partOne)
	fmt.Println("Part Two:", partTwo)
}
