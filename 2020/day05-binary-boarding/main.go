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
	var result = []int{0, 0}

	err := helpers.ScanFile("./input", func(s string) error {
		var row = []int{0, 127}
		var seat = []int{0, 7}
		for i, c := range s {
			if i < 7 {
				if c == rlower {
					row[1] = int(math.Floor(float64(row[0]+row[1]) / 2))
				} else {
					row[0] = int(math.Round(float64(row[0]+row[1]) / 2))
				}
			} else {
				if c == slower {
					seat[1] = int(math.Floor(float64(seat[0]+seat[1]) / 2))
				} else {
					seat[0] = int(math.Round(float64(seat[0]+seat[1]) / 2))
				}
			}
		}

		ID := row[0]*8 + seat[0]
		if result[0] < ID {
			result[0] = ID
		}
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", result[0])
	fmt.Println("Part Two:", result[1])
}
