package main

import (
	"fmt"
	"log"
	"math"

	"github.co/monodyle/aoc/helpers"
)

const (
	rlower = 'F'
	rupper = 'B'
	slower = 'L'
	supper = 'R'
)

func main() {
	var partOne int = 0 // highestID
	var partTwo int = 0
	var passes []map[string]int
	var lowestID int = 0

	err := helpers.ScanFile("./input", func(s string) error {
		var row int = findRow(s[:7])
		var seat int = findSeat(s[7:])
		var ID int = row*8 + seat
		if partOne < ID {
			partOne = ID
		}
		if lowestID != 0 {
			if lowestID > ID {
				lowestID = ID
			}
		} else {
			lowestID = ID
		}
		passes = append(passes, map[string]int{
			"row":  row,
			"seat": seat,
			"id":   ID,
		})
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	var total int = (partOne - lowestID + 1) * (lowestID + partOne) / 2
	//  ^ Arithmetic progression: https://en.wikipedia.org/wiki/Arithmetic_progression
	partTwo = total
	for _, pass := range passes {
		partTwo -= pass["id"]
	}

	fmt.Println("Part One:", partOne)
	fmt.Println("Part Two:", partTwo)
}

func middle(a, b int) int {
	return int(math.Floor(float64(a+b) / 2))
}

func findRow(s string) int {
	var l, h int = 0, 127
	for _, c := range s {
		if c == rlower {
			h = middle(l, h)
		} else {
			l = middle(l, h) + 1
		}
	}
	return l
}

func findSeat(s string) int {
	var l, h int = 0, 7
	for _, c := range s {
		if c == slower {
			h = middle(l, h)
		} else {
			l = middle(l, h) + 1
		}
	}
	return l
}
