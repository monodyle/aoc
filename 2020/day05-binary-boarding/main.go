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
	var passes []int
	var min int = 0

	err := helpers.ScanLines("./input", func(line string) error {
		var seatID int = getSeatID(line)
		if partOne < seatID {
			partOne = seatID
		}
		if min == 0 || min > seatID {
			min = seatID
		}
		passes = append(passes, seatID)
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	var total int = (partOne - min + 1) * (min + partOne) / 2
	//  ^ Arithmetic progression: https://en.wikipedia.org/wiki/Arithmetic_progression
	partTwo = total
	for _, seatID := range passes {
		partTwo -= seatID
	}

	fmt.Println("Part One:", partOne)
	fmt.Println("Part Two:", partTwo)
}

func getSeatID(s string) int {
	var row int = findRow(s[:7])
	var seat int = findSeat(s[7:])
	return row*8 + seat
}

func middle(a, b int) int {
	return int(math.Floor(float64(a+b) / 2))
}

func findRow(s string) int {
	var l, h int = 0, 127
	for _, c := range s {
		switch c {
		case rlower:
			h = middle(l, h)
		case rupper:
			l = middle(l, h) + 1
		}
	}
	return l
}

func findSeat(s string) int {
	var l, h int = 0, 7
	for _, c := range s {
		switch c {
		case slower:
			h = middle(l, h)
		case supper:
			l = middle(l, h) + 1
		}
	}
	return l
}
